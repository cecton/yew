#![allow(unused_parens)] // TODO

use yew::prelude::*;

fn compile_pass(option: Option<i32>) {
    html! {
        match (option) {
            Some(x) => {<span>{x}</span>},
            None => {<span/>},
        }
    };
    html! {
        match (option) {
            _ => {<span/>},
        }
    };
    html! { match (option) { _ => {} } };
    html! { match (option) { _ => {}, } };
    html! { match (option) { _ => {{42}}, } };
    html! { match (option) { _ => {{{let _x = 42; html!()}}}, } };
}

fn main() {}
