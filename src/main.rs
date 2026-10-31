//! Personal portfolio, built with Rust and Dioxus.
//!
//! Layout and visual design ported from chanhdai.com by Chánh Đại (MIT).
//! The cursor-chasing cat is a Rust port of oneko.js by adryd (MIT).

mod data;
mod header;
mod js;
mod mark;
mod profile;
mod sections;
mod ui;

use dioxus::prelude::*;

use header::{CommandMenu, CommandMenuOpen, ScrollToTop, SiteBottomNav, SiteHeader};
use profile::{FrameWidthDimension, ProfileHeader, SeparatorHeightDimension};
use sections::*;
use ui::Separator;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| CommandMenuOpen(Signal::new(false)));

    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        div { class: "group/layout relative isolate",
            SiteHeader {}
            main { class: "max-w-screen overflow-x-clip px-2",
                div { class: "[--separator-height:--spacing(8)]",
                    div { class: "mx-auto md:max-w-3xl",
                        FrameWidthDimension {}
                        ProfileHeader {}
                        Separator {
                            SeparatorHeightDimension { height: 32, class: "hidden lg:block" }
                        }

                        Overview {}
                        SocialLinks {}
                        GitHubContributions {}
                        Separator {}

                        Hello {}
                        Numbers {}
                        Testimonials {}
                        Separator {}

                        Components {}
                        Separator {}

                        Blocks {}
                        Separator {}

                        Blog {}
                        Separator {}

                        TechStack {}
                        Separator {}

                        Experiences {}
                        Separator {}

                        EducationPanel {}
                        Separator {}

                        Projects {}
                        Separator {}

                        RecognitionPanel {}
                    }
                }
            }
            div { class: "pointer-events-none fixed inset-x-0 bottom-0 z-40 lg:hidden", "aria-hidden": "true",
                div { class: "h-(--fade-bottom-height) bg-linear-to-b from-transparent to-background mask-linear-[to_top,var(--background)_25%,transparent] backdrop-blur-[1px]" }
            }
            SiteBottomNav {}
            ScrollToTop {}
            CommandMenu {}
        }
    }
}
