use num_rational::Ratio;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, LitInt, Token,
};

#[proc_macro]
pub fn mode_system(input: TokenStream) -> TokenStream {
    let system = parse_macro_input!(input as ModeSystem);
    quote!(#system).into()
}

struct ModeSystem {
    pub_token: Option<Token![pub]>,
    name: Ident,
    eq: Token![=],
    steps: ModeSteps,
}

impl Parse for ModeSystem {
    fn parse(stream: ParseStream) -> Result<Self> {
        let pub_token = stream.parse()?;
        let name = stream.parse()?;
        let eq = stream.parse()?;
        let content;
        bracketed!(content in stream);
        let steps = content.parse()?;
        Ok(Self {
            pub_token,
            name,
            eq,
            steps,
        })
    }
}

impl ToTokens for ModeSystem {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let pub_token = &self.pub_token;
        let name = &self.name;
        let eq = &self.eq;
        let steps = &self.steps;
        stream.append_all(quote! {
            #pub_token const #name: crate::pitch::mode::system::ModeSystem #eq
                crate::pitch::mode::system::ModeSystem(&[#steps]);
        })
    }
}

struct ModeSteps {
    steps: Vec<usize>,
    tone_count: usize,
}

impl ModeSteps {
    fn iter(&self, shift: usize) -> impl Iterator<Item = &usize> {
        self.steps[shift..].iter().chain(self.steps[..shift].iter())
    }

    fn steps<'a>(&'a self, shift: usize) -> impl Iterator<Item = Ratio<usize>> + 'a {
        let tone_count = self.tone_count;
        self.iter(shift).map(move |i| Ratio::new(*i, tone_count))
    }

    fn intervals<'a>(&'a self, shift: usize) -> impl Iterator<Item = Ratio<usize>> + 'a {
        let tone_count = self.tone_count;
        self.iter(shift).scan(0, move |state, i| {
            let value = Ratio::new(*state, tone_count);
            *state += i;
            Some(value)
        })
    }

    fn tones<'a>(&'a self, shift: usize) -> impl Iterator<Item = Ratio<usize>> + 'a {
        let step_count = self.steps.len();
        self.iter(shift)
            .enumerate()
            .map(move |(i, step)| (0..*step).map(move |_| Ratio::new(i, step_count)))
            .flatten()
    }
}

#[test]
fn steps_test() {
    let mode = ModeSteps {
        steps: vec![2, 1, 2, 2, 1, 2, 2],
        tone_count: 12,
    };
    assert_eq!(
        mode.steps(0).collect::<Vec<_>>(),
        vec![
            Ratio::new(2, 12),
            Ratio::new(1, 12),
            Ratio::new(2, 12),
            Ratio::new(2, 12),
            Ratio::new(1, 12),
            Ratio::new(2, 12),
            Ratio::new(2, 12),
        ]
    );
    assert_eq!(
        mode.intervals(0).collect::<Vec<_>>(),
        vec![
            Ratio::new(0, 12),
            Ratio::new(2, 12),
            Ratio::new(3, 12),
            Ratio::new(5, 12),
            Ratio::new(7, 12),
            Ratio::new(8, 12),
            Ratio::new(10, 12),
        ]
    );
    assert_eq!(
        mode.tones(0).collect::<Vec<_>>(),
        vec![
            Ratio::new(0, 7),
            Ratio::new(0, 7),
            Ratio::new(1, 7),
            Ratio::new(2, 7),
            Ratio::new(2, 7),
            Ratio::new(3, 7),
            Ratio::new(3, 7),
            Ratio::new(4, 7),
            Ratio::new(5, 7),
            Ratio::new(5, 7),
            Ratio::new(6, 7),
            Ratio::new(6, 7),
        ]
    );
}

impl Parse for ModeSteps {
    fn parse(stream: ParseStream) -> Result<Self> {
        let steps: Punctuated<LitInt, Token![,]> = Punctuated::parse_terminated(stream)?;
        let steps: Vec<_> = steps
            .iter()
            .map(|step| step.base10_parse().unwrap())
            .collect();
        let tone_count = steps.iter().sum();
        Ok(Self { steps, tone_count })
    }
}

impl ToTokens for ModeSteps {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        stream.append_all((0..self.steps.len()).map(|shift| {
            let steps = self.steps(shift).map(ratio_to_interval);
            let tones = self.tones(shift).map(ratio_to_interval);
            let intervals = self.intervals(shift).map(ratio_to_interval);
            quote! {
                crate::pitch::mode::intervals::ModeIntervals {
                    tones: &[#(#tones),*],
                    steps: &[#(#steps),*],
                    intervals: &[#(#intervals),*],
                },
            }
        }));
    }
}

fn ratio_to_interval(ratio: Ratio<usize>) -> TokenStream2 {
    let (numer, denom) = ratio.into();
    let numer = numer as i64;
    let denom = denom as i64;
    quote!(crate::pitch::interval::Interval(#numer, #denom))
}
