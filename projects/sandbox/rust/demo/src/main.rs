#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 3);

    cx.render(rsx! {
        h1 { "Hello CS Hub! Insidious John Chen Bot count: {count}" }
        button { onclick: move |_| count += 1, "Assimilate! Become a John Chen Bot!" }
        br {}
        button { onclick: move |_| count -= 1, "Destroy a John Chen Bot!" }
        h1 { "Brought to you by the ALPHA@CODERS" }
        img {
            src: "https://assets.pokemon.com/assets/cms2/img/pokedex/full/143.png",
            class: "primary_button",
            width: "192px"
        }
        h2 { "Welcome to the Real World" }
    })
}

