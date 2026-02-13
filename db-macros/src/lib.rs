use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Error, Fields, Lit,
    LitStr, Type,
};
use syn::meta::ParseNestedMeta;

#[proc_macro_derive(Entity, attributes(id, field, table))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // 解析表名
    let table_name = parse_table_name(&input.attrs)
        .unwrap_or_else(|| format!("t_{}", name.to_string().to_lowercase()));
    let table_name_lit = LitStr::new(&table_name, proc_macro2::Span::call_site());

    // 解析字段信息
    let fields_info = match &input.data {
        Data::Struct(data) => parse_fields(&data.fields),
        _ => panic!("Entity only supports structs"),
    };

    // 校验：所有字段必须是 Option 类型
    let mut errors = Vec::new();
    for field in &fields_info {
        if !field.is_option {
            let error = Error::new(
                field.field_span,
                format!(
                    "Field `{}` must be of type `Option<T>`, found `{}`",
                    field.field_name,
                    field.original_type_name
                ),
            );
            errors.push(error.to_compile_error());
        }
    }

    // 如果有错误，返回所有编译错误
    if !errors.is_empty() {
        return TokenStream::from(quote! {
            #(#errors)*
        });
    }

    // 找出主键字段
    let id_field = fields_info
        .iter()
        .find(|f| f.is_id)
        .expect("One field must be marked with #[id]");

    // ============ 标识符（用于字段访问） ============
    let id_field_ident = format_ident!("{}", &id_field.field_name);

    // ============ 字符串字面量（用于返回值） ============
    let id_field_name_lit = LitStr::new(&id_field.field_name, proc_macro2::Span::call_site());
    let id_column_name_lit = LitStr::new(&id_field.column_name, proc_macro2::Span::call_site());

    // 字段名列表 - 字符串字面量
    let field_names: Vec<_> = fields_info
        .iter()
        .map(|f| {
            LitStr::new(&f.field_name, proc_macro2::Span::call_site())
        })
        .collect();

    // 列名列表 - 字符串字面量
    let column_names: Vec<_> = fields_info
        .iter()
        .map(|f| {
            LitStr::new(&f.column_name, proc_macro2::Span::call_site())
        })
        .collect();

    // 字段初始化 - 标识符
    let field_inits = fields_info.iter().map(|f| {
        let name = format_ident!("{}", &f.field_name);
        quote! { #name: None }
    });

    // 生成 ColumnInfo 列表
    let column_infos = generate_column_infos(&fields_info);

    // 生成 get_value 的 match 分支
    let get_value_by_field_arms = generate_get_value_arms(&fields_info, true);
    let get_value_by_column_arms = generate_get_value_arms(&fields_info, false);

    // 生成 set_value 的 match 分支
    let set_value_by_field_arms = generate_set_value_arms(&fields_info, true);
    let set_value_by_column_arms = generate_set_value_arms(&fields_info, false);

    // 主键类型（从 Option 中提取）
    let key_type = extract_inner_type(&id_field.field_type);

    let expanded = quote! {
        impl Entity for #name {
            type K = #key_type;

            fn key(&self) -> Self::K {
                self.#id_field_ident.clone().unwrap_or_default()
            }

            fn key_name() -> &'static str {
                #id_column_name_lit
            }

            fn column_names() -> Vec<&'static str> {
                vec![#(#column_names),*]
            }

            fn field_names() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }

            fn table_name() -> &'static str {
                #table_name_lit
            }

            fn new() -> Self {
                Self {
                    #(#field_inits),*
                }
            }

            fn get_value_by_field_name(&self, field_name: &str) -> ParamValue {
                match field_name {
                    #(#get_value_by_field_arms)*
                    _ => ParamValue::Null,
                }
            }

            fn get_value_by_column_name(&self, column_name: &str) -> ParamValue {
                match column_name {
                    #(#get_value_by_column_arms)*
                    _ => ParamValue::Null,
                }
            }

            fn set_value_by_field_name(&mut self, field_name: &str, value: ParamValue) {
                match field_name {
                    #(#set_value_by_field_arms)*
                    _ => rustlog::error!("Field name not found: {}", field_name),
                }
            }

            fn set_value_by_column_name(&mut self, column_name: &str, value: ParamValue) {
                match column_name {
                    #(#set_value_by_column_arms)*
                    _ => rustlog::error!("Column name not found: {}", column_name),
                }
            }

            fn get_column_infos() -> Vec<ColumnInfo> {
                vec![#(#column_infos),*]
            }
        }
    };

    TokenStream::from(expanded)
}




