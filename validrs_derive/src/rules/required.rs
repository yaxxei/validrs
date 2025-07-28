use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
pub struct Required {
    msg: Option<String>,
}

impl Required {
    pub fn check(&self, field_name: &syn::Ident) -> Option<TokenStream> {
        let field_access = quote! { self.#field_name };
        let msg = match &self.msg {
            Some(v) => quote! { Some(#v.to_string()) },
            None => quote! { None },
        };

        Some(quote! { #field_access.validate_required(#msg)?; })
    }

    pub fn parse(call: &syn::ExprCall, field_name: &syn::Ident) -> syn::Result<Self> {
        let mut msg = None;

        for expr in call.args.iter() {
            if let syn::Expr::Assign(assign) = expr {
                if let syn::Expr::Path(p) = &*assign.left {
                    if p.path.segments.last().unwrap().ident == "msg" {
                        if let syn::Expr::Lit(lit) = &*assign.right {
                            if let syn::Lit::Str(lit_str) = &lit.lit {
                                msg = Some(lit_str.value());
                            }
                        }
                    }
                }
            }
        }

        Ok(Self { msg })
    }
}
