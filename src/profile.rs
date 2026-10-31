use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::data::USER;
use crate::mark::IsometricMark;
use crate::ui::{
    DimensionExtensionLine, DimensionLine, DimensionValue, HandwrittenArrow, HandwrittenNote, VerifiedIcon,
};

/// "768" measured across the top of the frame on wide screens.
#[component]
pub fn FrameWidthDimension() -> Element {
    rsx! {
        div { class: "relative hidden h-8 border-x border-x-line min-[49rem]:block",
            div { class: "pointer-events-none absolute -inset-x-px inset-y-0 text-dimension select-none", "aria-hidden": "true",
                DimensionExtensionLine { vertical: true, class: "inset-y-1 left-0" }
                DimensionExtensionLine { vertical: true, class: "inset-y-1 right-0" }
                DimensionLine { vertical: false, class: "absolute inset-x-0 top-1/2 -translate-y-1/2", label: "768" }
            }
        }
    }
}

/// "32" measured down the side of the striped gap under the profile.
#[component]
pub fn SeparatorHeightDimension(height: u32, #[props(default)] class: String) -> Element {
    rsx! {
        div { class: "pointer-events-none absolute inset-0 z-2 text-dimension select-none {class}", "aria-hidden": "true",
            DimensionExtensionLine { vertical: false, class: "top-0 -right-8 left-full -translate-y-px" }
            DimensionExtensionLine { vertical: false, class: "top-full -right-8 left-full" }
            DimensionLine { vertical: true, class: "absolute -inset-y-px -right-6 translate-x-1/2" }
            DimensionValue { class: "absolute top-1/2 left-full ml-8 -translate-y-1/2", "{height}" }
        }
    }
}

#[component]
fn DiameterDimension(diameter: u32, class: String) -> Element {
    rsx! {
        div { class: "pointer-events-none absolute inset-0 text-dimension select-none {class}", "aria-hidden": "true",
            DimensionExtensionLine { vertical: false, class: "top-0 right-1/2 -left-8" }
            DimensionExtensionLine { vertical: false, class: "top-full right-1/2 -left-8 -translate-y-px" }
            DimensionLine { vertical: true, class: "absolute inset-y-0 -left-6 -translate-x-1/2", label: "Ø{diameter}" }
        }
    }
}

#[component]
pub fn ProfileHeader() -> Element {
    rsx! {
        div { class: "screen-line-bottom grid grid-cols-[auto_1fr] grid-rows-[1fr_auto] overflow-y-clip border-x border-x-line after:z-1",
            figure { class: "relative col-span-2 p-2 sm:col-span-1 sm:col-start-2 sm:p-4",
                IsometricMark { text: USER.alias.to_string() }

                HandwrittenNote { class: "bottom-20 left-full w-36 flex-col items-start pointer-fine:xl:flex",
                    HandwrittenArrow { class: "-scale-y-100 -rotate-6" }
                    span { class: "ml-3 -rotate-6",
                        "follows your cursor"
                        span { class: "block" }
                        "click for a sound"
                    }
                }
            }

            div { class: "flex flex-col sm:row-span-2 sm:row-start-1",
                div { class: "screen-line-top mt-auto shrink-0 border-r border-line",
                    div { class: "-mx-px my-0 flex outline-none",
                        div { class: "relative z-10 size-30 rounded-full min-[24rem]:size-32 sm:size-40",
                            Avatar {}
                            div { class: "pointer-events-none absolute inset-0 rounded-[inherit] inset-ring-1 inset-ring-foreground dark:inset-ring-foreground/20" }
                            DiameterDimension { diameter: 160, class: "-z-1 hidden lg:block" }
                        }
                    }
                }
            }

            div { class: "flex flex-col",
                div { class: "z-1 mt-auto border-t border-line",
                    div { class: "flex -translate-x-px items-center gap-2 pl-4",
                        h1 { class: "-translate-y-px text-[2rem]/none font-medium tracking-tight",
                            "{USER.display_name}"
                        }
                        VerifiedIcon { class: "size-4.5 select-none" }
                        PronounceMyName {}
                    }
                    div { class: "h-12.5 border-t border-line py-1 pl-4 sm:h-9",
                        TextSwap { items: USER.taglines }
                    }
                }
            }
        }
    }
}

/// Placeholder avatar. Swap it for an `img` of your photo or illustration.
#[component]
fn Avatar() -> Element {
    rsx! {
        div { class: "flex size-full items-center justify-center overflow-hidden rounded-[inherit] bg-muted text-muted-foreground/60 select-none",
            svg { class: "size-full translate-y-[12%]", "viewBox": "0 0 160 160", fill: "none", "aria-label": "Avatar placeholder",
                circle { cx: "80", cy: "58", r: "28", stroke: "currentColor", stroke_width: "2", class: "fill-background" }
                path { d: "M24 160c4-34 28-52 56-52s52 18 56 52", stroke: "currentColor", stroke_width: "2", class: "fill-background" }
            }
        }
    }
}

#[component]
fn PronounceMyName() -> Element {
    rsx! {
        button {
            class: "relative flex touch-manipulation items-center justify-center text-muted-foreground transition-[color,scale] select-none hover:text-foreground active:scale-[0.9] [&_svg]:size-4.5",
            "aria-label": "Pronounce my name",
            onclick: move |_| crate::js::speak(USER.display_name),
            lucide_dioxus::Volume2 {}
        }
    }
}

/// Cycles through short lines every three seconds.
#[component]
fn TextSwap(items: &'static [&'static str]) -> Element {
    let mut index = use_signal(|| 0usize);
    use_hook(move || {
        if items.len() < 2 || crate::js::media_matches("(prefers-reduced-motion: reduce)") {
            return;
        }
        spawn(async move {
            loop {
                TimeoutFuture::new(3000).await;
                index.set((index() + 1) % items.len());
            }
        });
    });

    let text = items.get(index()).copied().unwrap_or_default();
    rsx! {
        p { class: "overflow-hidden font-mono text-sm text-balance text-muted-foreground",
            span { key: "{index}", class: "inline-block animate-text-swap", "{text}" }
        }
    }
}
