use std::fmt::Display;

use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub fn compile_error<S: Spanned, T: Display>(span: S, err: T) -> TokenStream {
    syn::Error::new(span.span(), err).to_compile_error().into()
}

pub fn apply_template<T: Display>(
    template: &str,
    values: &[(&str, Option<&T>)],
    field_name: &syn::Ident,
) -> syn::Result<String> {
    for (key, value) in values {
        if template.contains(&format!("{{{key}}}")) && value.is_none() {
            let error_msg = format!(
                "The 'msg' for field `{}` contains '{{{{{}}}}}', but '{}' is not specified",
                field_name, key, key
            );
            return Err(syn::Error::new(field_name.span(), error_msg));
        }
    }

    let mut result = template.to_string();
    for (key, value) in values {
        if let Some(value) = value {
            result = result.replace(&format!("{{{{{}}}}}", key), &value.to_string());
        }
    }

    Ok(result)
}
