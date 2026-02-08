use dominator::stylesheet;

/// Complete glassmorphic theme configuration.
/// Controls all visual appearance through CSS custom properties (`--glass-*`).
#[derive(Debug, Clone)]
pub struct GlassTheme {
    // Surface
    pub blur: String,
    pub blur_heavy: String,
    pub saturation: String,
    pub bg: String,
    pub bg_elevated: String,
    pub bg_inset: String,
    pub tint: String,

    // Borders
    pub border_color: String,
    pub border_color_hover: String,
    pub border_color_focus: String,
    pub border_width: String,
    pub border_radius_sm: String,
    pub border_radius: String,
    pub border_radius_lg: String,
    pub border_radius_xl: String,
    pub border_radius_full: String,

    // Accent colors
    pub accent: String,
    pub accent_hover: String,
    pub accent_muted: String,

    // Text colors
    pub text_primary: String,
    pub text_secondary: String,
    pub text_tertiary: String,
    pub text_on_accent: String,

    // Semantic colors
    pub success: String,
    pub success_muted: String,
    pub warning: String,
    pub warning_muted: String,
    pub error: String,
    pub error_muted: String,
    pub info: String,
    pub info_muted: String,

    // Shadows
    pub shadow_sm: String,
    pub shadow: String,
    pub shadow_lg: String,
    pub shadow_xl: String,
    pub shadow_inset: String,

    // Motion
    pub transition_fast: String,
    pub transition: String,
    pub transition_slow: String,
    pub ease: String,
}

impl Default for GlassTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl GlassTheme {
    /// Default dark glassmorphic theme (visionOS-inspired).
    pub fn dark() -> Self {
        Self {
            blur: "8px".into(),
            blur_heavy: "16px".into(),
            saturation: "120%".into(),
            bg: "rgba(255, 255, 255, 0.08)".into(),
            bg_elevated: "rgba(255, 255, 255, 0.12)".into(),
            bg_inset: "rgba(0, 0, 0, 0.15)".into(),
            tint: "rgba(255, 255, 255, 0.03)".into(),

            border_color: "rgba(255, 255, 255, 0.06)".into(),
            border_color_hover: "rgba(255, 255, 255, 0.10)".into(),
            border_color_focus: "rgba(255, 255, 255, 0.15)".into(),
            border_width: "1px".into(),
            border_radius_sm: "0.5rem".into(),
            border_radius: "0.75rem".into(),
            border_radius_lg: "1rem".into(),
            border_radius_xl: "1.5rem".into(),
            border_radius_full: "9999px".into(),

            accent: "rgba(99, 102, 241, 1)".into(),
            accent_hover: "rgba(129, 132, 255, 1)".into(),
            accent_muted: "rgba(99, 102, 241, 0.3)".into(),

            text_primary: "rgba(255, 255, 255, 0.95)".into(),
            text_secondary: "rgba(255, 255, 255, 0.65)".into(),
            text_tertiary: "rgba(255, 255, 255, 0.40)".into(),
            text_on_accent: "rgba(255, 255, 255, 1.0)".into(),

            success: "rgba(52, 211, 153, 1)".into(),
            success_muted: "rgba(52, 211, 153, 0.15)".into(),
            warning: "rgba(251, 191, 36, 1)".into(),
            warning_muted: "rgba(251, 191, 36, 0.15)".into(),
            error: "rgba(248, 113, 113, 1)".into(),
            error_muted: "rgba(248, 113, 113, 0.15)".into(),
            info: "rgba(96, 165, 250, 1)".into(),
            info_muted: "rgba(96, 165, 250, 0.15)".into(),

            shadow_sm: "inset 0 0.5px 0 0 rgba(255, 255, 255, 0.08), 0 1px 2px rgba(0, 0, 0, 0.2)".into(),
            shadow: "inset 0 0.5px 0 0 rgba(255, 255, 255, 0.1), 0 4px 16px rgba(0, 0, 0, 0.15)".into(),
            shadow_lg: "inset 0 0.5px 0 0 rgba(255, 255, 255, 0.12), 0 8px 32px rgba(0, 0, 0, 0.2)".into(),
            shadow_xl: "inset 0 1px 0 0 rgba(255, 255, 255, 0.14), 0 16px 48px rgba(0, 0, 0, 0.25)".into(),
            shadow_inset: "inset 0 1px 3px rgba(0, 0, 0, 0.15), inset 0 -0.5px 0 0 rgba(255, 255, 255, 0.06)".into(),

            transition_fast: "100ms".into(),
            transition: "200ms".into(),
            transition_slow: "400ms".into(),
            ease: "cubic-bezier(0.16, 1, 0.3, 1)".into(),
        }
    }

    /// Light glassmorphic theme for use on light backgrounds.
    pub fn light() -> Self {
        Self {
            bg: "rgba(255, 255, 255, 0.60)".into(),
            bg_elevated: "rgba(255, 255, 255, 0.70)".into(),
            bg_inset: "rgba(0, 0, 0, 0.05)".into(),
            tint: "rgba(255, 255, 255, 0.10)".into(),

            border_color: "rgba(0, 0, 0, 0.04)".into(),
            border_color_hover: "rgba(0, 0, 0, 0.08)".into(),
            border_color_focus: "rgba(0, 0, 0, 0.12)".into(),

            text_primary: "rgba(0, 0, 0, 0.90)".into(),
            text_secondary: "rgba(0, 0, 0, 0.60)".into(),
            text_tertiary: "rgba(0, 0, 0, 0.35)".into(),
            text_on_accent: "rgba(255, 255, 255, 1.0)".into(),

            shadow_sm: "inset 0 0.5px 0 0 rgba(255, 255, 255, 0.4), 0 1px 2px rgba(0, 0, 0, 0.06)".into(),
            shadow: "inset 0 0.5px 0 0 rgba(255, 255, 255, 0.5), 0 4px 16px rgba(0, 0, 0, 0.08)".into(),
            shadow_lg: "inset 0 0.5px 0 0 rgba(255, 255, 255, 0.5), 0 8px 32px rgba(0, 0, 0, 0.10)".into(),
            shadow_xl: "inset 0 1px 0 0 rgba(255, 255, 255, 0.6), 0 16px 48px rgba(0, 0, 0, 0.12)".into(),
            shadow_inset: "inset 0 1px 3px rgba(0, 0, 0, 0.06), inset 0 -0.5px 0 0 rgba(255, 255, 255, 0.3)".into(),

            ..Self::dark()
        }
    }

