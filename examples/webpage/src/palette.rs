//! ⌘K command palette.
//!
//! A fuzzy launcher over every route in the site. Worth reading as a DOMINATOR
//! sample: open/query/cursor are three `Mutable`s, the result list is a
//! `child_signal` over a `map_ref!` of two of them, and keyboard handling is a
//! single `global_event_preventable`. No component framework, no store, no
//! effect hooks.

use crate::pages::docs::{doc_sections, DocPage};
use dominator::routing::go_to_url;
use dominator::{events, html, Dom, DomBuilder, EventOptions};
use dwind::prelude::*;
use dwind_macros::dwclass;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use once_cell::sync::Lazy;
use web_sys::{window, HtmlElement};

#[derive(Clone)]
pub struct Command {
    pub label: String,
    pub group: String,
    pub target: Target,
}

#[derive(Clone)]
pub enum Target {
    Route(&'static str),
    Page(DocPage),
    External(&'static str),
}

impl Target {
    fn run(&self) {
        match self {
            Target::Route(url) => go_to_url(url),
            Target::Page(page) => page.goto(),
            Target::External(url) => {
                let _ = window().unwrap().open_with_url_and_target(url, "_blank");
            }
        }
    }

    fn glyph(&self) -> &'static str {
        match self {
            Target::External(_) => "↗",
            _ => "→",
        }
    }
}

fn commands() -> Vec<Command> {
    let mut out = vec![
        Command {
            label: "Home".into(),
            group: "go".into(),
            target: Target::Route("#/"),
        },
        Command {
            label: "Component gallery".into(),
            group: "go".into(),
            target: Target::Route("#/components"),
        },
        Command {
            label: "Live examples".into(),
            group: "go".into(),
            target: Target::Route("#/examples"),
        },
    ];

    for section in doc_sections() {
        for page in section.docs {
            out.push(Command {
                label: page.to_string(),
                group: format!("docs / {}", section.title.to_lowercase()),
                target: Target::Page(page),
            });
        }
    }

    out.push(Command {
        label: "GitHub repository".into(),
        group: "external".into(),
        target: Target::External("https://github.com/JedimEmO/dwind"),
    });
    out.push(Command {
        label: "dwind on crates.io".into(),
        group: "external".into(),
        target: Target::External("https://crates.io/crates/dwind"),
    });
    out.push(Command {
        label: "DOMINATOR framework".into(),
        group: "external".into(),
        target: Target::External("https://github.com/Pauan/rust-dominator"),
    });

    out
}

static COMMANDS: Lazy<Vec<Command>> = Lazy::new(commands);

/// Subsequence match with a bonus for consecutive and word-start hits — enough
/// fuzziness to feel modern without pulling in a matcher crate.
fn score(haystack: &str, needle: &str) -> Option<i32> {
    if needle.is_empty() {
        return Some(0);
    }

    let hay: Vec<char> = haystack.to_lowercase().chars().collect();
    let lowered = needle.to_lowercase();
    let mut needle = lowered.chars().peekable();
    let mut total = 0;
    let mut streak = 0;

    for (i, c) in hay.iter().enumerate() {
        match needle.peek() {
            Some(n) if n == c => {
                let word_start = i == 0 || !hay[i - 1].is_alphanumeric();
                total += 10 + streak * 5 + if word_start { 12 } else { 0 };
                streak += 1;
                needle.next();
            }
            Some(_) => streak = 0,
            None => break,
        }
    }

    if needle.peek().is_none() {
        Some(total)
    } else {
        None
    }
}

fn matches(query: &str) -> Vec<&'static Command> {
    let mut scored: Vec<(i32, &Command)> = COMMANDS
        .iter()
        .filter_map(|c| {
            let direct = score(&c.label, query);
            let grouped = score(&format!("{} {}", c.group, c.label), query).map(|s| s - 30);

            direct.or(grouped).map(|s| (s, c))
        })
        .collect();

    // Best score first; ties keep declaration order, which puts routes above
    // external links.
    scored.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
    scored.into_iter().map(|(_, c)| c).collect()
}

