//! Utility to test the behavior of lints when the code originates from an external macro.
//!
//! From: https://github.com/rust-lang/rust-clippy/blob/master/tests/ui/auxiliary/proc_macros.rs
extern crate proc_macro;
use proc_macro::{
    token_stream::IntoIter,
    Delimiter::{self, Parenthesis},
    Group, Ident, Literal, Punct,
    Spacing::{self, Alone},
    Span, TokenStream, TokenTree as TT,
};

type Result<T> = core::result::Result<T, TokenStream>;

/// Token used to escape the following token from the macro's span rules.
const ESCAPE_CHAR: char = '$';

/// Takes a sequence of tokens and return the tokens with the span set such that they appear to be
/// from an external macro. Tokens may be escaped with either `$ident` or `$(tokens)`.
#[proc_macro]
pub fn external(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    if let Err(e) = write_with_span(Span::mixed_site(), input.into_iter(), &mut res) {
        e
    } else {
        res
    }
}

/// Make a `compile_error!` pointing to the given span.
fn make_error(msg: &str, span: Span) -> TokenStream {
    TokenStream::from_iter([
        TT::Ident(Ident::new("compile_error", span)),
        TT::Punct(punct_with_span('!', Alone, span)),
        TT::Group({
            let mut msg = Literal::string(msg);
            msg.set_span(span);
            group_with_span(
                Parenthesis,
                TokenStream::from_iter([TT::Literal(msg)]),
                span,
            )
        }),
    ])
}

/// Copies all the tokens, replacing all their spans with the given span. Tokens can be escaped
/// either by `$ident` or `$(tokens)`.
fn write_with_span(s: Span, mut input: IntoIter, out: &mut TokenStream) -> Result<()> {
    while let Some(tt) = input.next() {
        match tt {
            TT::Punct(p) if p.as_char() == ESCAPE_CHAR => {
                expect_tt(
                    input.next(),
                    |tt| match tt {
                        tt @ (TT::Ident(_) | TT::Literal(_)) => {
                            out.extend([tt]);
                            Some(())
                        }
                        TT::Punct(mut p) if p.as_char() == ESCAPE_CHAR => {
                            p.set_span(s);
                            out.extend([TT::Punct(p)]);
                            Some(())
                        }
                        TT::Group(g) if g.delimiter() == Parenthesis => {
                            out.extend([TT::Group(group_with_span(
                                Delimiter::None,
                                g.stream(),
                                g.span(),
                            ))]);
                            Some(())
                        }
                        _ => None,
                    },
                    "an ident, a literal, or parenthesized tokens",
                    p.span(),
                )?;
            }
            TT::Group(g) => {
                let mut stream = TokenStream::new();
                write_with_span(s, g.stream().into_iter(), &mut stream)?;
                out.extend([TT::Group(group_with_span(g.delimiter(), stream, s))]);
            }
            mut tt => {
                tt.set_span(s);
                out.extend([tt]);
            }
        }
    }
    Ok(())
}

fn expect_tt<T>(
    tt: Option<TT>,
    f: impl FnOnce(TT) -> Option<T>,
    expected: &str,
    span: Span,
) -> Result<T> {
    match tt {
        None => Err(make_error(
            &format!("unexpected end of input, expected {expected}"),
            span,
        )),
        Some(tt) => {
            let span = tt.span();
            match f(tt) {
                Some(x) => Ok(x),
                None => Err(make_error(
                    &format!("unexpected token, expected {expected}"),
                    span,
                )),
            }
        }
    }
}

fn punct_with_span(c: char, spacing: Spacing, span: Span) -> Punct {
    let mut p = Punct::new(c, spacing);
    p.set_span(span);
    p
}

fn group_with_span(delimiter: Delimiter, stream: TokenStream, span: Span) -> Group {
    let mut g = Group::new(delimiter, stream);
    g.set_span(span);
    g
}
