#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use std::str::FromStr;
use syn::{parse_macro_input, Block, FnArg, ItemFn, Pat, ReturnType, Type};

#[proc_macro]
pub fn to_curry(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let mut in_body = true;
    let mut f = None;
    let mut body = None;
    let mut args = vec![];

    for tt in input.into_iter() {
        match tt {
            TokenTree::Group(b) => body = Some(b),
            TokenTree::Punct(p) if p.as_char() == '|' => in_body = !in_body,
            TokenTree::Ident(ident) => {
                if in_body {
                    f = Some(ident)
                } else {
                    args.push(ident)
                }
            }
            _ => (),
        }
    }

    let f = f.unwrap();
    let body = body.unwrap();

    let gen = quote!(
        #( move |#args| )* #f #body
    );

    gen.into()
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
    for arg in fn_args.into_iter() {
        let (ident, ty) = match arg {
            FnArg::Typed(p) => (p.pat, p.ty),
            FnArg::Receiver(_) => panic!("self parameter is unsupported now"),
        };
        arg_idents.push(ident);
        arg_types.push(ty);
    }

    let return_type = generate_return_type(&arg_types, fn_return_type);
    let body = generate_body(&arg_idents, &arg_types, body);
    let first_arg_ident = arg_idents.first().unwrap();
    let first_arg_type = arg_types.first().unwrap();

    let new_fn = quote!(
        #vis fn #fn_ident #impl_generics (#first_arg_ident: #first_arg_type) #return_type #where_clause {
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

    let types = &types[range.clone()];

    let fn_return_type = quote!(#fn_return_type).to_string();
    let mut token_stream = String::new();
    for ty in types.iter() {
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

    let idents = &idents[range.clone()];
    let types = &types[range];

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
