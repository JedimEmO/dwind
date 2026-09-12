//! Rectangles, margins and grids: where things go, in pixels.

/// An axis-aligned rectangle in pixel space, origin top-left.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn from_size(width: f64, height: f64) -> Self {
        Self::new(0.0, 0.0, width, height)
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// The rectangle left after removing `m` from each side. Never returns a
    /// negative size.
    pub fn inset(&self, m: Margins) -> Rect {
        Rect {
            x: self.x + m.left,
            y: self.y + m.top,
            width: (self.width - m.left - m.right).max(0.0),
            height: (self.height - m.top - m.bottom).max(0.0),
        }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.right() && y >= self.y && y <= self.bottom()
    }

    /// Horizontal range `(left, right)` for an x scale.
    pub fn x_range(&self) -> (f64, f64) {
        (self.x, self.right())
    }

    /// Vertical range `(bottom, top)` for a y scale: larger values map
    /// upwards on screen.
    pub fn y_range(&self) -> (f64, f64) {
        (self.bottom(), self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Margins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Margins {
    pub const fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn uniform(m: f64) -> Self {
        Self::new(m, m, m, m)
    }

    /// Room for a bottom x-axis and a left y-axis, sized from the widest
    /// y label and the font size. `y_label_width` comes from
    /// [`estimate_text_width`] or a real DOM measurement.
    pub fn for_axes(y_label_width: f64, font_size: f64) -> Self {
        let tick_len = 6.0;
        let gap = 4.0;
        Self {
            top: font_size,
            right: font_size,
            bottom: tick_len + gap + font_size * 1.4,
            left: tick_len + gap + y_label_width,
        }
    }
}

/// Rough width of `text` at `font_size` px in a UI sans, for reserving axis
/// margins before the DOM can measure. Deliberately generous (0.6em per
/// character): a slightly wide margin looks fine, a clipped label does not.
pub fn estimate_text_width(text: &str, font_size: f64) -> f64 {
    text.chars().count() as f64 * font_size * 0.6
}

/// The widest of several labels, estimated.
pub fn estimate_max_text_width<'a>(
    labels: impl IntoIterator<Item = &'a str>,
    font_size: f64,
) -> f64 {
    labels
        .into_iter()
        .map(|l| estimate_text_width(l, font_size))
        .fold(0.0, f64::max)
}

/// Where a legend sits relative to the plot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegendPosition {
    #[default]
    Top,
    Bottom,
    Right,
    None,
}

/// Lays `count` cells into `columns` columns inside `area`, each separated by
/// `gap`. Rows are added as needed; all cells share one size. Used for small
/// multiples.
pub fn grid(area: Rect, count: usize, columns: usize, gap: f64) -> Vec<Rect> {
    if count == 0 || columns == 0 {
        return vec![];
    }
    let rows = count.div_ceil(columns);
    let cell_w = ((area.width - gap * (columns as f64 - 1.0)) / columns as f64).max(0.0);
    let cell_h = ((area.height - gap * (rows as f64 - 1.0)) / rows as f64).max(0.0);
    (0..count)
        .map(|i| {
            let c = (i % columns) as f64;
            let r = (i / columns) as f64;
            Rect::new(
                area.x + c * (cell_w + gap),
                area.y + r * (cell_h + gap),
                cell_w,
                cell_h,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inset_never_negative() {
        let r = Rect::from_size(100.0, 50.0).inset(Margins::uniform(10.0));
        assert_eq!(r, Rect::new(10.0, 10.0, 80.0, 30.0));
        let tiny = Rect::from_size(5.0, 5.0).inset(Margins::uniform(10.0));
        assert_eq!(tiny.width, 0.0);
        assert_eq!(tiny.height, 0.0);
    }

    #[test]
    fn y_range_is_flipped() {
        let r = Rect::new(0.0, 10.0, 100.0, 40.0);
        assert_eq!(r.y_range(), (50.0, 10.0));
        assert_eq!(r.x_range(), (0.0, 100.0));
    }

    #[test]
    fn grid_layout() {
        let cells = grid(Rect::from_size(210.0, 110.0), 5, 2, 10.0);
        assert_eq!(cells.len(), 5);
        assert_eq!(cells[0], Rect::new(0.0, 0.0, 100.0, 30.0));
        assert_eq!(cells[1].x, 110.0);
        assert_eq!(cells[2].y, 40.0);
        assert_eq!(cells[4].y, 80.0);
    }
}
