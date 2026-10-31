//! Small building blocks shared across the page, ported from the shadcn-style
//! components on chanhdai.com.

use dioxus::prelude::*;

/// A full-width section framed by the page's vertical lines.
#[component]
pub fn Panel(
    #[props(default)] id: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        section {
            id: if !id.is_empty() { "{id}" },
            "data-slot": "panel",
            class: "screen-line-top screen-line-bottom relative border-x border-x-line {class}",
            {children}
        }
    }
}

#[component]
pub fn PanelHeader(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        header { class: "screen-line-bottom px-4 {class}", {children} }
    }
}

/// Section title linking to itself, with an optional count like "(9)".
#[component]
pub fn PanelTitle(id: String, title: String, count: Option<usize>) -> Element {
    rsx! {
        h2 { class: "group/panel-title relative font-heading text-3xl font-medium tracking-tight text-balance",
            a { href: "#{id}", "{title}" }
            if let Some(count) = count {
                sup { class: "top-[-0.75em] ml-1 text-sm font-medium tracking-normal text-muted-foreground",
                    "({count})"
                }
            }
            CopyLinkButton { id: id.clone() }
        }
    }
}

/// Appears on hover next to a section title and copies a link to it.
#[component]
fn CopyLinkButton(id: String) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        button {
            class: "absolute top-1 ml-1 inline-flex size-7 shrink-0 items-center justify-center rounded-lg text-muted-foreground opacity-0 transition-opacity group-hover/panel-title:opacity-100 hover:bg-muted [&_svg]:size-4",
            "aria-label": "Copy link to section",
            onclick: move |_| {
                let id = id.clone();
                spawn(async move {
                    crate::js::copy_text_with_location_hash(&id).await;
                    copied.set(true);
                    gloo_timers::future::TimeoutFuture::new(1500).await;
                    copied.set(false);
                });
            },
            if copied() {
                lucide_dioxus::Check {}
            } else {
                lucide_dioxus::Link {}
            }
        }
    }
}

#[component]
pub fn PanelContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { "data-slot": "panel-body", class: "p-4 {class}", {children} }
    }
}

/// The diagonal-striped gap between panels.
#[component]
pub fn Separator(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "stripe-divider h-8 w-full border-x border-x-line {class}", {children} }
    }
}

/// Uppercase, centred heading used by the "supported by" style panels.
#[component]
pub fn EyebrowTitle(text: String) -> Element {
    rsx! {
        h2 { class: "text-center text-sm/none font-medium tracking-wide text-muted-foreground uppercase",
            "{text}"
        }
    }
}

/// Small square chip framing a leading icon.
#[component]
pub fn IconTile(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            "data-slot": "icon-tile",
            class: "flex size-6 shrink-0 items-center justify-center rounded-md border border-muted-foreground/15 bg-muted text-muted-foreground ring-1 ring-border/50 ring-offset-1 ring-offset-background select-none dark:ring-line [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 {class}",
            {children}
        }
    }
}

#[component]
pub fn Tag(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        span { class: "inline-flex items-center rounded-full border bg-zinc-50 px-1.5 py-0.5 font-mono text-xs text-muted-foreground dark:bg-zinc-900 {class}",
            {children}
        }
    }
}

#[component]
pub fn TagList(items: &'static [&'static str], #[props(default)] class: String) -> Element {
    rsx! {
        if !items.is_empty() {
            ul { class: "flex flex-wrap gap-1.5 {class}",
                for item in items.iter() {
                    li { class: "flex",
                        Tag { "{item}" }
                    }
                }
            }
        }
    }
}

/// Thin vertical rule between metadata items.
#[component]
pub fn MetaSeparator() -> Element {
    rsx! {
        div { class: "h-4 w-px shrink-0 self-center bg-border", "aria-hidden": "true" }
    }
}

/// The secondary "All components →" style button.
#[component]
pub fn SecondaryButton(
    #[props(default)] href: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let class = "inline-flex h-8 shrink-0 items-center justify-center gap-2 rounded-[10px] bg-secondary pr-2.5 pl-3 text-sm font-medium whitespace-nowrap text-secondary-foreground shadow-[inset_0_0_1px] shadow-foreground/20 transition-all outline-none select-none hover:bg-secondary/80 focus-visible:ring-3 focus-visible:ring-ring/50 active:scale-[0.98] [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0";
    rsx! {
        if let Some(href) = href {
            a { class, href, {children} }
        } else {
            button {
                class,
                onclick: move |e| {
                    if let Some(handler) = onclick {
                        handler.call(e);
                    }
                },
                {children}
            }
        }
    }
}

