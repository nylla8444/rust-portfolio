//! Personal portfolio, built with Rust and Dioxus.
//!
//! Layout and visual design ported from chanhdai.com by Chánh Đại (MIT).
//! The cursor-chasing cat is a Rust port of oneko.js by adryd (MIT).

use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        div { class: "group/layout relative isolate",
            main { class: "max-w-screen overflow-x-clip px-2",
                div { class: "[--separator-height:--spacing(8)]",
                    div { class: "mx-auto md:max-w-3xl",                    }
                }
            }
        }
    }
}
