use proc_macro2::TokenStream;
use quote::quote;

pub struct Contains {
    values: Vec<String>,
    msg: Option<String>,
}

impl Contains {
    pub fn check(&self, field_name: &syn::Ident) -> Option<TokenStream> {
        let field_access = quote! { self.#field_name };

        let values = &self.values;
        let msg = match &self.msg {
            Some(v) => quote! { Some(#v.to_string()) },
            None => quote! { None },
        };

        Some(quote! { #field_access.validate_contains(&[#(#values),*], #msg)?; })
    }

    pub fn parse(call: &syn::ExprCall, field_name: &syn::Ident) -> syn::Result<Self> {
        let mut values = Vec::new();
        let mut msg = None;

        for expr in call.args.iter() {
            match expr {
                syn::Expr::Array(syn::ExprArray { elems, .. }) => {
                    for elem in elems {
                        if let syn::Expr::Lit(lit) = elem {
                            if let syn::Lit::Str(lit_str) = &lit.lit {
                                values.push(lit_str.value());
                            }
                        }
                    }
                }
                syn::Expr::Assign(assign) => {
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
                _ => continue,
            }
        }

        if values.is_empty() {
            return Err(syn::Error::new_spanned(
                call,
                "contains() validator requires at least one value",
            ));
        }

        Ok(Self { values, msg })
    }
}