#[derive(Clone)]
pub struct Palette {
    open: Mutable<bool>,
    query: Mutable<String>,
    cursor: Mutable<usize>,
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static GLOBAL: Palette = Palette::new();
}

/// The app has exactly one palette; anything that wants to open it (the header
/// button, the docs sidebar, a keyboard shortcut) shares this handle.
pub fn global() -> Palette {
    GLOBAL.with(|p| p.clone())
}

impl Palette {
    pub fn new() -> Self {
        Self {
            open: Mutable::new(false),
            query: Mutable::new(String::new()),
            cursor: Mutable::new(0),
        }
    }

    pub fn open(&self) {
        self.query.set(String::new());
        self.cursor.set(0);
        self.open.set(true);
    }

    fn close(&self) {
        self.open.set(false);
    }

    fn commit(&self) {
        let results = matches(&self.query.lock_ref());

        if let Some(cmd) = results.get(self.cursor.get()) {
            cmd.target.run();
        }

        self.close();
    }

    fn move_cursor(&self, delta: i32) {
        let len = matches(&self.query.lock_ref()).len();

        if len == 0 {
            return;
        }

        let next = (self.cursor.get() as i32 + delta).rem_euclid(len as i32);
        self.cursor.set_neq(next as usize);
    }

    /// Installs the global shortcut handler. Apply on the app root.
    pub fn shortcuts(&self) -> impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + use<> {
        let this = self.clone();

        move |builder| {
            let this = this.clone();

            builder.global_event_with_options(
                &EventOptions::preventable(),
                move |e: events::KeyDown| {
                    let key = e.key();
                    let open = this.open.get();

                    if e.ctrl_key() && key.eq_ignore_ascii_case("k") {
                        e.prevent_default();

                        if open {
                            this.close();
                        } else {
                            this.open();
                        }

                        return;
                    }

                    if !open {
                        // "/" opens search, the way every good docs site does.
                        if key == "/" {
                            e.prevent_default();
                            this.open();
                        }

                        return;
                    }

                    match key.as_str() {
                        "Escape" => {
                            e.prevent_default();
                            this.close();
                        }
                        "ArrowDown" => {
                            e.prevent_default();
                            this.move_cursor(1);
                        }
                        "ArrowUp" => {
                            e.prevent_default();
                            this.move_cursor(-1);
                        }
                        "Enter" => {
                            e.prevent_default();
                            this.commit();
                        }
                        _ => {}
                    }
                },
            )
        }
    }

    /// The overlay itself. Renders nothing while closed.
    pub fn render(&self) -> Dom {
        let this = self.clone();

        html!("div", {
            .child_signal(self.open.signal().map(move |open| {
                if !open {
                    return None;
                }

                Some(this.panel())
            }))
        })
    }

    fn panel(&self) -> Dom {
        let this = self.clone();

        html!("div", {
            .attr("role", "dialog")
            .attr("aria-modal", "true")
            .attr("aria-label", "Command palette")
            .style("position", "fixed")
            .style("inset", "0")
            .style("z-index", "9999")
            .child(html!("div", {
                .class("dw-palette-scrim")
                .style("position", "absolute")
                .style("inset", "0")
                .style("background", "rgba(2, 2, 3, 0.7)")
                .event({
                    let this = this.clone();
                    move |_: events::Click| this.close()
                })
            }))
            // Centring lives on the wrapper: the panel's entrance keyframes own
            // `transform`, and a fill-mode animation beats an inline style.
            .child(html!("div", {
                .dwclass!("flex justify-center w-full")
                .style("position", "absolute")
                .style("top", "14vh")
                .style("left", "0")
                .style("padding", "0 1rem")
                .child(html!("div", {
                    .class("dw-palette")
                    .dwclass!("rounded-lg overflow-hidden flex flex-col w-full")
                    .style("max-width", "38rem")
                    .style("background", "rgba(12, 12, 15, 0.86)")
                    .child(this.search_row())
                    .child(this.results())
                    .child(this.hint_row())
                }))
            }))
        })
    }

