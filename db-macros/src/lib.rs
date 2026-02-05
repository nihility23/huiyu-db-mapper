use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, ItemFn, LitStr, Signature, Type};
use syn::token::Async;

#[proc_macro_attribute]
pub fn datasource(attr: TokenStream, item: TokenStream) -> TokenStream {
    let db_name = parse_macro_input!(attr as LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let new_sig = Signature {
        asyncness: Some(syn::token::Async::default()),
        ..sig.clone()
    };

    let expanded = quote! {
        #vis #new_sig {
            // 关键：需要 `.await` 来获取实际返回值
            db_mapper::pool::datasource::DB_NAME_REGISTRY
                .scope(
                    std::cell::RefCell::new(Some(#db_name.to_string())),
                    async { #block }
                )
                .await
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn transactional(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    quote! {
        let result = {#input}
        if result.is_ok() {

        }
    }.into()
}

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
