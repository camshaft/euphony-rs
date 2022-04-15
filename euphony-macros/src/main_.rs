use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{self, Parse};

pub struct Main {
    input: syn::ItemFn,
}

impl Parse for Main {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let input = input.parse()?;
        Ok(Self { input })
    }
}

impl ToTokens for Main {
    fn to_tokens(&self, stream: &mut TokenStream) {
        let input = self.input.clone();

        let tokens = to_tokens(input).unwrap_or_else(|e| e.to_compile_error());
        stream.extend(tokens);
    }
}

pub fn to_tokens(mut input: syn::ItemFn) -> Result<TokenStream, syn::Error> {
    if input.sig.ident == "main" && !input.sig.inputs.is_empty() {
        let msg = "the main function cannot accept arguments";
        return Err(syn::Error::new_spanned(&input.sig.ident, msg));
    }

    let sig = &mut input.sig;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = input.vis;

    if sig.asyncness.is_none() {
        let msg = "the async keyword is missing from the function declaration";
        return Err(syn::Error::new_spanned(sig.fn_token, msg));
    }

    sig.asyncness = None;

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            euphony::runtime::Runtime::from_env().block_on(async #body)
        }
    };

    Ok(result)
}
