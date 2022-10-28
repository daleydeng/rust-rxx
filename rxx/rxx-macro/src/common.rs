use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

#[derive(Debug)]
pub enum ReturnType<T> {
    None,
    Object(T),
    Atomic(T),
    NewPtr(T),
}

pub fn get_attr(attrs: &[syn::Attribute]) -> syn::Result<Option<syn::MetaList>> {
    if attrs.len() > 1 {
        let mut ts = TokenStream::new();
        ts.extend(attrs.iter().map(syn::Attribute::to_token_stream));
        return Err(syn::Error::new(
            ts.span(),
            format!("Attribute must be 1, not {}", attrs.len()),
        ));
    }
    let Some(attr) = attrs.first() else {
	return Ok(None);
    };

    let meta = attr.parse_meta()?;
    let mp = meta.path();
    if !mp.is_ident("ffi") {
        if let Some(id) = mp.get_ident() {
            return Err(syn::Error::new(
                mp.span(),
                format!("Meta path ({id})!= ffi"),
            ));
        } else {
            return Err(syn::Error::new(mp.span(), "Meta path isnt ident"));
        }
    }

    if let syn::Meta::List(meta_list) = meta {
        Ok(Some(meta_list))
    } else {
        Err(syn::Error::new(meta.span(), "Meta need to be list"))
    }
}