    /// Convert to raw CSS variable declarations.
    pub fn to_style_sheet_raw(&self) -> String {
        format!(
            "\
--glass-blur: {blur};\
--glass-blur-heavy: {blur_heavy};\
--glass-saturation: {saturation};\
--glass-bg: {bg};\
--glass-bg-elevated: {bg_elevated};\
--glass-bg-inset: {bg_inset};\
--glass-tint: {tint};\
--glass-border-color: {border_color};\
--glass-border-color-hover: {border_color_hover};\
--glass-border-color-focus: {border_color_focus};\
--glass-border-width: {border_width};\
--glass-border-radius-sm: {border_radius_sm};\
--glass-border-radius: {border_radius};\
--glass-border-radius-lg: {border_radius_lg};\
--glass-border-radius-xl: {border_radius_xl};\
--glass-border-radius-full: {border_radius_full};\
--glass-accent: {accent};\
--glass-accent-hover: {accent_hover};\
--glass-accent-muted: {accent_muted};\
--glass-text-primary: {text_primary};\
--glass-text-secondary: {text_secondary};\
--glass-text-tertiary: {text_tertiary};\
--glass-text-on-accent: {text_on_accent};\
--glass-success: {success};\
--glass-success-muted: {success_muted};\
--glass-warning: {warning};\
--glass-warning-muted: {warning_muted};\
--glass-error: {error};\
--glass-error-muted: {error_muted};\
--glass-info: {info};\
--glass-info-muted: {info_muted};\
--glass-shadow-sm: {shadow_sm};\
--glass-shadow: {shadow};\
--glass-shadow-lg: {shadow_lg};\
--glass-shadow-xl: {shadow_xl};\
--glass-shadow-inset: {shadow_inset};\
--glass-transition-fast: {transition_fast};\
--glass-transition: {transition};\
--glass-transition-slow: {transition_slow};\
--glass-ease: {ease};",
            blur = self.blur,
            blur_heavy = self.blur_heavy,
            saturation = self.saturation,
            bg = self.bg,
            bg_elevated = self.bg_elevated,
            bg_inset = self.bg_inset,
            tint = self.tint,
            border_color = self.border_color,
            border_color_hover = self.border_color_hover,
            border_color_focus = self.border_color_focus,
            border_width = self.border_width,
            border_radius_sm = self.border_radius_sm,
            border_radius = self.border_radius,
            border_radius_lg = self.border_radius_lg,
            border_radius_xl = self.border_radius_xl,
            border_radius_full = self.border_radius_full,
            accent = self.accent,
            accent_hover = self.accent_hover,
            accent_muted = self.accent_muted,
            text_primary = self.text_primary,
            text_secondary = self.text_secondary,
            text_tertiary = self.text_tertiary,
            text_on_accent = self.text_on_accent,
            success = self.success,
            success_muted = self.success_muted,
            warning = self.warning,
            warning_muted = self.warning_muted,
            error = self.error,
            error_muted = self.error_muted,
            info = self.info,
            info_muted = self.info_muted,
            shadow_sm = self.shadow_sm,
            shadow = self.shadow,
            shadow_lg = self.shadow_lg,
            shadow_xl = self.shadow_xl,
            shadow_inset = self.shadow_inset,
            transition_fast = self.transition_fast,
            transition = self.transition,
            transition_slow = self.transition_slow,
            ease = self.ease,
        )
    }

    pub fn with_accent(mut self, accent: impl Into<String>) -> Self {
        self.accent = accent.into();
        self
    }

    pub fn with_accent_hover(mut self, accent_hover: impl Into<String>) -> Self {
        self.accent_hover = accent_hover.into();
        self
    }

    pub fn with_blur(mut self, blur: impl Into<String>) -> Self {
        self.blur = blur.into();
        self
    }

    pub fn with_blur_heavy(mut self, blur_heavy: impl Into<String>) -> Self {
        self.blur_heavy = blur_heavy.into();
        self
    }

    pub fn with_bg(mut self, bg: impl Into<String>) -> Self {
        self.bg = bg.into();
        self
    }

    pub fn with_border_radius(mut self, radius: impl Into<String>) -> Self {
        self.border_radius = radius.into();
        self
    }
}

/// Apply the glass theme stylesheet to `:root`.
/// Must be called once during initialization, after `dwind::stylesheet()`.
pub fn apply_glass_theme(theme: Option<GlassTheme>) {
    let theme = theme.unwrap_or_default();

    stylesheet!(":root", {
        .raw(&theme.to_style_sheet_raw())
    });

    glass_css::apply_glass_stylesheet();
    base_css::apply_base_stylesheet();
    chart_css::apply_chart_stylesheet();
}

pub mod glass_css {
    include!(concat!(env!("OUT_DIR"), "/glass.rs"));
}

pub mod base_css {
    include!(concat!(env!("OUT_DIR"), "/base.rs"));
}

pub mod chart_css {
    include!(concat!(env!("OUT_DIR"), "/chart.rs"));
}
