use dominator::DomBuilder;
use web_sys::HtmlElement;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GlassSurfaceLevel {
    /// Default surface — subtle frosted glass.
    Base,
    /// Elevated surface — cards, modals, dropdowns. More blur and opacity.
    Elevated,
    /// Inset/recessed surface — inputs, toggle tracks. Dark inward appearance.
    Inset,
}

/// Applies the glassmorphic surface effect to a DomBuilder.
/// All values reference `--glass-*` CSS variables, so they respect the active theme.
pub fn glass_surface_mixin(
    level: GlassSurfaceLevel,
) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |b| {
        let bg_var = match level {
            GlassSurfaceLevel::Base => "var(--glass-bg)",
            GlassSurfaceLevel::Elevated => "var(--glass-bg-elevated)",
            GlassSurfaceLevel::Inset => "var(--glass-bg-inset)",
        };

        let blur_var = match level {
            GlassSurfaceLevel::Base | GlassSurfaceLevel::Inset => "var(--glass-blur)",
            GlassSurfaceLevel::Elevated => "var(--glass-blur-heavy)",
        };

        let shadow_var = match level {
            GlassSurfaceLevel::Base => "var(--glass-shadow-sm)",
            GlassSurfaceLevel::Elevated => "var(--glass-shadow)",
            GlassSurfaceLevel::Inset => "var(--glass-shadow-inset)",
        };

        let blur_value = format!(
            "blur({}) saturate(var(--glass-saturation))",
            blur_var
        );

        b.style("background", bg_var)
            .style(["backdrop-filter", "-webkit-backdrop-filter"], &blur_value)
            .style("border", "none")
            .style("box-shadow", shadow_var)
    }
}
