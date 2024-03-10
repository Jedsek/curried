#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use std::str::FromStr;
use syn::{parse_macro_input, Block, FnArg, ItemFn, Pat, ReturnType, Type};

#[proc_macro]
pub fn to_curry(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let (mut fn_name, mut body) = (None, None);
    let mut not_in_body = true;
    let mut args = vec![];

    for tt in input {
        match tt {
            TokenTree::Group(group) => {
                body = Some(group);
                break;
            }
            TokenTree::Ident(ident) if not_in_body => {
                fn_name = Some(ident);
                not_in_body = false;
            }
            _ => (),
        }
    }

    for tt in body.clone().unwrap().stream().into_iter() {
        if let TokenTree::Ident(ident) = tt {
            args.push(ident)
        }
    }
    let body = body.unwrap();

    let closure = quote!(
        #( move |#args| )* #fn_name #body
    );

    closure.into()
}

#[proc_macro_attribute]
pub fn curry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as ItemFn);

    let (body, sig, vis) = (parsed.block, parsed.sig, parsed.vis);

    let fn_return_type = sig.output;
    let (fn_ident, fn_args) = (sig.ident, sig.inputs);
    let (impl_generics, _ty_generics, where_clause) = sig.generics.split_for_impl();

    let mut arg_idents = vec![];
    let mut arg_types = vec![];
    for arg in fn_args {
        let (ident, ty) = match arg {
            FnArg::Typed(p) => (p.pat, p.ty),
            FnArg::Receiver(_) => panic!("self parameter is unsupported now"),
        };
        arg_idents.push(ident);
        arg_types.push(ty);
    }

    let return_type = generate_return_type(&arg_types, fn_return_type);
    let body = generate_body(&arg_idents, &arg_types, body);
    let first_arg_ident = arg_idents.first();
    let first_arg_type = arg_types.first();

    let first_arg = if first_arg_ident.is_some() {
        quote!(#first_arg_ident: #first_arg_type)
    } else {
        quote!()
    };

    let new_fn = quote!(
        #vis fn #fn_ident #impl_generics (#first_arg) #return_type #where_clause {
            #body
        }
    );

    // println!("{}", new_fn);

    new_fn.into()
}

fn generate_return_type(
    types: &[Box<Type>],
    fn_return_type: ReturnType,
) -> proc_macro2::TokenStream {
    let last = types.len();
    let range = 1..last;
    let len = range.len();

    let types = types.get(range).unwrap_or_default();

    let fn_return_type = quote!(#fn_return_type).to_string();
    let mut token_stream = String::new();
    for ty in types {
        let ty = quote!(#ty);
        token_stream += &format!("-> Box<dyn FnOnce({ty})");
    }
    token_stream += &fn_return_type;
    token_stream += &">".repeat(len);

    proc_macro2::TokenStream::from_str(&token_stream).unwrap()
}

fn generate_body(
    idents: &[Box<Pat>],
    types: &[Box<Type>],
    body: Box<Block>,
) -> proc_macro2::TokenStream {
    let last = types.len();
    let range = 1..last;
    let len = range.len();

    let idents = idents.get(range.clone()).unwrap_or_default();
    let types = types.get(range).unwrap_or_default();

    let body = quote!(#body);
    let mut token_stream = String::new();
    for (id, ty) in idents.iter().zip(types.iter()) {
        let (ident, ty) = (quote!(#id), quote!(#ty));
        token_stream += &format!("Box::new( move |{ident}: {ty}| ");
    }
    token_stream += &format!("{body}");
    token_stream += &")".repeat(len);

    proc_macro2::TokenStream::from_str(&token_stream).unwrap()
}
