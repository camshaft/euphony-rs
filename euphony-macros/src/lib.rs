use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod cents;
mod main_;
mod mode_system;
mod node;

#[proc_macro]
pub fn cents(input: TokenStream) -> TokenStream {
    let cents = parse_macro_input!(input as cents::Cents);
    quote!(#cents).into()
}

#[proc_macro]
pub fn mode_system(input: TokenStream) -> TokenStream {
    let system = parse_macro_input!(input as mode_system::ModeSystem);
    quote!(#system).into()
}

#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let main = parse_macro_input!(input as main_::Main);
    quote!(#main).into()
}

#[proc_macro_derive(Node, attributes(node, input))]
pub fn derive_processor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    node::Node::parse(&input).into()
}
