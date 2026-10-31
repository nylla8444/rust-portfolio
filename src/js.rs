//! Thin wrappers over browser APIs that are simpler to reach through JS than web-sys.

use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Run a JS snippet (its body may `return` a value) and decode the result.
pub async fn eval<T: DeserializeOwned>(code: &str) -> Option<T> {
    document::eval(code).join::<T>().await.ok()
}

/// Escape a Rust string into a JS string literal.
pub fn js_str(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '<' => out.push_str("\\u003c"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub async fn copy_text(text: &str) {
    let code = format!("await navigator.clipboard.writeText({}); return true;", js_str(text));
    let _ = eval::<bool>(&code).await;
}

/// Copies the page URL pointing at `#id`.
pub async fn copy_text_with_location_hash(id: &str) {
    let code = format!(
        "const u = new URL(location.href); u.hash = {}; await navigator.clipboard.writeText(u.toString()); return true;",
        js_str(id)
    );
    let _ = eval::<bool>(&code).await;
}

/// A short metallic click, synthesised so there's no audio file to ship.
pub fn play_click() {
    document::eval(
        r#"
        const Ctx = window.AudioContext || window.webkitAudioContext;
        if (!Ctx) return;
        window.__clickCtx = window.__clickCtx || new Ctx();
        const ctx = window.__clickCtx;
        const t = ctx.currentTime;
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = "triangle";
        osc.frequency.setValueAtTime(1800, t);
        osc.frequency.exponentialRampToValueAtTime(600, t + 0.05);
        gain.gain.setValueAtTime(0.18, t);
        gain.gain.exponentialRampToValueAtTime(0.001, t + 0.08);
        osc.connect(gain).connect(ctx.destination);
        osc.start(t);
        osc.stop(t + 0.09);
        "#,
    );
}

/// Reads the name aloud with the Web Speech API.
pub fn speak(text: &str) {
    document::eval(&format!(
        "if ('speechSynthesis' in window) {{ speechSynthesis.cancel(); speechSynthesis.speak(new SpeechSynthesisUtterance({})); }}",
        js_str(text)
    ));
}

pub fn media_matches(query: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.match_media(query).ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

pub fn now_hours() -> u32 {
    js_sys::Date::new_0().get_hours()
}

pub fn window_size() -> (f64, f64) {
    let w = web_sys::window().unwrap();
    let width = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0);
    let height = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(768.0);
    (width, height)
}

pub fn scroll_y() -> f64 {
    web_sys::window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0)
}

/// Registers a window event listener for the lifetime of the page.
pub fn on_window<E: JsCast + 'static>(event: &str, mut handler: impl FnMut(E) + 'static) {
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
        if let Ok(e) = e.dyn_into::<E>() {
            handler(e);
        }
    });
    if let Some(w) = web_sys::window() {
        let _ = w.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref());
    }
    closure.forget();
}

pub fn is_dark() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .map(|el| el.class_list().contains("dark"))
        .unwrap_or(false)
}

pub fn set_dark(dark: bool) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    if let Some(el) = document.document_element() {
        let _ = el.class_list().toggle_with_force("dark", dark);
    }
    if let Ok(Some(meta)) = document.query_selector("meta[name=\"theme-color\"]") {
        let _ = meta.set_attribute("content", if dark { "#09090b" } else { "#ffffff" });
    }
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item("theme", if dark { "dark" } else { "light" });
    }
}

/// True while focus is in a text field, so single-key shortcuts stay out of the way.
pub fn is_typing() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element())
        .map(|el| {
            let tag = el.tag_name().to_ascii_lowercase();
            tag == "input" || tag == "textarea" || el.has_attribute("contenteditable")
        })
        .unwrap_or(false)
}
