use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_quote,
    punctuated::Punctuated,
    Arm, Expr, Ident, Path, Token,
};

pub struct DispatchStruct {
    name: Path,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for DispatchStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;

        let content;
        braced!(content in input);
        let fields = Punctuated::parse_terminated(&content)?;

        Ok(Self { name, fields })
    }
}

impl ToTokens for DispatchStruct {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;

        if self.fields.is_empty() {
            tokens.extend(quote!(Box::new(#name { })));
            return;
        }

        let expr = self.fields.iter().map(|field| &field.expr);

        let patterns: Vec<_> = self
            .fields
            .iter()
            .map(|field| field.arms.iter().collect::<Vec<_>>())
            .collect();
        let patterns: Vec<_> = patterns.iter().map(|pattern| &pattern[..]).collect();
        let patterns = permutate::Permutator::new(&patterns[..]).map(|fields| {
            let pats = fields.iter().map(|f| &f.1.pat);
            let mut guards = fields
                .iter()
                .filter_map(|f| f.1.guard.as_ref().map(|(_, expr)| expr))
                .peekable();
            let cons = fields.iter().map(|(name, arm)| {
                let body = &arm.body;
                quote!(#name: #body,)
            });

            let if_guard = if guards.peek().is_some() {
                quote!(if)
            } else {
                quote!()
            };

            quote!((#(#pats,)*) #if_guard #(#guards)&&* => Box::new(#name { #(#cons)* }),)
        });

        tokens.extend(quote!(
            match (#(#expr,)*) {
                #(#patterns)*
            }
        ));
    }
}

pub struct Field {
    expr: Box<Expr>,
    arms: Vec<(Ident, Arm)>,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let _: Token![:] = input.parse()?;
        let Value { expr, arms } = Value::parse(input, &name)?;
        let arms = arms.into_iter().map(|arm| (name.clone(), arm)).collect();

        Ok(Self { expr, arms })
    }
}

struct Value {
    expr: Box<Expr>,
    arms: Vec<Arm>,
}

impl Value {
    fn parse(input: ParseStream, name: &Ident) -> Result<Self> {
        let value = input.parse()?;

        Ok(match value {
            Expr::Match(m) => Self {
                expr: m.expr,
                arms: m.arms,
            },
            other => Self {
                expr: Box::new(other),
                arms: vec![Arm {
                    attrs: vec![],
                    pat: parse_quote!(#name),
                    guard: None,
                    fat_arrow_token: Token![=>](Span::call_site()),
                    body: parse_quote!(#name),
                    comma: None,
                }],
            },
        })
    }
}
