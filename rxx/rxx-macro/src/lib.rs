use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

mod common;
use common::*;

mod genc;
mod genrs;

#[proc_macro]
pub fn genrs_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input with syn::Block::parse_within);

    let tts = input.iter().map(|stmt| -> TokenStream {
        match stmt {
            syn::Stmt::Item(syn::Item::Fn(item)) => {
                if item.block.stmts.is_empty() {
                    genrs::parse_fn(&item.attrs, &item.vis, &item.sig, None, None)
                        .unwrap_or_else(syn::Error::into_compile_error)
                } else {
                    item.to_token_stream()
                }
            }
            syn::Stmt::Item(syn::Item::Impl(item)) => {
                genrs::parse_impl(item).unwrap_or_else(syn::Error::into_compile_error)
            }
            _ => unimplemented!("unimplemented for stmt {:?}", stmt),
        }
    });

    let tt = quote! {
        #(#tts)*
    };
    tt.into()
}

#[proc_macro]
pub fn genc_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input with syn::Block::parse_within);

    let mut tts = vec![];
    for stmt in input {
        match &stmt {
            syn::Stmt::Item(syn::Item::Fn(item)) => {
                if item.block.stmts.is_empty() {
                    tts.push(
                        genc::parse_fn(&item.attrs, &item.sig, None, None)
                            .unwrap_or_else(syn::Error::into_compile_error),
                    );
                } else {
                    tts.push(item.to_token_stream());
                }
            }
            syn::Stmt::Item(syn::Item::Impl(item)) => {
                tts.extend(
                    genc::parse_impl(item)
                        .into_iter()
                        .map(|i| i.unwrap_or_else(syn::Error::into_compile_error)),
                );
            }
            m => {
		tts.push(m.to_token_stream());
	    },
        }
    }

    let tt = quote! {
    {
        let mut __out = Vec::new();
        #(#tts)*
	__out
    }
    };
    tt.into()
}
