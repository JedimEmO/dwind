# example-html-highlight-macro

This crate provides a macro to embed highlighted HTML versions of your tagged code into your binary.
Useful for providing in-browser highlighted code examples that doesn't run stale!

## Usage

```rust
#[example_html(themes = ["base16-ocean.dark"])]
fn variants() -> Dom {
    html!("div", {
        .text("Hello, world!")
    })
}
```