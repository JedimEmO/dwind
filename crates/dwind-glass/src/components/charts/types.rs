/// Color palette for chart series, mapping color indices to CSS color strings.
#[derive(Debug, Clone)]
pub struct ChartPalette {
    pub colors: Vec<String>,
    pub highlight: Vec<String>,
}

impl Default for ChartPalette {
    fn default() -> Self {
        Self::glass()
    }
}

impl ChartPalette {
    /// Default palette using glass theme semantic colors + extended set.
    pub fn glass() -> Self {
        Self {
            colors: vec![
                "var(--glass-accent)".into(),
                "var(--glass-success)".into(),
                "var(--glass-warning)".into(),
                "var(--glass-error)".into(),
                "var(--glass-info)".into(),
                "rgba(168, 85, 247, 1)".into(),
                "rgba(236, 72, 153, 1)".into(),
                "rgba(20, 184, 166, 1)".into(),
            ],
            highlight: vec![
                "var(--glass-accent-hover)".into(),
                "rgba(52, 211, 153, 0.85)".into(),
                "rgba(251, 191, 36, 0.85)".into(),
                "rgba(248, 113, 113, 0.85)".into(),
                "rgba(96, 165, 250, 0.85)".into(),
                "rgba(192, 132, 252, 0.85)".into(),
                "rgba(244, 114, 182, 0.85)".into(),
                "rgba(45, 212, 191, 0.85)".into(),
            ],
        }
    }

    pub fn color(&self, index: usize) -> &str {
        &self.colors[index % self.colors.len()]
    }

    pub fn highlight_color(&self, index: usize) -> &str {
        &self.highlight[index % self.highlight.len()]
    }

    /// Get the color with reduced opacity for area fills.
    pub fn area_color(&self, index: usize) -> String {
        let base = self.color(index);
        // For CSS variable colors, wrap in a semi-transparent overlay
        if base.starts_with("var(") {
            format!("color-mix(in srgb, {} 30%, transparent)", base)
        } else if let Some(stripped) = base.strip_suffix(')') {
            // Replace alpha in rgba()
            if let Some(prefix) = stripped.rsplit_once(',') {
                format!("{}, 0.2)", prefix.0)
            } else {
                base.to_string()
            }
        } else {
            base.to_string()
        }
    }
}

/// Configuration for bar chart layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarMode {
    Grouped,
    Stacked,
}
