# DWIND

Bringing tailwind-like syntax and utility classes to your DOMINATOR web application!

This project provide utilities to


The example app is hosted on github pages here:
https://jedimemo.github.io/dwind/examples

Docs are hosted here:
https://jedimemo.github.io/dwind/doc/dwind/index.html


## Crates

| Crate | What it is |
| --- | --- |
| `dwind` | Compile-time utility classes (`dwclass!`) and keyframes for DOMINATOR |
| `dwui` | Themeable, accessible UI components built on dwind |
| `dwind-dviz` | Reactive SVG charts: line, area, bar, scatter, donut, stat tiles, sparklines, small multiples, live windows |
| `dwind-dviz-core` | DOM-free chart primitives: scales, ticks, geometry, formatting, validated palettes |
| `dwind-dviz-data` | Signal-backed data sources for static and live charts |

The chart gallery is at https://jedimemo.github.io/dwind/examples/#/charts;
`crates/dwind-dviz/README.md` has the quick start.

## Setting up an application

The simplest way to  get started is to user the template repository with cargo-generate:

```sh
cargo install cargo-generate
cargo generate JedimEmO/dwind-template
```

The template code can be found here: https://github.com/JedimEmO/dwind-template
