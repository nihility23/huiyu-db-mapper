use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Type};

// #[proc_macro_attribute]
// pub fn mapper(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as DeriveInput);
//
//     let struct_name = &input.ident;
//
//     let attr_parser = syn::meta::parser(|meta| {
//         Ok(())
//     });
//     // 解析参数
//     let _args = parse_macro_input!(attr with attr_parser);
//     // 手动解析 TokenStream 来获取参数
//     let attr_string = attr.to_string();
//     let parts: Vec<&str> = attr_string.split(',').map(|s| s.trim()).collect();
//
//     if parts.len() > 1{
//         // compile_error!(
//         //     concat!("mapper宏`", stringify!(#field_name), "`只能有一个参数")
//         // );
//     }
//
//     let expanded = quote! {
//         #input
//
//
//     };
//
//     expanded.into()
// }

#[proc_macro_derive(CheckAllOption, attributes(allow_non_option))]
pub fn derive_all_option(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("仅支持具名字段的结构体"),
        },
        _ => panic!("仅支持结构体类型"),
    };

    let mut errors = Vec::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if !is_option_type(&field.ty) && !has_skip_attr(&field.attrs) {
            errors.push(quote! {
                compile_error!(
                    concat!("字段 `", stringify!(#field_name), "` 必须为Option类型")
                );
            });
        }
    }

    if errors.is_empty() {
        TokenStream::from(quote! {
            impl #struct_name {
                /// 生成的方法用于验证派生宏已正确应用
                pub fn __validate_all_option_fields() {}
            }
        })
    } else {
        TokenStream::from(quote! { #(#errors)* })
    }
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        type_path.path.segments.iter().any(|seg| seg.ident == "Option")
    } else {
        false
    }
}

fn has_skip_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("allow_non_option"))
}
