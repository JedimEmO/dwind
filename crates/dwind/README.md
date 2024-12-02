# DWIND

This crate provides tailwind-like syntax and utility classes to be used from your DOMINATOR web applications!

It allows you to apply classes using pseudo selectors and signals, removing the indirection of CSS files for understanding how compoents look.
Component libraries do not need to bundle some out-of-band CSS definitions, as everything is compiled into your rust binary.

As an added benefit, tree shaking is done by the normal rust dead code elimination, removing the need for extra tooling.

For online examples, check out the example app here: https://jedimemo.github.io/dwind/examples/

## Usage

```rust
fn hello() {
    html!("div", {
        .dwclass!("hover:font-extrabold")
        .dwclass_signal!("text-l", always(true))
        .text("Hello, world!")
        
        // Make the children stylish!
        .dwclas!("[& > *]:text-picton-blue-500")
        .children([
            text("Bob"),
            text("Alice"),
        ])
    })
}
```
