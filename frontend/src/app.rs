#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;
use crate::page;

pub fn App(cx: Scope) -> Element {
    let more_html = rsx!( h3 { "more things!" } );
    use_init_atom_root(cx);
    cx.render(rsx! {
        h1{ "hello dudeeee!" }
        div{}
        p{}
        more_html

        // Router {
        //     Route { to: "some_url", page::Register {}  },
        // }
    })
}
