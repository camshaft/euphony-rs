use codec::decode::DecoderBuffer;
use euphony_sc_core::synthdef;
use heck::SnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use std::path::PathBuf;
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

        let target = if let Some(_) = input.parse::<Option<syn::Token![,]>>()? {
            input.parse()?
        } else {
            None
        };

        Ok(Self { name, path, target })
    }
}

#[proc_macro]
pub fn include_synthdef(input: TokenStream) -> TokenStream {
    let input: Input = parse_macro_input!(input);
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    include_synthdef_impl(input, &root)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn name_to_ident(name: &str, span: proc_macro2::Span) -> Ident {
    let mut name = name.to_snake_case();

    match name.as_ref() {
        "type" | "send" => name.push('_'),
        _ => {}
    }

    Ident::new(&name, span)
}

fn include_synthdef_impl(input: Input, root: &str) -> Result<TokenStream2, Error> {
    let span = input.name.span();
    let path = PathBuf::from(root).join(input.path.value());

    if !path.exists() {
        return Err(Error::new(span, format!("{:?} does not exist", path)));
    }

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
        .map(|name| name_to_ident(&name.name, span))
        .collect::<Vec<_>>();

    let field_values = synth
        .param_names
        .iter()
        .map(|name| {
            let id = name.index;
            let name = name_to_ident(&name.name, span);
            quote!(self.params.#name.map(|v| (osc::control::Id::Index(#id), v)))
        })
        .collect::<Vec<_>>();

    let field_debug_new = synth.param_names.iter().map(|name| {
        let id = name.index as usize;
        let name = name_to_ident(&name.name, span);
        let default = synth.params[id];
        quote!(if let Some(value) = self.params.#name {
            s.field(stringify!(#name), &value);
        } else {
            s.field(stringify!(#name), &osc::control::Value::from(#default));
        })
    });

    let synthname = &synth.name;

    Ok(quote!(
        pub mod #name {
            use euphony_sc::{osc, track::{self, Track}};
            use core::{fmt, ops};

            pub const fn new() -> New {
                Synth::new()
            }

            pub const fn load() -> Load {
                Synth::load()
            }

            pub struct Synth {
                id: osc::node::Id,
                track: track::Handle,
            }

            impl fmt::Debug for Synth {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.debug_tuple(concat!(module_path!(), "::Synth")).field(&self.id).finish()
                }
            }

            impl Synth {
                pub const DEFINITION: &'static [u8; #len] = #def;

                pub const fn new() -> New {
                    New::DEFAULT
                }

                pub fn set(&mut self) -> Set {
                    Set {
                        params: Params::DEFAULT,
                        id: self.id,
                        track: &self.track,
                    }
                }

                pub const fn load() -> Load {
                    Load::new()
                }
            }

            impl Drop for Synth {
                fn drop(&mut self) {
                    self.track.free(self.id)
                }
            }

            #[derive(Clone, Copy, PartialEq)]
            pub struct Params {
                #(
                    pub #fields: Option<osc::control::Value>,
                )*
            }

            impl Default for Params {
                fn default() -> Self {
                    Self {
                        #(#fields: None,)*
                    }
                }
            }

            impl fmt::Debug for Params {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let mut s = f.debug_struct(concat!(module_path!(), "::Params"));

                    #(
                        if let Some(value) = self.#fields {
                            s.field(stringify!(#fields), &value);
                        }
                    )*

                    s.finish()
                }
            }

            impl Params {
                pub const DEFAULT: Self = Self {
                    #(#fields: None,)*
                };

                #(
                    pub fn #fields<V: Into<osc::control::Value>>(&mut self, value: V) -> &mut Self {
                        self.#fields = Some(value.into());
                        self
                    }
                )*
            }

            #[derive(Clone, Copy, PartialEq)]
            pub struct New {
                pub params: Params,
                pub action: Option<osc::group::Action>,
                pub target: Option<osc::node::Id>,
            }

            impl Default for New {
                fn default() -> Self {
                    Self::DEFAULT
                }
            }

            impl New {
                pub const DEFAULT: Self = Self {
                    params: Params::DEFAULT,
                    action: None,
                    target: None,
                };

                #(
                    pub fn #fields<V: Into<osc::control::Value>>(&mut self, value: V) -> &mut Self {
                        self.params.#fields = Some(value.into());
                        self
                    }
                )*
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

                fn send(self, track: &track::Handle) -> Synth {
                    // make sure the server has the synthdef loaded
                    track.send(load());

                    let values = [
                        #(#field_values),*
                    ];

                    let id = track.new(#synthname, self.action, self.target, &values[..]);

                    Synth {
                        id,
                        track: track.clone()
                    }
                }
            }

            pub struct Set<'a> {
                pub params: Params,
                id: osc::node::Id,
                track: &'a track::Handle,
            }

            impl<'a> Set<'a> {
                #(
                    pub fn #fields<V: Into<osc::control::Value>>(&mut self, value: V) -> &mut Self {
                        self.params.#fields = Some(value.into());
                        self
                    }
                )*
            }

            impl<'a> fmt::Debug for Set<'a> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let mut s = f.debug_struct(concat!(module_path!(), "::Set"));

                    #(
                        if let Some(value) = self.params.#fields {
                            s.field(stringify!(#fields), &value);
                        }
                    )*

                    s.finish()
                }
            }

            impl<'a> euphony_sc::Message for Set<'a> {
                type Output = ();

                fn send(self, track: &track::Handle) {
                    let values = [
                        #(#field_values),*
                    ];

                    track.set(self.id, &values[..]);
                }
            }

            #[derive(Clone, Copy, PartialEq)]
            pub struct Load;

            impl Default for Load {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl Load {
                pub const fn new() -> Self {
                    Self
                }

                pub const fn as_osc(&self) -> osc::synthdef::Receive<'static> {
                    osc::synthdef::Receive {
                        buffer: Synth::DEFINITION
                    }
                }
            }

            impl fmt::Debug for Load {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.debug_struct(concat!(module_path!(), "::Load")).finish()
                }
            }

            impl euphony_sc::Message for Load {
                type Output = ();

                fn send(self, track: &track::Handle) {
                    track.load(#synthname, Synth::DEFINITION);
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
        include_synthdef_impl(input, env!("CARGO_MANIFEST_DIR")).unwrap();
    }
}
