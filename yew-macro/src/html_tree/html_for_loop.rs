use super::HtmlBranch;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Expr, Label, Pat, Token};

pub struct HtmlForLoop {
    label: Option<Label>,
    for_token: Token![for],
    pat: Pat,
    _in_token: Token![in],
    expr: Box<Expr>,
    body: HtmlBranch,
}

impl PeekValue<()> for HtmlForLoop {
    fn peek(cursor: Cursor) -> Option<()> {
        let (ident, _) = cursor.ident()?;
        (ident == "for").as_option()
    }
}

impl Parse for HtmlForLoop {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let label = input.parse()?;
        let for_token = input.parse()?;
        let pat = input.parse()?;
        let in_token = input.parse()?;
        let expr = input.parse()?;
        let body = input.parse()?;

        Ok(Self {
            label,
            for_token,
            pat,
            _in_token: in_token,
            expr,
            body,
        })
    }
}

impl ToTokens for HtmlForLoop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            label,
            for_token,
            pat,
            _in_token,
            expr,
            body,
        } = self;
        let key = quote! { None }; // TODO
        let new_tokens = quote_spanned! {for_token.span()=>
            {
            let mut vec = Vec::new();
            #label for #pat in #expr {
                vec.push(#body);
            }
            ::yew::virtual_dom::VNode::VList(
                ::yew::virtual_dom::VList::new_with_children(vec, #key)
            )
            }
        };

        tokens.extend(new_tokens);
    }
}
