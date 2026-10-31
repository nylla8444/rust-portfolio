//! The panels that make up the page, top to bottom.

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::data::*;
use crate::ui::*;

// ---------------------------------------------------------------------------
// Overview

#[component]
fn IntroItem(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "flex items-center gap-4 font-mono text-sm {class}", {children} }
    }
}

#[component]
pub fn Overview() -> Element {
    let maps = format!("https://www.google.com/maps/search/?api=1&query={}", USER.address.replace(' ', "+"));
    rsx! {
        Panel {
            h2 { class: "sr-only", "Overview" }
            PanelContent { class: "grid gap-x-4 gap-y-2.5 sm:grid-cols-2",
                for job in USER.jobs.iter() {
                    IntroItem { class: "sm:col-span-2",
                        IconTile {
                            if job.title.to_lowercase().contains("engineer") || job.title.to_lowercase().contains("developer") {
                                lucide_dioxus::CodeXml {}
                            } else if job.title.to_lowercase().contains("student") {
                                lucide_dioxus::GraduationCap {}
                            } else {
                                lucide_dioxus::BriefcaseBusiness {}
                            }
                        }
                        p { class: "text-balance",
                            "{job.title} "
                            span { "aria-label": "at", "@" }
                            a { class: "link ml-0.5 font-medium", href: job.website, target: "_blank", rel: "noopener", "{job.company}" }
                        }
                    }
                }

                IntroItem {
                    IconTile { lucide_dioxus::MapPin {} }
                    p { class: "text-balance",
                        a { class: "link", href: "{maps}", target: "_blank", rel: "noopener", "{USER.address}" }
                    }
                }

                LocalTimeItem {}

                CopyableItem {
                    text: USER.email,
                    href: format!("mailto:{}", USER.email),
                    icon: rsx! { lucide_dioxus::Mail {} },
                }

                CopyableItem {
                    text: USER.phone,
                    href: format!("tel:{}", USER.phone.replace(' ', "")),
                    icon: rsx! { lucide_dioxus::Phone {} },
                }
            }

            div { class: "pointer-events-none absolute inset-y-0 left-1/2 -z-1 w-px -translate-x-2.25 border-r border-dashed border-line max-sm:hidden" }
        }
    }
}

/// Email or phone row; a copy button shows up on hover.
#[component]
fn CopyableItem(text: &'static str, href: String, icon: Element) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        IntroItem { class: "group",
            IconTile { {icon} }
            p { class: "flex text-balance",
                a { class: "link", href: "{href}", "{text}" }
            }
            div { class: "-translate-x-2.5 opacity-0 transition-opacity ease-out group-hover:opacity-100 group-has-focus-visible:opacity-100 pointer-coarse:opacity-100",
                button {
                    class: "inline-flex size-6 items-center justify-center rounded-md text-muted-foreground hover:bg-muted [&_svg]:size-3.5",
                    "aria-label": "Copy",
                    onclick: move |_| {
                        spawn(async move {
                            crate::js::copy_text(text).await;
                            copied.set(true);
                            TimeoutFuture::new(1500).await;
                            copied.set(false);
                        });
                    },
                    if copied() {
                        lucide_dioxus::Check {}
                    } else {
                        lucide_dioxus::Copy {}
                    }
                }
            }
        }
    }
}

fn clock_hands(hour: u32, minute: u32) -> String {
    let h = (hour % 12) as f64;
    let m = minute as f64;
    let tau = std::f64::consts::TAU;
    let minute_angle = m / 60.0 * tau;
    let hour_angle = (h + m / 60.0) / 12.0 * tau;
    format!(
        "M12 12 L{:.3} {:.3} M12 12 L{:.3} {:.3}",
        12.0 + 3.6 * hour_angle.sin(),
        12.0 - 3.6 * hour_angle.cos(),
        12.0 + 6.0 * minute_angle.sin(),
        12.0 - 6.0 * minute_angle.cos(),
    )
}