// 字段信息结构体
struct FieldInfo {
    field_name: String,
    column_name: String,
    field_type: Type,
    original_type_name: String,
    field_span: proc_macro2::Span,
    is_option: bool,
    inner_type: Option<Type>,
    is_id: bool,
    is_auto_increment: bool,
    is_nullable: bool,
    key_generate_type: Option<String>,
    fill_on_update: bool,
    fill_on_insert: bool,
}

// 解析表名
fn parse_table_name(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("table") {
            let mut table_name = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        let v = lit_str.value(); // 这里只是演示，实际需要存储
                        table_name = Some(v);
                    }
                }
                Ok(())
            });
            return table_name;
        }
    }
    None
}

// 判断是否为 Option 类型
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

// 提取 Option 的内部类型
fn extract_inner_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty.clone();
                    }
                }
            }
        }
    }
    ty.clone()
}

// 获取类型名称（用于错误信息）
fn get_type_name(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        let segments = &type_path.path.segments;
        if !segments.is_empty() {
            let last = segments.last().unwrap();
            let mut name = last.ident.to_string();

            if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                let args_str: Vec<String> = args.args.iter().map(|arg| {
                    if let syn::GenericArgument::Type(ty) = arg {
                        get_type_name(ty)
                    } else {
                        "_".to_string()
                    }
                }).collect();
                if !args_str.is_empty() {
                    name.push_str(&format!("<{}>", args_str.join(", ")));
                }
            }
            return name;
        }
    }
    "_".to_string()
}
fn parse_fields(fields: &Fields) -> Vec<FieldInfo> {
    let mut fields_info = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();

        // 基础字段信息
        let mut field_info = FieldInfo {
            field_name: field_name.clone(),
            column_name: field_name,
            field_type: field.ty.clone(),
            original_type_name: get_type_name(&field.ty),
            field_span: field.span(),
            is_option: is_option_type(&field.ty),
            inner_type: if is_option_type(&field.ty) {
                Some(extract_inner_type(&field.ty))
            } else {
                None
            },
            is_id: false,
            is_auto_increment: false,
            is_nullable: true,
            key_generate_type: None,
            fill_on_update: false,
            fill_on_insert: false,
        };

        // 解析所有属性
        for attr in &field.attrs {
            if attr.path().is_ident("id") {
                parse_id_attributes(attr, &mut field_info);
            } else if attr.path().is_ident("field") {
                parse_field_attributes(attr, &mut field_info);
            }
        }

        fields_info.push(field_info);
    }

    fields_info
}

fn parse_id_attributes(attr: &Attribute, info: &mut FieldInfo) {
    info.is_id = true;
    info.is_nullable = false;

    if attr.meta.require_list().is_err() {
        return;
    }

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("column") {
            info.column_name = parse_string_lit(&meta)?;
        } else if meta.path.is_ident("auto_increment") {
            info.is_auto_increment = parse_bool_lit(&meta)?;
        } else if meta.path.is_ident("key_generate_type") {
            info.key_generate_type = Some(parse_string_lit(&meta)?);
        }
        Ok(())
    });
}

fn parse_field_attributes(attr: &Attribute, info: &mut FieldInfo) {
    if attr.meta.require_list().is_err() {
        return;
    }

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("column") {
            info.column_name = parse_string_lit(&meta)?;
        } else if meta.path.is_ident("nullable") {
            info.is_nullable = parse_bool_lit(&meta)?;
        } else if meta.path.is_ident("fill_on_update") {
            info.fill_on_update = parse_bool_lit(&meta)?;
        } else if meta.path.is_ident("fill_on_insert") {
            info.fill_on_insert = parse_bool_lit(&meta)?;
        } else if meta.path.is_ident("key_generate_type") {
            info.key_generate_type = Some(parse_string_lit(&meta)?);
        }
        Ok(())
    });
}

