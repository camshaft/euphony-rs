use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(cmd), forward_attrs(allow, cfg))]
pub struct Command {
    ident: Ident,
    data: darling::ast::Data<darling::util::Ignored, Field>,
}

impl Command {
    pub fn parse(input: &DeriveInput) -> TokenStream {
        match Self::from_derive_input(input) {
            Ok(command) => quote!(#command),
            Err(err) => err.write_errors(),
        }
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, data } = self;

        let name = ident.to_string();

        let fields = if let darling::ast::Data::Struct(fields) = data {
            fields
        } else {
            panic!("enums aren't supported");
        };

        let defaults = fields
            .iter()
            .enumerate()
            .map(|(idx, field)| field.impl_defaults(idx));

        let into_fields = fields
            .iter()
            .enumerate()
            .map(|(idx, field)| field.impl_into_field(idx));

        let from_fields = fields
            .iter()
            .enumerate()
            .map(|(idx, field)| field.impl_from_field(idx));

        tokens.extend(quote!(
            impl Default for #ident {
                fn default() -> Self {
                    Self {
                        #(#defaults),*
                    }
                }
            }

            impl From<#ident> for Node<'static> {
                fn from(value: #ident) -> Self {
                    Node {
                        name: #name,
                        inputs: vec![#(#into_fields),*]
                    }
                }
            }

            impl<'a> core::convert::TryFrom<Node<'a>> for #ident {
                type Error = ();

                fn try_from(node: Node<'a>) -> core::result::Result<Self, Self::Error> {
                    if node.name == #name {
                        Ok(Self {
                            #(#from_fields),*
                        })
                    } else {
                        Err(())
                    }
                }
            }

            impl core::ops::Add<NodeValue> for #ident {
                type Output = NodeValue;

                fn add(self, rhs: NodeValue) -> Self::Output {
                    // let value =                    Add(value, rhs)
                    todo!()
                }
            }
        ))
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(cmd))]
pub struct Field {
    ident: Option<Ident>,
    #[darling(default)]
    default: Option<syn::Lit>,
}

impl Field {
    fn impl_into_field(&self, idx: usize) -> TokenStream {
        let Self { ident, .. } = self;

        if let Some(ident) = ident {
            quote!(value.#ident)
        } else {
            let idx = syn::Index::from(idx);
            quote!(value.#idx)
        }
    }

    fn impl_from_field(&self, idx: usize) -> TokenStream {
        let Self { ident, .. } = self;

        if let Some(ident) = ident {
            quote!(#ident: node.inputs[#idx])
        } else {
            let idx_t = syn::Index::from(idx);
            quote!(#idx_t: node.inputs[#idx])
        }
    }

    fn impl_defaults(&self, idx: usize) -> TokenStream {
        let Self { ident, default } = self;

        let mut tokens = quote!();

        if let Some(ident) = ident {
            ident.to_tokens(&mut tokens);
        } else {
            syn::Index::from(idx).to_tokens(&mut tokens);
        }

        tokens.extend(quote!(:));

        if let Some(value) = default {
            tokens.extend(quote!(From::from(#value)));
        } else {
            tokens.extend(quote!(Default::default()));
        }

        tokens
    }
}
