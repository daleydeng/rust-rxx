use crate::{get_attr, ReturnType};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::spanned::Spanned;

fn get_path_segment(t: &syn::Type) -> Option<&syn::PathSegment> {
    let syn::Type::Path(syn::TypePath {path: syn::Path{segments, ..}, ..}) = t else {
        return None;
    };

    if segments.len() != 1 {
        return None;
    }

    segments.first()
}

fn deref_pointer_type(t: &syn::Type) -> Option<&syn::Type> {
    let seg = get_path_segment(t);

    let Some(syn::PathSegment{
        arguments: syn::PathArguments::AngleBracketed(
            syn::AngleBracketedGenericArguments{
                args, ..}), ..}) = seg else {
        return None;
    };

    if args.len() != 1 {
        return None;
    }

    let Some(syn::GenericArgument::Type(inside_t)) = args.first() else {
        return None;
    };

    Some(inside_t)
}

fn replace_type_token(
    ty: &syn::Type,
    type_dic: &HashMap<syn::Type, Box<syn::Type>>,
) -> Box<syn::Type> {
    let mut tt = TokenStream::new();
    for i in ty.into_token_stream().into_iter() {
        if let Some(t) = type_dic.get(&syn::parse_quote!(#i)) {
            tt.extend(t.to_token_stream());
        } else {
            tt.extend([i]);
        }
    }
    Box::new(syn::parse_quote!(#tt))
}

fn replace_type_dic(
    ty: &syn::Type,
    type_dic: &HashMap<syn::Type, Box<syn::Type>>,
) -> Box<syn::Type> {
    match ty {
        syn::Type::Reference(v) => Box::new(syn::Type::Reference(syn::TypeReference {
            elem: replace_type_dic(&v.elem, type_dic),
            ..v.clone()
        })),

        syn::Type::Ptr(v) => Box::new(syn::Type::Ptr(syn::TypePtr {
            elem: replace_type_dic(&v.elem, type_dic),
            ..v.clone()
        })),

        syn::Type::Path(_) => {
            if let Some(t) = type_dic.get(&syn::parse_quote!(#ty)) {
                t.clone()
            } else {
                Box::new(ty.clone())
            }
        }
        _ => unimplemented!("unsupported type replace {:?}", ty),
    }
}

pub fn parse_fn(
    attrs: &[syn::Attribute],
    vis: &syn::Visibility,
    sig: &syn::Signature,
    type_dic: Option<&HashMap<syn::Type, Box<syn::Type>>>,
    self_ty: Option<(&syn::Type, &[&syn::Lifetime])>,
) -> syn::Result<TokenStream> {
    let fn_name = &sig.ident;
    let mut link_name = fn_name.clone();
    let mut ret_mode = ReturnType::Object(());

    if let Some(meta_list) = get_attr(attrs)? {
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
                        ret_mode = ReturnType::Atomic(());
                    } else if p.is_ident("object") {
                        ret_mode = ReturnType::Object(());
                    } else if p.is_ident("new_ptr") {
                        ret_mode = ReturnType::NewPtr(());
                    }
                }
                m => panic!("uncovered meta here {:?}", m),
            }
        }
    }

    let ret_type = match &sig.output {
        syn::ReturnType::Default => ReturnType::None,
        syn::ReturnType::Type(_, t) => {
            let mut t1 = t.clone();
            if let Some(type_dic) = type_dic {
                t1 = replace_type_dic(&t1, type_dic);
            }

            match ret_mode {
                ReturnType::Atomic(_) => ReturnType::Atomic(t1),
                ReturnType::Object(_) => ReturnType::Object(t1),
                ReturnType::NewPtr(_) => ReturnType::NewPtr(t1),
                _ => panic!("ret_mode invalid! {:?}", ret_mode),
            }
        }
    };

    let unsafety = &sig.unsafety;
    let mut c_decl_inputs = sig.inputs.clone();
    let mut c_call_inputs = sig
        .inputs
        .iter()
        .map(|i| match i {
            syn::FnArg::Typed(syn::PatType { pat, .. }) => quote! {#pat},
            syn::FnArg::Receiver(_) => quote! {self},
        })
        .collect::<syn::punctuated::Punctuated<_, syn::Token![,]>>();

    for i in &mut c_decl_inputs {
        if let syn::FnArg::Typed(syn::PatType { ty, .. }) = i {
            if let Some(type_dic) = type_dic {
                *ty = replace_type_dic(ty, type_dic);
            }
        }
    }

    let mut fn_generics = sig.generics.clone();

    if let ReturnType::Object(_) = ret_type {
        if !c_decl_inputs.empty_or_trailing() {
            c_decl_inputs.push_punct(syn::Token![,](Span::call_site()));
        }
        if !c_call_inputs.empty_or_trailing() {
            c_call_inputs.push_punct(syn::Token![,](Span::call_site()));
        }
    }

    if let Some(decl_first) = c_decl_inputs.first_mut() {
        let decl_self: Option<syn::FnArg> = match decl_first {
            syn::FnArg::Receiver(v) => {
                let ref_ = match &v.reference {
                    Some((_, lt)) => Some(quote! { &#lt }),
                    _ => None,
                };
                let mut_ = v.mutability;
                let Some((self_ty, self_lt)) = self_ty else {
                    return Err(syn::Error::new(
                        decl_first.span(),
                        "self not in class envinrment"
                    ));
                };

                for i in self_lt {
                    fn_generics.params.push_value(syn::GenericParam::Lifetime(
                        syn::LifetimeDef::new((*i).clone()),
                    ));
                }

                Some(syn::parse_quote! {
                    this: #ref_ #mut_ #self_ty
                })
            }
            _ => None,
        };

        if let Some(decl_self) = decl_self {
            *decl_first = decl_self;
        }
    }

    let t = if unsafety.is_none() {
        match ret_type {
            ReturnType::Object(rt) => quote! {
                #vis #sig {
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

            ReturnType::Atomic(rt) => quote! {
                #vis #sig {
                extern "C" {
                    #[link_name = stringify!(#link_name)]
                    fn __func #fn_generics (#c_decl_inputs) -> #rt;
                }
                unsafe {
                    __func(#c_call_inputs)
                }
                }
            },

            ReturnType::NewPtr(rt) => {
                let Some(inside_t) = deref_pointer_type(&rt)  else {
                    return Err(syn::Error::new(
                        rt.span(),
                        "new_ptr required Pointee type"));
                };

                let Some(ptr_name) = get_path_segment(&rt) else {
                    return Err(syn::Error::new(
                        rt.span(),
                        "new_ptr required Pointer Name"));
                };
                let ptr_name = &ptr_name.ident;

                quote! {
                    #vis #sig {
                    extern "C" {
                        #[link_name = stringify!(#link_name)]
                        fn __func #fn_generics (#c_decl_inputs) -> *mut #inside_t;
                    }
                    unsafe {
                        #ptr_name::from_raw(__func(#c_call_inputs))
                    }
                    }
                }
            }
            ReturnType::None => quote! {
                #vis #sig {
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
        match ret_type {
            ReturnType::Object(rt) => quote! {
                #vis #sig {
                extern "C" {
                    #[link_name = stringify!(#link_name)]
                    fn __func #fn_generics (#c_decl_inputs __ret: *mut #rt);
                }
                let mut __ret = std::mem::MaybeUninit::<#rt>::uninit();
                let mut __ret_ptr = __ret.as_mut_ptr();
                __func (#c_call_inputs __ret_ptr);
                __ret.assume_init()
                }
            },

            ReturnType::Atomic(rt) => quote! {
                #vis #sig {
                extern "C" {
                    #[link_name = stringify!(#link_name)]
                    fn __func #fn_generics (#c_decl_inputs) -> #rt;
                }
                __func(#c_call_inputs)
                }
            },

            ReturnType::NewPtr(_) => todo!(),

            ReturnType::None => quote! {
                #vis #sig {
                extern "C" {
                    #[link_name = stringify!(#link_name)]
                    fn __func #fn_generics (#c_decl_inputs);
                }
                __func(#c_call_inputs)
                }
            },
        }
    };

    Ok(t)
}

pub fn parse_impl(item_impl: &syn::ItemImpl) -> syn::Result<TokenStream> {
    let self_ty = &item_impl.self_ty;
    let self_lt: Vec<_> = item_impl
        .generics
        .params
        .iter()
        .filter_map(|i| match i {
            syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. }) => Some(lifetime),
            _ => None,
        })
        .collect();

    let mut type_dic = HashMap::new();
    type_dic.insert(syn::parse_quote! {Self}, self_ty.clone());

    let tts = item_impl.items.iter().map(|i| match i {
        syn::ImplItem::Method(v) => {
            if v.block.stmts.is_empty() {
                parse_fn(
                    &v.attrs,
                    &v.vis,
                    &v.sig,
                    Some(&type_dic),
                    Some((self_ty, &self_lt)),
                )
                .unwrap_or_else(syn::Error::into_compile_error)
            } else {
                v.into_token_stream()
            }
        }
        syn::ImplItem::Type(v) => {
            let t = &v.ident;
            type_dic.insert(
                syn::parse_quote! {Self::#t},
                replace_type_token(&v.ty, &type_dic),
            );

            v.into_token_stream()
        }
        _ => unimplemented!("unimplemented for {:#?}", i),
    });

    let unsafety = &item_impl.unsafety;
    let generics = &item_impl.generics;
    let trait_ = match &item_impl.trait_ {
        Some((_, p, _)) => Some(quote! { #p for }),
        _ => None,
    };

    let tt = quote! {
    #unsafety impl #generics  #trait_ #self_ty {
            #(#tts)*
    }
    };
    Ok(tt)
}
