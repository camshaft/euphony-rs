use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    if input.sig.ident == "main" && !input.sig.inputs.is_empty() {
        let msg = "the main function cannot accept arguments";
        return syn::Error::new_spanned(&input.sig.ident, msg)
            .to_compile_error()
            .into();
    }

    parse(input).unwrap_or_else(|e| e.to_compile_error().into())
}

fn parse(mut input: syn::ItemFn) -> Result<TokenStream, syn::Error> {
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

    Ok(result.into())
}