    fn search_row(&self) -> Dom {
        let this = self.clone();

        html!("div", {
            .dwclass!("flex flex-row align-items-center gap-3 p-l-5 p-r-5 h-14 border-b border-woodsmoke-800")
            .child(html!("span", {
                .class("font-code")
                .dwclass!("text-candlelight-400 text-l flex-none")
                .text("⌘")
            }))
            .child(html!("input" => web_sys::HtmlInputElement, {
                .class("dw-palette-input")
                .attr("type", "text")
                .attr("placeholder", "Jump to a page, a utility group, the repo…")
                .attr("aria-label", "Search")
                .attr("autocomplete", "off")
                .focused(true)
                .with_node!(element => {
                    .event({
                        let this = this.clone();
                        move |_: events::Input| {
                            this.query.set(element.value());
                            this.cursor.set_neq(0);
                        }
                    })
                })
            }))
            .child(html!("kbd", {
                .class("font-code")
                .dwclass!("text-xs text-woodsmoke-500 border border-woodsmoke-800 rounded-md p-l-2 p-r-2 p-t-1 p-b-1 flex-none")
                .text("esc")
            }))
        })
    }

    fn results(&self) -> Dom {
        let this = self.clone();

        html!("div", {
            .class("dw-scrollbar")
            .dwclass!("flex flex-col p-2 overflow-y-auto")
            .style("max-height", "min(24rem, 50vh)")
            .child_signal(map_ref! {
                let query = self.query.signal_cloned(),
                let cursor = self.cursor.signal() => move {
                    let results = matches(query);
                    let cursor = *cursor;

                    if results.is_empty() {
                        Some(html!("div", {
                            .class("font-code")
                            .dwclass!("text-sm text-woodsmoke-500 p-6 text-center")
                            .text("// no matches")
                        }))
                    } else {
                        Some(html!("div", {
                            .dwclass!("flex flex-col gap-1")
                            .children(results.into_iter().enumerate().map(|(i, cmd)| {
                                this.result_row(cmd, i, i == cursor)
                            }))
                        }))
                    }
                }
            })
        })
    }

    fn result_row(&self, cmd: &'static Command, index: usize, active: bool) -> Dom {
        let this = self.clone();

        html!("button", {
            .attr("type", "button")
            .dwclass!("flex flex-row align-items-center gap-3 w-full text-left")
            .dwclass!("rounded-md p-l-3 p-r-3 p-t-2 p-b-2 border-none cursor-pointer transition-colors")
            .style("background", if active { "rgba(213, 182, 95, 0.10)" } else { "transparent" })
            .style("color", "inherit")
            .style("font", "inherit")
            .child(html!("span", {
                .class("font-code")
                .dwclass!("text-xs flex-none w-4")
                .apply(move |b| if active {
                    dwclass!(b, "text-candlelight-400")
                } else {
                    dwclass!(b, "text-woodsmoke-600")
                })
                .text(cmd.target.glyph())
            }))
            .child(html!("span", {
                .dwclass!("text-sm grow")
                .apply(move |b| if active {
                    dwclass!(b, "text-woodsmoke-50")
                } else {
                    dwclass!(b, "text-woodsmoke-300")
                })
                .text(&cmd.label)
            }))
            .child(html!("span", {
                .class("font-code")
                .dwclass!("text-xs text-woodsmoke-600 flex-none")
                .text(&cmd.group)
            }))
            .event({
                let this = this.clone();
                move |_: events::PointerEnter| this.cursor.set_neq(index)
            })
            .event(move |_: events::Click| {
                cmd.target.run();
                this.close();
            })
        })
    }

    fn hint_row(&self) -> Dom {
        html!("div", {
            .class("font-code")
            .dwclass!("flex flex-row gap-4 p-l-5 p-r-5 p-t-2 p-b-2 border-t border-woodsmoke-800 text-xs text-woodsmoke-600")
            .child(html!("span", { .text("↑↓ navigate") }))
            .child(html!("span", { .text("⏎ open") }))
            .child(html!("span", { .text("esc dismiss") }))
        })
    }
}
