use codec::decode::DecoderBuffer;
use euphony_sc_core::synthdef;
use heck::SnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error,
};

#[derive(Debug)]
struct Input {
    name: proc_macro2::Ident,
    path: syn::LitStr,
    target: Option<syn::LitStr>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let path = input.parse()?;
        let target = input.parse()?;
        Ok(Self { name, path, target })
    }
}

#[proc_macro]
pub fn include_synthdef(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    include_synthdef_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn include_synthdef_impl(input: Input) -> Result<TokenStream2, Error> {
    let span = input.name.span();
    let path = input.path.value();
    let buffer = std::fs::read(&path).map_err(|err| Error::new(span, err))?;
    let buffer: &[u8] = buffer.as_ref();

    let (container, _) = buffer
        .decode::<synthdef::Container>()
        .map_err(|err| Error::new(span, err))?;

    let synth = if let Some(target) = input.target {
        container
            .defs
            .iter()
            .find(|def| def.name == target.value())
            .ok_or_else(|| Error::new(span, &format!("could not find synthdef {:?}", target)))?
    } else {
        container
            .defs
            .iter()
            .next()
            .ok_or_else(|| Error::new(span, &format!("synthdef {:?} is empty", path)))?
    };

    let name = input.name;
    let len = buffer.len();
    let def = syn::LitByteStr::new(buffer, span);

    let fields = synth
        .param_names
        .iter()
        .map(|name| Ident::new(&name.name.to_snake_case(), span))
        .collect::<Vec<_>>();

    let field_values = synth
        .param_names
        .iter()
        .map(|name| {
            let id = name.index;
            let name = Ident::new(&name.name.to_snake_case(), span);
            quote!(self.#name.map(|v| (osc::control::Id::Index(#id), v)))
        })
        .collect::<Vec<_>>();

    let field_debug_new = synth.param_names.iter().map(|name| {
        let id = name.index as usize;
        let name = name.name.to_snake_case();
        let ident = Ident::new(&name, span);
        let default = synth.params[id];
        quote!(if let Some(value) = self.#ident {
            s.field(#name, &value);
        } else {
            s.field(#name, &#default);
        })
    });

    let synthname = &synth.name;

    Ok(quote!(
        pub mod #name {
            use euphony_sc::{osc, server};
            use core::fmt;

            pub struct Synth {
                id: osc::node::Id,
                server: server::Handle,
            }

            impl fmt::Debug for Synth {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.debug_tuple(concat!(module_path!(), "::Synth")).field(&self.id).finish()
                }
            }

            impl Synth {
                pub const DEF: &'static [u8; #len] = #def;

                pub const fn new() -> New {
                    New::new()
                }

                pub fn set(&mut self) -> Set {
                    Set::new(self.id, &self.server)
                }

                pub const fn receive() -> osc::synthdef::Receive<'static> {
                    osc::synthdef::Receive {
                        buffer: Self::DEF,
                    }
                }
            }

            impl Drop for Synth {
                fn drop(&mut self) {
                    self.server.free(self.id)
                }
            }

            pub const fn new() -> New {
                Synth::new()
            }

            pub const fn receive() -> osc::synthdef::Receive<'static> {
                Synth::receive()
            }

            #[derive(Clone, Copy, PartialEq)]
            pub struct New {
                #(
                    pub #fields: Option<osc::control::Value>,
                )*
                action: Option<osc::group::Action>,
                target: Option<osc::node::Id>,
            }

            impl Default for New {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl New {
                pub const fn new() -> Self {
                    Self {
                        #(#fields: None,)*
                        action: None,
                        target: None,
                    }
                }

                #(
                    pub fn #fields<V: Into<osc::control::Value>>(&mut self, value: V) -> &mut Self {
                        self.#fields = Some(value.into());
                        self
                    }
                )*

                pub fn send(self) -> Synth {
                    euphony_sc::Message::send_current(self)
                }
            }

            impl fmt::Debug for New {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let mut s = f.debug_struct(concat!(module_path!(), "::New"));

                    #(#field_debug_new)*

                    s.finish()
                }
            }

            impl euphony_sc::Message for New {
                type Output = Synth;

                fn send(self, server: &server::Handle) -> Synth {
                    use euphony_sc::codec::encode::EncoderBuffer;

                    let id = server.assign();
                    let action = self.action.unwrap_or_else(|| server.default_add_action());
                    let target = self.target.unwrap_or_else(|| server.default_target());

                    let values = [
                        #(#field_values),*
                    ];

                    let mut buffer = server.alloc();

                    let (len, _) = (&mut buffer[..]).encode(osc::synth::NewOptional {
                        name: #synthname,
                        id,
                        action,
                        target,
                        values: &values[..],
                    }).unwrap();

                    buffer.truncate(len);
                    server.send(buffer);

                    Synth {
                        id,
                        server: server.clone()
                    }
                }
            }

            pub struct Set<'a> {
                #(
                    #fields: Option<osc::control::Value>,
                )*
                id: osc::node::Id,
                server: &'a server::Handle,
            }

            impl<'a> Set<'a> {
                pub fn new(id: osc::node::Id, server: &'a server::Handle) -> Self {
                    Self {
                        #(#fields: None,)*
                        id,
                        server,
                    }
                }

                #(
                    pub fn #fields<V: Into<osc::control::Value>>(&mut self, value: V) -> &mut Self {
                        self.#fields = Some(value.into());
                        self
                    }
                )*

                pub fn send(self) {
                    euphony_sc::Message::send_current(self)
                }
            }

            impl<'a> fmt::Debug for Set<'a> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let mut s = f.debug_struct(concat!(module_path!(), "::Set"));

                    #(
                        if let Some(value) = self.#fields {
                            s.field(stringify!(#fields), &value);
                        }
                    )*

                    s.finish()
                }
            }

            impl<'a> euphony_sc::Message for Set<'a> {
                type Output = ();

                fn send(self, server: &server::Handle) {
                    use euphony_sc::codec::encode::EncoderBuffer;

                    let values = [
                        #(#field_values),*
                    ];

                    let mut buffer = server.alloc();

                    let (len, _) = (&mut buffer[..]).encode(osc::node::SetOptional {
                        id: self.id,
                        controls: &values[..],
                    }).unwrap();

                    buffer.truncate(len);
                    server.send(buffer);
                }
            }
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = syn::parse2(quote!(
            MySynthDef,
            "../euphony-sc-core/artifacts/v1.scsyndef"
        ))
        .unwrap();
        include_synthdef_impl(input).unwrap();
    }
}
