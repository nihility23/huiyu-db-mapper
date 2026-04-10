use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::meta::ParseNestedMeta;
use syn::{parse_macro_input, spanned::Spanned, Attribute, Block, Data, DeriveInput, Error, Fields, ItemFn, ItemStruct, Lit, LitStr, Type};

use syn::punctuated::Punctuated;
use syn::Token;
use syn::Meta;

#[proc_macro_attribute]
pub fn mapper(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏参数，例如 #[mapper(PermissionEntity)]
    let args = parse_macro_input!(args with Punctuated::<Meta, Token![,]>::parse_terminated);
    let entity_type = match args.first() {
        Some(Meta::Path(path)) => {
            path.segments.last().unwrap().ident.clone()
        }
        _ => panic!("Expected entity type as argument, e.g., #[mapper(XxxxxEntity)]"),
    };

    // 解析被标注的结构体，例如 pub struct PermissionMapper;
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;

    // 生成实现代码
    let expanded = quote! {
        #input

        impl huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper<#entity_type> for #struct_name {
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn transactional(input: TokenStream) -> TokenStream {
    // 解析输入为一个代码块
    let block = parse_macro_input!(input as Block);

    // 生成 transactional_exec 调用并直接作为表达式返回
    let expanded = quote! {
        huiyu_db_util::huiyu_db_mapper::query::transactional::transactional_exec(async || {
            #block
        }).await
    };

    TokenStream::from(expanded)
}

/// 数据源属性宏
///
/// # 用法
/// ```rust
/// #[datasource("mysql")]
/// async fn query_user(id: i32) -> Result<User, Error> {
///     // 函数体，可以直接使用数据源
/// }
/// ```
#[proc_macro_attribute]
pub fn datasource(args: TokenStream, input: TokenStream) -> TokenStream {
    let db_name = parse_macro_input!(args as LitStr);
    let db_name_str = db_name.value();
    let func = parse_macro_input!(input as ItemFn);

    if func.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &func.sig,
            "datasource 宏只能用于 async 函数"
        ).to_compile_error().into();
    }

    let vis = &func.vis;
    let sig = &func.sig;
    let block = &func.block;
    let attrs = &func.attrs;
    let fn_name = &sig.ident;
    println!("fn_name: {}", fn_name.to_string());
    println!("sig: {:?}", sig);

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            use std::sync::Arc;
            use huiyu_db_util::huiyu_db_mapper_core::pool::datasource::DB_NAME_REGISTRY;

            let ds_name: Arc<String> = Arc::new(#db_name_str.to_string());
            // 执行 scope 并返回结果
            let result = DB_NAME_REGISTRY.scope(ds_name, async {
                #block
            }).await;

            result
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Mapping, attributes(field))]
pub fn derive_mapping(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

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


    // 生成 get_value 的 match 分支
    let get_value_by_field_arms = generate_get_value_arms(&fields_info, true);
    let get_value_by_column_arms = generate_get_value_arms(&fields_info, false);

    // 生成 set_value 的 match 分支
    let set_value_by_field_arms = generate_set_value_arms(&fields_info, true);
    let set_value_by_column_arms = generate_set_value_arms(&fields_info, false);


    let expanded = quote! {

        // use db_mapper::base::entity::huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType;
        // use db_mapper::base::entity::Entity;
        // use db_mapper::base::entity::huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnInfo;
        // use db_mapper::base::param::huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue;

        impl huiyu_db_util::huiyu_db_mapper_core::base::mapping::Mapping for #name {


            fn column_names() -> Vec<&'static str> {
                vec![#(#column_names),*]
            }

            fn field_names() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }

            fn new() -> Self {
                Self {
                    #(#field_inits),*
                }
            }

            fn get_value_by_field_name(&self, field_name: &str) -> huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue {
                match field_name {
                    #(#get_value_by_field_arms)*
                    _ => huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue::Null,
                }
            }

            fn get_value_by_column_name(&self, column_name: &str) -> huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue {
                match column_name {
                    #(#get_value_by_column_arms)*
                    _ => huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue::Null,
                }
            }

            fn set_value_by_field_name(&mut self, field_name: &str, value: huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue) {
                match field_name {
                    #(#set_value_by_field_arms)*
                    _ => panic!("Field name not found: {}", field_name),
                }
            }

            fn set_value_by_column_name(&mut self, column_name: &str, value: huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue) {
                match column_name {
                    #(#set_value_by_column_arms)*
                    _ => panic!("Column name not found: {}", column_name),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_derive(Entity, attributes(id, field, table))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // 是否大小写敏感
    let (table_name_opt, is_case_sensitive) = parse_table_info(&input.attrs);
    let table_name = table_name_opt.unwrap_or_else(|| format!("t_{}", name.to_string().to_lowercase()));
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
    let id_column_info = generate_column_info(&id_field);

    // ============ 字符串字面量（用于返回值） ============
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

    // 生成 huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnInfo 列表
    let column_infos = generate_column_infos(&fields_info);
    let column_const_names = generate_column_names(&fields_info);

    // 生成 get_value 的 match 分支
    let get_value_by_field_arms = generate_get_value_arms(&fields_info, true);
    let get_value_by_column_arms = generate_get_value_arms(&fields_info, false);

    // 生成 set_value 的 match 分支
    let set_value_by_field_arms = generate_set_value_arms(&fields_info, true);
    let set_value_by_column_arms = generate_set_value_arms(&fields_info, false);

    // 主键类型（从 Option 中提取）
    let key_type = extract_inner_type(&id_field.field_type);

    let expanded = quote! {

        // use db_mapper::base::entity::huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType;
        // use db_mapper::base::entity::Entity;
        // use db_mapper::base::entity::huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnInfo;
        // use db_mapper::base::param::huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue;
        impl #name{
            #(#column_const_names)*
        }
        impl huiyu_db_util::huiyu_db_mapper_core::base::entity::Entity for #name {
            type K = #key_type;

            fn is_case_sensitive() -> bool {
                #is_case_sensitive
            }

            fn key(&self) -> Self::K {
                self.#id_field_ident.clone().unwrap_or_default()
            }

            fn key_name() -> &'static str {
                #id_column_name_lit
            }

            fn key_info() -> Option<huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnInfo> {
                Some(#id_column_info)
            }

            fn table_name() -> &'static str {
                #table_name_lit
            }

            fn get_column_infos() -> Vec<huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnInfo> {
                vec![#(#column_infos),*]
            }
        }

        impl huiyu_db_util::huiyu_db_mapper_core::base::mapping::Mapping for #name {


            fn column_names() -> Vec<&'static str> {
                vec![#(#column_names),*]
            }

            fn field_names() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }

            fn new() -> Self {
                Self {
                    #(#field_inits),*
                }
            }

            fn get_value_by_field_name(&self, field_name: &str) -> huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue {
                match field_name {
                    #(#get_value_by_field_arms)*
                    _ => huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue::Null,
                }
            }

            fn get_value_by_column_name(&self, column_name: &str) -> huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue {
                match column_name {
                    #(#get_value_by_column_arms)*
                    _ => huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue::Null,
                }
            }

            fn set_value_by_field_name(&mut self, field_name: &str, value: huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue) {
                match field_name {
                    #(#set_value_by_field_arms)*
                    _ => panic!("Field name not found: {}", field_name),
                }
            }

            fn set_value_by_column_name(&mut self, column_name: &str, value: huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue) {
                match column_name {
                    #(#set_value_by_column_arms)*
                    _ => panic!("Column name not found: {}", column_name),
                }
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
    column_type: String,
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

fn parse_table_info(attrs: &[Attribute]) -> (Option<String>,bool) {
    let mut table_name = None;
    let mut table_sensitive = false;
    for attr in attrs {
        if attr.path().is_ident("table") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("case_sensitive") {
                    table_sensitive = parse_bool_lit(&meta)?;
                }else if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit {
                        table_name = Some(lit_str.value());
                    }
                }else {
                    return Err(syn::Error::new(meta.path.span(), "Unknown attribute"));
                }
                Ok(())
            });
        }
    }
    (table_name, table_sensitive)
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
            column_type: "varchar".to_string(),
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
        } else if meta.path.is_ident("type") {
            info.column_type = parse_string_lit(&meta)?;
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
        } else if meta.path.is_ident("type") {
            info.column_type = parse_string_lit(&meta)?;
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

fn generate_column_infos(fields_info: &[FieldInfo]) -> Vec<proc_macro2::TokenStream> {
    fields_info
        .iter()
        .map(|f| generate_column_info(f))
        .collect()
}

fn generate_column_names(fields_info: &[FieldInfo]) -> Vec<proc_macro2::TokenStream> {
    fields_info
        .iter()
        .map(|f| generate_column_name(f))
        .collect()
}

fn generate_column_name(field: &FieldInfo) -> proc_macro2::TokenStream {
    let column_name_lit = LitStr::new(&field.column_name, proc_macro2::Span::call_site());
    let column_name = format_ident!("{}", field.column_name.to_uppercase());
    quote! {
        pub const #column_name: &'static str = #column_name_lit;
    }
}

fn generate_column_info(f: &FieldInfo) -> proc_macro2::TokenStream {
    let field_name_lit = LitStr::new(&f.field_name, proc_macro2::Span::call_site());
    let column_name_lit = LitStr::new(&f.column_name, proc_macro2::Span::call_site());

    let is_nullable = f.is_nullable;
    let is_auto_increment = f.is_auto_increment;
    let is_id = f.is_id;
    let fill_on_update = f.fill_on_update;
    let fill_on_insert = f.fill_on_insert;

    let ty = f.inner_type.as_ref().unwrap_or(&f.field_type);
    let column_type = infer_column_type(ty);
    let field_type = infer_field_type(ty);

    // 主键生成策略（保持为 Option<String>）
    let key_generate_type = match &f.key_generate_type {
        Some(s) => {
            let s_lit = LitStr::new(s, proc_macro2::Span::call_site());
            quote! { Some(#s_lit.to_string()) }
        }
        None => quote! { None },
    };

    quote! {
        huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnInfo {
            field_name: #field_name_lit,
            field_type: #field_type,
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
}

fn infer_field_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            match segment.ident.to_string().as_str() {
                "i8" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::I8 },
                "i16" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::I16 },
                "i32" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::I32 },
                "i64" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::I64 },
                "u8" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::U8 },
                "u16" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::U16 },
                "u32" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::U32 },
                "u64" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::U64 },
                "usize" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::USize },
                "f32" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::F32 },
                "f64" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::F64 },
                "bool" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::Bool },
                "String" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::String },
                "DateTime" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::FieldType::DateTime },
                _ => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Null },
            }
        }
        _ => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Null },
    }
}

// 推断列类型
fn infer_column_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            match segment.ident.to_string().as_str() {
                "i8" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::TinyInt },
                "i16" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::SmallInt },
                "i32" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Integer },
                "i64" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::BigInt },
                "u8" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::TinyInt },
                "u16" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::SmallInt },
                "u32" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Integer },
                "u64" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::BigInt },
                "usize" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Integer },
                "f32" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Float },
                "f64" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Double },
                "bool" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Bool },
                "String" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Varchar },
                "DateTime" => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::DateTime },
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            if let Type::Path(inner_path) = inner {
                                if let Some(inner_seg) = inner_path.path.segments.last() {
                                    if inner_seg.ident == "u8" {
                                        return quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Blob };
                                    }
                                }
                            }
                        }
                    }
                    quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Clob }
                }
                _ => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Null },
            }
        }
        _ => quote! { huiyu_db_util::huiyu_db_mapper_core::base::entity::ColumnType::Null },
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
                    huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue::Null
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
