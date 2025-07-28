use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::apply_template;

pub struct Range {
    min: Option<usize>,
    max: Option<usize>,
    msg: Option<String>,
}

impl Range {
    pub fn check(&self, field_name: &syn::Ident) -> Option<TokenStream> {
        let field_access = quote! { self.#field_name };

        let min = match &self.min {
            Some(v) => quote! { Some(#v) },
            None => quote! { None },
        };

        let max = match &self.max {
            Some(v) => quote! { Some(#v) },
            None => quote! { None },
        };

        let msg = match &self.msg {
            Some(v) => quote! { Some(#v.to_string()) },
            None => quote! { None },
        };

        Some(quote! { #field_access.validate_range(#min, #max, #msg)?; })
    }

    pub fn parse(call: &syn::ExprCall, field_name: &syn::Ident) -> syn::Result<Self> {
        let mut min = None;
        let mut max = None;
        let mut msg = None;

        for arg in call.args.iter() {
            if let syn::Expr::Assign(assign) = arg {
                let ident = if let syn::Expr::Path(p) = &*assign.left {
                    p.path.segments.last().unwrap().ident.to_string()
                } else {
                    continue;
                };

                match ident.as_str() {
                    "min" | "max" => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Int(lit),
                            ..
                        }) = &*assign.right.clone()
                        {
                            let value = lit.base10_parse::<usize>().ok();
                            if ident == "min" {
                                min = value;
                            } else {
                                max = value;
                            }
                        }
                    }
                    "msg" => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }) = &*assign.right.clone()
                        {
                            let result = apply_template(
                                &lit.value(),
                                &[("min", min.as_ref()), ("max", max.as_ref())],
                                field_name,
                            )?;
                            msg = Some(result);
                        }
                    }
                    _ => continue,
                }
            }
        }

        return Ok(Self { min, max, msg });
    }
}
