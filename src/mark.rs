//! The 3D lettering at the top of the page: block letters extruded and drawn in
//! isometric projection, with a highlight that follows the cursor and a press
//! effect on click. Same idea as the isometric mark on chanhdai.com, but generated
//! from your alias instead of a hand-drawn logo.

use std::collections::HashSet;
use std::fmt::Write;

use dioxus::prelude::*;

const ROWS: i32 = 5;
const LETTER_WIDTH: i32 = 5;
const LETTER_GAP: i32 = 1;

/// Size of one block on the ground plane, and how tall the letters stand.
const CELL: f64 = 32.0;
const HEIGHT: f64 = 18.0;
const PRESSED_HEIGHT: f64 = 9.0;

const COS30: f64 = 0.866_025_403_784_438_6;

/// A blocky 5×5 alphabet where every letter is one edge-connected shape, so it
/// still reads as letters once it's extruded and tilted.
fn glyph(ch: char) -> [&'static str; 5] {
    match ch.to_ascii_uppercase() {
        'A' => ["#####", "#...#", "#####", "#...#", "#...#"],
        'B' => ["####.", "#..#.", "#####", "#...#", "#####"],
        'C' => ["#####", "#....", "#....", "#....", "#####"],
        'D' => ["####.", "#..##", "#...#", "#..##", "####."],
        'E' => ["#####", "#....", "####.", "#....", "#####"],
        'F' => ["#####", "#....", "####.", "#....", "#...."],
        'G' => ["#####", "#....", "#.###", "#...#", "#####"],
        'H' => ["#...#", "#...#", "#####", "#...#", "#...#"],
        'I' => ["#####", "..#..", "..#..", "..#..", "#####"],
        'J' => ["#####", "...#.", "...#.", "#..#.", "####."],
        'K' => ["#..##", "#.##.", "###..", "#.##.", "#..##"],
        'L' => ["#....", "#....", "#....", "#....", "#####"],
        'M' => ["#####", "#.#.#", "#.#.#", "#...#", "#...#"],
        'N' => ["##..#", "###.#", "#.###", "#..##", "#...#"],
        'O' => ["#####", "#...#", "#...#", "#...#", "#####"],
        'P' => ["#####", "#...#", "#####", "#....", "#...."],
        'Q' => ["#####", "#...#", "#...#", "#..##", "#####"],
        'R' => ["#####", "#...#", "#####", "#..##", "#...#"],
        'S' => ["#####", "#....", "#####", "....#", "#####"],
        'T' => ["#####", "..#..", "..#..", "..#..", "..#.."],
        'U' => ["#...#", "#...#", "#...#", "#...#", "#####"],
        'V' => ["#...#", "#...#", "##.##", ".###.", "..#.."],
        'W' => ["#...#", "#...#", "#.#.#", "#.#.#", "#####"],
        'X' => ["##.##", ".###.", "..#..", ".###.", "##.##"],
        'Y' => ["#...#", "#...#", "#####", "..#..", "..#.."],
        'Z' => ["#####", "...##", "..##.", ".##..", "#####"],
        _ => [".....", ".....", ".....", ".....", "....."],
    }
}

/// Filled cells for a word, as (column, row) on the ground grid.
pub fn cells(text: &str) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for (i, ch) in text.chars().enumerate() {
        let x0 = i as i32 * (LETTER_WIDTH + LETTER_GAP);
        for (row, line) in glyph(ch).iter().enumerate() {
            for (col, px) in line.chars().enumerate() {
                if px == '#' {
                    out.push((x0 + col as i32, row as i32));
                }
            }
        }
    }
    out
}

fn columns(text: &str) -> i32 {
    let n = text.chars().count() as i32;
    (n * (LETTER_WIDTH + LETTER_GAP) - LETTER_GAP).max(1)
}

/// Columns run up-right and rows run down-right, so the word reads along the
/// rising isometric axis.
struct Projection {
    ox: f64,
    oy: f64,
}

impl Projection {
    fn point(&self, col: f64, row: f64, z: f64) -> (f64, f64) {
        (
            self.ox + (col + row) * COS30 * CELL,
            self.oy + (row - col) * 0.5 * CELL - z,
        )
    }
}

#[derive(PartialEq)]
struct CellShapes {
    sides: Vec<String>,
    top: String,
    edges: String,
}

#[derive(PartialEq)]
struct Drawing {
    width: f64,
    height: f64,
    cells: Vec<CellShapes>,
    guides: Vec<String>,
    angle: Option<AngleMark>,
}

#[derive(PartialEq)]
struct AngleMark {
    line: String,
    arc: String,
    label_x: String,
    label_y: String,
}

fn polygon(points: &[(f64, f64)]) -> String {
    let mut d = String::new();
    for (i, (x, y)) in points.iter().enumerate() {
        let _ = write!(d, "{}{x:.2} {y:.2}", if i == 0 { "M" } else { "L" });
    }
    d.push('Z');
    d
}

