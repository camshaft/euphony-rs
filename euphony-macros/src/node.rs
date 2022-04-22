use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, Attribute, DeriveInput, Ident, Token};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(node), forward_attrs(input, buffer, doc))]
pub struct Node {
    ident: Ident,
    id: syn::LitInt,
    module: Option<syn::Path>,
    attrs: Vec<syn::Attribute>,
}

impl Node {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        match Self::from_derive_input(input) {
            Ok(command) => quote!(#command),
            Err(err) => err.write_errors(),
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inputs: Vec<Input> = vec![];
        let buffers: Vec<()> = vec![];
        let mut has_error = false;
        let mut docs = String::new();
        for attr in &self.attrs {
            if attr.path.is_ident("doc") {
                match syn::parse2::<Doc>(attr.tokens.clone()) {
                    Ok(doc) => {
                        docs.push_str(&doc.contents.value());
                        docs.push('\n');
                    }
                    Err(err) => {
                        has_error = true;
                        err.to_compile_error().to_tokens(tokens);
                    }
                }
                continue;
            }

            if attr.path.is_ident("buffer") {
                // TODO
                continue;
            }

            match Attribute::parse_args(attr) {
                Ok(v) => {
                    inputs.push(v);
                }
                Err(err) => {
                    has_error = true;
                    err.to_compile_error().to_tokens(tokens);
                }
            }
        }

        if has_error {
            return;
        }

        let id = &self.id;
        let name = &self.ident;
        let name_str = self.ident.to_string();

        let module = if let Some(m) = self.module.as_ref() {
            let parts = m.segments.iter().map(|s| s.ident.to_string());
            quote!(vec![#(#parts.to_string()),*])
        } else {
            quote!(Default::default())
        };

        let test_name = Ident::new(
            &format!("euphony_node_test_{}", name_str),
            self.ident.span(),
        );

        let mut test_inputs = quote!();
        let mut process_inputs = quote!();
        let process_buffers = quote!();
        let mut triggers = quote!();
        let mut defaults = quote!();
        let mut input_len: usize = 0;

        for (id, input) in inputs.iter().enumerate() {
            let id = input.id.unwrap_or(id as u64);

            input.test(id, &mut test_inputs);
            input_len += 1;

            let default = input.default.unwrap_or(0.0);
            quote!(#default,).to_tokens(&mut defaults);

            // triggers are not passed on each process call
            if let Some(trigger) = input.trigger.as_ref() {
                quote!(#id => {
                    self.#trigger(value);
                    true
                })
                .to_tokens(&mut triggers);
                continue;
            }

            let id = id as usize;
            quote!(inputs.get(#id), ).to_tokens(&mut process_inputs);
        }

        let buffer_len = buffers.len();

        quote!(
            #[test]
            #[allow(non_snake_case)]
            fn #test_name() {
                let node = ::euphony_node::reflect::Node {
                    name: #name_str.to_string(),
                    module: #module,
                    impl_path: module_path!().to_string(),
                    id: #id,
                    inputs: vec![#test_inputs],
                    docs: #docs.to_string(),
                };

                node.test(env!("CARGO_MANIFEST_DIR"));
                ::insta::assert_debug_snapshot!(#name_str, node);
            }

            impl #name {
                #[inline]
                pub fn spawn() -> ::euphony_node::BoxProcessor {
                    ::euphony_node::spawn::<#input_len, #buffer_len, Self>(Self::default())
                }

                #[inline]
                pub fn validate_parameter(param: u64, value: ::euphony_node::ParameterValue) -> Result<(), ::euphony_node::Error> {
                    // TODO
                    let _ = param;
                    let _ = value;
                    Ok(())
                }
            }

            impl ::euphony_node::Node<#input_len, #buffer_len> for #name {
                const DEFAULTS: [f64; #input_len] = [#defaults];

                #[inline]
                fn trigger(&mut self, param: u64, value: f64) -> bool {
                    match param {
                        #triggers
                        _ => {
                            let _ = value;
                            false
                        }
                    }
                }

                #[inline]
                fn process(
                    &mut self,
                    inputs: ::euphony_node::Inputs<#input_len>,
                    buffers: ::euphony_node::Buffers<#buffer_len>,
                    output: &mut [::euphony_node::Sample],
                ) {
                    self.render(#process_inputs #process_buffers output);
                }

                // TODO add process_full
            }
        )
        .to_tokens(tokens)
    }
}

struct Doc {
    contents: syn::LitStr,
}

impl parse::Parse for Doc {
    fn parse(input: &parse::ParseBuffer) -> parse::Result<Self> {
        let _: Token![=] = input.parse()?;
        let contents = input.parse()?;
        Ok(Self { contents })
    }
}

mod kw {
    use syn::custom_keyword as kw;
    kw!(id);
    kw!(trigger);
    kw!(default);
}

#[derive(Debug)]
struct Input {
    name: Ident,
    id: Option<u64>,
    trigger: Option<Ident>,
    default: Option<f64>,
}

impl Input {
    fn test(&self, id: u64, tokens: &mut TokenStream) {
        let name = self.name.to_string();
        let default = self.default.unwrap_or(0.0);
        let trigger = self.trigger.is_some();
        quote!(
            ::euphony_node::reflect::Input {
                name: #name.to_string(),
                id: #id,
                trigger: #trigger,
                default: #default,
            },
        )
        .to_tokens(tokens)
    }
}

impl parse::Parse for Input {
    fn parse(parser: parse::ParseStream) -> parse::Result<Self> {
        let name = parser.parse()?;

        let mut input = Self {
            name,
            id: None,
            trigger: None,
            default: None,
        };

        while !parser.is_empty() {
            let _: Token![,] = parser.parse()?;

            let l = parser.lookahead1();
            if l.peek(kw::id) {
                let _: kw::id = parser.parse()?;
                let _: Token![=] = parser.parse()?;
                let id: syn::LitInt = parser.parse()?;
                let id = id.base10_parse()?;
                input.id = Some(id);
            } else if l.peek(kw::trigger) {
                let _: kw::trigger = parser.parse()?;
                let _: Token![=] = parser.parse()?;
                input.trigger = Some(parser.parse()?);
            } else if l.peek(kw::default) {
                let _: kw::default = parser.parse()?;
                let _: Token![=] = parser.parse()?;
                let id: syn::LitFloat = parser.parse()?;
                let id = id.base10_parse()?;
                input.default = Some(id);
            } else {
                return Err(l.error());
            }
        }

        Ok(input)
    }
}