/// Centred row holding a [`SecondaryButton`] under a panel.
#[component]
pub fn PanelFooter(children: Element) -> Element {
    rsx! {
        div { class: "screen-line-top flex justify-center py-4", {children} }
    }
}

/// Handwritten margin note, only shown on wide screens (callers add `lg:flex` etc).
#[component]
pub fn HandwrittenNote(class: String, children: Element) -> Element {
    rsx! {
        div {
            "data-slot": "handwritten-note",
            "aria-hidden": "true",
            class: "pointer-events-none absolute hidden font-handwritten text-xl/none tracking-normal text-muted-foreground select-none {class}",
            {children}
        }
    }
}

/// Points down-left. Rotate or mirror it to aim at the subject.
#[component]
pub fn HandwrittenArrow(#[props(default)] class: String) -> Element {
    rsx! {
        svg {
            class: "size-8 shrink-0 text-muted-foreground {class}",
            "viewBox": "0 0 40 40",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: "M34 4c1 15-5 26-21 30" }
            path { d: "m22 37-9-3 7.5-8" }
        }
    }
}

// ---------------------------------------------------------------------------
// Dimension lines, the technical-drawing annotations around the page.

#[component]
pub fn DimensionArrow(vertical: bool, #[props(default)] class: String) -> Element {
    let (w, h, d) = if vertical { (7, 9, "M3.5.5 6.5 8.5h-6Z") } else { (9, 7, "M.5 3.5 8.5.5v6Z") };
    rsx! {
        svg {
            class: "shrink-0 {class}",
            width: "{w}",
            height: "{h}",
            "viewBox": "0 0 {w} {h}",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1",
            stroke_linejoin: "round",
            path { d }
        }
    }
}

#[component]
pub fn DimensionValue(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        span { class: "font-mono text-xs/none text-dimension-foreground tabular-nums {class}", {children} }
    }
}

/// A measured span with arrowheads at both ends and an optional label in the middle.
#[component]
pub fn DimensionLine(vertical: bool, class: String, label: Option<String>) -> Element {
    let rule = if vertical { "w-px" } else { "h-px" };
    rsx! {
        div { class: if vertical { "flex flex-col items-center py-px {class}" } else { "flex items-center px-px {class}" },
            DimensionArrow { vertical }
            div { class: "flex-1 bg-current {rule}" }
            if let Some(label) = label {
                DimensionValue { class: if vertical { "py-1" } else { "px-1" }, "{label}" }
            }
            div { class: "flex-1 bg-current {rule}" }
            DimensionArrow { vertical, class: if vertical { "-scale-y-100" } else { "-scale-x-100" } }
        }
    }
}

#[component]
pub fn DimensionExtensionLine(vertical: bool, class: String) -> Element {
    rsx! {
        div { class: if vertical { "absolute w-px bg-current {class}" } else { "absolute h-px bg-current {class}" } }
    }
}

// ---------------------------------------------------------------------------
// Brand icons (simple-icons shapes), lucide doesn't ship these.

#[component]
pub fn XIcon(#[props(default)] class: String) -> Element {
    rsx! {
        svg { class, "viewBox": "0 0 24 24", "aria-hidden": "true",
            path {
                fill: "currentColor",
                d: "m22.991 23-8.533-12.612L22.42 1h-2.77l-6.422 7.575L8.105 1H1.123l8.225 12.158L1 23h2.77l6.81-8.03L16.015 23H23zM7.193 2.769l12.49 18.462h-2.76L4.43 2.769z",
            }
        }
    }
}

