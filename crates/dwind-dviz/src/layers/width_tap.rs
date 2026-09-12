//! A layer that draws nothing and reports the plot width, for downsampling
//! data to what the plot can show.

use dominator::svg;
use futures_signals::signal::{Mutable, SignalExt};

use crate::chart::{Layer, layer};

/// Writes the plot width in px into `width` whenever the frame changes.
pub fn width_tap(width: Mutable<f64>) -> Layer {
    layer(move |ctx| {
        let frame = ctx.frame();
        svg!("g", {
            .class("dviz-width-tap")
            .future(frame.signal_ref(|f| f.plot.width).dedupe().for_each(move |w| {
                width.set_neq(w.max(1.0));
                async {}
            }))
        })
    })
}
