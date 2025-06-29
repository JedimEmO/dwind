use crate::pages::docs::doc_pages::doc_page::{doc_page_sub_header, doc_page_title};
use crate::pages::docs::example_box::example_box;
use dominator::Dom;
use dwind::prelude::*;
use dwind_macros::dwclass;

pub fn examples_page() -> Dom {
    html!("div", {
        .dwclass!("w-full")
        .child(doc_page_title("Welcome to dwind"))

        .child(html!("div", {
            .dwclass!("m-b-6")
            .child(html!("p", {
                .dwclass!("text-woodsmoke-300 m-b-4")
                .text("dwind is a Rust library that brings Tailwind-like utility classes to the DOMINATOR web framework. It provides compile-time CSS generation with full type safety and zero runtime overhead.")
            }))
            .child(html!("p", {
                .dwclass!("text-woodsmoke-300 m-b-4")
                .text("This example app demonstrates the power of dwind's utility-first approach to styling web applications in Rust.")
            }))
        }))

        .child(doc_page_sub_header("How to Use This App"))
        .child(html!("div", {
            .dwclass!("m-b-6")
            .child(html!("div", {
                .dwclass!("text-woodsmoke-300")
                .child(html!("div", {
                    .dwclass!("m-b-2")
                    .text("• Navigate through the sidebar to explore different dwind features")
                }))
                .child(html!("div", {
                    .dwclass!("m-b-2")
                    .text("• Click \"dwui\" in the header to see UI components built with dwind")
                }))
                .child(html!("div", {
                    .dwclass!("m-b-2")
                    .text("• Each example shows both the rendered output and can be inspected for implementation details")
                }))
                .child(html!("div", {
                    .text("• Try resizing your browser to see responsive design in action")
                }))
            }))
        }))

        .child(doc_page_sub_header("Key Features"))
        .child(html!("div", {
            .dwclass!("grid grid-cols-1 @md:grid-cols-2 gap-4 m-b-6")
            .child(html!("div", {
                .dwclass!("p-4 bg-woodsmoke-900 rounded-lg border border-woodsmoke-800")
                .child(html!("h4", {
                    .dwclass!("font-bold text-picton-blue-400 m-b-2")
                    .text("🚀 Zero Runtime Overhead")
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300 text-sm")
                    .text("All CSS is generated at compile time. No runtime parsing or style injection.")
                }))
            }))
            .child(html!("div", {
                .dwclass!("p-4 bg-woodsmoke-900 rounded-lg border border-woodsmoke-800")
                .child(html!("h4", {
                    .dwclass!("font-bold text-picton-blue-400 m-b-2")
                    .text("🔒 Type Safety")
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300 text-sm")
                    .text("Invalid class names are caught at compile time, not runtime.")
                }))
            }))
            .child(html!("div", {
                .dwclass!("p-4 bg-woodsmoke-900 rounded-lg border border-woodsmoke-800")
                .child(html!("h4", {
                    .dwclass!("font-bold text-picton-blue-400 m-b-2")
                    .text("📱 Responsive Design")
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300 text-sm")
                    .text("Container queries and responsive utilities built-in.")
                }))
            }))
            .child(html!("div", {
                .dwclass!("p-4 bg-woodsmoke-900 rounded-lg border border-woodsmoke-800")
                .child(html!("h4", {
                    .dwclass!("font-bold text-picton-blue-400 m-b-2")
                    .text("🎨 Theming Support")
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300 text-sm")
                    .text("Design tokens and CSS variables for consistent theming.")
                }))
            }))
        }))

        .child(doc_page_sub_header("Quick Example"))
        .child(example_box(
            html!("div", {
                .dwclass!("flex flex-col gap-4 p-4 bg-woodsmoke-900 rounded")
                .child(html!("div", {
                    .dwclass!("flex justify-between align-items-center")
                    .child(html!("h3", {
                        .dwclass!("text-xl font-bold text-woodsmoke-50")
                        .text("Hello dwind!")
                    }))
                    .child(html!("button", {
                        .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 bg-picton-blue-600 hover:bg-picton-blue-700 text-woodsmoke-50 rounded-full transition-colors")
                        .text("Get Started")
                    }))
                }))
                .child(html!("p", {
                    .dwclass!("text-woodsmoke-300")
                    .text("This card demonstrates dwind's utility classes: flexbox layout, spacing, colors, hover states, and transitions - all with compile-time safety!")
                }))
            }),
            true
        ))

        .child(doc_page_sub_header("Explore More"))
        .child(html!("div", {
            .dwclass!("flex gap-4 m-t-4")
            .child(html!("a", {
                .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 bg-woodsmoke-800 hover:bg-woodsmoke-700 text-woodsmoke-200 rounded cursor-pointer transition-colors")
                .text("View Colors →")
                .event(|_: dominator::events::Click| {
                    dominator::routing::go_to_url("#/docs/colors");
                })
            }))
            .child(html!("a", {
                .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 bg-woodsmoke-800 hover:bg-woodsmoke-700 text-woodsmoke-200 rounded cursor-pointer transition-colors")
                .text("Responsive Design →")
                .event(|_: dominator::events::Click| {
                    dominator::routing::go_to_url("#/docs/responsive-design");
                })
            }))
            .child(html!("a", {
                .dwclass!("p-l-4 p-r-4 p-t-2 p-b-2 bg-picton-blue-600 hover:bg-picton-blue-700 text-woodsmoke-50 rounded cursor-pointer transition-colors")
                .text("DWUI Components →")
                .event(|_: dominator::events::Click| {
                    dominator::routing::go_to_url("#/dwui-examples");
                })
            }))
        }))
    })
}