#[component]
pub fn GitHubIcon(#[props(default)] class: String) -> Element {
    rsx! {
        svg { class, "viewBox": "0 0 24 24", "aria-hidden": "true",
            path {
                fill: "currentColor",
                d: "M12 0C5.37 0 0 5.372 0 11.997 0 17.3 3.438 21.795 8.205 23.38c.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.725-4.042-1.609-4.042-1.609C4.422 17.77 3.633 17.4 3.633 17.4c-1.087-.744.084-.73.084-.73 1.205.085 1.838 1.237 1.838 1.237 1.07 1.834 2.809 1.304 3.495.997.108-.775.417-1.304.76-1.604-2.665-.3-5.466-1.332-5.466-5.929 0-1.31.465-2.38 1.235-3.219-.135-.303-.54-1.523.105-3.175 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.4 3-.405 1.02.006 2.04.138 3 .404 2.28-1.551 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.608-2.805 5.623-5.475 5.918.42.36.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.284 0 .315.21.69.825.57C20.565 21.79 24 17.291 24 11.997 24 5.372 18.627 0 12 0",
            }
        }
    }
}

#[component]
pub fn LinkedInIcon(#[props(default)] class: String) -> Element {
    rsx! {
        svg { class, "viewBox": "0 0 24 24", "aria-hidden": "true",
            path {
                fill: "currentColor",
                d: "M22.274 0H1.728C.692 0 0 .685 0 1.715v20.569C0 23.316.864 24 1.727 24h20.546C23.31 24 24 23.315 24 22.285V1.716C24.001.684 23.31 0 22.274 0M7.08 20.4H3.454V8.915h3.625zM5.352 7.371c-1.209 0-2.07-.856-2.07-2.056s.863-2.059 2.07-2.059c1.21 0 2.073.859 2.073 2.059S6.388 7.37 5.352 7.37M20.548 20.4h-3.626v-5.485c0-1.371 0-3.087-1.9-3.087-1.898 0-2.073 1.372-2.073 2.916V20.4H9.325V8.915h3.454v1.541c.69-1.2 2.073-1.885 3.453-1.885 3.627 0 4.316 2.4 4.316 5.485z",
            }
        }
    }
}

#[component]
pub fn SocialIcon(name: crate::data::SocialName, #[props(default)] class: String) -> Element {
    use crate::data::SocialName;
    match name {
        SocialName::X => rsx! { XIcon { class } },
        SocialName::GitHub => rsx! { GitHubIcon { class } },
        SocialName::LinkedIn => rsx! { LinkedInIcon { class } },
    }
}

#[component]
pub fn VerifiedIcon(#[props(default)] class: String) -> Element {
    rsx! {
        svg { class, xmlns: "http://www.w3.org/2000/svg", "viewBox": "0 0 24 24", "aria-hidden": "true",
            path {
                fill: "currentColor",
                d: "M24 12a4.454 4.454 0 0 0-2.564-3.91 4.437 4.437 0 0 0-.948-4.578 4.436 4.436 0 0 0-4.577-.948A4.44 4.44 0 0 0 12 0a4.423 4.423 0 0 0-3.9 2.564 4.434 4.434 0 0 0-2.43-.178 4.425 4.425 0 0 0-2.158 1.126 4.42 4.42 0 0 0-1.12 2.156 4.42 4.42 0 0 0 .183 2.421A4.456 4.456 0 0 0 0 12a4.465 4.465 0 0 0 2.576 3.91 4.433 4.433 0 0 0 .936 4.577 4.459 4.459 0 0 0 4.577.95A4.454 4.454 0 0 0 12 24a4.439 4.439 0 0 0 3.91-2.563 4.26 4.26 0 0 0 5.526-5.526A4.453 4.453 0 0 0 24 12Zm-13.709 4.917-4.38-4.378 1.652-1.663 2.646 2.646L15.83 7.4l1.72 1.591-7.258 7.926Z",
            }
        }
    }
}

/// "1234" -> "1.2K", matching Intl's compact notation closely enough for counts.
pub fn format_compact(n: u64) -> String {
    let (value, suffix) = match n {
        0..1_000 => return n.to_string(),
        1_000..1_000_000 => (n as f64 / 1_000.0, "K"),
        1_000_000..1_000_000_000 => (n as f64 / 1_000_000.0, "M"),
        _ => (n as f64 / 1_000_000_000.0, "B"),
    };
    let rounded = (value * 10.0).round() / 10.0;
    if rounded.fract() == 0.0 || rounded >= 100.0 {
        format!("{}{suffix}", rounded.round() as u64)
    } else {
        format!("{rounded:.1}{suffix}")
    }
}

/// "1234567" -> "1,234,567"
pub fn format_number(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}
