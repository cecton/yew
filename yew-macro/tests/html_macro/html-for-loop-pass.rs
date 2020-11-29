use yew::prelude::*;

fn compile_pass() {
    html! { for x in 0..3 { <span>{x}</span> } };
}

fn main() {}
