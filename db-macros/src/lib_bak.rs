// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, ItemStruct, Type};
//
// /***
//     校验字段是否为Option,数据库查询空和空串有不同含义，要求必须为Option
//  */
// // #[proc_macro_attribute]
// // pub fn check_option(_attr: TokenStream, item: TokenStream) -> TokenStream {
// //     let input = parse_macro_input!(item as ItemStruct);
// //     let struct_name = &input.ident;
// //
// //     let mut errors = Vec::new();
// //
// //     if let Fields::Named(fields) = &input.fields {
// //         for field in &fields.named {
// //             let field_name = &field.ident;
// //             let is_option = match &field.ty {
// //                 syn::Type::Path(type_path) => {
// //                     type_path.path.segments.iter().any(|seg| seg.ident == "Option")
// //                 }
// //                 _ => false
// //             };
// //
// //             if !is_option {
// //                 let error = syn::Error::new_spanned(
// //                     &field.ty,
// //                     format!("字段 `{}` 必须是Option类型", field_name.as_ref().unwrap())
// //                 );
// //                 errors.push(error.to_compile_error());
// //             }
// //         }
// //     }
// //
// //     let expanded = quote! {
// //         #input
// //
// //         #(#errors)*
// //     };
// //
// //     expanded.into()
// // }
// //
// //
// // #[proc_macro_derive(CheckAllOption)]
// // pub fn derive_check_all_option(input: TokenStream) -> TokenStream {
// //     let input = parse_macro_input!(input as DeriveInput);
// //     let struct_name = &input.ident;
// //     println!("ddddddd");
// //     // 检查字段类型
// //     let fields = match input.data {
// //         Data::Struct(data) => match data.fields {
// //             Fields::Named(fields) => fields.named,
// //             _ => panic!("仅支持具名字段的结构体"),
// //         },
// //         _ => panic!("仅支持结构体类型"),
// //     };
// //
// //     // 生成校验逻辑
// //     let checks = fields.iter().map(|field| {
// //         let field_name = &field.ident;
// //         let field_type = &field.ty;
// //         quote! {
// //             if !stringify!(#field_type).starts_with("Option") {
// //                 panic!("字段 {} 必须为Option类型", stringify!(#field_name));
// //             }
// //         }
// //     });
// //
// //     // 生成impl代码块
// //     let expanded = quote! {
// //         impl #struct_name {
// //             pub fn check_all_option_fields() {
// //                 #(#checks)*
// //             }
// //         }
// //     };
// //
// //     TokenStream::from(expanded)
// // }
//
// #[proc_macro_derive(AllOption, attributes(allow_non_option))]
// pub fn derive_all_option(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = &input.ident;
//
//     let fields = match &input.data {
//         Data::Struct(data) => match &data.fields {
//             Fields::Named(fields) => &fields.named,
//             _ => panic!("仅支持具名字段的结构体"),
//         },
//         _ => panic!("仅支持结构体类型"),
//     };
//
//     let mut errors = Vec::new();
//     for field in fields {
//         let field_name = field.ident.as_ref().unwrap();
//         if !is_option_type(&field.ty) && !has_skip_attr(&field.attrs) {
//             errors.push(quote! {
//                 compile_error!(
//                     concat!("字段 `", stringify!(#field_name), "` 必须为Option类型")
//                 );
//             });
//         }
//     }
//
//     if errors.is_empty() {
//         TokenStream::from(quote! {
//             impl #struct_name {
//                 /// 生成的方法用于验证派生宏已正确应用
//                 pub fn __validate_all_option_fields() {}
//             }
//         })
//     } else {
//         TokenStream::from(quote! { #(#errors)* })
//     }
// }
//
// fn is_option_type(ty: &Type) -> bool {
//     if let Type::Path(type_path) = ty {
//         type_path.path.segments.iter().any(|seg| seg.ident == "Option")
//     } else {
//         false
//     }
// }
//
// fn has_skip_attr(attrs: &[Attribute]) -> bool {
//     attrs.iter().any(|attr| attr.path().is_ident("allow_non_option"))
// }
