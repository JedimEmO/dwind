# dwind-macros

This crate provides the core macros used by the DWIND style system.

For examples of usage, check out the example application here: https://jedimemo.github.io/dwind/examples/

## Usage 

### dwclass! and dwclass_signal!

The main macros used from applications are dwclass and dwclass_signal.
These can be used on `DomBuilder<N>` to use dwind classes on your dom elements.

```rust
fn hello() {
    html!("div", {
        .dwclass!("hover:font-extrabold")
        .dwclass_signal!("text-l", always(true))
    })
}
```

### dwgenerate_map!

This macro lets you pre-declare classes to use in your dwind application.
Typically used in style packages, such as dwind itself

```rust
#[macro_export]
macro_rules! width_generator {
    ($width:tt) => {
        const_format::formatcp!("width: {};", $width)
    };
}

// Invokes the width_generator!(a, b) macro for all elements in the provided array of tuples
dwgenerate_map!(
    "w",
    "width-",
    [
        // these tuples are turned into arguments to the invokes generator macro
        ("p-5", "5%"),
    ]
);
```