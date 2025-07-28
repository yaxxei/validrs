use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::rules::contains::Contains;
use crate::rules::required::Required;
use crate::rules::{length::Length, range::Range};
use crate::utils::compile_error;

mod rules;
mod utils;

#[proc_macro_derive(Valid, attributes(valid))]
pub fn valid(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut validations = Vec::new();

    if let syn::Data::Struct(data) = &input.data {
        for field in &data.fields {
            let field_name = field.ident.as_ref().unwrap();

            for attr in &field.attrs {
                if !attr.path().is_ident("valid") {
                    continue;
                }

                let validators = match attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
                ) {
                    Ok(list) => list,
                    Err(err) => return err.to_compile_error().into(),
                };

                for expr in validators {
                    match expr {
                        syn::Expr::Call(ref call) => {
                            let validator_name = match &*call.func {
                                syn::Expr::Path(p) => {
                                    p.path.segments.last().unwrap().ident.to_string()
                                }
                                _ => continue,
                            };

                            match validator_name.as_str() {
                                "len" => {
                                    match Length::parse(call, field_name) {
                                        Ok(length) => match length.check(field_name) {
                                            Some(token) => {
                                                validations.push(token);
                                            }
                                            None => continue,
                                        },
                                        Err(err) => {
                                            return compile_error(field, err).into();
                                        }
                                    };
                                }
                                "rng" => match Range::parse(call, field_name) {
                                    Ok(rule) => {
                                        if let Some(token) = rule.check(field_name) {
                                            validations.push(token);
                                        }
                                    }
                                    Err(err) => {
                                        return compile_error(field, err).into();
                                    }
                                },
                                "contains" => match Contains::parse(call, field_name) {
                                    Ok(rule) => {
                                        if let Some(token) = rule.check(field_name) {
                                            validations.push(token);
                                        }
                                    }
                                    Err(err) => {
                                        return compile_error(field, err).into();
                                    }
                                },
                                "required" => match Required::parse(call, field_name) {
                                    Ok(rule) => {
                                        if let Some(token) = rule.check(field_name) {
                                            validations.push(token);
                                        }
                                    }
                                    Err(err) => {
                                        return compile_error(field, err).into();
                                    }
                                },
                                _ => {
                                    return compile_error(
                                        call,
                                        format!("Unknown validator: {}", validator_name),
                                    )
                                    .into();
                                }
                            }
                        }
                        syn::Expr::Path(ref path) => {
                            let validator_name =
                                &*path.path.segments.last().unwrap().ident.to_string();

                            match validator_name {
                                "required" => {
                                    if let Some(token) = Required::default().check(field_name) {
                                        validations.push(token);
                                    }
                                }
                                _ => {
                                    return compile_error(
                                        path,
                                        format!("Unknown validator: {}", validator_name),
                                    )
                                    .into();
                                }
                            }
                        }
                        _ => {
                            return compile_error(field_name, format!("Unknown validator")).into();
                        }
                    }
                    // if let syn::Expr::Call(ref call) = expr {
                    //     let validator_name = match &*call.func {
                    //         syn::Expr::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
                    //         _ => continue,
                    //     };

                    //     match validator_name.as_str() {
                    //         "len" => {
                    //             match Length::parse(call, field_name) {
                    //                 Ok(length) => match length.check(field_name) {
                    //                     Some(token) => {
                    //                         validations.push(token);
                    //                     }
                    //                     None => continue,
                    //                 },
                    //                 Err(err) => {
                    //                     return compile_error(field, err).into();
                    //                 }
                    //             };
                    //         }
                    //         "rng" => match Range::parse(call, field_name) {
                    //             Ok(rule) => {
                    //                 if let Some(token) = rule.check(field_name) {
                    //                     validations.push(token);
                    //                 }
                    //             }
                    //             Err(err) => {
                    //                 return compile_error(field, err).into();
                    //             }
                    //         },
                    //         "contains" => match Contains::parse(call, field_name) {
                    //             Ok(rule) => {
                    //                 if let Some(token) = rule.check(field_name) {
                    //                     validations.push(token);
                    //                 }
                    //             }
                    //             Err(err) => {
                    //                 return compile_error(field, err).into();
                    //             }
                    //         },
                    //         "required" => match Required::parse(call, field_name) {
                    //             Ok(rule) => {
                    //                 if let Some(token) = rule.check(field_name) {
                    //                     validations.push(token);
                    //                 }
                    //             }
                    //             Err(err) => {
                    //                 return compile_error(field, err).into();
                    //             }
                    //         },
                    //         _ => {
                    //             return compile_error(
                    //                 call,
                    //                 format!("Unknown validator: {}", validator_name),
                    //             )
                    //             .into();
                    //         }
                    //     }
                    // }
                }
            }
        }
    }

    let r#gen = quote! {
        impl validrs::validate::Validate for #name {
            fn validate(&self) -> validrs::error::Result<()> {
                #(#validations)*

                Ok(())
            }
        }
    };

    r#gen.into()
}

// #[proc_macro_derive(Validate, attributes(validate))]
// pub fn hello(input: TokenStream) -> proc_macro::TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let name = input.ident;

//     let mut validations = Vec::new();

