use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromField)]
#[darling(attributes(osc))]
struct Field {
    ident: Option<proc_macro2::Ident>,
    ty: syn::Type,
    #[darling(default)]
    encoder: Option<syn::Path>,
    #[darling(default)]
    flatten: bool,
    #[darling(default)]
    len_prefix: Option<syn::Path>,
    #[darling(default)]
    skip: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(osc), forward_attrs(allow, cfg))]
struct Args {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<darling::util::Ignored, Field>,
    attrs: Vec<syn::Attribute>,
    #[darling(default)]
    address: Option<syn::LitStr>,
}

#[proc_macro_derive(Message, attributes(osc))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args: Args = FromDeriveInput::from_derive_input(&input).unwrap();
    if args.address.is_some() {
        impl_addressable(args).into()
    } else {
        impl_value(args).into()
    }
}

fn impl_addressable(args: Args) -> TokenStream2 {
    let Args {
        ident,
        mut generics,
        data,
        attrs,
        address,
    } = args;

    let address = address.unwrap();

    if !address.value().starts_with('/') {
        panic!("address must start with '/'");
    }

    let (impl_generics, type_generics, where_generics) = to_generics(&mut generics);

    let fields = to_fields(&data);

    quote!(
        #(#attrs)*
        impl #impl_generics euphony_osc::codec::encode::TypeEncoder<EuphonyOscMacrosBuffer> for #ident #type_generics #where_generics {
            fn encode_type(self, buffer: EuphonyOscMacrosBuffer) -> euphony_osc::codec::buffer::Result<(), EuphonyOscMacrosBuffer> {
                let address = unsafe {
                    euphony_osc::types::Address::new_unchecked(#address)
                };

                let arguments = (#(#fields,)*);

                euphony_osc::types::Packet {
                    address,
                    arguments,
                }.encode_type(buffer)
            }
        }
    )
}

fn impl_value(args: Args) -> TokenStream2 {
    let Args {
        ident,
        mut generics,
        data,
        attrs,
        ..
    } = args;

    let (impl_generics, type_generics, where_generics) = to_generics(&mut generics);

    let fields = to_fields(&data).collect::<Vec<_>>();

    quote!(
        #(#attrs)*
        impl #impl_generics euphony_osc::codec::encode::TypeEncoder<EuphonyOscMacrosBuffer> for #ident #type_generics #where_generics {
            fn encode_type(self, buffer: EuphonyOscMacrosBuffer) -> euphony_osc::codec::buffer::Result<(), EuphonyOscMacrosBuffer> {
                #(let (_, buffer) = #fields.encode_type(buffer)?;)*
                Ok(((), buffer))
            }
        }

        #(#attrs)*
        impl #impl_generics euphony_osc::types::Tagged<EuphonyOscMacrosBuffer> for #ident #type_generics #where_generics {
            fn encode_tag(&self, buffer: EuphonyOscMacrosBuffer) -> euphony_osc::codec::buffer::Result<(), EuphonyOscMacrosBuffer> {
                #(let (_, buffer) = #fields.encode_tag(buffer)?;)*
                Ok(((), buffer))
            }
        }
    )
}

fn to_generics(
    generics: &mut syn::Generics,
) -> (TokenStream2, syn::TypeGenerics, Option<&syn::WhereClause>) {
    generics.params.push(
        syn::parse2(quote!(
            EuphonyOscMacrosBuffer: euphony_osc::codec::encode::EncoderBuffer
        ))
        .unwrap(),
    );

    let impl_generics = generics.split_for_impl().0;
    let impl_generics = quote!(#impl_generics);

    generics.params.pop().unwrap();

    let (_, type_generics, where_generics) = generics.split_for_impl();

    (impl_generics, type_generics, where_generics)
}

fn to_fields(
    data: &darling::ast::Data<darling::util::Ignored, Field>,
) -> impl Iterator<Item = TokenStream2> + '_ {
    match data {
        darling::ast::Data::Struct(fields) => fields
            .fields
            .iter()
            .filter(|f| !f.skip)
            .enumerate()
            .map(|(idx, field)| {
                let Field {
                    ident,
                    encoder,
                    flatten,
                    len_prefix,
                    ..
                } = field;

                let mut tokens = if let Some(ident) = ident {
                    quote!(self.#ident)
                } else {
                    let idx = syn::Index::from(idx);
                    quote!(((self).#idx))
                };

                if *flatten {
                    tokens = quote!(euphony_osc::types::Flatten(#tokens));
                }

                if let Some(len_prefix) = len_prefix {
                    tokens = quote!(<euphony_osc::types::LenPrefix<#len_prefix, _>>::new(#tokens));
                }

                if let Some(encoder) = encoder {
                    tokens = quote!(#encoder(#tokens));
                }

                tokens
            }),

        _ => todo!("handle enums"),
    }
}
