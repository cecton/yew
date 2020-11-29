#![allow(unreachable_code)]
#![allow(unused_variables)]

use super::HtmlBranch;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{braced, token, Expr, ExprParen, Pat, Token};

#[derive(Debug)]
pub struct HtmlMatch {
    match_token: Token![match],
    expr: ExprParen, // TODO should be Expr but it's not working
    _brace_token: token::Brace,
    arms: Vec<HtmlArm>,
}

impl PeekValue<()> for HtmlMatch {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "match").as_option()
    }
}

impl Parse for HtmlMatch {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let match_token = input.parse()?;
        //todo!("{:?}", input.parse::<Expr>()); // TODO should be Expr but it's not working
        let expr = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let mut arms = Vec::new();
        while let Ok(arm) = content.parse() {
            arms.push(arm);
        }

        Ok(Self {
            match_token,
            expr,
            _brace_token: brace_token,
            arms,
        })
    }
}

impl ToTokens for HtmlMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            match_token,
            expr,
            arms,
            ..
        } = self;

        tokens.extend(quote_spanned! {match_token.span=>
            match #expr { #( #arms )* }
        });
    }
}

#[derive(Debug)]
pub struct HtmlArm {
    pat: Pat,
    guard: Option<(Token![if], Box<Expr>)>,
    _fat_arrow_token: Token![=>],
    body: HtmlBranch,
    _comma: Option<Token![,]>,
}

impl Parse for HtmlArm {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let pat = input.parse()?;
        let guard = input
            .parse::<Token![if]>()
            .ok()
            .map(|if_token| input.parse().map(|expr| (if_token, expr)))
            .transpose()?;
        let fat_arrow_token = input.parse()?;
        let body = input.parse()?;
        let comma = input.parse().ok();

        Ok(Self {
            pat,
            guard,
            _fat_arrow_token: fat_arrow_token,
            body,
            _comma: comma,
        })
    }
}

impl ToTokens for HtmlArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            pat, guard, body, ..
        } = self;
        let guard = guard.as_ref().map(|(_, x)| quote! { if #x });

        tokens.extend(quote! {
            #pat #guard => #body,
        });
    }
}
