use crate::get_attr;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

fn to_c_type(t: &syn::Type) -> String {
    use syn::Type::*;
    match t {
        Ptr(syn::TypePtr {
            mutability, elem, ..
        }) => {
            if mutability.is_some() {
                format!("{} *", elem.to_token_stream())
            } else {
                format!("{} const *", elem.to_token_stream())
            }
        }
        Reference(syn::TypeReference {
            mutability, elem, ..
        }) => {
            if mutability.is_some() {
                format!("{} &", elem.to_token_stream())
            } else {
                format!("{} const &", elem.to_token_stream())
            }
        }
        Path(syn::TypePath { path, .. }) => path.to_token_stream().to_string(),
        _ => {
            unimplemented!("uncovered type {:#?}", t);
        }
    }
}

pub fn parse_fn(
    attrs: &[syn::Attribute],
    sig: &syn::Signature,
    self_ty: Option<&syn::LitStr>,
    link_prefix: Option<&syn::LitStr>,
) -> syn::Result<TokenStream> {
    let mut self_ty = self_ty.map(<_>::clone);
    let fn_name = syn::LitStr::new(
        &(sig.ident.to_string() + &sig.generics.to_token_stream().to_string()),
        sig.ident.span(),
    );
    let mut c_fn_token = fn_name.clone();

    let mut link_name = fn_name;
    if let Some(lp) = link_prefix {
        link_name = syn::LitStr::new(&(lp.value() + "_" + &link_name.value()), link_name.span());
    }

    let mut rt_token = quote! { rxx_build::ReturnType::None };
    let mut rt = syn::LitStr::new("", sig.output.span());
    if let syn::ReturnType::Type(_, t) = &sig.output {
        rt = syn::LitStr::new(&to_c_type(t), sig.output.span());
        rt_token = quote! {rxx_build::ReturnType::Object(#rt)};
    }

    if let Some(meta_list) = get_attr(attrs)? {
        for i in meta_list.nested {
            match i {
                syn::NestedMeta::Meta(syn::Meta::NameValue(m)) => {
                    let key = &m.path.get_ident().unwrap().to_string();
                    let syn::Lit::Str(val) = m.lit else {
			continue
		    };

                    if key == "link_name" {
                        link_name = val;
                    } else if key == "c_fn" {
                        c_fn_token = val;
                    } else if key == "ns" {
                        c_fn_token = syn::LitStr::new(
                            &format!("{}::{}", val.value(), c_fn_token.value()),
                            c_fn_token.span(),
                        );
                    } else if key == "cls" {
                        self_ty = Some(val);
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
                    if !rt.value().is_empty() {
                        if p.is_ident("atomic") {
                            rt_token = quote! {rxx_build::ReturnType::Atomic(#rt)};
                        } else if p.is_ident("object") {
                            rt_token = quote! {rxx_build::ReturnType::Object(#rt)};
                        }
                    }
                }
                m => panic!("uncovered meta here {m:?}"),
            }
        }
    }

    if self_ty.is_some() {
        c_fn_token = syn::LitStr::new(&format!("&$C::{}", c_fn_token.value()), c_fn_token.span());
    }

    let mut arg_tokens = vec![];
    let mut is_mut = false;

    for inp in sig.inputs.iter() {
        match inp {
            syn::FnArg::Receiver(syn::Receiver { mutability, .. }) => {
                if mutability.is_some() {
                    is_mut = true;
                }
            }
            syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                let arg_name = syn::LitStr::new(&pat.to_token_stream().to_string(), pat.span());
                let arg_type = syn::LitStr::new(&to_c_type(ty), ty.span());
                arg_tokens.push(quote! {(#arg_type, #arg_name)});
            }
        }
    }

    let self_ty_token = if self_ty.is_some() {
        quote! {Some(#self_ty)}
    } else {
        quote! {None}
    };

    let tt = quote! {
    __out.push(rxx_build::genc_fn(
        &strfmt::strfmt(#link_name, &tpl_vars).unwrap(),
        rxx_build::FnSig {
        cls: #self_ty_token,
        is_mut: #is_mut,
        c_fn: #c_fn_token,
        ret_type: #rt_token,
        args: &[#(#arg_tokens),*],
        }
    ));
    };
    Ok(tt)
}

pub fn parse_impl(item_impl: &syn::ItemImpl) -> Vec<syn::Result<TokenStream>> {
    let self_ty = &item_impl.self_ty;
    let self_ty = syn::LitStr::new(&self_ty.to_token_stream().to_string(), self_ty.span());

    let mut link_prefix = syn::LitStr::new("", Span::call_site());

    if let Ok(Some(meta_list)) = get_attr(&item_impl.attrs) {
        for i in meta_list.nested {
            match i {
                syn::NestedMeta::Meta(syn::Meta::NameValue(m)) => {
                    let key = &m.path.get_ident().unwrap().to_string();
                    let syn::Lit::Str(val) = m.lit else {
			continue
		    };

                    if key == "link_prefix" {
                        link_prefix = val;
                    }
                }
                _ => {
                    unimplemented!("{:#?} not covered", i);
                }
            }
        }
    }

    let tts = item_impl
        .items
        .iter()
        .map(|i| match i {
            syn::ImplItem::Method(v) => {
                if v.block.stmts.is_empty() {
                    parse_fn(&v.attrs, &v.sig, Some(&self_ty), Some(&link_prefix))
                } else {
                    Ok(v.into_token_stream())
                }
            }
            _ => unimplemented!("unimplemented for {:#?}", i),
        })
        .collect();
    tts
}