fn segment(d: &mut String, a: (f64, f64), b: (f64, f64)) {
    let _ = write!(d, "M{:.2} {:.2}L{:.2} {:.2}", a.0, a.1, b.0, b.1);
}

fn draw(text: &str, height: f64) -> Drawing {
    let cols = columns(text);
    let filled: HashSet<(i32, i32)> = cells(text).into_iter().collect();
    let has = |c: i32, r: i32| filled.contains(&(c, r));

    // Leave room above for the letters' height and the angle label.
    let pad = 4.0;
    let proj = Projection { ox: pad, oy: pad + HEIGHT + cols as f64 * 0.5 * CELL };
    let width = (cols + ROWS) as f64 * COS30 * CELL + pad * 2.0;
    let total_height = (cols + ROWS) as f64 * 0.5 * CELL + HEIGHT + pad * 2.0;

    // Back to front: cells further down the screen are closer to the viewer.
    let mut order: Vec<(i32, i32)> = filled.iter().copied().collect();
    order.sort_by_key(|&(c, r)| (r - c, c));

    let p = |c: i32, r: i32, z: f64| proj.point(c as f64, r as f64, z);
    let mut shapes = Vec::new();
    for (c, r) in order {
        let mut sides = Vec::new();
        let mut edges = String::new();
        let south_open = !has(c, r + 1);
        let west_open = !has(c - 1, r);

        if south_open {
            sides.push(polygon(&[p(c, r + 1, height), p(c + 1, r + 1, height), p(c + 1, r + 1, 0.0), p(c, r + 1, 0.0)]));
            segment(&mut edges, p(c, r + 1, 0.0), p(c + 1, r + 1, 0.0));
            // Vertical edges only where the face stops, so long walls read as one surface.
            if !(has(c - 1, r) && !has(c - 1, r + 1)) {
                segment(&mut edges, p(c, r + 1, 0.0), p(c, r + 1, height));
            }
            if !(has(c + 1, r) && !has(c + 1, r + 1)) {
                segment(&mut edges, p(c + 1, r + 1, 0.0), p(c + 1, r + 1, height));
            }
        }
        if west_open {
            sides.push(polygon(&[p(c, r, height), p(c, r + 1, height), p(c, r + 1, 0.0), p(c, r, 0.0)]));
            segment(&mut edges, p(c, r, 0.0), p(c, r + 1, 0.0));
            if !(has(c, r - 1) && !has(c - 1, r - 1)) {
                segment(&mut edges, p(c, r, 0.0), p(c, r, height));
            }
            if !(has(c, r + 1) && !has(c - 1, r + 1)) {
                segment(&mut edges, p(c, r + 1, 0.0), p(c, r + 1, height));
            }
        }

        // Outline of the top face wherever it meets empty space.
        if !has(c, r - 1) {
            segment(&mut edges, p(c, r, height), p(c + 1, r, height));
        }
        if !has(c + 1, r) {
            segment(&mut edges, p(c + 1, r, height), p(c + 1, r + 1, height));
        }
        if south_open {
            segment(&mut edges, p(c, r + 1, height), p(c + 1, r + 1, height));
        }
        if west_open {
            segment(&mut edges, p(c, r, height), p(c, r + 1, height));
        }

        let top = polygon(&[p(c, r, height), p(c + 1, r, height), p(c + 1, r + 1, height), p(c, r + 1, height)]);
        shapes.push(CellShapes { sides, top, edges });
    }

    // Long dashed construction lines along the isometric axes.
    let far = 2000.0;
    let along = |from: (f64, f64), dir: (f64, f64)| {
        let mut d = String::new();
        segment(&mut d, (from.0 - dir.0 * far, from.1 - dir.1 * far), (from.0 + dir.0 * far, from.1 + dir.1 * far));
        d
    };
    let u = (COS30, -0.5);
    let v = (COS30, 0.5);
    let guides = vec![
        along(p(0, ROWS, 0.0), u),
        along(p(0, 0, 0.0), v),
        along(p(cols, 0, 0.0), v),
    ];

    // The 30° angle at the highest corner, where the wedge to the horizontal is empty.
    let angle = filled
        .iter()
        .copied()
        .max_by_key(|&(c, r)| (c - r, c))
        .map(|(c, r)| {
            let vertex = p(c + 1, r, height);
            let radius = 70.0;
            let start = (vertex.0 + radius, vertex.1);
            let end = (vertex.0 + radius * COS30, vertex.1 + radius * 0.5);
            let mut line = String::new();
            segment(&mut line, vertex, (vertex.0 + radius + 12.0, vertex.1));
            let arc = format!("M{:.2} {:.2}A{radius} {radius} 0 0 1 {:.2} {:.2}", start.0, start.1, end.0, end.1);
            let mid = 15f64.to_radians();
            AngleMark {
                line,
                arc,
                label_x: format!("{:.1}", vertex.0 + (radius + 16.0) * mid.cos()),
                label_y: format!("{:.1}", vertex.1 + (radius + 16.0) * mid.sin()),
            }
        });

    Drawing { width, height: total_height, cells: shapes, guides, angle }
}

