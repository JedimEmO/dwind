//! Colors, ramps, and the palette validator.
//!
//! Color is not hand-picked in dwind-dviz. Every color does one of four jobs
//! (categorical, sequential, diverging, status), and a palette is only legal
//! if it passes the computable checks in [`validate_categorical`] and
//! [`validate_ordinal`]. The thresholds and the CVD simulation model are part
//! of the standard, not implementation details; they are calibrated to
//! Machado, Oliveira & Fernandes (2009) at severity 1.0.
//!
//! The shipped defaults in [`defaults`] are validated by this crate's own
//! test suite in both light and dark mode.

use std::fmt;

/// An sRGB color with 8-bit channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// A color in the OKLab perceptual space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

/// Light or dark rendering surface. The lightness band and default surface
/// color depend on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Light,
    Dark,
}

/// Color-vision deficiency to simulate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cvd {
    Protan,
    Deutan,
    Tritan,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid hex color {0:?}: expected #rrggbb")]
pub struct ParseColorError(pub String);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parses `#rrggbb` or `rrggbb`.
    pub fn from_hex(hex: &str) -> Result<Self, ParseColorError> {
        let h = hex.trim().trim_start_matches('#');
        if h.len() != 6 || !h.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ParseColorError(hex.to_string()));
        }
        let ch = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap();
        Ok(Self::rgb(ch(0), ch(2), ch(4)))
    }

    /// Parses at compile time in a `const` context; panics on bad input,
    /// which is what a hard-coded palette table wants.
    pub const fn hex(hex: &str) -> Self {
        let b = hex.as_bytes();
        let start = if b[0] == b'#' { 1 } else { 0 };
        assert!(b.len() - start == 6, "hex color must be 6 digits");
        const fn nib(c: u8) -> u8 {
            match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => panic!("invalid hex digit"),
            }
        }
        const fn pair(b: &[u8], i: usize) -> u8 {
            nib(b[i]) * 16 + nib(b[i + 1])
        }
        Self::rgb(pair(b, start), pair(b, start + 2), pair(b, start + 4))
    }

    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Linear-light RGB in `0.0..=1.0`.
    pub fn to_linear(self) -> [f64; 3] {
        fn lin(c: u8) -> f64 {
            let c = c as f64 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        [lin(self.r), lin(self.g), lin(self.b)]
    }

    pub fn from_linear([r, g, b]: [f64; 3]) -> Self {
        fn enc(c: f64) -> u8 {
            let c = c.clamp(0.0, 1.0);
            let s = if c <= 0.0031308 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            };
            (s * 255.0).round() as u8
        }
        Self::rgb(enc(r), enc(g), enc(b))
    }

    pub fn to_oklab(self) -> Oklab {
        Oklab::from_linear(self.to_linear())
    }

    /// WCAG relative luminance.
    pub fn luminance(self) -> f64 {
        let [r, g, b] = self.to_linear();
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// WCAG contrast ratio between two colors, `1.0..=21.0`.
    pub fn contrast(self, other: Color) -> f64 {
        let (a, b) = (self.luminance(), other.luminance());
        let (hi, lo) = if a > b { (a, b) } else { (b, a) };
        (hi + 0.05) / (lo + 0.05)
    }

    /// The color as seen with a simulated deficiency.
    pub fn simulate(self, cvd: Cvd) -> Color {
        Color::from_linear(simulate_linear(self.to_linear(), cvd))
    }

    /// Ink that clears contrast on top of this color: white on dark fills,
    /// near-black on light ones. For labels set inside a filled mark.
    pub fn ink_on(self) -> Color {
        let white = Color::rgb(255, 255, 255);
        let black = Color::rgb(11, 11, 11);
        if self.contrast(white) >= self.contrast(black) {
            white
        } else {
            black
        }
    }

    /// `rgb(r g b / alpha)` for a translucent wash of this color.
    pub fn with_alpha(self, alpha: f64) -> String {
        format!(
            "rgb({} {} {} / {})",
            self.r,
            self.g,
            self.b,
            alpha.clamp(0.0, 1.0)
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl std::str::FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::from_hex(s)
    }
}

impl Oklab {
    pub fn from_linear([r, g, b]: [f64; 3]) -> Self {
        let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
        let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
        let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
        Self {
            l: 0.210_454_255_3 * l + 0.793_617_785_0 * m - 0.004_072_046_8 * s,
            a: 1.977_998_495_1 * l - 2.428_592_205_0 * m + 0.450_593_709_9 * s,
            b: 0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766_0 * s,
        }
    }

    pub fn to_linear(self) -> [f64; 3] {
        let l_ = self.l + 0.396_337_777_4 * self.a + 0.215_803_757_3 * self.b;
        let m_ = self.l - 0.105_561_345_8 * self.a - 0.063_854_172_8 * self.b;
        let s_ = self.l - 0.089_484_177_5 * self.a - 1.291_485_548_0 * self.b;
        let (l, m, s) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
        [
            4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s,
            -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s,
            -0.004_196_086_3 * l - 0.703_418_614_8 * m + 1.707_614_701_0 * s,
        ]
    }

    pub fn to_color(self) -> Color {
        Color::from_linear(self.to_linear())
    }

    /// OKLCH chroma.
    pub fn chroma(self) -> f64 {
        self.a.hypot(self.b)
    }

    /// OKLCH hue in degrees, `0.0..360.0`.
    pub fn hue(self) -> f64 {
        (self.b.atan2(self.a).to_degrees() % 360.0 + 360.0) % 360.0
    }

    /// Euclidean distance ×100; the ΔE used throughout the validator.
    pub fn delta_e(self, other: Oklab) -> f64 {
        100.0
            * ((self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2))
                .sqrt()
    }

    pub fn lerp(self, other: Oklab, t: f64) -> Oklab {
        Oklab {
            l: self.l + (other.l - self.l) * t,
            a: self.a + (other.a - self.a) * t,
            b: self.b + (other.b - self.b) * t,
        }
    }
}

// Machado, Oliveira & Fernandes (2009) at severity 1.0, applied in linear RGB.
const MACHADO_PROTAN: [[f64; 3]; 3] = [
    [0.152286, 1.052583, -0.204868],
    [0.114503, 0.786281, 0.099216],
    [-0.003882, -0.048116, 1.051998],
];
const MACHADO_DEUTAN: [[f64; 3]; 3] = [
    [0.367322, 0.860646, -0.227968],
    [0.280085, 0.672501, 0.047413],
    [-0.011820, 0.042940, 0.968881],
];
const MACHADO_TRITAN: [[f64; 3]; 3] = [
    [1.255528, -0.076749, -0.178779],
    [-0.078411, 0.930809, 0.147602],
    [0.004733, 0.691367, 0.303900],
];

fn simulate_linear([r, g, b]: [f64; 3], cvd: Cvd) -> [f64; 3] {
    let m = match cvd {
        Cvd::Protan => MACHADO_PROTAN,
        Cvd::Deutan => MACHADO_DEUTAN,
        Cvd::Tritan => MACHADO_TRITAN,
    };
    let row = |k: [f64; 3]| (k[0] * r + k[1] * g + k[2] * b).clamp(0.0, 1.0);
    [row(m[0]), row(m[1]), row(m[2])]
}

/// ΔE (OKLab ×100) between two colors, optionally under a simulated
/// deficiency.
pub fn delta_e(a: Color, b: Color, cvd: Option<Cvd>) -> f64 {
    let (la, lb) = match cvd {
        Some(c) => (
            Oklab::from_linear(simulate_linear(a.to_linear(), c)),
            Oklab::from_linear(simulate_linear(b.to_linear(), c)),
        ),
        None => (a.to_oklab(), b.to_oklab()),
    };
    la.delta_e(lb)
}

// ---------------------------------------------------------------------------
// Ramps

/// An ordered list of color stops interpolated in OKLab. Used for
/// sequential and diverging scales and for stepping ordinal palettes.
#[derive(Debug, Clone, PartialEq)]
pub struct Ramp {
    stops: Vec<Color>,
}

impl Ramp {
    /// Panics on fewer than two stops.
    pub fn new(stops: Vec<Color>) -> Self {
        assert!(stops.len() >= 2, "a ramp needs at least two stops");
        Self { stops }
    }

    pub fn stops(&self) -> &[Color] {
        &self.stops
    }

    /// Reverses the direction, e.g. to flip the dark anchor for dark mode.
    pub fn reversed(&self) -> Ramp {
        let mut stops = self.stops.clone();
        stops.reverse();
        Ramp { stops }
    }

    /// The color at `t` in `0.0..=1.0`, clamped.
    pub fn at(&self, t: f64) -> Color {
        let t = if t.is_finite() {
            t.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let segments = (self.stops.len() - 1) as f64;
        let pos = t * segments;
        let i = (pos.floor() as usize).min(self.stops.len() - 2);
        let frac = pos - i as f64;
        self.stops[i]
            .to_oklab()
            .lerp(self.stops[i + 1].to_oklab(), frac)
            .to_color()
    }

    /// `n` evenly spaced colors from the ramp, endpoints included.
    pub fn steps(&self, n: usize) -> Vec<Color> {
        match n {
            0 => vec![],
            1 => vec![self.at(0.5)],
            _ => (0..n).map(|i| self.at(i as f64 / (n - 1) as f64)).collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Validation

/// Thresholds for the categorical checks. The defaults are the standard;
/// change them only to be stricter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thresholds {
    /// OKLCH L band per mode: (light, dark).
    pub lightness_light: (f64, f64),
    pub lightness_dark: (f64, f64),
    pub chroma_floor: f64,
    /// Adjacent-pair ΔE under protan/deutan: pass at target, warn at floor.
    pub cvd_target: f64,
    pub cvd_floor: f64,
    /// Worst-pair ΔE under normal vision; a hard gate.
    pub normal_floor: f64,
    /// WCAG contrast against the surface; below is a warning that obligates
    /// visible labels or a table view.
    pub contrast_min: f64,
    /// Ordinal ramp: minimum ΔL between adjacent steps.
    pub ordinal_min_delta_l: f64,
    /// Ordinal ramp: the lightest step vs surface.
    pub ordinal_light_floor: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            lightness_light: (0.43, 0.77),
            lightness_dark: (0.48, 0.67),
            chroma_floor: 0.10,
            cvd_target: 8.0,
            cvd_floor: 6.0,
            normal_floor: 15.0,
            contrast_min: 3.0,
            ordinal_min_delta_l: 0.06,
            ordinal_light_floor: 2.0,
        }
    }
}

/// Which pairs of slots the CVD checks compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pairs {
    /// Neighbors only: stacks, bars, lines, where assignment never skips.
    #[default]
    Adjacent,
    /// Every pair: scatter, bubble, choropleth, small multiples, where any
    /// two marks can touch. Strictly harder; caps the series count.
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    /// Legal only with secondary encoding (labels, gaps, texture) or, for
    /// contrast, a relief channel (visible labels or table view).
    Warn,
    Fail,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Check {
    pub name: &'static str,
    pub status: Status,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub checks: Vec<Check>,
}

impl Report {
    /// True when no check failed. Warnings still pass but carry obligations.
    pub fn ok(&self) -> bool {
        self.checks.iter().all(|c| c.status != Status::Fail)
    }

    pub fn has_warnings(&self) -> bool {
        self.checks.iter().any(|c| c.status == Status::Warn)
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.checks {
            let glyph = match c.status {
                Status::Pass => "PASS",
                Status::Warn => "WARN",
                Status::Fail => "FAIL",
            };
            writeln!(f, "[{glyph}] {:<22} {}", c.name, c.detail)?;
        }
        Ok(())
    }
}

pub const SURFACE_LIGHT: Color = Color::hex("#fbfbfb");
pub const SURFACE_DARK: Color = Color::hex("#121215");

pub fn default_surface(mode: Mode) -> Color {
    match mode {
        Mode::Light => SURFACE_LIGHT,
        Mode::Dark => SURFACE_DARK,
    }
}

fn pairlist(n: usize, pairs: Pairs) -> Vec<(usize, usize)> {
    match pairs {
        Pairs::Adjacent => (0..n.saturating_sub(1)).map(|i| (i, i + 1)).collect(),
        Pairs::All => (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect(),
    }
}

/// The categorical checks: lightness band, chroma floor, CVD separation,
/// normal-vision floor, and contrast against the surface.
///
/// The structural checks (fixed hue order, documented values only) are
/// enforced by how the theme assigns slots, not measured here.
pub fn validate_categorical(
    palette: &[Color],
    mode: Mode,
    surface: Option<Color>,
    pairs: Pairs,
    t: &Thresholds,
) -> Report {
    let surface = surface.unwrap_or_else(|| default_surface(mode));
    let (lo, hi) = match mode {
        Mode::Light => t.lightness_light,
        Mode::Dark => t.lightness_dark,
    };
    let mut checks = Vec::with_capacity(5);
    let n = palette.len();

    // Lightness band.
    let off: Vec<String> = palette
        .iter()
        .map(|c| (c, c.to_oklab().l))
        .filter(|(_, l)| *l < lo || *l > hi)
        .map(|(c, l)| format!("{c} L={l:.3}"))
        .collect();
    checks.push(Check {
        name: "Lightness band",
        status: if off.is_empty() {
            Status::Pass
        } else {
            Status::Fail
        },
        detail: if off.is_empty() {
            format!("all {n} inside L {lo}–{hi}")
        } else {
            format!("outside band: {}", off.join(", "))
        },
    });

    // Chroma floor.
    let gray: Vec<String> = palette
        .iter()
        .map(|c| (c, c.to_oklab().chroma()))
        .filter(|(_, ch)| *ch < t.chroma_floor)
        .map(|(c, ch)| format!("{c} C={ch:.3}"))
        .collect();
    checks.push(Check {
        name: "Chroma floor",
        status: if gray.is_empty() {
            Status::Pass
        } else {
            Status::Fail
        },
        detail: if gray.is_empty() {
            format!("all {n} >= {}", t.chroma_floor)
        } else {
            format!("below floor (reads gray): {}", gray.join(", "))
        },
    });

    // CVD separation.
    let pl = pairlist(n, pairs);
    let label = match pairs {
        Pairs::Adjacent => "adjacent",
        Pairs::All => "all-pairs",
    };
    let mut worst: Option<(f64, Cvd, Color, Color)> = None;
    for cvd in [Cvd::Protan, Cvd::Deutan] {
        for &(i, j) in &pl {
            let d = delta_e(palette[i], palette[j], Some(cvd));
            if worst.is_none_or(|w| d < w.0) {
                worst = Some((d, cvd, palette[i], palette[j]));
            }
        }
    }
    let tritan = pl
        .iter()
        .map(|&(i, j)| delta_e(palette[i], palette[j], Some(Cvd::Tritan)))
        .fold(f64::INFINITY, f64::min);
    let (status, detail) = match worst {
        None => (Status::Pass, "n/a (fewer than two slots)".to_string()),
        Some((d, cvd, a, b)) => (
            if d >= t.cvd_target {
                Status::Pass
            } else if d >= t.cvd_floor {
                Status::Warn
            } else {
                Status::Fail
            },
            format!("worst {label} {a}↔{b} ΔE {d:.1} ({cvd:?}) · tritan {tritan:.1}"),
        ),
    };
    checks.push(Check {
        name: "CVD separation",
        status,
        detail,
    });

    // Normal-vision floor.
    let mut nworst: Option<(f64, Color, Color)> = None;
    for &(i, j) in &pl {
        let d = delta_e(palette[i], palette[j], None);
        if nworst.is_none_or(|w| d < w.0) {
            nworst = Some((d, palette[i], palette[j]));
        }
    }
    let (status, detail) = match nworst {
        None => (Status::Pass, "n/a (fewer than two slots)".to_string()),
        Some((d, a, b)) => (
            if d >= t.normal_floor {
                Status::Pass
            } else {
                Status::Fail
            },
            format!(
                "worst {label} {a}↔{b} ΔE {d:.1} (normal){}",
                if d >= t.normal_floor {
                    ""
                } else {
                    " — hard to tell apart even with full color vision"
                }
            ),
        ),
    };
    checks.push(Check {
        name: "Normal-vision floor",
        status,
        detail,
    });

    // Contrast vs surface.
    let low: Vec<String> = palette
        .iter()
        .map(|c| (c, c.contrast(surface)))
        .filter(|(_, r)| *r < t.contrast_min)
        .map(|(c, r)| format!("{c} {r:.2}:1"))
        .collect();
    checks.push(Check {
        name: "Contrast vs surface",
        status: if low.is_empty() {
            Status::Pass
        } else {
            Status::Warn
        },
        detail: if low.is_empty() {
            format!("all {n} >= {}:1", t.contrast_min)
        } else {
            format!(
                "below {}:1 — relief required (visible labels or table view): {}",
                t.contrast_min,
                low.join(", ")
            )
        },
    });

    Report { checks }
}

/// The ordinal-ramp checks: one hue, monotone lightness with visible gaps,
/// and a lightest step that still clears the surface.
pub fn validate_ordinal(
    palette: &[Color],
    mode: Mode,
    surface: Option<Color>,
    t: &Thresholds,
) -> Report {
    let surface = surface.unwrap_or_else(|| default_surface(mode));
    let labs: Vec<Oklab> = palette.iter().map(|c| c.to_oklab()).collect();
    let ls: Vec<f64> = labs.iter().map(|l| l.l).collect();
    let mut checks = Vec::with_capacity(4);

    let fwd = ls.windows(2).all(|w| w[0] <= w[1]);
    let rev = ls.windows(2).all(|w| w[0] >= w[1]);
    let mono = fwd || rev;
    checks.push(Check {
        name: "Lightness monotone",
        status: if mono { Status::Pass } else { Status::Fail },
        detail: if mono {
            "steps read light→dark".to_string()
        } else {
            format!(
                "out of order — L values {:?}",
                ls.iter()
                    .map(|l| (l * 1000.0).round() / 1000.0)
                    .collect::<Vec<_>>()
            )
        },
    });

    let thin: Vec<String> = ls
        .windows(2)
        .enumerate()
        .map(|(i, w)| (i, (w[1] - w[0]).abs()))
        .filter(|(_, g)| *g < t.ordinal_min_delta_l)
        .map(|(i, g)| format!("{}↔{} ΔL={g:.3}", palette[i], palette[i + 1]))
        .collect();
    checks.push(Check {
        name: "Adjacent ΔL",
        status: if thin.is_empty() {
            Status::Pass
        } else {
            Status::Fail
        },
        detail: if thin.is_empty() {
            format!("all gaps >= {}", t.ordinal_min_delta_l)
        } else {
            format!("steps too close: {}", thin.join(", "))
        },
    });

    let lightest = palette
        .iter()
        .zip(&ls)
        .reduce(|a, b| match mode {
            Mode::Light => {
                if b.1 > a.1 {
                    b
                } else {
                    a
                }
            }
            Mode::Dark => {
                if b.1 < a.1 {
                    b
                } else {
                    a
                }
            }
        })
        .map(|(c, _)| *c);
    let (status, detail) = match lightest {
        None => (Status::Pass, "n/a (empty)".to_string()),
        Some(c) => {
            let cr = c.contrast(surface);
            (
                if cr >= t.ordinal_light_floor {
                    Status::Pass
                } else {
                    Status::Fail
                },
                format!("{c} at {cr:.2}:1 vs surface"),
            )
        }
    };
    checks.push(Check {
        name: "Light-end contrast",
        status,
        detail,
    });

    let hues: Vec<f64> = labs.iter().map(|l| l.hue()).collect();
    let mut spread = if hues.is_empty() {
        0.0
    } else {
        hues.iter().cloned().fold(f64::MIN, f64::max)
            - hues.iter().cloned().fold(f64::MAX, f64::min)
    };
    if spread > 180.0 {
        spread = 360.0 - spread;
    }
    let one_hue = spread <= 40.0;
    checks.push(Check {
        name: "Single hue",
        status: if one_hue { Status::Pass } else { Status::Fail },
        detail: format!(
            "hue spread {spread:.0}°{}",
            if one_hue {
                ""
            } else {
                " — >40°, not a one-hue ramp"
            }
        ),
    });

    Report { checks }
}

/// Finds the ordering of `palette` that maximises the minimum adjacent
/// CVD ΔE, by exhaustive search. Fine for eight colors (40k orders);
/// do not call it with many more.
pub fn best_adjacent_order(palette: &[Color]) -> Vec<Color> {
    fn permutations(items: &mut Vec<usize>, k: usize, out: &mut Vec<Vec<usize>>) {
        if k == items.len() {
            out.push(items.clone());
            return;
        }
        for i in k..items.len() {
            items.swap(k, i);
            permutations(items, k + 1, out);
            items.swap(k, i);
        }
    }
    if palette.len() < 3 {
        return palette.to_vec();
    }
    let mut orders = Vec::new();
    permutations(&mut (0..palette.len()).collect(), 0, &mut orders);
    let score = |order: &[usize]| {
        order
            .windows(2)
            .flat_map(|w| {
                [Cvd::Protan, Cvd::Deutan]
                    .into_iter()
                    .map(move |c| delta_e(palette[w[0]], palette[w[1]], Some(c)))
            })
            .fold(f64::INFINITY, f64::min)
    };
    let best = orders
        .iter()
        .max_by(|a, b| score(a).partial_cmp(&score(b)).unwrap())
        .unwrap();
    best.iter().map(|&i| palette[i]).collect()
}

// ---------------------------------------------------------------------------
// Defaults

/// The reference palette instance. Every value here has been validated in
/// both modes by the test suite; to re-brand, replace these and keep the
/// tests.
pub mod defaults {
    use super::{Color, Mode, Ramp};

    /// Categorical slots in fixed order for the light surface. The order is
    /// the CVD-safety mechanism; never re-sort it at runtime.
    pub const CATEGORICAL_LIGHT: [Color; 8] = [
        Color::hex("#2a78d6"), // blue
        Color::hex("#eb6834"), // orange
        Color::hex("#1baf7a"), // aqua
        Color::hex("#eda100"), // yellow
        Color::hex("#e87ba4"), // magenta
        Color::hex("#008300"), // green
        Color::hex("#4a3aa7"), // violet
        Color::hex("#e34948"), // red
    ];

    /// The same eight hues, re-stepped for the dark surface.
    pub const CATEGORICAL_DARK: [Color; 8] = [
        Color::hex("#3987e5"),
        Color::hex("#d95926"),
        Color::hex("#199e70"),
        Color::hex("#c98500"),
        Color::hex("#d55181"),
        Color::hex("#008300"),
        Color::hex("#9085e9"),
        Color::hex("#e66767"),
    ];

    /// How many leading categorical slots validate under all-pairs
    /// comparison (scatter, bubble, maps, small multiples). Past this,
    /// fold to "Other" or facet.
    pub const CATEGORICAL_ALL_PAIRS_CAP: usize = 3;

    pub fn categorical(mode: Mode) -> &'static [Color; 8] {
        match mode {
            Mode::Light => &CATEGORICAL_LIGHT,
            Mode::Dark => &CATEGORICAL_DARK,
        }
    }

    /// The blue sequential steps 100..=700, light to dark.
    pub const SEQUENTIAL_BLUE: [Color; 13] = [
        Color::hex("#cde2fb"),
        Color::hex("#b7d3f6"),
        Color::hex("#9ec5f4"),
        Color::hex("#86b6ef"),
        Color::hex("#6da7ec"),
        Color::hex("#5598e7"),
        Color::hex("#3987e5"),
        Color::hex("#2a78d6"),
        Color::hex("#256abf"),
        Color::hex("#1c5cab"),
        Color::hex("#184f95"),
        Color::hex("#104281"),
        Color::hex("#0d366b"),
    ];

    /// Sequential ramp for a mode. On the light surface the light end means
    /// "near zero" and recedes toward the surface; on dark the anchor flips
    /// so the *dark* end recedes.
    pub fn sequential(mode: Mode) -> Ramp {
        let ramp = Ramp::new(SEQUENTIAL_BLUE.to_vec());
        match mode {
            Mode::Light => ramp,
            Mode::Dark => ramp.reversed(),
        }
    }

    /// Ordinal ramp: the sequential steps whose pale end still clears 2:1
    /// on the surface (step 250 on light, no darker than step 600 on dark).
    pub fn ordinal(mode: Mode) -> Ramp {
        match mode {
            Mode::Light => Ramp::new(SEQUENTIAL_BLUE[3..].to_vec()),
            Mode::Dark => Ramp::new(SEQUENTIAL_BLUE[..11].to_vec()).reversed(),
        }
    }

    pub const DIVERGING_MID_LIGHT: Color = Color::hex("#ececef");
    pub const DIVERGING_MID_DARK: Color = Color::hex("#35353b");
    pub const DIVERGING_NEG: Color = Color::hex("#2a78d6"); // blue pole
    pub const DIVERGING_POS: Color = Color::hex("#d03b3b"); // red pole

    /// Diverging ramp blue → neutral gray → red with equal arms.
    pub fn diverging(mode: Mode) -> Ramp {
        let mid = match mode {
            Mode::Light => DIVERGING_MID_LIGHT,
            Mode::Dark => DIVERGING_MID_DARK,
        };
        Ramp::new(vec![DIVERGING_NEG, mid, DIVERGING_POS])
    }

    /// Reserved status colors; identical in both modes and always paired
    /// with an icon and a label.
    pub const STATUS_GOOD: Color = Color::hex("#0ca30c");
    pub const STATUS_WARNING: Color = Color::hex("#fab219");
    pub const STATUS_SERIOUS: Color = Color::hex("#ec835a");
    pub const STATUS_CRITICAL: Color = Color::hex("#d03b3b");

    /// Chrome and ink for a mode. The neutrals are the woodsmoke steps of
    /// the dwind palette (cool grays), so charts sit on a dwui surface
    /// without a warm/cool mismatch.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Chrome {
        pub surface: Color,
        pub ink_primary: Color,
        pub ink_secondary: Color,
        pub ink_muted: Color,
        pub grid: Color,
        pub axis: Color,
        pub delta_up_good: Color,
    }

    pub const CHROME_LIGHT: Chrome = Chrome {
        surface: super::SURFACE_LIGHT,
        ink_primary: Color::hex("#020203"),
        ink_secondary: Color::hex("#61616a"),
        ink_muted: Color::hex("#7d7d87"),
        grid: Color::hex("#e0e0e4"),
        axis: Color::hex("#c7c7cd"),
        delta_up_good: Color::hex("#006300"),
    };

    pub const CHROME_DARK: Chrome = Chrome {
        surface: super::SURFACE_DARK,
        ink_primary: Color::hex("#fbfbfb"),
        ink_secondary: Color::hex("#aeaeb6"),
        ink_muted: Color::hex("#7d7d87"),
        grid: Color::hex("#29292f"),
        axis: Color::hex("#34343b"),
        delta_up_good: Color::hex("#0ca30c"),
    };

    pub fn chrome(mode: Mode) -> Chrome {
        match mode {
            Mode::Light => CHROME_LIGHT,
            Mode::Dark => CHROME_DARK,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::defaults::*;
    use super::*;

    fn cat(mode: Mode, pairs: Pairs, n: usize) -> Report {
        validate_categorical(
            &categorical(mode)[..n],
            mode,
            None,
            pairs,
            &Thresholds::default(),
        )
    }

    #[test]
    fn hex_roundtrip_and_parse() {
        assert_eq!(
            Color::from_hex("#2A78d6").unwrap(),
            Color::rgb(42, 120, 214)
        );
        assert_eq!(Color::hex("2a78d6").to_hex(), "#2a78d6");
        assert!(Color::from_hex("#12345").is_err());
        assert!(Color::from_hex("#zz0000").is_err());
    }

    #[test]
    fn oklab_roundtrip() {
        for c in CATEGORICAL_LIGHT {
            let back = c.to_oklab().to_color();
            assert!((c.r as i32 - back.r as i32).abs() <= 1, "{c} vs {back}");
            assert!((c.g as i32 - back.g as i32).abs() <= 1);
            assert!((c.b as i32 - back.b as i32).abs() <= 1);
        }
    }

    #[test]
    fn contrast_matches_wcag_reference() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        assert!((black.contrast(white) - 21.0).abs() < 1e-9);
        assert!((STATUS_GOOD.contrast(SURFACE_LIGHT) - 3.24).abs() < 0.01);
        assert!((STATUS_CRITICAL.contrast(SURFACE_DARK) - 3.89).abs() < 0.01);
    }

    #[test]
    fn default_categorical_passes_adjacent_in_both_modes() {
        for mode in [Mode::Light, Mode::Dark] {
            let r = cat(mode, Pairs::Adjacent, 8);
            assert!(r.ok(), "{mode:?}\n{r}");
            let cvd = r
                .checks
                .iter()
                .find(|c| c.name == "CVD separation")
                .unwrap();
            assert_eq!(cvd.status, Status::Pass, "{mode:?}: {}", cvd.detail);
        }
    }

    #[test]
    fn default_categorical_worst_pairs_match_reference() {
        // Numbers documented in the reference palette: worst adjacent CVD
        // 9.1 light / 8.4 dark, normal 19.6 / 19.3.
        let light = cat(Mode::Light, Pairs::Adjacent, 8);
        assert!(
            light.checks[2].detail.contains("ΔE 9.1"),
            "{}",
            light.checks[2].detail
        );
        assert!(
            light.checks[3].detail.contains("ΔE 19.6"),
            "{}",
            light.checks[3].detail
        );
        let dark = cat(Mode::Dark, Pairs::Adjacent, 8);
        assert!(
            dark.checks[2].detail.contains("ΔE 8.4"),
            "{}",
            dark.checks[2].detail
        );
        assert!(
            dark.checks[3].detail.contains("ΔE 19.3"),
            "{}",
            dark.checks[3].detail
        );
    }

    #[test]
    fn all_pairs_cap_holds() {
        for mode in [Mode::Light, Mode::Dark] {
            let r = cat(mode, Pairs::All, CATEGORICAL_ALL_PAIRS_CAP);
            assert!(r.ok(), "{mode:?}\n{r}");
            assert_eq!(
                r.checks[2].status,
                Status::Pass,
                "{mode:?}: {}",
                r.checks[2].detail
            );
        }
        let four = cat(Mode::Light, Pairs::All, 4);
        assert!(!four.ok(), "four slots must not validate all-pairs");
    }

    #[test]
    fn light_mode_relief_is_a_warning_not_a_failure() {
        let r = cat(Mode::Light, Pairs::Adjacent, 8);
        let contrast = r
            .checks
            .iter()
            .find(|c| c.name == "Contrast vs surface")
            .unwrap();
        assert_eq!(contrast.status, Status::Warn);
        assert!(
            contrast.detail.contains("#e87ba4"),
            "magenta is sub-3:1 on light"
        );
    }

    #[test]
    fn ordinal_ramps_validate() {
        for mode in [Mode::Light, Mode::Dark] {
            let steps = ordinal(mode).stops().to_vec();
            let r = validate_ordinal(&steps, mode, None, &Thresholds::default());
            // The full ramp has fine steps; the check is for a *stepped*
            // ordinal palette, so test five steps out of it.
            let five = ordinal(mode).steps(5);
            let r5 = validate_ordinal(&five, mode, None, &Thresholds::default());
            assert!(r5.ok(), "{mode:?}\n{r5}");
            assert_eq!(r.checks[0].status, Status::Pass, "{mode:?} monotone");
            assert_eq!(r.checks[3].status, Status::Pass, "{mode:?} single hue");
        }
    }

    #[test]
    fn sequential_ramp_is_monotone_one_hue() {
        let r = validate_ordinal(&SEQUENTIAL_BLUE, Mode::Light, None, &Thresholds::default());
        assert_eq!(r.checks[0].status, Status::Pass);
        assert_eq!(r.checks[3].status, Status::Pass);
    }

    #[test]
    fn categorical_validator_rejects_a_sequential_ramp() {
        let r = validate_categorical(
            &SEQUENTIAL_BLUE,
            Mode::Light,
            None,
            Pairs::Adjacent,
            &Thresholds::default(),
        );
        assert!(!r.ok());
    }

    #[test]
    fn ramp_interpolates_endpoints_and_middle() {
        let ramp = Ramp::new(vec![Color::rgb(0, 0, 0), Color::rgb(255, 255, 255)]);
        assert_eq!(ramp.at(0.0), Color::rgb(0, 0, 0));
        assert_eq!(ramp.at(1.0), Color::rgb(255, 255, 255));
        let mid = ramp.at(0.5);
        assert!(
            mid.r > 90 && mid.r < 140,
            "perceptual midpoint, not 128: {mid}"
        );
        assert_eq!(ramp.at(f64::NAN), Color::rgb(0, 0, 0));
        assert_eq!(ramp.steps(3).len(), 3);
    }

    #[test]
    fn diverging_midpoint_is_neutral() {
        for mode in [Mode::Light, Mode::Dark] {
            let mid = diverging(mode).at(0.5);
            assert!(
                mid.to_oklab().chroma() < 0.02,
                "{mode:?} midpoint {mid} must read as gray"
            );
        }
    }

    #[test]
    fn ink_on_picks_readable_side() {
        assert_eq!(Color::hex("#eda100").ink_on(), Color::rgb(11, 11, 11));
        assert_eq!(Color::hex("#4a3aa7").ink_on(), Color::rgb(255, 255, 255));
    }

    #[test]
    fn best_order_scores_at_least_the_shipped_order() {
        let shipped = cat(Mode::Light, Pairs::Adjacent, 8);
        let best = best_adjacent_order(&CATEGORICAL_LIGHT);
        let r = validate_categorical(&best, Mode::Light, None, Pairs::All, &Thresholds::default());
        let _ = r; // all-pairs over eight is expected to fail; only adjacent matters here
        let rb = validate_categorical(
            &best,
            Mode::Light,
            None,
            Pairs::Adjacent,
            &Thresholds::default(),
        );
        let worst = |r: &Report| -> f64 {
            let d = &r.checks[2].detail;
            let s = d.split("ΔE ").nth(1).unwrap();
            s.split(' ').next().unwrap().parse().unwrap()
        };
        assert!(worst(&rb) >= worst(&shipped) - 0.05);
    }
}
