use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssUnit {
    Px,
    Rem,
    Em,
    Percent,
    Vw,
    Vh,
}

impl fmt::Display for CssUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = match self {
            Self::Px => "px",
            Self::Rem => "rem",
            Self::Em => "em",
            Self::Percent => "%",
            Self::Vw => "vw",
            Self::Vh => "vh",
        };
        formatter.write_str(unit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssLength {
    pub value: f32,
    pub unit: CssUnit,
}

impl CssLength {
    pub const fn new(value: f32, unit: CssUnit) -> Self {
        Self { value, unit }
    }

    pub const fn px(value: f32) -> Self {
        Self::new(value, CssUnit::Px)
    }
}

impl fmt::Display for CssLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.value, self.unit)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Margins {
    pub top: Option<CssLength>,
    pub right: Option<CssLength>,
    pub bottom: Option<CssLength>,
    pub left: Option<CssLength>,
}

impl Margins {
    pub const ZERO: Self = Self {
        top: Some(CssLength::px(0.0)),
        right: Some(CssLength::px(0.0)),
        bottom: Some(CssLength::px(0.0)),
        left: Some(CssLength::px(0.0)),
    };

    pub(crate) fn inline_style(self) -> String {
        let sides = [
            ("padding-top", self.top),
            ("padding-right", self.right),
            ("padding-bottom", self.bottom),
            ("padding-left", self.left),
        ];
        sides
            .into_iter()
            .filter_map(|(name, length)| length.map(|value| format!("{name}: {value};")))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorMode {
    Light,
    Dark,
    #[default]
    Switchable,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HtmlOptions {
    pub margins: Option<Margins>,
    pub enable_custom_scripts: Option<bool>,
    pub mode: Option<ColorMode>,
}

impl HtmlOptions {
    pub(crate) fn custom_scripts_enabled(&self) -> bool {
        self.enable_custom_scripts.unwrap_or(true)
    }

    pub(crate) fn color_mode(&self) -> ColorMode {
        self.mode.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_margins_remove_all_page_padding() {
        assert_eq!(
            Margins::ZERO.inline_style(),
            "padding-top: 0px; padding-right: 0px; padding-bottom: 0px; padding-left: 0px;"
        );
    }

    #[test]
    fn margins_support_typed_css_units() {
        let margins = Margins {
            top: Some(CssLength::new(1.5, CssUnit::Rem)),
            left: Some(CssLength::new(2.0, CssUnit::Percent)),
            ..Default::default()
        };

        assert_eq!(
            margins.inline_style(),
            "padding-top: 1.5rem; padding-left: 2%;"
        );
    }
}
