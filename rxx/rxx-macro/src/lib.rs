#![feature(let_else)]

use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::Span;

enum ReturnType<T> {
    None,
    Object(T),
    Atomic(T),
}

fn parse_fn_item(item_fn: &syn::ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let attrs = &item_fn.attrs;
    if attrs.len() > 1 {panic!("Attribute must be 1, not {}", attrs.len());}
    let fn_sig = &item_fn.sig;
    let fn_name = &fn_sig.ident;

    let mut link_name = fn_name.clone();
    let mut ret_mode = "object";

    if !attrs.is_empty() {
	let attr = attrs.first().unwrap();
	let meta = attr.parse_meta()?;
	assert!(meta.path().is_ident("ffi"));
	if let syn::Meta::List(meta_list) = meta {
	    for i in meta_list.nested {
		match i {
		    syn::NestedMeta::Meta(syn::Meta::NameValue(m)) => {
			if m.path.get_ident().unwrap() == "link_name" {
			    if let syn::Lit::Str(l) = m.lit {
				link_name = syn::Ident::new(&l.value(), l.span());
			    }
			}
		    }
		    syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
			if p.is_ident("atomic") {
			    ret_mode = "atomic";
			} else if p.is_ident("object") {
			    ret_mode = "object";
			}
		    },
		    m => panic!("uncovered meta here {:?}", m)
		}
	    }
	}
    };

    let ret_type = match &fn_sig.output {
	syn::ReturnType::Default => {
	    ReturnType::None
	},
	syn::ReturnType::Type(_, t) => {
	    if ret_mode == "atomic" {
		ReturnType::Atomic(t)
	    } else if ret_mode == "object" {
		ReturnType::Object(t)
	    } else {
		panic!("ret_mode invalid! {}", ret_mode);
	    }
	},
    };

    let fn_vis = &item_fn.vis;
    let unsafety = &fn_sig.unsafety;
    let mut c_decl_inputs = fn_sig.inputs.clone();
    let mut c_call_inputs = fn_sig.inputs.iter().filter_map(|i| {
	if let syn::FnArg::Typed(syn::PatType{pat: pt, ..}) = i {
	    Some(pt)
	} else {
	    None
	}
    }).collect::<syn::punctuated::Punctuated<_, syn::Token![,]>>();

    if let ReturnType::Object(_) = ret_type {
	if !c_decl_inputs.empty_or_trailing() {
	    c_decl_inputs.push_punct(syn::Token![,](Span::call_site()));
	}
	if !c_call_inputs.empty_or_trailing() {
	    c_call_inputs.push_punct(syn::Token![,](Span::call_site()));
	}
    }
    let fn_generics = &fn_sig.generics;

    let t = if unsafety.is_none() {
	match ret_type {
	    ReturnType::Object(rt) =>
		quote!{
		    #fn_vis #fn_sig {
			extern "C" {
			    #[link_name = stringify!(#link_name)]
			    fn __func #fn_generics (#c_decl_inputs __ret: *mut #rt);
			}
			unsafe {
			    let mut __ret = std::mem::MaybeUninit::<#rt>::uninit();
			    let mut __ret_ptr = __ret.as_mut_ptr();
			    __func (#c_call_inputs __ret_ptr);
			    __ret.assume_init()
			}
		    }
		},

	    ReturnType::Atomic(rt) =>
		quote!{
		    #fn_vis #fn_sig {
			extern "C" {
			    #[link_name = stringify!(#link_name)]
			    fn __func #fn_generics (#c_decl_inputs) -> #rt;
			}
			unsafe {
			    __func(#c_call_inputs)
			}
		    }
		},

	    ReturnType::None =>
		quote!{
		    #fn_vis #fn_sig {
			extern "C" {
			    #[link_name = stringify!(#link_name)]
			    fn __func #fn_generics (#c_decl_inputs);
			}
			unsafe {
			    __func(#c_call_inputs)
			}
		    }
		},
	}
    } else {
	unimplemented!("unsafety unsupported!")
    };

    Ok(t)
}

#[proc_macro]
pub fn genrs_fn(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input with syn::Block::parse_within);

    let tts = input.iter().filter_map(|stmt| {
	if let syn::Stmt::Item(syn::Item::Fn(item_fn )) = stmt {
	    match parse_fn_item(item_fn) {
		Ok(v) => Some(v),
		Err(e) => Some(e.to_compile_error()),
	    }
	} else {
	    None
	}
    });

    let tt = quote!{
	#(#tts)*
    };
    tt.into()
}