#[component]
pub fn IsometricMark(text: String) -> Element {
    let mut pressed = use_signal(|| false);
    let drawing = use_memo(move || draw(&text, if pressed() { PRESSED_HEIGHT } else { HEIGHT }));
    let drawing = drawing.read();
    let view_box = format!("0 0 {:.0} {:.0}", drawing.width, drawing.height);

    rsx! {
        svg {
            class: "h-auto w-full touch-manipulation overflow-visible select-none [--pattern:color-mix(in_oklab,var(--foreground)_12%,var(--background))] [--stroke:color-mix(in_oklab,var(--foreground)_16%,var(--background))]",
            "viewBox": "{view_box}",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            onpointerdown: move |_| {
                pressed.set(true);
                crate::js::play_click();
            },
            onpointerup: move |_| pressed.set(false),
            onpointerleave: move |_| pressed.set(false),

            defs {
                pattern {
                    id: "mark-hatch",
                    x: "0",
                    y: "0",
                    width: "10",
                    height: "10",
                    "patternUnits": "userSpaceOnUse",
                    path {
                        d: "M-1 1l2 -2M0 10l10 -10M9 11l2 -2",
                        stroke: "var(--pattern)",
                        stroke_width: "1",
                    }
                }
                MarkLight { width: drawing.width, height: drawing.height }
            }

            g { class: "stroke-line", stroke_width: "1", stroke_dasharray: "4 2",
                for d in drawing.guides.iter() {
                    path { d: "{d}" }
                }
            }

            for cell in drawing.cells.iter() {
                g {
                    for side in cell.sides.iter() {
                        path { class: "fill-background", d: "{side}" }
                    }
                    path { class: "fill-background", d: "{cell.top}" }
                    path { fill: "url(#mark-hatch)", d: "{cell.top}" }
                    path { stroke: "var(--stroke)", stroke_width: "1", d: "{cell.edges}" }
                    path { stroke: "url(#mark-light)", stroke_width: "1", d: "{cell.edges}" }
                }
            }

            if let Some(angle) = drawing.angle.as_ref() {
                g { class: "pointer-events-none stroke-dimension", stroke_width: "1",
                    path { d: "{angle.line}" }
                    path { d: "{angle.arc}" }
                    text {
                        class: "fill-dimension-foreground stroke-none font-mono text-[11px]",
                        x: "{angle.label_x}",
                        y: "{angle.label_y}",
                        "dominant-baseline": "middle",
                        "30°"
                    }
                }
            }
        }
    }
}

/// The cursor-following highlight, split out so mouse moves only re-render the gradient.
#[component]
fn MarkLight(width: f64, height: f64) -> Element {
    let mut light = use_signal(|| (0.5f64, 0.5f64));
    use_hook(|| {
        if crate::js::media_matches("(hover: none)") || crate::js::media_matches("(prefers-reduced-motion: reduce)") {
            return;
        }
        crate::js::on_window::<web_sys::MouseEvent>("mousemove", move |e| {
            let (w, h) = crate::js::window_size();
            light.set((e.client_x() as f64 / w, e.client_y() as f64 / h));
        });
    });
    let (lx, ly) = light();
    let cx = format!("{:.1}", lx * width);
    let cy = format!("{:.1}", ly * height);
    let r = format!("{:.0}", width * 0.36);

    rsx! {
        radialGradient {
            id: "mark-light",
            cx: "{cx}",
            cy: "{cy}",
            r: "{r}",
            "gradientUnits": "userSpaceOnUse",
            stop { class: "[stop-color:var(--color-zinc-700)] dark:[stop-color:#fff]" }
            stop {
                class: "[stop-color:var(--color-zinc-400)] dark:[stop-color:var(--color-zinc-600)]",
                offset: "1",
                "stop-opacity": "0",
            }
        }
    }
}

/// Flat, front-on version of the lettering for the header and footer.
#[component]
pub fn FlatMark(text: String, #[props(default)] class: String) -> Element {
    let cols = columns(&text);
    let cells = cells(&text);
    rsx! {
        svg {
            class: "{class}",
            "viewBox": "0 0 {cols} {ROWS}",
            fill: "currentColor",
            "aria-hidden": "true",
            "shape-rendering": "crispEdges",
            for (c, r) in cells {
                rect { x: "{c}", y: "{r}", width: "1.02", height: "1.02" }
            }
        }
    }
}
