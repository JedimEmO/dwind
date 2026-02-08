use crate::router::Page;

#[derive(Clone)]
pub struct SectionEntry {
    pub name: &'static str,
    pub id: &'static str,
    pub keywords: &'static str,
    pub page: Page,
}

pub fn sections_for_page(page: Page) -> Vec<SectionEntry> {
    all_sections()
        .into_iter()
        .filter(|e| e.page == page)
        .collect()
}

pub fn all_sections() -> Vec<SectionEntry> {
    vec![
        // Core
        SectionEntry {
            name: "Buttons",
            id: "section-buttons",
            keywords: "button variant filled glass ghost danger size small medium large disabled loading",
            page: Page::Core,
        },
        SectionEntry {
            name: "Cards",
            id: "section-cards",
            keywords: "card default elevated inset container surface",
            page: Page::Core,
        },
        SectionEntry {
            name: "Badges",
            id: "section-badges",
            keywords: "badge label tag success warning error info accent",
            page: Page::Core,
        },
        SectionEntry {
            name: "Text Input",
            id: "section-text-input",
            keywords: "text input field form username password email validation",
            page: Page::Core,
        },
        SectionEntry {
            name: "Toggle",
            id: "section-toggle",
            keywords: "toggle switch on off boolean",
            page: Page::Core,
        },
        SectionEntry {
            name: "Modal",
            id: "section-modal",
            keywords: "modal dialog popup overlay backdrop",
            page: Page::Core,
        },
        SectionEntry {
            name: "Checkbox",
            id: "section-checkbox",
            keywords: "checkbox check box tick form validation",
            page: Page::Core,
        },
        SectionEntry {
            name: "Radio Group",
            id: "section-radio-group",
            keywords: "radio group option select single choice form validation",
            page: Page::Core,
        },
        SectionEntry {
            name: "Select",
            id: "section-select",
            keywords: "select dropdown picker option choose form validation",
            page: Page::Core,
        },
        SectionEntry {
            name: "Combobox",
            id: "section-combobox",
            keywords: "combobox autocomplete search select filter typeahead form validation",
            page: Page::Core,
        },
        // Navigation
        SectionEntry {
            name: "Breadcrumbs",
            id: "section-breadcrumbs",
            keywords: "breadcrumb trail path navigation crumb",
            page: Page::Navigation,
        },
        SectionEntry {
            name: "Tabs",
            id: "section-tabs",
            keywords: "tabs tab panel switch view tabbed",
            page: Page::Navigation,
        },
        SectionEntry {
            name: "Dropdown Menu",
            id: "section-dropdown-menu",
            keywords: "dropdown menu context actions popover",
            page: Page::Navigation,
        },
        SectionEntry {
            name: "Sidebar",
            id: "section-sidebar",
            keywords: "sidebar side navigation drawer collapse expand",
            page: Page::Navigation,
        },
        // Data Display
        SectionEntry {
            name: "Avatar",
            id: "section-avatar",
            keywords: "avatar profile picture image initials user",
            page: Page::Data,
        },
        SectionEntry {
            name: "Stat Cards",
            id: "section-stat-cards",
            keywords: "stat card metric kpi number trend statistics",
            page: Page::Data,
        },
        SectionEntry {
            name: "Progress Bar",
            id: "section-progress-bar",
            keywords: "progress bar loading percentage indicator",
            page: Page::Data,
        },
        SectionEntry {
            name: "Table",
            id: "section-table",
            keywords: "table data grid rows columns striped",
            page: Page::Data,
        },
        SectionEntry {
            name: "Tooltip",
            id: "section-tooltip",
            keywords: "tooltip hover hint popover info",
            page: Page::Data,
        },
        // Charts
        SectionEntry {
            name: "Line Chart",
            id: "section-line-chart",
            keywords: "chart line graph trend time series data visualization",
            page: Page::Charts,
        },
        SectionEntry {
            name: "Bar Chart",
            id: "section-bar-chart",
            keywords: "chart bar graph comparison category grouped stacked",
            page: Page::Charts,
        },
        SectionEntry {
            name: "Pie & Donut",
            id: "section-pie-chart",
            keywords: "chart pie donut ring proportion percentage slice",
            page: Page::Charts,
        },
        SectionEntry {
            name: "Sparkline",
            id: "section-sparkline",
            keywords: "chart sparkline mini inline trend compact",
            page: Page::Charts,
        },
        // Theme
        SectionEntry {
            name: "Theme Selector",
            id: "section-theme-selector",
            keywords: "theme preset midnight ember ocean color accent switch",
            page: Page::Theme,
        },
        SectionEntry {
            name: "Preview",
            id: "section-preview",
            keywords: "preview demo sample theme live",
            page: Page::Theme,
        },
    ]
}
