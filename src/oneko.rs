//! A Rust port of oneko.js, the little cat that chases your cursor.
//!
//! Original: https://github.com/adryd325/oneko.js
//! Copyright © 2022 adryd. Released under the MIT License (see licenses/oneko.js-MIT.txt).
//! Seen on ramx.in by Ramkrishna Swarnkar, which is where the idea came from.

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const SPEED: f64 = 10.0;
const FRAME_MS: u32 = 100;
const SPRITE: &str = "/oneko/oneko.gif";

#[derive(Clone, Copy, PartialEq)]
enum Idle {
    Sleeping,
    ScratchSelf,
    ScratchWallN,
    ScratchWallS,
    ScratchWallE,
    ScratchWallW,
}

fn frames(name: &str) -> &'static [(i32, i32)] {
    match name {
        "idle" => &[(-3, -3)],
        "alert" => &[(-7, -3)],
        "scratchSelf" => &[(-5, 0), (-6, 0), (-7, 0)],
        "scratchWallN" => &[(0, 0), (0, -1)],
        "scratchWallS" => &[(-7, -1), (-6, -2)],
        "scratchWallE" => &[(-2, -2), (-2, -3)],
        "scratchWallW" => &[(-4, 0), (-4, -1)],
        "tired" => &[(-3, -2)],
        "sleeping" => &[(-2, 0), (-2, -1)],
        "N" => &[(-1, -2), (-1, -3)],
        "NE" => &[(0, -2), (0, -3)],
        "E" => &[(-3, 0), (-3, -1)],
        "SE" => &[(-5, -1), (-5, -2)],
        "S" => &[(-6, -3), (-7, -2)],
        "SW" => &[(-5, -3), (-6, -1)],
        "W" => &[(-4, -2), (-4, -3)],
        "NW" => &[(-1, 0), (-1, -1)],
        _ => &[(-3, -3)],
    }
}

fn idle_name(idle: Idle) -> &'static str {
    match idle {
        Idle::Sleeping => "sleeping",
        Idle::ScratchSelf => "scratchSelf",
        Idle::ScratchWallN => "scratchWallN",
        Idle::ScratchWallS => "scratchWallS",
        Idle::ScratchWallE => "scratchWallE",
        Idle::ScratchWallW => "scratchWallW",
    }
}

fn random_below(n: usize) -> usize {
    (js_sys::Math::random() * n as f64) as usize % n.max(1)
}

/// The cat's whole state, advanced one step every 100ms like the original.
struct Neko {
    x: f64,
    y: f64,
    mouse_x: f64,
    mouse_y: f64,
    frame_count: usize,
    idle_time: u32,
    idle: Option<Idle>,
    idle_frame: usize,
    sprite: (i32, i32),
}

impl Neko {
    fn new() -> Self {
        Self {
            x: 32.0,
            y: 32.0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            frame_count: 0,
            idle_time: 0,
            idle: None,
            idle_frame: 0,
            sprite: (-3, -3),
        }
    }

    fn set_sprite(&mut self, name: &str, frame: usize) {
        let set = frames(name);
        self.sprite = set[frame % set.len()];
    }

    fn reset_idle(&mut self) {
        self.idle = None;
        self.idle_frame = 0;
    }

    fn idle_step(&mut self, width: f64, height: f64) {
        self.idle_time += 1;

        // Every ~20 seconds, maybe start a nap or a scratch.
        if self.idle_time > 10 && random_below(200) == 0 && self.idle.is_none() {
            let mut options = vec![Idle::Sleeping, Idle::ScratchSelf];
            if self.x < 32.0 {
                options.push(Idle::ScratchWallW);
            }
            if self.y < 32.0 {
                options.push(Idle::ScratchWallN);
            }
            if self.x > width - 32.0 {
                options.push(Idle::ScratchWallE);
            }
            if self.y > height - 32.0 {
                options.push(Idle::ScratchWallS);
            }
            self.idle = Some(options[random_below(options.len())]);
        }

        match self.idle {
            Some(Idle::Sleeping) => {
                if self.idle_frame < 8 {
                    self.set_sprite("tired", 0);
                } else {
                    self.set_sprite("sleeping", self.idle_frame / 4);
                    if self.idle_frame > 192 {
                        self.reset_idle();
                    }
                }
            }
            Some(idle) => {
                self.set_sprite(idle_name(idle), self.idle_frame);
                if self.idle_frame > 9 {
                    self.reset_idle();
                }
            }
            None => {
                self.set_sprite("idle", 0);
                return;
            }
        }
        self.idle_frame += 1;
    }

    fn step(&mut self, width: f64, height: f64) {
        self.frame_count += 1;
        let dx = self.x - self.mouse_x;
        let dy = self.y - self.mouse_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < SPEED || distance < 48.0 {
            self.idle_step(width, height);
            return;
        }

        self.reset_idle();

        if self.idle_time > 1 {
            self.set_sprite("alert", 0);
            // Count down after being alerted before moving.
            self.idle_time = self.idle_time.min(7) - 1;
            return;
        }

        let mut direction = String::new();
        if dy / distance > 0.5 {
            direction.push('N');
        }
        if dy / distance < -0.5 {
            direction.push('S');
        }
        if dx / distance > 0.5 {
            direction.push('W');
        }
        if dx / distance < -0.5 {
            direction.push('E');
        }
        self.set_sprite(&direction, self.frame_count);

        self.x -= dx / distance * SPEED;
        self.y -= dy / distance * SPEED;
        self.x = self.x.clamp(16.0, (width - 16.0).max(16.0));
        self.y = self.y.clamp(16.0, (height - 16.0).max(16.0));
    }
}

#[component]
pub fn Oneko() -> Element {
    // Like the original, stay away for reduced motion; also skip touch screens,
    // where there's no cursor to chase.
    let enabled = use_hook(|| {
        !crate::js::media_matches("(prefers-reduced-motion: reduce)") && !crate::js::media_matches("(hover: none)")
    });

    let mut view = use_signal(|| (32.0f64, 32.0f64, (-3, -3)));

    use_hook(move || {
        if !enabled {
            return;
        }
        let neko = std::rc::Rc::new(std::cell::RefCell::new(Neko::new()));

        let mouse = neko.clone();
        crate::js::on_window::<web_sys::MouseEvent>("mousemove", move |e| {
            let mut n = mouse.borrow_mut();
            n.mouse_x = e.client_x() as f64;
            n.mouse_y = e.client_y() as f64;
        });

        spawn(async move {
            loop {
                TimeoutFuture::new(FRAME_MS).await;
                let (w, h) = crate::js::window_size();
                let mut n = neko.borrow_mut();
                n.step(w, h);
                let next = (n.x, n.y, n.sprite);
                drop(n);
                if *view.peek() != next {
                    view.set(next);
                }
            }
        });
    });

    if !enabled {
        return rsx! {};
    }

    let (x, y, (sx, sy)) = view();
    let left = x - 16.0;
    let top = y - 16.0;
    let bg_x = sx * 32;
    let bg_y = sy * 32;
    rsx! {
        div {
            id: "oneko",
            "aria-hidden": "true",
            style: "width: 32px; height: 32px; position: fixed; pointer-events: none; image-rendering: pixelated; z-index: 2147483647; left: {left}px; top: {top}px; background-image: url({SPRITE}); background-position: {bg_x}px {bg_y}px;",
        }
    }
}
