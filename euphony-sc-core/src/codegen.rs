use crate::synthdef;
use codec::decode::DecoderBuffer;
use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::path::Path;
use syn::Error;

#[derive(Clone, Debug)]
pub struct Param<'a> {
    pub name: Ident,
    pub id: usize,
    pub default: f32,
    pub attrs: &'a [syn::Attribute],
}

pub fn name_to_ident(name: &str, span: proc_macro2::Span) -> Ident {
    let mut name = name.to_snake_case();

    match name.as_ref() {
        "type" | "send" => name.push('_'),
        _ => {}
    }

    Ident::new(&name, span)
}

pub fn include_synthdef(path: &Path, span: Span) -> TokenStream2 {
    include_synthdef_impl(path, span).unwrap_or_else(|err| err.into_compile_error())
}

fn include_synthdef_impl(path: &Path, span: Span) -> Result<TokenStream2, Error> {
    let buffer = std::fs::read(path).map_err(|err| Error::new(span, err))?;
    let buffer: &[u8] = buffer.as_ref();

    let (container, _) = buffer
        .decode::<synthdef::Container>()
        .map_err(|err| Error::new(span, err))?;

    let synth = container
        .defs
        .get(0)
        .ok_or_else(|| Error::new(span, &format!("synthdef {:?} is empty", path)))?;
    let path_str = path.to_str().unwrap();

    let params: Vec<_> = synth
        .param_names
        .iter()
        .map(|name| {
            let id = name.index as usize;
            let name = name_to_ident(&name.name, span);
            let default = synth.params[id];
            Param {
                name,
                id,
                default,
                attrs: &[],
            }
        })
        .collect();

    let params_impl = create_params(&[], &Ident::new("Params", span), &params);

    let synthname = &synth.name;

    Ok(quote!(
        #params_impl

        pub fn new() -> SynthDef {
            euphony_sc::_macro_support::Parameters::new(move |_create_params| {
                static SYNTHDEF: &[u8] = include_bytes!(#path_str);
                euphony_sc::_macro_support::external_synthdef(#synthname, SYNTHDEF)
            })
        }
    ))
}

pub fn create_synthdef<T: quote::ToTokens>(item: &T) -> TokenStream2 {
    quote!({
        fn __euphony_item_path__() {}
        fn __euphony_resolve_item_path__<T>(_: T) -> &'static str {
            ::core::any::type_name::<T>()
        }
        let euphony_synthdef_name = __euphony_resolve_item_path__(__euphony_item_path__)
            .strip_suffix("::__euphony_item_path__")
            .unwrap_or(module_path!());
        euphony_sc::_macro_support::Parameters::new(move |create_params| {
            static __SYNTHDEF: euphony_sc::_macro_support::SynthCell =
                euphony_sc::_macro_support::SynthCell::new();

            __SYNTHDEF.get_or_init(|| {
                euphony_sc::_macro_support::synthdef(euphony_synthdef_name, move || {
                    (#item)(create_params())
                })
            }).as_ref()
        })
    })
}

pub fn create_params(attrs: &[syn::Attribute], name: &Ident, parameters: &[Param]) -> TokenStream2 {
    let synthdef = Ident::new("SynthDef", name.span());
    let synth = Ident::new("Synth", name.span());

    let mut fields = quote!();
    let mut setters = quote!();
    let mut def_params = quote!();
    let mut instance_params = quote!();
    let mut defaults = quote!();
    let mut debug = quote!();
    let mut values = quote!();

    for param in parameters {
        let id = param.id as u32;
        let attrs = &param.attrs;
        let name = &param.name;
        let name_str = name.to_string();
        let default = &param.default;

        fields.extend(quote!(#(#attrs)* pub #name: euphony_sc::_macro_support::Param,));

        setters.extend(quote!(
            #(#attrs)*
            #[must_use]
            pub fn #name<Value: Into<euphony_sc::_macro_support::Param>>(mut self, #name: Value) -> Self {
                self.#name = #name.into();
                self
            }
        ));

        def_params
            .extend(quote!(#name: euphony_sc::_macro_support::param(#id, #name_str, #default),));

        instance_params.extend(quote!(#name: euphony_sc::_macro_support::param_instance(#id),));

        defaults.extend(quote!(#name: euphony_sc::_macro_support::Param::from(#default),));

        debug.extend(quote!(self.#name.debug_field(#name_str, &mut s);));

        let id = id as i32;
        values.extend(quote!(
            self
                .#name
                .control_value(track)
                .map(|value| (euphony_sc::osc::control::Id::Index(#id), value)),
        ));
    }

    quote!(
        #(#attrs)*
        pub struct #name<_Meta = ()> {
            #fields

            #[doc(hidden)]
            _meta: _Meta,
        }

        impl<_Meta> #name<_Meta> {
            #setters
        }

        /// A set of parameters with an attached synth definition
        pub type #synthdef = #name<euphony_sc::_macro_support::SynthDef<#name>>;

        impl euphony_sc::_macro_support::Parameters for #synthdef {
            type Desc = #name;

            /// Creates a set of parameters with an associated synthdef
            fn new<F>(desc: F) -> #synthdef
                where F: FnOnce(fn() -> #name) -> euphony_sc::_macro_support::SynthDescRef
            {
                fn create_params() -> #name {
                    #name {
                        #def_params
                        _meta: (),
                    }
                }

                let desc = desc(create_params);
                let _meta = <euphony_sc::_macro_support::SynthDef<#name>>::new(desc);
                // with instance params we can distinguish between set/unset
                #synthdef {
                    #instance_params
                    _meta,
                }
            }
        }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!(#name))
                    .finish()
            }
        }

        impl core::fmt::Debug for #synthdef {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let mut s = f.debug_struct(self._meta.name());

                #debug

                s.finish()
            }
        }

        impl euphony_sc::Message for #synthdef {
            type Output = #synth;

            fn send(self, track: &euphony_sc::track::Handle) -> Self::Output {
                use euphony_sc::track::Track;

                // make sure the server has the synthdef loaded
                let synthdef = self._meta.name();
                track.load(synthdef, self._meta.desc());

                let values = [
                    #values
                ];

                // TODO how to support these?
                let action = None;
                let target = None;

                let id = track.play(synthdef, action, target, &values[..]);

                let synth = euphony_sc::_macro_support::Synth::new(id, track.clone(), synthdef);

                #synth(synth)
            }
        }

        pub struct #synth(euphony_sc::_macro_support::Synth);

        impl core::fmt::Debug for #synth {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl #synth {
            #[must_use]
            pub fn set(&mut self) -> #name<&mut euphony_sc::_macro_support::Synth> {
                #name {
                    #instance_params
                    _meta: &mut self.0
                }
            }
        }

        impl euphony_sc::Message for #name<&mut euphony_sc::_macro_support::Synth> {
            type Output = ();

            fn send(self, track: &euphony_sc::track::Handle) -> Self::Output {
                use euphony_sc::track::Track;

                // the synthdef is already loaded so just update the values
                let values = [
                    #values
                ];

                track.set(self._meta.id(), &values[..]);
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codegen_test() {
        let path = Path::new("../artifacts/v1.scsyndef");
        include_synthdef(&path, Span::call_site());
    }
}
