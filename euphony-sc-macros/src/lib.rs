use euphony_sc_core::codegen::{create_params, create_synthdef, Param};
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Token,
};

#[proc_macro]
pub fn synthdef(input: TokenStream) -> TokenStream {
    let item: SynthDefInput = parse_macro_input!(input);
    item.to_tokens()
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

struct SynthDefInput {
    def: Box<syn::ItemFn>,
    drop: Option<Box<syn::ItemFn>>,
}

impl SynthDefInput {
    fn to_tokens(&self) -> syn::parse::Result<TokenStream2> {
        synthdef_fn_impl(&self.def, self.drop.as_deref())
    }
}

impl Parse for SynthDefInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let def = input.parse()?;
        let def = Box::new(def);

        let drop = if input.peek(Token![fn]) {
            let drop = input.parse()?;
            let drop = Box::new(drop);
            Some(drop)
        } else {
            None
        };

        Ok(Self { def, drop })
    }
}

fn synthdef_fn_impl(
    item: &syn::ItemFn,
    drop: Option<&syn::ItemFn>,
) -> syn::parse::Result<TokenStream2> {
    let span = item.span();
    let attrs = &item.attrs;
    let vis = &item.vis;
    let name = &item.sig.ident;
    let args = &item.sig.inputs;
    let block = &item.block;

    let mut params = vec![];

    for (id, arg) in args.iter().enumerate() {
        let (name, default) = if let syn::FnArg::Typed(arg) = arg {
            let name = if let syn::Pat::Ident(name) = arg.pat.as_ref() {
                name.ident.clone()
            } else {
                panic!("invalid arg");
            };
            let default = type_to_default(arg.ty.as_ref())?;
            (name, default)
        } else {
            panic!("invalid param");
        };

        params.push(Param {
            name,
            id,
            default,
            attrs: &[], // TODO
        });
    }

    let params_impl = create_params(&[], &Ident::new("Params", span), &params);

    let def_params = params.iter().map(|param| {
        let name = &param.name;
        quote!(let #name = __euphony_params.#name.value();)
    });

    let load = quote!(|__euphony_params: Params| {
        use euphony_sc::_macro_support::ugen::prelude::*;
        use super::*;
        #(#def_params)*
        #block
    });

    let drop = if let Some(drop) = drop {
        let mut drop = drop.clone();
        drop.sig.ident = Ident::new("_drop", drop.sig.ident.span());
        quote!(|synth: Synth| {
            #drop
            _drop(synth)
        })
    } else {
        quote!(|synth: Synth| {
            synth.free();
        })
    };

    let def = create_synthdef(&load, &drop);

    Ok(quote!(
        #(#attrs)* #vis mod #name {
            use super::*;

            #params_impl

            pub fn new() -> SynthDef {
                #def
            }
        }
        #vis use #name::new as #name;
    ))
}

#[proc_macro]
pub fn params(input: TokenStream) -> TokenStream {
    let item: syn::ItemStruct = parse_macro_input!(input);
    match params_impl(&item) {
        Ok(out) => out,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn params_impl(item: &syn::ItemStruct) -> syn::parse::Result<TokenStream2> {
    let attrs = &item.attrs;
    let name = &item.ident;
    let mut params = vec![];

    for (id, field) in item.fields.iter().enumerate() {
        let id = id as _;
        let attrs = &field.attrs;
        let name = field.ident.as_ref().unwrap().clone();
        let default = type_to_default(&field.ty)?;
        params.push(Param {
            id,
            name,
            default,
            attrs,
        });
    }

    let out = euphony_sc_core::codegen::create_params(attrs, name, &params);

    Ok(out)
}

fn type_to_default(ty: &syn::Type) -> syn::parse::Result<f32> {
    let span = ty.span();
    if let syn::Type::Path(path) = ty {
        let segment = &path.path.segments[0];
        // TODO assert is `f32`
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            if let syn::GenericArgument::Const(syn::Expr::Lit(syn::ExprLit { lit, .. })) =
                args.args.first().unwrap()
            {
                match lit {
                    syn::Lit::Float(v) => return Ok(v.base10_parse().unwrap()),
                    syn::Lit::Int(v) => return Ok(v.base10_parse().unwrap()),
                    _ => {}
                }
            }
        } else {
            return Ok(0.0);
        }
    };

    Err(syn::parse::Error::new(span, "invalid parameter type"))
}