// 辅助函数：解析字符串字面量
fn parse_string_lit(meta: &ParseNestedMeta) -> Result<String, syn::Error> {
    let value = meta.value()?;
    let lit: Lit = value.parse()?;
    match lit {
        Lit::Str(lit_str) => Ok(lit_str.value()),
        _ => Err(meta.error("expected string literal")),
    }
}

// 辅助函数：解析布尔字面量
fn parse_bool_lit(meta: &ParseNestedMeta) -> Result<bool, syn::Error> {
    let value = meta.value()?;
    let lit: Lit = value.parse()?;
    match lit {
        Lit::Bool(lit_bool) => Ok(lit_bool.value()),
        _ => Err(meta.error("expected boolean literal")),
    }
}
// 解析字段属性
// fn parse_fields(fields: &Fields) -> Vec<FieldInfo> {
//     let mut fields_info = Vec::new();
//
//     for field in fields.iter() {
//         let field_name = field.ident.as_ref().unwrap().to_string();
//         let mut column_name = field_name.clone();
//         let mut is_id = false;
//         let mut is_auto_increment = false;
//         let mut is_nullable = true;
//
//         let field_type = field.ty.clone();
//         let field_span = field.span();
//         let is_option = is_option_type(&field_type);
//         let inner_type = if is_option {
//             Some(extract_inner_type(&field_type))
//         } else {
//             None
//         };
//         let original_type_name = get_type_name(&field_type);
//
//         // 解析字段属性
//         for attr in &field.attrs {
//             if attr.path().is_ident("id") {
//                 is_id = true;
//                 is_nullable = false;
//
//                 // 解析 id 属性参数
//                 if attr.meta.require_list().is_ok() {
//                     let _ = attr.parse_nested_meta(|meta| {
//                         if meta.path.is_ident("column") {
//                             let value = meta.value()?;
//                             let lit: Lit = value.parse()?;
//                             if let Lit::Str(lit_str) = lit {
//                                 column_name = lit_str.value();
//                             }
//                         } else if meta.path.is_ident("auto_increment") {
//                             let value = meta.value()?;
//                             let lit: Lit = value.parse()?;
//                             if let Lit::Bool(lit_bool) = lit {
//                                 is_auto_increment = lit_bool.value();
//                             }
//                         }
//                         Ok(())
//                     });
//                 }
//             } else if attr.path().is_ident("field") {
//                 // 解析 field 属性参数
//                 if attr.meta.require_list().is_ok() {
//                     let _ = attr.parse_nested_meta(|meta| {
//                         if meta.path.is_ident("column") {
//                             let value = meta.value()?;
//                             let lit: Lit = value.parse()?;
//                             if let Lit::Str(lit_str) = lit {
//                                 column_name = lit_str.value();
//                             }
//                         } else if meta.path.is_ident("nullable") {
//                             let value = meta.value()?;
//                             let lit: Lit = value.parse()?;
//                             if let Lit::Bool(lit_bool) = lit {
//                                 is_nullable = lit_bool.value();
//                             }
//                         }
//                         Ok(())
//                     });
//                 }
//             }
//         }
//
//         fields_info.push(FieldInfo {
//             field_name,
//             column_name,
//             field_type,
//             original_type_name,
//             field_span,
//             is_option,
//             inner_type,
//             is_id,
//             is_auto_increment,
//             is_nullable,
//         });
//     }
//
//     fields_info
// }

