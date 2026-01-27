use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, ItemFn, ReturnType, Type};

#[proc_macro_attribute]
pub fn transactional(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析方法定义
    let input = parse_macro_input!(item as ItemFn);
    let method_vis = &input.vis;
    let method_sig = &input.sig;
    let method_block = &input.block;

    // 解析属性参数（可选）
    let attr_string = attr.to_string();
    let tx_manager = if attr_string.is_empty() {
        // 默认使用 self.tx_manager
        quote! { self.tx_manager }
    } else {
        let tx_manager_ident = syn::parse_str::<syn::Ident>(&attr_string.trim()).unwrap();
        // 检查是否包含 self.
        if attr_string.contains("self.") {
            quote! { #tx_manager_ident }
        } else {
            // 如果没有包含 self.，则添加它
            quote! { self.#tx_manager_ident }
        }
    };

    // 生成新的方法实现，添加事务支持
    let expanded = quote! {
        #method_vis #method_sig {
            #tx_manager.begin();

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #method_block
            }));

            match result {
                Ok(inner_result) => {
                    match inner_result {
                        Ok(value) => {
                            #tx_manager.commit();
                            Ok(value)
                        }
                        Err(e) => {
                            #tx_manager.rollback();
                            Err(e)
                        }
                    }
                }
                Err(panic_err) => {
                    #tx_manager.rollback();
                    Err(format!("Transaction panicked: {:?}", panic_err).into())
                }
            }
        }
    };

    expanded.into()
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