/// Your local time, plus how far ahead or behind the visitor you are.
#[component]
fn LocalTimeItem() -> Element {
    let mut clock = use_signal(|| (String::new(), 12u32, 0u32, String::new()));

    use_hook(move || {
        spawn(async move {
            let code = format!(
                r#"
                const timeZone = {tz};
                const now = new Date();
                const time = new Intl.DateTimeFormat("en-US", {{ timeZone, hour: "2-digit", minute: "2-digit", hour12: true }}).format(now);
                const hour = parseInt(time, 10);
                const minute = parseInt(time.slice(3), 10);
                const viewerOffset = -now.getTimezoneOffset();
                const targetOffset = (new Date(now.toLocaleString("en-US", {{ timeZone }})).getTime() - new Date(now.toLocaleString("en-US", {{ timeZone: "UTC" }})).getTime()) / 60000;
                const hoursDiff = Math.abs(targetOffset - viewerOffset) / 60;
                const diff = hoursDiff < 1 ? " // same time" : ` // ${{Math.floor(hoursDiff)}}h ${{targetOffset > viewerOffset ? "ahead" : "behind"}}`;
                return [time, hour, minute, diff];
                "#,
                tz = crate::js::js_str(USER.time_zone)
            );
            loop {
                if let Some(next) = crate::js::eval::<(String, u32, u32, String)>(&code).await {
                    clock.set(next);
                }
                TimeoutFuture::new(60_000).await;
            }
        });
    });

    let (time, hour, minute, diff) = clock();
    let hands = clock_hands(hour, minute);
    rsx! {
        IntroItem {
            IconTile {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    "viewBox": "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "{hands}" }
                }
            }
            p { class: "text-balance",
                span { "{time}" }
                span { class: "text-muted-foreground", "aria-hidden": "true", "{diff}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Social links

#[component]
pub fn SocialLinks() -> Element {
    rsx! {
        Panel { class: "screen-line-bottom-none screen-line-top-none",
            h2 { class: "sr-only", "Social links" }
            PanelContent {
                ul { class: "flex flex-wrap gap-2",
                    for item in SOCIAL_LINKS.iter() {
                        li {
                            a {
                                class: "inline-flex size-8 items-center justify-center rounded-[10px] border border-border bg-background text-foreground/80 transition-all hover:bg-muted hover:text-foreground active:scale-[0.98] dark:border-input dark:bg-input/30 dark:hover:bg-input/50 [&_svg]:size-4.5",
                                href: item.href,
                                target: "_blank",
                                rel: "noopener",
                                title: "{item.title} ({item.handle})",
                                SocialIcon { name: item.name }
                                span { class: "sr-only", "{item.title}" }
                            }
                        }
                    }
                }
            }
            HandwrittenNote { class: "-top-7 right-full mr-4 w-20 flex-col items-end lg:flex",
                span { class: "-rotate-6", "follow me" }
                HandwrittenArrow { class: "size-7 translate-x-3 -scale-x-100 -rotate-6" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// GitHub contributions

#[derive(Clone, PartialEq, serde::Deserialize)]
struct Activity {
    date: String,
    count: u32,
    level: u8,
}

#[derive(serde::Deserialize)]
struct ContributionsResponse {
    contributions: Vec<Activity>,
}

const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

fn parse_date(date: &str) -> (i32, u32, u32) {
    let mut parts = date.split('-').map(|p| p.parse::<i32>().unwrap_or(1));
    let y = parts.next().unwrap_or(1970);
    let m = parts.next().unwrap_or(1) as u32;
    let d = parts.next().unwrap_or(1) as u32;
    (y, m, d)
}

/// 0 = Sunday (Sakamoto's method).
fn weekday(y: i32, m: u32, d: u32) -> usize {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    ((y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d as i32).rem_euclid(7)) as usize
}

fn pretty_date(date: &str) -> String {
    let (y, m, d) = parse_date(date);
    format!("{d} {} {y}", MONTHS[(m.clamp(1, 12) - 1) as usize])
}

fn level_class(level: u8) -> &'static str {
    match level {
        0 => "fill-muted-foreground/5",
        1 => "fill-muted-foreground/20",
        2 => "fill-muted-foreground/40",
        3 => "fill-muted-foreground/60",
        _ => "fill-muted-foreground/80",
    }
}

const BLOCK: usize = 12;
const GAP: usize = 2;
const LABEL_HEIGHT: usize = 23;

#[component]
pub fn GitHubContributions() -> Element {
    let data = use_resource(|| async move {
        let code = format!(
            "const r = await fetch('https://github-contributions-api.jogruber.de/v4/{}?y=last'); if (!r.ok) return {{ contributions: [] }}; return await r.json();",
            USER.github_username
        );
        crate::js::eval::<ContributionsResponse>(&code).await.map(|r| r.contributions).unwrap_or_default()
    });

    rsx! {
        Panel {
            h2 { class: "sr-only", "GitHub contributions" }
            match &*data.read() {
                Some(days) if !days.is_empty() => rsx! { ContributionGraph { days: days.clone() } },
                Some(_) => rsx! {},
                None => rsx! {
                    div { class: "flex h-45 w-full items-center justify-center text-muted-foreground [&_svg]:animate-spin",
                        lucide_dioxus::Loader {}
                    }
                },
            }
        }
    }
}

#[component]
fn ContributionGraph(days: Vec<Activity>) -> Element {
    let (y0, m0, d0) = parse_date(&days[0].date);
    let offset = weekday(y0, m0, d0);
    let weeks = (days.len() + offset).div_ceil(7);
    let width = weeks * (BLOCK + GAP) - GAP;
    let height = LABEL_HEIGHT + 7 * (BLOCK + GAP) - GAP;
    let total: u32 = days.iter().map(|d| d.count).sum();

    // A month label sits over the first week that starts in that month.
    let mut labels: Vec<(usize, &str)> = Vec::new();
    let mut last_month = 0;
    for (i, day) in days.iter().enumerate() {
        let slot = i + offset;
        if !slot.is_multiple_of(7) && i != 0 {
            continue;
        }
        let (_, m, _) = parse_date(&day.date);
        if m != last_month {
            labels.push((slot / 7, MONTHS[(m - 1) as usize]));
            last_month = m;
        }
    }
    // Drop the first label when the next one follows too closely to fit.
    if labels.len() > 1 && labels[1].0 - labels[0].0 < 3 {
        labels.remove(0);
    }

    let first = pretty_date(&days[0].date);
    let last = pretty_date(&days[days.len() - 1].date);
    let profile = format!("https://github.com/{}", USER.github_username);
    let view_box = format!("0 0 {width} {height}");

    rsx! {
        figure {
            div { class: "mx-auto flex w-max max-w-full flex-col gap-4 py-4",
                div { class: "no-scrollbar max-w-full overflow-x-auto overflow-y-hidden px-4",
                    svg {
                        class: "block overflow-visible",
                        width: "{width}",
                        height: "{height}",
                        "viewBox": "{view_box}",
                        "aria-label": "GitHub contributions",
                        g { class: "fill-current text-muted-foreground",
                            for (week, name) in labels {
                                text { x: "{week * (BLOCK + GAP)}", y: "0", "dominant-baseline": "hanging", font_size: "14", "{name}" }
                            }
                        }
                        for (i, day) in days.iter().enumerate() {
                            rect {
                                class: level_class(day.level),
                                x: "{(i + offset) / 7 * (BLOCK + GAP)}",
                                y: "{LABEL_HEIGHT + (i + offset) % 7 * (BLOCK + GAP)}",
                                width: "{BLOCK}",
                                height: "{BLOCK}",
                                title { "{day.count} contribution{plural(day.count)} on {pretty_date(&day.date)}" }
                            }
                        }
                    }
                }
                div { class: "flex flex-wrap items-center gap-x-4 gap-y-2 px-4 text-sm",
                    figcaption { class: "text-pretty text-muted-foreground tabular-nums",
                        "{format_compact(total as u64)} contributions, {first} – {last}. Source: "
                        a { class: "link-underline", href: "{profile}", target: "_blank", rel: "noopener", "GitHub" }
                        "."
                    }
                    div { class: "ml-auto flex items-center gap-[3px]", "aria-hidden": "true",
                        span { class: "mr-1 text-muted-foreground", "Less" }
                        for level in 0..5u8 {
                            svg { width: "{BLOCK}", height: "{BLOCK}",
                                rect { class: level_class(level), width: "{BLOCK}", height: "{BLOCK}" }
                            }
                        }
                        span { class: "ml-1 text-muted-foreground", "More" }
                    }
                }
            }
        }
    }
}

fn plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "s" }
}

// ---------------------------------------------------------------------------
// About

#[component]
pub fn Hello() -> Element {
    let greeting = match crate::js::now_hours() {
        0..12 => "Good morning",
        12..17 => "Good afternoon",
        _ => "Good evening",
    };
    rsx! {
        Panel { id: "hello", class: "screen-line-bottom-none",
            PanelHeader {
                h2 { class: "sr-only", "About" }
                div { class: "font-heading text-3xl font-medium tracking-tight text-balance", "aria-hidden": "true", "{greeting}" }
            }
            PanelContent {
                div { class: "typeset typeset-description [&_li]:ps-0.5 [&_ul]:ps-3.5",
                    ul {
                        for line in USER.about.iter() {
                            li { "{line}" }
                        }
                    }
                }
            }
            div { class: "screen-line-bottom h-px" }
        }
    }
}

// ---------------------------------------------------------------------------
// Numbers and testimonials

#[component]
pub fn Numbers() -> Element {
    rsx! {
        Panel { id: "numbers", class: "screen-line-bottom-none",
            div { class: "h-px" }
            div { class: "h-4" }
            div { class: "screen-line-bottom h-px" }
            div { class: "flex items-center justify-center py-4",
                EyebrowTitle { text: "Growing with the community" }
            }
            div { class: "screen-line-bottom h-px" }
            dl { class: "grid gap-4 p-4",
                for stat in NUMBERS.iter() {
                    div { class: "flex items-baseline gap-2",
                        dt { class: "flex flex-1 items-baseline gap-2 text-sm/4 after:flex-1 after:translate-y-0.75 after:border-b after:border-dashed after:border-line",
                            "{stat.label}"
                        }
                        dd { class: "text-sm/4 font-medium tabular-nums",
                            data { value: "{stat.value}", title: "{format_number(stat.value)}", "{format_compact(stat.value)}" }
                        }
                    }
                }
            }
        }
    }
}

fn initials(name: &str) -> String {
    name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect()
}

#[component]
fn TestimonialItem(item: &'static Testimonial, #[props(default)] class: String, #[props(default)] show_icon: bool) -> Element {
    rsx! {
        figure { class: "group/testimonial relative flex h-full flex-col {class}",
            blockquote { class: "grow px-4 py-3 font-serif text-base text-pretty text-foreground",
                p { "{item.quote}" }
            }
            figcaption { class: "grid grid-cols-[auto_1fr] grid-rows-2 items-center gap-x-3.5 px-4 pt-1 pb-3",
                div { class: "relative row-span-2 size-8 shrink-0",
                    div { class: "flex size-8 items-center justify-center rounded-full bg-muted font-mono text-[11px] text-muted-foreground select-none",
                        "{initials(item.author_name)}"
                    }
                    div { class: "pointer-events-none absolute inset-0 rounded-full inset-ring-1 inset-ring-black/10 dark:inset-ring-white/15" }
                }
                div { class: "flex items-center gap-1.5 text-sm/4.5 font-semibold text-foreground",
                    a { href: item.url, target: "_blank", rel: "noopener",
                        span { class: "absolute inset-0", "aria-hidden": "true" }
                        "{item.author_name}"
                    }
                }
                div { class: "text-xs/4 text-balance text-muted-foreground", "{item.author_tagline}" }
            }
            if show_icon {
                div { class: "pointer-events-none absolute right-3 bottom-3 flex size-8 items-center justify-center [&_svg]:size-4 [&_svg]:text-muted-foreground/80", "aria-hidden": "true",
                    lucide_dioxus::Quote {}
                }
            }
        }
    }
}

/// An endlessly scrolling row; the items are rendered twice so the loop is seamless.
#[component]
fn Marquee(items: Vec<&'static Testimonial>, reverse: bool) -> Element {
    let duration = items.len() * 328 / 30;
    let animation = if reverse { "animate-marquee-reverse" } else { "animate-marquee" };
    rsx! {
        div { class: "relative overflow-hidden sm:col-span-2",
            div { class: "pointer-events-none absolute inset-y-0 left-0 z-10 w-12 bg-linear-to-r from-background to-transparent" }
            div { class: "pointer-events-none absolute inset-y-0 right-0 z-10 w-12 bg-linear-to-l from-background to-transparent" }
            div {
                class: "flex w-max items-stretch {animation} motion-reduce:animate-none",
                style: "--marquee-duration: {duration}s",
                for copy in 0..2 {
                    for (i, item) in items.iter().copied().enumerate() {
                        div {
                            key: "{copy}-{i}",
                            class: "mx-1 w-xs shrink-0 rounded-xl bg-background inset-ring-1 inset-ring-foreground/20 transition-[background-color] ease-out hover:bg-accent-muted",
                            "aria-hidden": if copy == 1 { "true" },
                            TestimonialItem { item }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Testimonials() -> Element {
    let all: Vec<&'static Testimonial> = TESTIMONIALS_1.iter().chain(TESTIMONIALS_2.iter()).collect();
    let featured: Vec<&'static Testimonial> = all.iter().copied().filter(|t| t.featured).collect();
    let row1: Vec<&'static Testimonial> = TESTIMONIALS_1.iter().filter(|t| !t.featured).collect();
    let row2: Vec<&'static Testimonial> = TESTIMONIALS_2.iter().filter(|t| !t.featured).collect();

    rsx! {
        Panel { id: "testimonials",
            div { class: "h-px" }
            div { class: "h-4" }
            div { class: "screen-line-bottom h-px" }
            div { class: "flex items-center justify-center py-4",
                EyebrowTitle { text: "Trusted by past teammates & mentors" }
            }
            div { class: "screen-line-bottom h-px" }

            div { class: "grid gap-4 py-4 sm:hidden",
                for item in all.iter().copied().take(9) {
                    TestimonialItem { item, class: "screen-line-top screen-line-bottom" }
                }
            }

            div { class: "hidden gap-2 p-2 sm:grid sm:grid-cols-2",
                for item in featured {
                    div { class: "rounded-xl bg-background inset-ring-1 inset-ring-foreground/20",
                        TestimonialItem { item, show_icon: true }
                    }
                }
                Marquee { items: row1, reverse: false }
                Marquee { items: row2, reverse: true }
            }

            HandwrittenNote { class: "top-8 left-full ml-3 w-36 flex-col items-start xl:flex",
                span { class: "-rotate-3",
                    "made my day,"
                    span { class: "block" }
                    "keeps me going"
                }
                HandwrittenArrow { class: "mt-2 -rotate-3" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Components, blocks and blog

#[allow(dead_code)] // Commented out in main.rs until there's work to list.
#[component]
pub fn Components() -> Element {
    rsx! {
        Panel { id: "components",
            PanelHeader {
                PanelTitle { id: "components", title: "Components", count: COMPONENTS.len() }
            }
            HandwrittenNote { class: "top-4 left-full ml-3 w-36 flex-col items-start xl:flex",
                span { class: "-rotate-3",
                    "free to copy"
                    span { class: "block" }
                    "yours to keep"
                }
                HandwrittenArrow { class: "mt-2 -rotate-3" }
            }
            div { class: "relative pt-4",
                div { class: "pointer-events-none absolute inset-0 -z-1 grid grid-cols-1 max-sm:hidden sm:grid-cols-2 md:grid-cols-3",
                    div { class: "border-r border-line" }
                    div { class: "border-r border-line max-md:hidden" }
                }
                div { class: "screen-line-bottom h-px" }
                ul { class: "grid grid-cols-1 overflow-x-clip sm:grid-cols-2 md:grid-cols-3",
                    for c in COMPONENTS.iter().take(9) {
                        li { class: "max-sm:screen-line-bottom sm:max-md:nth-[2n+1]:screen-line-bottom md:nth-[3n+1]:screen-line-bottom",
                            a { class: "flex items-center gap-2 p-4 pr-2 transition-[background-color] ease-out hover:bg-accent-muted", href: "#components",
                                h3 { class: "line-clamp-1 leading-snug font-medium text-balance", "{c.title}" }
                                if c.new {
                                    span { class: "flex shrink-0 items-center justify-center",
                                        span { class: "flex size-1.5 rounded-sm bg-info" }
                                        span { class: "sr-only", "(New)" }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "screen-line-top h-4 before:-top-px" }
            }
            PanelFooter {
                SecondaryButton { href: "#components",
                    "All components"
                    lucide_dioxus::ArrowRight {}
                }
            }
        }
    }
}

/// Grey placeholder standing in for a cover image or mockup.
#[component]
fn CoverPlaceholder(label: &'static str) -> Element {
    rsx! {
        div { class: "relative select-none",
            div { class: "flex aspect-1200/630 items-center justify-center rounded-xl bg-muted/60 bg-[radial-gradient(var(--color-line)_1px,transparent_0)] bg-size-[10px_10px] font-mono text-xs text-muted-foreground grayscale transition-[filter] duration-300 group-hover/post:grayscale-0",
                "{label}"
            }
            div { class: "pointer-events-none absolute inset-0 rounded-xl inset-ring-1 inset-ring-black/15 dark:inset-ring-white/15" }
        }
    }
}

#[component]
fn CardGrid(cards: &'static [Card], kind: &'static str) -> Element {
    rsx! {
        div { class: "relative py-4",
            div { class: "pointer-events-none absolute inset-0 -z-1 grid grid-cols-1 gap-4 max-sm:hidden sm:grid-cols-2",
                div { class: "border-r border-line" }
                div { class: "border-l border-line" }
            }
            ul { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                for card in cards.iter().take(4) {
                    li { class: "max-sm:screen-line-top max-sm:screen-line-bottom sm:nth-[2n+1]:screen-line-top sm:nth-[2n+1]:screen-line-bottom",
                        div { class: "group/post relative flex h-full flex-col gap-2 p-2 transition-[background-color] ease-out hover:bg-accent-muted",
                            CoverPlaceholder { label: kind }
                            div { class: "flex flex-col gap-1 p-2",
                                h3 { class: "text-lg/snug font-medium text-balance",
                                    a { href: "#{kind}",
                                        span { class: "absolute inset-0", "aria-hidden": "true" }
                                        "{card.title}"
                                    }
                                }
                                p { class: "text-sm text-muted-foreground", "{card.subtitle}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)] // Commented out in main.rs until there's work to list.
#[component]
pub fn Blocks() -> Element {
    rsx! {
        Panel { id: "blocks",
            PanelHeader {
                PanelTitle { id: "blocks", title: "Blocks", count: BLOCKS.len() }
            }
            HandwrittenNote { class: "top-16 right-full mr-4 w-36 flex-col items-end xl:flex",
                span { class: "-rotate-3 text-right",
                    "same deal,"
                    span { class: "block" }
                    "bigger pieces"
                }
                HandwrittenArrow { class: "-scale-x-100 -rotate-6" }
            }
            CardGrid { cards: BLOCKS, kind: "blocks" }
            PanelFooter {
                SecondaryButton { href: "#blocks",
                    "All blocks"
                    lucide_dioxus::ArrowRight {}
                }
            }
        }
    }
}

#[component]
pub fn Blog() -> Element {
    rsx! {
        Panel { id: "blog",
            PanelHeader {
                PanelTitle { id: "blog", title: "Blog", count: POSTS.len() }
            }
            CardGrid { cards: POSTS, kind: "blog" }
            PanelFooter {
                SecondaryButton { href: "#blog",
                    "All posts"
                    lucide_dioxus::ArrowRight {}
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Stack

#[component]
pub fn TechStack() -> Element {
    rsx! {
        Panel { id: "stack",
            PanelHeader {
                PanelTitle { id: "stack", title: "Stack" }
            }
            div { class: "relative [--badge-height:--spacing(6)] [--col-left-width:--spacing(48)]",
                div { class: "pointer-events-none absolute inset-y-0 left-(--col-left-width) -z-1 w-px border-r border-dashed border-line max-sm:hidden", "aria-hidden": "true" }
                for (index, (category, items)) in TECH_STACK.iter().enumerate() {
                    div { class: "grid items-start gap-y-2 border-b border-line py-4 last:border-none sm:grid-cols-[var(--col-left-width)_1fr]",
                        div { class: "pl-4 text-sm/(--badge-height)",
                            span { class: "mr-1.5 font-mono text-muted-foreground/80 select-none", "aria-hidden": "true",
                                "{index + 1:02}"
                            }
                            "{category}"
                        }
                        ul { class: "flex flex-wrap gap-1.5 px-4",
                            for item in items.iter() {
                                li { class: "flex",
                                    a {
                                        class: "flex h-(--badge-height) items-center justify-center gap-1.25 rounded-full bg-zinc-50/80 px-2 font-mono text-xs text-foreground inset-ring-1 inset-ring-border dark:bg-zinc-900/80",
                                        href: item.href,
                                        target: "_blank",
                                        rel: "noopener",
                                        "{item.title}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Experience, education, projects, recognition

/// "MM.YYYY" or "YYYY" -> months since year 0, rounding a bare year to its first or last month.
fn period_months(s: &str, last_month_if_year: bool) -> i64 {
    if let Some((m, y)) = s.split_once('.') {
        y.parse::<i64>().unwrap_or(0) * 12 + m.parse::<i64>().unwrap_or(1) - 1
    } else {
        s.parse::<i64>().unwrap_or(0) * 12 + if last_month_if_year { 11 } else { 0 }
    }
}

fn format_duration(start: &str, end: Option<&str>) -> String {
    if let Some(end) = end
        && !start.contains('.')
        && !end.contains('.')
    {
        let years = end.parse::<i64>().unwrap_or(0) - start.parse::<i64>().unwrap_or(0);
        return if years > 0 { format!("{years}y") } else { String::new() };
    }
    let from = period_months(start, false);
    let to = match end {
        Some(end) => period_months(end, true),
        None => {
            let now = js_sys::Date::new_0();
            now.get_full_year() as i64 * 12 + now.get_month() as i64
        }
    };
    let total = to - from + 1;
    match total {
        ..=0 => String::new(),
        1..12 => format!("{total}m"),
        _ if total % 12 == 0 => format!("{}y", total / 12),
        _ => format!("{}y {}m", total / 12, total % 12),
    }
}

#[component]
fn Period(start: &'static str, end: Option<&'static str>) -> Element {
    rsx! {
        span { class: "flex items-center gap-0.5 tabular-nums",
            span { "{start}" }
            span { class: "font-mono", "—" }
            if let Some(end) = end {
                span { "{end}" }
            } else {
                span { class: "[&_svg]:size-4.5 [&_svg]:translate-y-[0.5px]", "aria-label": "Present",
                    lucide_dioxus::Infinity { stroke_width: 1 }
                }
            }
        }
    }
}

/// Up-down chevrons that swap to a single chevron pointing the way it will move.
#[component]
fn ChevronsIcon(open: bool) -> Element {
    rsx! {
        span { class: "flex shrink-0 text-muted-foreground [&_svg]:h-lh [&_svg]:w-4",
            if open {
                lucide_dioxus::ChevronsDownUp {}
            } else {
                lucide_dioxus::ChevronsUpDown {}
            }
        }
    }
}

/// The hover pill behind collapsible rows, starting after the icon.
const ROW_TRIGGER: &str = "group relative block w-full text-left before:absolute before:-top-1 before:-right-1 before:-bottom-1.5 before:left-7 before:-z-1 before:rounded-lg before:transition-[background-color] before:ease-out hover:before:bg-accent-muted outline-none focus-visible:before:inset-ring-2 focus-visible:before:inset-ring-ring/50";

#[component]
fn Prose(lines: &'static [&'static str], class: String) -> Element {
    rsx! {
        div { class: "typeset typeset-description {class}",
            if lines.len() == 1 {
                p { "{lines[0]}" }
            } else {
                ul {
                    for line in lines.iter() {
                        li { "{line}" }
                    }
                }
            }
        }
    }
}

#[component]
fn PositionItem(position: &'static Position) -> Element {
    let has_description = !position.description.is_empty();
    let mut open = use_signal(|| position.expanded && has_description);
    let duration = format_duration(position.start, position.end);

    rsx! {
        div { class: "group/experience-position relative",
            div { class: "pointer-events-none absolute bottom-0 left-3 hidden size-4 bg-background group-last/experience-position:flex",
                span { class: "size-full -translate-y-2.25 rounded-bl-sm border-b border-l" }
            }
            button {
                class: if has_description { "{ROW_TRIGGER}" } else { "{ROW_TRIGGER} cursor-default before:content-none" },
                disabled: !has_description,
                onclick: move |_| open.toggle(),
                div { class: "relative z-1 mb-1 flex items-start gap-3 text-base",
                    IconTile { lucide_dioxus::BriefcaseBusiness {} }
                    h4 { class: "flex-1 font-medium text-balance", "{position.title}" }
                    if has_description {
                        ChevronsIcon { open: open() }
                    }
                }
                dl { class: "flex items-center gap-2 pl-9 text-sm text-muted-foreground",
                    if !position.employment_type.is_empty() {
                        div {
                            dt { class: "sr-only", "Employment Type" }
                            dd { "{position.employment_type}" }
                        }
                        MetaSeparator {}
                    }
                    div {
                        dt { class: "sr-only", "Employment Period" }
                        dd { Period { start: position.start, end: position.end } }
                    }
                    if !duration.is_empty() {
                        MetaSeparator {}
                        div {
                            dt { class: "sr-only", "Duration" }
                            dd { class: "tabular-nums", "{duration}" }
                        }
                    }
                }
            }
            if open() {
                Prose { lines: position.description, class: "pt-3 pb-1 pl-9" }
            }
            TagList { items: position.skills, class: "pt-3 pl-9" }
        }
    }
}

#[component]
fn ExperienceItem(experience: &'static Experience) -> Element {
    rsx! {
        div { id: "experience-{experience.id}", class: "group/experience screen-line-bottom scroll-mt-14 space-y-4 py-4",
            div { class: "flex items-start gap-3 sm:items-center",
                div { class: "flex size-6 shrink-0 items-center justify-center select-none",
                    span { class: "flex size-2 rounded-full bg-zinc-300 dark:bg-zinc-600" }
                }
                div { class: "flex min-w-0 flex-1 flex-col gap-x-3 gap-y-1 pr-1 sm:flex-row sm:items-baseline sm:justify-between",
                    h3 { class: "text-xl/6 font-medium",
                        a { class: "link", href: experience.website, target: "_blank", rel: "noopener", "{experience.company}" }
                    }
                    dl { class: "flex min-w-0 items-center gap-1.5 text-sm whitespace-nowrap text-muted-foreground",
                        dt { class: "sr-only", "Location" }
                        dd { class: "truncate", "{experience.location}" }
                        dt { class: "sr-only", "Location type" }
                        dd { "({experience.location_type})" }
                        if experience.current {
                            dt { class: "sr-only", "Employment status" }
                            dd {
                                span { class: "sr-only", "Current" }
                                span { class: "relative flex size-2.5 translate-px items-center justify-center",
                                    span { class: "absolute inline-flex size-2.5 animate-ping rounded-full bg-info opacity-50" }
                                    span { class: "relative inline-flex size-1.5 rounded-full bg-info" }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "relative space-y-4 before:absolute before:left-3 before:h-full before:w-px before:bg-border",
                for position in experience.positions.iter() {
                    PositionItem { position }
                }
            }
        }
    }
}

#[component]
fn ShowMoreButton(open: Signal<bool>) -> Element {
    rsx! {
        SecondaryButton { onclick: move |_| open.toggle(),
            if open() { "Show less" } else { "Show more" }
            span { class: if open() { "rotate-180 transition-transform [&_svg]:size-4" } else { "transition-transform [&_svg]:size-4" },
                lucide_dioxus::ChevronDown {}
            }
        }
    }
}

#[component]
pub fn Experiences() -> Element {
    const MAX: usize = 3;
    let open = use_signal(|| false);
    rsx! {
        Panel { id: "experience",
            PanelHeader {
                PanelTitle { id: "experience", title: "Experience" }
            }
            div { class: "px-4",
                for experience in EXPERIENCES.iter().take(MAX) {
                    ExperienceItem { experience }
                }
                if open() {
                    for experience in EXPERIENCES.iter().skip(MAX) {
                        ExperienceItem { experience }
                    }
                }
            }
            if EXPERIENCES.len() > MAX {
                div { class: "-mt-px flex items-center justify-center py-4",
                    ShowMoreButton { open }
                }
            }
        }
    }
}

#[component]
fn EducationItem(item: &'static Education) -> Element {
    let has_description = !item.description.is_empty();
    let mut open = use_signal(|| item.expanded && has_description);
    rsx! {
        div { class: "group/education-item relative before:absolute before:left-3 before:h-full before:w-px before:bg-border",
            div { class: "pointer-events-none absolute bottom-0 left-3 hidden size-4 bg-background group-last/education-item:flex", "aria-hidden": "true",
                span { class: "size-full -translate-y-2.25 rounded-bl-sm border-b border-l" }
            }
            button {
                class: if has_description { "{ROW_TRIGGER}" } else { "{ROW_TRIGGER} cursor-default before:content-none" },
                disabled: !has_description,
                onclick: move |_| open.toggle(),
                div { class: "relative z-1 mb-1 flex items-start gap-3 text-base",
                    IconTile { lucide_dioxus::GraduationCap {} }
                    h3 { class: "flex-1 font-medium text-balance", "{item.school}" }
                    if has_description {
                        ChevronsIcon { open: open() }
                    }
                }
                dl { class: "flex flex-wrap items-center gap-x-2 pl-9 text-sm text-muted-foreground",
                    div {
                        dt { class: "sr-only", "Study period" }
                        dd { Period { start: item.start, end: item.end } }
                    }
                    if !item.degree.is_empty() {
                        MetaSeparator {}
                        div {
                            dt { class: "sr-only", "Degree" }
                            dd { "{item.degree}" }
                        }
                    }
                    if !item.field.is_empty() {
                        MetaSeparator {}
                        div {
                            dt { class: "sr-only", "Field of study" }
                            dd { "{item.field}" }
                        }
                    }
                }
            }
            if open() {
                Prose { lines: item.description, class: "pt-3 pb-1 pl-9" }
            }
            TagList { items: item.skills, class: "pt-3 pl-9" }
        }
    }
}

#[component]
pub fn EducationPanel() -> Element {
    rsx! {
        Panel { id: "education",
            PanelHeader {
                PanelTitle { id: "education", title: "Education" }
            }
            for item in EDUCATION.iter() {
                div { id: "education-{item.id}", class: "screen-line-bottom scroll-mt-14 p-4",
                    EducationItem { item }
                }
            }
        }
    }
}

#[component]
fn ProjectItem(project: &'static Project) -> Element {
    let mut open = use_signal(|| project.expanded);
    let single = project.end == Some(project.start);
    rsx! {
        div {
            div { class: "relative flex items-center hover:bg-accent-muted",
                IconTile { class: "mx-4", lucide_dioxus::Box {} }
                div { class: "flex flex-1 items-center gap-2 border-l border-dashed border-line p-4",
                    div { class: "flex-1",
                        h3 { class: "mb-1 leading-snug font-medium text-balance",
                            button { class: "text-left", onclick: move |_| open.toggle(),
                                span { class: "absolute inset-0", "aria-hidden": "true" }
                                "{project.title}"
                            }
                        }
                        dl { class: "text-sm text-muted-foreground",
                            dt { class: "sr-only", "Period" }
                            dd {
                                if single {
                                    span { "{project.start}" }
                                } else {
                                    Period { start: project.start, end: project.end }
                                }
                            }
                        }
                    }
                    a {
                        class: "relative flex size-6 shrink-0 items-center justify-center text-muted-foreground after:absolute after:-inset-2 hover:text-foreground [&_svg]:pointer-events-none [&_svg]:size-4",
                        href: project.link,
                        target: "_blank",
                        rel: "noopener",
                        "aria-label": "Open project",
                        title: "Open project",
                        lucide_dioxus::Link {}
                    }
                    span { class: "shrink-0 text-muted-foreground [&_svg]:size-4",
                        if open() {
                            lucide_dioxus::ChevronsDownUp {}
                        } else {
                            lucide_dioxus::ChevronsUpDown {}
                        }
                    }
                }
            }
            if open() {
                div { class: "space-y-4 border-t border-line p-4",
                    Prose { lines: project.description, class: "" }
                    TagList { items: project.skills }
                }
            }
        }
    }
}

#[component]
fn CollapsibleRows(count: usize, max: usize, children: Element, rest: Element) -> Element {
    let open = use_signal(|| false);
    rsx! {
        ul { {children} }
        if open() {
            ul { {rest} }
        }
        if count > max {
            div { class: "screen-line-top -mt-px flex items-center justify-center py-4",
                ShowMoreButton { open }
            }
        }
    }
}

#[component]
pub fn Projects() -> Element {
    const MAX: usize = 4;
    rsx! {
        Panel { id: "projects",
            PanelHeader {
                PanelTitle { id: "projects", title: "Projects", count: PROJECTS.len() }
            }
            CollapsibleRows {
                count: PROJECTS.len(),
                max: MAX,
                rest: rsx! {
                    for project in PROJECTS.iter().skip(MAX) {
                        li { class: "border-b border-line", ProjectItem { project } }
                    }
                },
                for project in PROJECTS.iter().take(MAX) {
                    li { class: "border-b border-line", ProjectItem { project } }
                }
            }
        }
    }
}

#[component]
fn RecognitionItem(entry: &'static Recognition) -> Element {
    let mut open = use_signal(|| false);
    match entry {
        Recognition::Award { title, prize, date, grade, description } => {
            let can_expand = !description.is_empty();
            rsx! {
                div {
                    div { class: "relative flex items-center hover:bg-accent-muted",
                        IconTile { class: "mx-4", lucide_dioxus::Crown {} }
                        div { class: "flex flex-1 items-center gap-2 border-l border-dashed border-line p-4",
                            div { class: "flex-1",
                                h3 { class: "mb-1 leading-snug font-medium text-balance",
                                    if can_expand {
                                        button { class: "text-left", onclick: move |_| open.toggle(),
                                            span { class: "absolute inset-0", "aria-hidden": "true" }
                                            "{title}"
                                        }
                                    } else {
                                        "{title}"
                                    }
                                }
                                dl { class: "flex flex-wrap items-center gap-x-2 text-sm text-muted-foreground",
                                    KindTerm { kind: "Award" }
                                    div {
                                        dt { class: "sr-only", "Prize" }
                                        dd { "{prize}" }
                                    }
                                    MetaSeparator {}
                                    div {
                                        dt { class: "sr-only", "Awarded in" }
                                        dd { "{date}" }
                                    }
                                    MetaSeparator {}
                                    div {
                                        dt { class: "sr-only", "Received in" }
                                        dd { "{grade}" }
                                    }
                                }
                            }
                            if can_expand {
                                span { class: "shrink-0 text-muted-foreground [&_svg]:size-4",
                                    if open() {
                                        lucide_dioxus::ChevronsDownUp {}
                                    } else {
                                        lucide_dioxus::ChevronsUpDown {}
                                    }
                                }
                            }
                        }
                    }
                    if open() {
                        Prose { lines: description, class: "border-t border-line p-4" }
                    }
                }
            }
        }
        Recognition::Credential { kind, title, issuer, date, url } => rsx! {
            div { class: "relative flex items-center pr-4 hover:bg-accent-muted",
                IconTile { class: "mx-4", lucide_dioxus::CircleCheckBig {} }
                div { class: "flex-1 space-y-1 border-l border-dashed border-line p-4",
                    h3 { class: "leading-snug font-medium text-balance",
                        a { href: *url, target: "_blank", rel: "noopener",
                            span { class: "absolute inset-0", "aria-hidden": "true" }
                            "{title}"
                        }
                    }
                    dl { class: "flex flex-wrap items-center gap-x-2 text-sm text-muted-foreground",
                        KindTerm { kind: *kind }
                        div {
                            dt { class: "sr-only", "Issued by" }
                            dd {
                                span { "aria-hidden": "true", "@" }
                                span { class: "ml-0.5", "{issuer}" }
                            }
                        }
                        MetaSeparator {}
                        div {
                            dt { class: "sr-only", "Issued on" }
                            dd { "{date}" }
                        }
                    }
                }
                span { class: "text-muted-foreground [&_svg]:size-4", lucide_dioxus::ArrowUpRight {} }
            }
        },
    }
}

#[component]
fn KindTerm(kind: &'static str) -> Element {
    rsx! {
        div { class: "max-sm:mb-1 max-sm:basis-full",
            dt { class: "sr-only", "Type" }
            dd {
                Tag { class: "flex h-5 w-fit", "{kind}" }
            }
        }
    }
}

#[component]
pub fn RecognitionPanel() -> Element {
    const MAX: usize = 4;
    rsx! {
        Panel { id: "recognition",
            PanelHeader {
                PanelTitle { id: "recognition", title: "Recognition", count: RECOGNITION.len() }
            }
            CollapsibleRows {
                count: RECOGNITION.len(),
                max: MAX,
                rest: rsx! {
                    for entry in RECOGNITION.iter().skip(MAX) {
                        li { class: "border-b border-line", RecognitionItem { entry } }
                    }
                },
                for entry in RECOGNITION.iter().take(MAX) {
                    li { class: "border-b border-line", RecognitionItem { entry } }
                }
            }
        }
    }
}