// 生成 ColumnInfo
fn generate_column_infos(fields_info: &[FieldInfo]) -> Vec<proc_macro2::TokenStream> {
    fields_info
        .iter()
        .map(|f| {
            let field_name_lit = LitStr::new(&f.field_name, proc_macro2::Span::call_site());
            let column_name_lit = LitStr::new(&f.column_name, proc_macro2::Span::call_site());

            let is_nullable = f.is_nullable;
            let is_auto_increment = f.is_auto_increment;
            let is_id = f.is_id;
            let fill_on_update = f.fill_on_update;
            let fill_on_insert = f.fill_on_insert;

            let ty = f.inner_type.as_ref().unwrap_or(&f.field_type);
            let column_type = infer_column_type(ty);

            // 主键生成策略（保持为 Option<String>）
            let key_generate_type = match &f.key_generate_type {
                Some(s) => {
                    let s_lit = LitStr::new(s, proc_macro2::Span::call_site());
                    quote! { Some(#s_lit.to_string()) }
                }
                None => quote! { None },
            };

            quote! {
                ColumnInfo {
                    field_name: #field_name_lit,
                    column_name: #column_name_lit,
                    column_type: #column_type,
                    is_nullable: #is_nullable,
                    is_auto_increment: #is_auto_increment,
                    is_primary_key: #is_id,
                    key_generate_type: #key_generate_type.into(),
                    fill_on_update: #fill_on_update,
                    fill_on_insert: #fill_on_insert,
                }
            }
        })
        .collect()
}

// 推断列类型
fn infer_column_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            match segment.ident.to_string().as_str() {
                "i8" => quote! { ColumnType::I8 },
                "i16" => quote! { ColumnType::I16 },
                "i32" => quote! { ColumnType::I32 },
                "i64" => quote! { ColumnType::I64 },
                "u8" => quote! { ColumnType::U8 },
                "u16" => quote! { ColumnType::U16 },
                "u32" => quote! { ColumnType::U32 },
                "u64" => quote! { ColumnType::U64 },
                "usize" => quote! { ColumnType::USize },
                "f32" => quote! { ColumnType::F32 },
                "f64" => quote! { ColumnType::F64 },
                "bool" => quote! { ColumnType::Bool },
                "String" => quote! { ColumnType::String },
                "DateTime" => quote! { ColumnType::DateTime },
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            if let Type::Path(inner_path) = inner {
                                if let Some(inner_seg) = inner_path.path.segments.last() {
                                    if inner_seg.ident == "u8" {
                                        return quote! { ColumnType::Blob };
                                    }
                                }
                            }
                        }
                    }
                    quote! { ColumnType::Clob }
                }
                _ => quote! { ColumnType::Null },
            }
        }
        _ => quote! { ColumnType::Null },
    }
}

// 生成 get_value 的 match 分支
fn generate_get_value_arms(fields_info: &[FieldInfo], use_field_name: bool) -> Vec<proc_macro2::TokenStream> {
    fields_info
        .iter()
        .map(|f| {
            // ✅ match 模式：字符串字面量
            let pattern = if use_field_name {
                LitStr::new(&f.field_name, proc_macro2::Span::call_site())
            } else {
                LitStr::new(&f.column_name, proc_macro2::Span::call_site())
            };

            // ✅ 字段访问：标识符
            let field_ident = format_ident!("{}", &f.field_name);

            let conversion = quote!{
                {if self.#field_ident.is_none(){
                    ParamValue::Null
                }else{
                    self.#field_ident.clone().unwrap().into()
                }}
            };
            quote! {
                #pattern => #conversion,
            }
        })
        .collect()
}

// 生成 set_value 的 match 分支
fn generate_set_value_arms(fields_info: &[FieldInfo], use_field_name: bool) -> Vec<proc_macro2::TokenStream> {
    fields_info
        .iter()
        .map(|f| {
            // ✅ match 模式：字符串字面量
            let pattern = if use_field_name {
                LitStr::new(&f.field_name, proc_macro2::Span::call_site())
            } else {
                LitStr::new(&f.column_name, proc_macro2::Span::call_site())
            };

            // ✅ 字段访问：标识符
            let field_ident = format_ident!("{}", &f.field_name);

            let conversion = quote! { {self.#field_ident = value.into();} };

            quote! {
                #pattern => #conversion
            }
        })
        .collect()
}