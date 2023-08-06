mod dracula;
mod gruvbox;
mod nord;
mod solarized;

use std::fmt;

use iced::Color;
use serde::{Deserialize, Serialize};

use super::palette::Palette;

/// Custom style with any relevant metadata
pub struct CustomPalette {
    /// Color scheme's palette
    palette: Palette,
    /// Extra colors such as the favorites star
    extension: PaletteExtension,
}

/// Extension color for themes.
pub struct PaletteExtension {
    /// Color of favorites star
    pub starred: Color,
    /// Badge/logo alpha
    pub chart_badge_alpha: f32,
    /// Round borders alpha
    pub round_borders_alpha: f32,
    /// Round containers alpha
    pub round_containers_alpha: f32,
}

/// Built in extra styles
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
#[serde(tag = "custom")]
pub enum ExtraStyles {
    Dracula,
    GruvboxDark,
    GruvboxLight,
    NordLight,
    NordDark,
    SolarizedDark,
    SolarizedLight,
}

impl ExtraStyles {
    /// [`Palette`] of the [`ExtraStyles`] variant
    #[inline]
    pub fn to_palette(self) -> Palette {
        match self {
            ExtraStyles::Dracula => dracula::dracula().palette,
            ExtraStyles::GruvboxDark => gruvbox::gruvbox_dark().palette,
            ExtraStyles::GruvboxLight => gruvbox::gruvbox_light().palette,
            ExtraStyles::NordLight => nord::nord_day().palette,
            ExtraStyles::NordDark => nord::nord_night().palette,
            ExtraStyles::SolarizedDark => solarized::solarized_dark().palette,
            ExtraStyles::SolarizedLight => solarized::solarized_light().palette,
        }
    }

    /// Extension colors for the current [`ExtraStyles`] variant
    #[inline]
    pub fn to_ext(self) -> PaletteExtension {
        match self {
            ExtraStyles::Dracula => dracula::dracula().extension,
            ExtraStyles::GruvboxDark => gruvbox::gruvbox_dark().extension,
            ExtraStyles::GruvboxLight => gruvbox::gruvbox_light().extension,
            ExtraStyles::NordLight => nord::nord_day().extension,
            ExtraStyles::NordDark => nord::nord_night().extension,
            ExtraStyles::SolarizedDark => solarized::solarized_dark().extension,
            ExtraStyles::SolarizedLight => solarized::solarized_light().extension,
        }
    }

    /// Theme is a night/dark style
    #[inline]
    pub const fn is_nightly(self) -> bool {
        match self {
            ExtraStyles::Dracula
            | ExtraStyles::GruvboxDark
            | ExtraStyles::NordDark
            | ExtraStyles::SolarizedDark => true,
            ExtraStyles::GruvboxLight | ExtraStyles::NordLight | ExtraStyles::SolarizedLight => {
                false
            }
        }
    }

    /// Slice of all implemented custom styles
    #[inline]
    pub const fn all_styles() -> &'static [Self] {
        &[
            ExtraStyles::Dracula,
            ExtraStyles::GruvboxDark,
            ExtraStyles::GruvboxLight,
            ExtraStyles::NordLight,
            ExtraStyles::NordDark,
            ExtraStyles::SolarizedDark,
            ExtraStyles::SolarizedLight,
        ]
    }
}

impl fmt::Display for ExtraStyles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExtraStyles::Dracula => write!(f, "Dracula"),
            ExtraStyles::GruvboxDark => write!(f, "Gruvbox (Night)"),
            ExtraStyles::GruvboxLight => write!(f, "Gruvbox (Day)"),
            ExtraStyles::NordLight => write!(f, "Nord (Day)"),
            ExtraStyles::NordDark => write!(f, "Nord (Night)"),
            ExtraStyles::SolarizedLight => write!(f, "Solarized (Day)"),
            ExtraStyles::SolarizedDark => write!(f, "Solarized (Night)"),
        }
    }
}