//     if let syn::Data::Struct(data) = &input.data {
//         for field in &data.fields {
//             let field_name = field.ident.as_ref().unwrap();
//             for attr in &field.attrs {
//                 match parse_length_attribute(attr, field_name) {
//                     Ok((min, max, msg)) => {
//                         let field_access = quote! { self.#field_name };

//                         let check = match (min, max, msg) {
//                             (Some(min), Some(max), Some(msg)) => quote! {
//                                 if #field_access.len() < #min || #field_access.len() > #max {
//                                    return Err(validrs::error::Error::Custom(#msg.into()));
//                                 }
//                             },
//                             (Some(min), None, Some(msg)) => quote! {
//                                 if #field_access.len() < #min {
//                                     return Err(validrs::error::Error::Custom(#msg.into()));
//                                 }
//                             },
//                             (None, Some(max), Some(msg)) => quote! {
//                                 if #field_access.len() > #max {
//                                     return Err(validrs::error::Error::Custom(#msg.into()));
//                                 }
//                             },
//                             (None, Some(max), None) => quote! {
//                                 if #field_access.len() > #max {
//                                    return Err(validrs::error::Error::InvalidLength {min: None, max: Some(#max)});
//                                 }
//                             },
//                             (Some(min), None, None) => quote! {
//                                 if #field_access.len() < #min {
//                                     return Err(validrs::error::Error::InvalidLength {min: Some(#min), max: None});
//                                 }
//                             },
//                             (Some(min), Some(max), None) => quote! {
//                                 if #field_access.len() < #min || #field_access.len() > #max {
//                                     return Err(validrs::error::Error::InvalidLength {min: Some(#min), max: Some(#max)});
//                                 }
//                             },
//                             (None, None, Some(_)) | (None, None, None) => continue,
//                         };

//                         validations.push(check);
//                     }
//                     Err(err) => return err.into(),
//                 }
//             }
//         }
//     }

//     let r#gen = quote! {
//         impl validrs::validate::Validate for #name {
//             fn validate(&self) -> validrs::error::Result<()> {
//                 #(#validations)*

//                 Ok(())
//             }
//         }
//     };

//     r#gen.into()
// }

// fn parse_length_attribute(
//     attr: &syn::Attribute,
//     field_name: &syn::Ident,
// ) -> Result<(Option<usize>, Option<usize>, Option<String>), TokenStream> {
//     if !attr.path().is_ident("validate") {
//         return Ok((None, None, None));
//     }

//     let meta = attr.parse_args::<syn::Expr>().ok();

//     if let Some(syn::Expr::Call(call)) = meta {
//         if let syn::Expr::Path(path) = &*call.func {
//             if path.path.is_ident("len") {
//                 let mut min = None;
//                 let mut max = None;
//                 let mut msg = None;
//                 let mut error: Option<TokenStream> = None;

//                 for arg in call.args.iter() {
//                     if let syn::Expr::Assign(assign) = arg {
//                         let ident = if let syn::Expr::Path(p) = &*assign.left {
//                             p.path.segments.last().unwrap().ident.to_string()
//                         } else {
//                             continue;
//                         };

//                         match ident.as_str() {
//                             "min" | "max" => {
//                                 if let syn::Expr::Lit(syn::ExprLit {
//                                     lit: syn::Lit::Int(lit),
//                                     ..
//                                 }) = &*assign.right.clone()
//                                 {
//                                     let value = lit.base10_parse::<usize>().ok();
//                                     if ident == "min" {
//                                         min = value;
//                                     } else {
//                                         max = value;
//                                     }
//                                 }
//                             }
//                             "msg" => {
//                                 if let syn::Expr::Lit(syn::ExprLit {
//                                     lit: syn::Lit::Str(lit),
//                                     ..
//                                 }) = &*assign.right.clone()
//                                 {
//                                     let mut lit_str = lit.value();

//                                     if lit_str.contains("{{min}}") && min.is_none() {
//                                         error = Some(
//                                             quote! {
//                                                 compile_error!(
//                                                     concat!(
//                                                         "The 'msg' for field `",
//                                                         stringify!(#field_name),
//                                                         "` contains '{{min}}', but 'min' is not specified"
//                                                     )
//                                                 );
//                                             }
//                                             .into(),
//                                         );
//                                     }

//                                     if lit_str.contains("{{max}}") && max.is_none() {
//                                         error = Some(
//                                             quote! {
//                                                 compile_error!(
//                                                     concat!(
//                                                         "The 'msg' for field `",
//                                                         stringify!(#field_name),
//                                                         "` contains '{{max}}', but 'max' is not specified"
//                                                     )
//                                                 );
//                                             }
//                                             .into(),
//                                         );
//                                     }

//                                     if let Some(min) = min {
//                                         lit_str = lit_str.replace("{{min}}", &min.to_string());
//                                     }
//                                     if let Some(max) = max {
//                                         lit_str = lit_str.replace("{{max}}", &max.to_string());
//                                     }
//                                     msg = Some(lit_str);
//                                 }
//                             }
//                             _ => continue,
//                         }
//                     }
//                 }

//                 if let Some(err) = error {
//                     return Err(err);
//                 }

//                 return Ok((min, max, msg));
//             }
//         }
//     }

//     Ok((None, None, None))
// }
