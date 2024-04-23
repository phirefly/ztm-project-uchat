#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Create Account" }
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: move |_| {}, //Closure
            button {
                class: "btn", //See the style for this in tailwind.css
                r#type: "submit", //r# is used because this is a reserved word
                "Sign up now"
            }

        }


    })
}