use dioxus::prelude::*;

use crate::data::{CREDITS, SITE_SUBTITLE, SITE_TITLE, SOCIAL_LINKS, SocialName, USER};
use crate::mark::FlatMark;
use crate::ui::SocialIcon;

const STACK: &[&str] = &["Rust", "Dioxus 0.7", "Tailwind CSS v4", "dioxus-vercel"];

#[component]
fn Field(label: &'static str, #[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "flex min-w-0 flex-col gap-1 bg-background px-4 py-3 {class}",
            dt { class: "text-[0.625rem]/4 font-medium tracking-wider text-muted-foreground uppercase", "{label}" }
            dd { class: "flex-1 text-sm", {children} }
        }
    }
}

/// Footer laid out as the title block of a technical drawing.
#[component]
pub fn SiteFooterCad() -> Element {
    let sha = env!("BUILD_SHA");
    let date = env!("BUILD_DATE");
    let version = env!("CARGO_PKG_VERSION");
    let github = SOCIAL_LINKS.iter().find(|l| l.name == SocialName::GitHub);
    let source = format!("https://github.com/{}", USER.source_repo);

    rsx! {
        footer { class: "max-w-screen overflow-x-clip px-2",
            div { class: "mx-auto border-x border-x-line md:max-w-3xl",
                div { class: "screen-line-top screen-line-bottom before:z-1",
                    div { class: "stripe-divider h-12" }
                }

                div { class: "relative",
                    div { class: "screen-line-bottom flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 px-4 py-3 font-mono text-sm",
                        span { class: "font-medium", "{SITE_TITLE}" }
                        span { class: "font-sans text-muted-foreground", "{SITE_SUBTITLE}" }
                    }

                    dl { class: "grid grid-cols-2 gap-px bg-line font-mono md:grid-cols-4",
                        Field { label: "Crafted by",
                            if let Some(gh) = github {
                                a { class: "link-underline", href: gh.href, target: "_blank", rel: "noopener", "@{gh.handle}" }
                            }
                        }
                        Field { label: "Build",
                            if sha.is_empty() {
                                span { class: "text-muted-foreground", "local" }
                            } else {
                                "{sha}"
                            }
                        }
                        Field { label: "Date",
                            time { datetime: "{date}", "{date}" }
                        }
                        // Was "Registry: N items" (the component count); back when Components returns.
                        Field { label: "Version", "v{version}" }
                        Field { label: "Deployed on",
                            span { class: "font-sans", "aria-hidden": "true", "▲" }
                            span { class: "sr-only", "Vercel" }
                        }
                        Field { label: "Source code",
                            a { class: "link-underline", href: "{source}", target: "_blank", rel: "noopener", "GitHub" }
                        }
                        Field { label: "License", "MIT License" }
                        Field { label: "Typeface", "Geist" }
                        Field { label: "Stack", class: "col-span-2",
                            ul { class: "flex flex-col gap-1",
                                for entry in STACK.iter() {
                                    li { "{entry}" }
                                }
                            }
                        }
                        Field { label: "Analytics",
                            span { class: "text-muted-foreground", "None yet" }
                        }
                        Field { label: "Deployed with",
                            a { class: "link-underline", href: "https://crates.io/crates/dioxus-vercel", target: "_blank", rel: "noopener", "dioxus-vercel" }
                        }
                        Field { label: "Credits", class: "col-span-2",
                            ul { class: "flex flex-col gap-1",
                                for (name, href) in CREDITS.iter() {
                                    li {
                                        a { class: "link-underline", href: *href, target: "_blank", rel: "noopener", "{name}" }
                                    }
                                }
                            }
                        }
                        Field { label: "Approved by", class: "col-span-2",
                            div { class: "flex h-full flex-col justify-end gap-2",
                                span { class: "ml-1 self-start font-handwritten text-4xl/none text-blue-900 dark:text-foreground", "{USER.display_name}" }
                                span { class: "border-t border-line pt-2 font-sans text-sm/none font-medium tracking-wide uppercase", "{USER.display_name}" }
                            }
                        }
                    }
                }

                div { class: "screen-line-top h-4" }

                div { class: "screen-line-top screen-line-bottom flex items-center gap-3 px-4 py-3 text-muted-foreground",
                    a { class: "mr-auto text-muted-foreground transition-[color] hover:text-foreground", href: "#", "aria-label": "Home",
                        FlatMark { text: USER.alias.to_string(), class: "h-4" }
                    }
                    for (i, link) in SOCIAL_LINKS.iter().enumerate() {
                        if i > 0 {
                            div { class: "h-4 w-px self-center bg-border" }
                        }
                        a {
                            class: "flex items-center transition-[color] hover:text-foreground",
                            href: link.href,
                            target: "_blank",
                            rel: "noopener",
                            "aria-label": "{link.title}",
                            SocialIcon { name: link.name, class: "size-4" }
                        }
                    }
                }
            }

            PixelWordmark {}
            div { class: "h-(--fade-bottom-height)" }
        }
    }
}

/// Your alias in big block letters, half tucked below the fold, lit by the cursor.
#[component]
fn PixelWordmark() -> Element {
    let word: String = USER.alias.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let mut light = use_signal(|| 0.5f64);
    let cols = word.chars().count() as i32 * 6 - 1;
    let view_box = format!("-0.5 -0.5 {} 6", cols + 1);
    let x1 = format!("{:.3}", light() * cols as f64);
    let x2 = format!("{:.3}", light() * cols as f64 + cols as f64 * 0.4);

    rsx! {
        div { class: "screen-line-bottom after:z-1 after:bg-foreground/15",
            div {
                class: "overflow-hidden",
                // The wordmark spans the viewport, so the window width is a good enough frame.
                onmousemove: move |e| {
                    let (width, _) = crate::js::window_size();
                    light.set((e.client_coordinates().x / width).clamp(0.0, 1.0));
                },
                onmouseleave: move |_| light.set(0.5),
                div { class: "flex w-full translate-y-[37.5%] items-center justify-center px-2",
                    svg { class: "size-full md:max-w-3xl", "viewBox": "{view_box}", fill: "none", xmlns: "http://www.w3.org/2000/svg", "aria-hidden": "true",
                        defs {
                            linearGradient {
                                id: "wordmark-light",
                                x1: "{x1}",
                                y1: "0",
                                x2: "{x2}",
                                y2: "0",
                                "gradientUnits": "userSpaceOnUse",
                                stop { class: "[stop-color:var(--color-zinc-200)] dark:[stop-color:var(--color-zinc-800)]" }
                                stop { offset: "0.5", class: "[stop-color:var(--color-zinc-400)] dark:[stop-color:var(--color-zinc-500)]" }
                                stop { offset: "1", class: "[stop-color:var(--color-zinc-200)] dark:[stop-color:var(--color-zinc-800)]" }
                            }
                        }
                        g { fill: "url(#wordmark-light)", "shape-rendering": "crispEdges",
                            for (c, r) in crate::mark::cells(&word) {
                                rect { x: "{c}", y: "{r}", width: "1.02", height: "1.02" }
                            }
                        }
                    }
                }
            }
        }
    }
}
