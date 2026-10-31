use dioxus::prelude::*;

use crate::data::{MAIN_NAV, SOCIAL_LINKS, USER};
use crate::mark::FlatMark;
use crate::ui::{GitHubIcon, format_compact};

/// Whether the command menu is open; shared by the header and the mobile nav.
#[derive(Clone, Copy)]
pub struct CommandMenuOpen(pub Signal<bool>);

#[component]
pub fn SiteHeader() -> Element {
    rsx! {
        header { class: "sticky top-0 z-50 h-(--header-height) max-w-screen overflow-x-clip bg-background px-2 pt-1",
            div { class: "screen-line-top screen-line-bottom mx-auto flex h-full items-center gap-2 border-x border-x-line pr-2 pl-4 after:z-1 sm:gap-4 md:max-w-3xl",
                a { href: "#", "aria-label": "Home",
                    FlatMark { text: USER.alias.to_string(), class: "h-6 shrink-0" }
                }

                div { class: "flex-1" }

                nav { class: "flex items-center gap-4 max-sm:hidden",
                    for item in MAIN_NAV.iter() {
                        a {
                            class: "text-sm font-medium tracking-wide text-muted-foreground transition-[color] hover:text-foreground",
                            href: item.href,
                            "{item.title}"
                        }
                    }
                }

                div { class: "flex items-center",
                    div { class: "mr-2 h-5 w-px self-center bg-border max-sm:hidden" }
                    CommandMenuTrigger { class: "max-sm:hidden" }
                    div { class: "mx-2 h-5 w-px self-center bg-border max-sm:hidden" }
                    GitHubStars {}
                    div { class: "mx-2 h-5 w-px self-center bg-border" }
                    ThemeToggle {}
                }
            }
        }
    }
}

