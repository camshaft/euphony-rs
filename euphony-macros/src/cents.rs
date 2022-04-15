use num_rational::Ratio;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result},
    LitInt, Token,
};

pub struct Cents {
    numerator: LitInt,
    denominator: Option<LitInt>,
}

impl Parse for Cents {
    fn parse(stream: ParseStream) -> Result<Self> {
        let numerator = stream.parse()?;
        let denominator = if stream.peek(Token![/]) {
            Some(stream.parse()?)
        } else {
            None
        };
        Ok(Self {
            numerator,
            denominator,
        })
    }
}

impl ToTokens for Cents {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let span = self.numerator.span();
        let numerator: u64 = self.numerator.base10_parse().unwrap();
        let denominator = self
            .denominator
            .as_ref()
            .map(|v| v.base10_parse().unwrap())
            .unwrap_or(1);
        let ratio = Ratio::new(numerator, denominator) / 1200;
        let pow = *ratio.numer() as f64 / *ratio.denom() as f64;
        let freq = 2.0f64.powf(pow);
        stream.append_all(quote_spanned! { span => {
           #[allow(clippy::approx_constant)]
           #freq
        }})
    }
}
