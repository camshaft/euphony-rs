use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, Token,
};

struct Input(Punctuated<Expr, Token![,]>);

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let input = Punctuated::parse_terminated(input)?;
        Ok(Self(input))
    }
}

#[proc_macro]
pub fn p(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let mut output = TokenStream2::new();
    if let Err(err) = input.to_pattern(&mut output) {
        return err.to_compile_error().into();
    }
    quote!({
        #[allow(unused_imports)]
        use euphony_pattern::*;
        #output
    })
    .into()
}

trait ToPattern {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()>;
}

impl<'a, T: ToPattern> ToPattern for &'a T {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        T::to_pattern(self, out)
    }
}

impl<T: ToPattern> ToPattern for Option<T> {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        if let Some(t) = self.as_ref() {
            t.to_pattern(out)?;
        }
        Ok(())
    }
}

impl<T: ToPattern> ToPattern for Box<T> {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        self.as_ref().to_pattern(out)
    }
}

fn group<I: Iterator<Item = T>, T: ToPattern>(iter: I) -> Result<TokenStream2> {
    let mut exprs = TokenStream2::new();
    for child in iter {
        child.to_pattern(&mut exprs)?;
        exprs.extend(quote!(,));
    }
    Ok(quote!((#exprs)))
}

impl ToPattern for Input {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        if self.0.is_empty() {
            out.extend(quote!(Rest::new()));
            return Ok(());
        }
        let exprs = group(self.0.iter())?;
        out.extend(quote!(Group::new(#exprs)));
        Ok(())
    }
}

impl ToPattern for Expr {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        use Expr::*;

        match self {
            Array(e) => e.to_pattern(out)?,
            Binary(e) => e.to_pattern(out)?,
            Block(e) => e.to_tokens(out),
            Closure(e) => e.to_tokens(out),
            Index(e) => e.to_pattern(out)?,
            Lit(e) => e.to_pattern(out)?,
            Macro(e) => e.to_tokens(out),
            MethodCall(e) => e.to_pattern(out)?,
            Paren(e) => e.to_pattern(out)?,
            Path(e) => e.to_pattern(out)?,
            Range(e) => e.to_pattern(out)?,
            Reference(e) => e.to_pattern(out)?,
            Try(e) => e.to_pattern(out)?,
            Tuple(e) => e.to_pattern(out)?,
            Unsafe(e) => e.to_tokens(out),
            Verbatim(_) => todo!("underscore"),
            /*
            Box(ExprBox),
            Call(ExprCall),
            Cast(ExprCast),
            Closure(ExprClosure),
            Field(ExprField),
            Group(ExprGroup),
            If(ExprIf),
            Match(ExprMatch),
            Repeat(ExprRepeat),
            Struct(ExprStruct),
            Type(ExprType),
            Unary(ExprUnary),
            */
            _ => return Err(syn::parse::Error::new(self.span(), "invalid expression")),
        }

        Ok(())
    }
}

impl ToPattern for syn::BinOp {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        use syn::BinOp::*;
        let span = self.span();

        proc_macro2::Ident::new(
            match self {
                Add(_) => "hold",
                //Sub(_) => "todo",
                Mul(_) => "repeat",
                Div(_) => "slow",
                Rem(_) => "polym",
                /*
                Add(Add),
                Sub(Sub),
                Mul(Star),
                Div(Div),
                Rem(Rem),
                And(AndAnd),
                Or(OrOr),
                BitXor(Caret),
                BitAnd(And),
                BitOr(Or),
                Shl(Shl),
                Shr(Shr),
                Eq(EqEq),
                Lt(Lt),
                Le(Le),
                Ne(Ne),
                Ge(Ge),
                Gt(Gt),
                AddEq(AddEq),
                SubEq(SubEq),
                MulEq(MulEq),
                DivEq(DivEq),
                RemEq(RemEq),
                BitXorEq(CaretEq),
                BitAndEq(AndEq),
                BitOrEq(OrEq),
                ShlEq(ShlEq),
                ShrEq(ShrEq),
                        */
                _ => return Err(syn::parse::Error::new(self.span(), "invalid operator")),
            },
            span,
        )
        .to_tokens(out);

        Ok(())
    }
}

impl ToPattern for syn::ExprArray {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        if self.elems.is_empty() {
            out.extend(quote!(Rest::new()));
            return Ok(());
        }
        let exprs = group(self.elems.iter())?;
        out.extend(quote!(Group::new(#exprs)));
        Ok(())
    }
}

impl ToPattern for syn::ExprBinary {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        let span = self.span();
        out.extend(quote_spanned!(span => PatternExt::));
        self.op.to_pattern(out)?;
        let mut args = TokenStream2::new();
        self.left.to_pattern(&mut args)?;
        args.extend(quote!(,));
        self.right.to_pattern(&mut args)?;
        out.extend(quote!((#args)));
        Ok(())
    }
}

impl ToPattern for syn::ExprIndex {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        todo!()
    }
}

impl ToPattern for syn::ExprLit {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        let exprs = self;
        out.extend(quote!(Ident::new(#exprs)));
        Ok(())
    }
}

impl ToPattern for syn::ExprMethodCall {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        self.receiver.to_pattern(out)?;
        self.dot_token.to_tokens(out);
        self.method.to_tokens(out);
        self.turbofish.to_tokens(out);
        out.extend(group(self.args.iter())?);
        Ok(())
    }
}

impl ToPattern for syn::ExprParen {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        let mut t = TokenStream2::new();
        self.expr.to_pattern(&mut t)?;
        out.extend(quote!((#t)));
        Ok(())
    }
}

impl ToPattern for syn::ExprPath {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        let mut t = TokenStream2::new();
        self.to_tokens(&mut t);
        out.extend(quote!(Ident::new(#t)));
        Ok(())
    }
}

impl ToPattern for syn::ExprRange {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        use syn::RangeLimits::*;
        self.from.to_pattern(out)?;
        match self.limits {
            HalfOpen(t) => t.to_tokens(out),
            Closed(t) => t.to_tokens(out),
        }
        self.to.to_pattern(out)?;
        Ok(())
    }
}

/// References are spliced in as patterns
impl ToPattern for syn::ExprReference {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        self.expr.to_tokens(out);
        Ok(())
    }
}

impl ToPattern for syn::ExprTry {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        self.expr.to_pattern(out)?;
        out.extend(quote!(.degrade()));
        Ok(())
    }
}

impl ToPattern for syn::ExprTuple {
    fn to_pattern(&self, out: &mut TokenStream2) -> Result<()> {
        if self.elems.is_empty() {
            out.extend(quote!(Rest::new()));
            return Ok(());
        }
        let exprs = group(self.elems.iter())?;
        out.extend(quote!(Alternate::new(#exprs)));
        Ok(())
    }
}