#[component]
fn CommandMenuTrigger(#[props(default)] class: String) -> Element {
    let mut open = use_context::<CommandMenuOpen>().0;
    rsx! {
        button {
            class: "flex h-8 items-center gap-2 rounded-lg border border-transparent px-1.5 text-sm text-muted-foreground transition-colors hover:bg-muted hover:text-foreground {class}",
            "aria-label": "Open command menu",
            onclick: move |_| open.set(true),
            lucide_dioxus::Search { size: 16 }
            kbd { class: "pointer-events-none flex h-5 items-center gap-1 rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none",
                "Ctrl K"
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct Repo {
    stargazers_count: u64,
}

#[component]
fn GitHubStars() -> Element {
    let stars = use_resource(|| async move {
        let code = format!(
            "const r = await fetch('https://api.github.com/repos/{}'); if (!r.ok) return {{ stargazers_count: 0 }}; return await r.json();",
            USER.source_repo
        );
        crate::js::eval::<Repo>(&code).await.map(|r| r.stargazers_count).unwrap_or(0)
    });
    let count = (*stars.read()).map(format_compact).unwrap_or_else(|| "0".into());

    rsx! {
        a {
            class: "flex h-8 items-center gap-1.5 rounded-lg px-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground [&_svg]:size-4",
            href: "https://github.com/{USER.source_repo}",
            target: "_blank",
            rel: "noopener",
            "aria-label": "Source code on GitHub",
            GitHubIcon {}
            span { class: "font-mono text-[13px] tabular-nums", "{count}" }
        }
    }
}

/// Adapted from toggles.dev "Dark Side": a half-filled circle that turns over.
#[component]
fn ThemeToggle() -> Element {
    let mut dark = use_signal(crate::js::is_dark);

    use_hook(move || {
        crate::js::on_window::<web_sys::KeyboardEvent>("keydown", move |e| {
            if e.key() == "d" && !e.ctrl_key() && !e.meta_key() && !e.alt_key() && !crate::js::is_typing() {
                let next = !crate::js::is_dark();
                crate::js::set_dark(next);
                dark.set(next);
            }
        });
    });

    rsx! {
        button {
            class: "relative inline-flex size-8 touch-manipulation items-center justify-center rounded-[10px] transition-all hover:bg-muted active:scale-[0.98] dark:hover:bg-muted/50",
            "aria-label": "Toggle mode (D)",
            title: "Toggle mode (D)",
            onclick: move |_| {
                let next = !dark();
                crate::js::set_dark(next);
                crate::js::play_click();
                dark.set(next);
            },
            svg { class: "size-3.5", "viewBox": "0 0 32 32", fill: "currentColor", "aria-hidden": "true",
                path {
                    class: "origin-center motion-safe:transition-transform motion-safe:duration-500 motion-safe:ease-[ease]",
                    style: if dark() { "transform: rotate(180deg)" },
                    d: "M16 .5C7.4.5.5 7.4.5 16S7.4 31.5 16 31.5 31.5 24.6 31.5 16 24.6.5 16 .5zm0 28.1V3.4C23 3.4 28.6 9 28.6 16S23 28.6 16 28.6z",
                }
            }
        }
    }
}

struct Command {
    group: &'static str,
    title: String,
    href: String,
    external: bool,
}

fn commands() -> Vec<Command> {
    let mut out = vec![Command { group: "Pages", title: "Home".into(), href: "#".into(), external: false }];
    for (title, id) in [
        ("About", "hello"),
        ("Components", "components"),
        ("Blocks", "blocks"),
        ("Blog", "blog"),
        ("Stack", "stack"),
        ("Experience", "experience"),
        ("Education", "education"),
        ("Projects", "projects"),
        ("Recognition", "recognition"),
    ] {
        out.push(Command { group: "Sections", title: title.into(), href: format!("#{id}"), external: false });
    }
    for link in SOCIAL_LINKS {
        out.push(Command { group: "Social", title: link.title.into(), href: link.href.into(), external: true });
    }
    out
}

/// A small Ctrl+K palette for jumping around the page.
#[component]
pub fn CommandMenu() -> Element {
    let mut open = use_context::<CommandMenuOpen>().0;
    let mut query = use_signal(String::new);
    let mut active = use_signal(|| 0usize);

    use_hook(move || {
        crate::js::on_window::<web_sys::KeyboardEvent>("keydown", move |e| {
            let key = e.key().to_ascii_lowercase();
            if key == "k" && (e.ctrl_key() || e.meta_key()) {
                e.prevent_default();
                open.toggle();
            } else if key == "/" && !crate::js::is_typing() && !open() {
                e.prevent_default();
                open.set(true);
            }
        });
    });

    if !open() {
        return rsx! {};
    }

    let q = query().to_lowercase();
    let items: Vec<Command> = commands().into_iter().filter(|c| q.is_empty() || c.title.to_lowercase().contains(&q)).collect();
    let selected = active().min(items.len().saturating_sub(1));

    let mut close = move || {
        open.set(false);
        query.set(String::new());
        active.set(0);
    };

    let go = move |href: String, external: bool| {
        if external {
            document::eval(&format!("window.open({}, '_blank', 'noopener')", crate::js::js_str(&href)));
        } else {
            document::eval(&format!("location.hash = {};", crate::js::js_str(href.trim_start_matches('#'))));
        }
    };

    let targets: Vec<(String, bool)> = items.iter().map(|c| (c.href.clone(), c.external)).collect();
    // Show each group heading above the first item in that group.
    let rows: Vec<(usize, Command, bool)> = items
        .into_iter()
        .enumerate()
        .scan("", |last, (i, c)| {
            let first = c.group != *last;
            *last = c.group;
            Some((i, c, first))
        })
        .collect();
    let empty = rows.is_empty();

    rsx! {
        div {
            class: "fixed inset-0 z-[100] flex items-start justify-center bg-black/30 px-4 pt-[15vh] backdrop-blur-[2px]",
            onclick: move |_| close(),
            div {
                class: "w-full max-w-lg overflow-hidden rounded-xl bg-popover text-popover-foreground shadow-2xl ring-1 ring-foreground/10 dark:ring-foreground/20",
                onclick: move |e| e.stop_propagation(),
                div { class: "flex items-center gap-2 border-b px-3",
                    span { class: "text-muted-foreground [&_svg]:size-4", lucide_dioxus::Search {} }
                    input {
                        class: "h-11 flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground",
                        placeholder: "Type a command or search...",
                        autofocus: true,
                        onmounted: move |e| async move {
                            let _ = e.set_focus(true).await;
                        },
                        value: "{query}",
                        oninput: move |e| {
                            query.set(e.value());
                            active.set(0);
                        },
                        onkeydown: move |e| {
                            match e.key() {
                                Key::Escape => close(),
                                Key::ArrowDown => active.set((selected + 1).min(targets.len().saturating_sub(1))),
                                Key::ArrowUp => active.set(selected.saturating_sub(1)),
                                Key::Enter => {
                                    if let Some((href, external)) = targets.get(selected).cloned() {
                                        go(href, external);
                                        close();
                                    }
                                }
                                _ => {}
                            }
                        },
                    }
                    kbd { class: "rounded-sm bg-muted px-1 font-sans text-xs text-muted-foreground", "Esc" }
                }
                div { class: "no-scrollbar max-h-80 overflow-y-auto p-1",
                    if empty {
                        p { class: "py-6 text-center text-sm text-muted-foreground", "No results found." }
                    }
                    for (i, item, first_in_group) in rows.into_iter() {
                        if first_in_group {
                            div { class: "px-2 pt-2 pb-1 text-xs font-medium text-muted-foreground", "{item.group}" }
                        }
                        button {
                            class: if i == selected { "flex w-full items-center gap-2 rounded-lg bg-accent px-2 py-1.5 text-left text-sm" } else { "flex w-full items-center gap-2 rounded-lg px-2 py-1.5 text-left text-sm" },
                            onmouseenter: move |_| active.set(i),
                            onclick: {
                                let href = item.href.clone();
                                let external = item.external;
                                move |_| {
                                    go(href.clone(), external);
                                    close();
                                }
                            },
                            span { class: "text-muted-foreground [&_svg]:size-4",
                                if item.external {
                                    lucide_dioxus::ArrowUpRight {}
                                } else {
                                    lucide_dioxus::Hash {}
                                }
                            }
                            "{item.title}"
                        }
                    }
                }
            }
        }
    }
}

/// Floating pill nav for phones, where the header links are hidden.
#[component]
pub fn SiteBottomNav() -> Element {
    let mut open = use_context::<CommandMenuOpen>().0;
    rsx! {
        div { class: "fixed bottom-2 left-1/2 z-50 flex w-fit -translate-x-1/2 items-center rounded-xl bg-popover py-1 pr-1 pl-2.5 shadow-md ring ring-foreground/10 sm:hidden dark:ring-foreground/20",
            button {
                class: "flex min-w-20 items-center gap-2 text-sm text-muted-foreground",
                onclick: move |_| open.set(true),
                lucide_dioxus::Search { size: 16 }
                "Search"
            }
            div { class: "mr-1 ml-2.5 h-6 w-px self-center bg-border" }
            nav { class: "flex items-center",
                a { class: "rounded-lg px-2 py-1.5 text-sm font-medium text-foreground", href: "#", "Home" }
                a { class: "rounded-lg px-2 py-1.5 text-sm font-medium text-muted-foreground", href: "#projects", "Projects" }
                a { class: "rounded-lg px-2 py-1.5 text-sm font-medium text-muted-foreground", href: "#blog", "Blog" }
            }
        }
    }
}

/// Appears once you've scrolled a screen down.
#[component]
pub fn ScrollToTop() -> Element {
    let mut visible = use_signal(|| false);
    use_hook(move || {
        crate::js::on_window::<web_sys::Event>("scroll", move |_| {
            let show = crate::js::scroll_y() > 600.0;
            if *visible.peek() != show {
                visible.set(show);
            }
        });
    });

    rsx! {
        button {
            class: if visible() { "fixed right-4 bottom-4 z-50 inline-flex size-9 items-center justify-center rounded-full bg-popover text-muted-foreground shadow-md ring ring-foreground/10 transition-all hover:text-foreground max-sm:bottom-16 dark:ring-foreground/20 [&_svg]:size-4" } else { "pointer-events-none fixed right-4 bottom-4 z-50 inline-flex size-9 translate-y-2 items-center justify-center rounded-full opacity-0 transition-all max-sm:bottom-16 [&_svg]:size-4" },
            "aria-label": "Scroll to top",
            onclick: move |_| {
                document::eval("window.scrollTo({ top: 0, behavior: 'smooth' })");
            },
            lucide_dioxus::ArrowUp {}
        }
    }
}
