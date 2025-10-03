//! Shadow garnishes
//!
//! A `Shadow` garnish draws a drop shadow for your widget using
//! Unicode shade characters: light '░', medium '▒', dark '▓',
//! or full '█' with the specified offset. The shadow color can be
//! styled using a `Style` garnish.
//!
//! # Example
//!
//! ```rust
//! use ratatui::{text::Text, style::{Color, Style}};
//! use ratatui_garnish::GarnishableWidget;
//! use ratatui_garnish::shadow::Shadow;
//!
//! // Light blue shadow offset by 3 columns, 2 rows
//! let widget = Text::raw("Hello, world!")
//!     .garnish(Style::default().fg(Color::Blue))
//!     .garnish(Shadow::new(3, 2).light());
//!     
//! // Dark shadow with default offset (1, 1)
//! let widget = Text::raw("Button")
//!     .garnish(Shadow::default().dark());
//! ```
//!
//! # `HalfShadow`
//!
//! `HalfShadow` provides sub-character precision by specifying offsets
//! in half-character lengths. It uses various Unicode block characters
//! to create smooth shadow edges. Only full-opacity shadows are supported.
//!
//! # Example
//!
//! ```rust
//! use ratatui::text::Text;
//! use ratatui_garnish::GarnishableWidget;
//! use ratatui_garnish::shadow::HalfShadow;
//!
//! // Subtle shadow with 0.5 character offset
//! let widget = Text::raw("Smooth text")
//!     .garnish(HalfShadow::default());
//!     
//! // Larger shadow with 1.5 character horizontal, 1 character vertical offset
//! let widget = Text::raw("Dialog box")
//!     .garnish(HalfShadow::new(3, 2));
//! ```
use crate::RenderModifier;
use ratatui::layout::{Position, Rect};

/// A shadow garnish that renders a drop shadow using Unicode shade characters.
///
/// The shadow is drawn with a specified character (`░`, `▒`, `▓`, or `█`) at the given offsets.
/// Offsets are in full character lengths, with positive values shifting the shadow right and down,
/// and negative values shifting it left and up.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Shadow {
    x_offset: i8,
    y_offset: i8,
    symbol: char,
}

impl Shadow {
    /// Creates a medium shadow with the given offsets.
    ///
    /// # Arguments
    ///
    /// * `x_offset` - Horizontal offset in character columns (positive = right)
    /// * `y_offset` - Vertical offset in character rows (positive = down)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_garnish::shadow::Shadow;
    /// let shadow = Shadow::new(2, 1); // 2 columns right, 1 row down
    /// ```
    #[must_use = "method returns a new instance"]
    pub fn new(x_offset: i8, y_offset: i8) -> Self {
        Self {
            x_offset,
            y_offset,
            ..Default::default()
        }
    }

    /// Sets the shadow to use the light shade character (`░`).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_garnish::shadow::Shadow;
    /// let shadow = Shadow::new(1, 1).light();
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn light(mut self) -> Self {
        self.symbol = '░';
        self
    }

    /// Sets the shadow to use the medium shade character (`▒`).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_garnish::shadow::Shadow;
    /// let shadow = Shadow::new(1, 1).medium();
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn medium(mut self) -> Self {
        self.symbol = '▒';
        self
    }

    /// Sets the shadow to use the dark shade character (`▓`).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_garnish::shadow::Shadow;
    /// let shadow = Shadow::new(1, 1).dark();
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn dark(mut self) -> Self {
        self.symbol = '▓';
        self
    }

    /// Sets the shadow to use the full shade character (`█`).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_garnish::shadow::Shadow;
    /// let shadow = Shadow::new(1, 1).full();
    /// ```
    #[must_use = "method returns a new instance and doqes not mutate the original"]
    pub const fn full(mut self) -> Self {
        self.symbol = '█';
        self
    }
}

impl Default for Shadow {
    /// Creates a `Shadow` with medium shade (`▒`) and offsets of 1, 1.
    fn default() -> Self {
        Self {
            x_offset: 1,
            y_offset: 1,
            symbol: '▒',
        }
    }
}

impl RenderModifier for Shadow {
    fn modify_area(&self, area: ratatui::prelude::Rect) -> ratatui::prelude::Rect {
        let width = area
            .width
            .saturating_sub(u16::from(self.x_offset.unsigned_abs()));
        let height = area
            .height
            .saturating_sub(u16::from(self.y_offset.unsigned_abs()));
        if width == 0 || height == 0 {
            return Rect::ZERO;
        }

        let (x, y) = if self.x_offset > 0 && self.y_offset > 0 {
            (0, 0)
        } else if self.x_offset > 0 {
            (0, u16::from(self.y_offset.unsigned_abs()))
        } else if self.y_offset > 0 {
            (u16::from(self.x_offset.unsigned_abs()), 0)
        } else {
            (
                u16::from(self.x_offset.unsigned_abs()),
                u16::from(self.y_offset.unsigned_abs()),
            )
        };

        Rect {
            x: area.x + x,
            y: area.y + y,
            width,
            height,
        }
    }

    fn before_render(&self, area: Rect, buffer: &mut ratatui::prelude::Buffer) {
        let widget_area = self.modify_area(area).intersection(buffer.area);

        let (horizontal_x, vertical_x) = if self.x_offset < 0 {
            (area.x, area.x)
        } else {
            (
                widget_area.x + u16::from(self.x_offset.unsigned_abs()),
                area.x + widget_area.width,
            )
        };

        let (horizontal_y, vertical_y) = if self.y_offset < 0 {
            (area.y, area.y)
        } else {
            (
                area.y + widget_area.height,
                widget_area.y + u16::from(self.y_offset.unsigned_abs()),
            )
        };

        for x in horizontal_x..(horizontal_x + widget_area.width) {
            for y in horizontal_y..(horizontal_y + u16::from(self.y_offset.unsigned_abs())) {
                buffer[(x, y)].set_char(self.symbol);
            }
        }

        for x in vertical_x..(vertical_x + u16::from(self.x_offset.unsigned_abs())) {
            for y in vertical_y..(vertical_y + widget_area.height) {
                buffer[(x, y)].set_char(self.symbol);
            }
        }
    }
}

/// Renders a shadow with sub-character precision using half-character offsets.
///
/// Uses the full shade character (`█`) for whole character offsets and half or quadrant characters
/// (e.g., `▗`, `▖`) for half-character offsets, allowing finer positioning control.
/// Offsets are specified in half-character lengths (e.g., `3` means 1.5 characters).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HalfShadow {
    x_offset: i8,
    y_offset: i8,
}

impl HalfShadow {
    /// Creates a shadow using the given half-character offsets.
    ///
    /// Offsets are in half-character lengths (e.g., `3` means 1.5 characters).
    /// Positive offsets shift the shadow right and down; negative offsets shift it left and up.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ratatui_garnish::shadow::HalfShadow;
    /// // 1.5 character right, 1.0 character down
    /// let shadow = HalfShadow::new(3, 2);
    /// ```
    #[must_use = "method returns a new instance"]
    pub const fn new(x_offset: i8, y_offset: i8) -> Self {
        Self { x_offset, y_offset }
    }
}

impl Default for HalfShadow {
    /// Creates a `HalfShadow` with offsets of 1, 1 (0.5 characters each).
    fn default() -> Self {
        Self {
            x_offset: 1,
            y_offset: 1,
        }
    }
}

impl RenderModifier for HalfShadow {
    fn modify_area(&self, area: ratatui::prelude::Rect) -> ratatui::prelude::Rect {
        let x = self.x_offset / 2 + self.x_offset % 2;
        let y = self.y_offset / 2 + self.y_offset % 2;
        let width = area.width.saturating_sub(u16::from(x.unsigned_abs()));
        let height = area.height.saturating_sub(u16::from(y.unsigned_abs()));
        if width == 0 || height == 0 {
            return Rect::ZERO;
        }

        let (x, y) = if self.x_offset > 0 && self.y_offset > 0 {
            (0, 0)
        } else if self.x_offset > 0 {
            (0, u16::from(y.unsigned_abs()))
        } else if self.y_offset > 0 {
            (u16::from(x.unsigned_abs()), 0)
        } else {
            (u16::from(x.unsigned_abs()), u16::from(y.unsigned_abs()))
        };

        Rect {
            x: area.x + x,
            y: area.y + y,
            width,
            height,
        }
    }

    fn before_render(&self, area: Rect, buffer: &mut ratatui::prelude::Buffer) {
        let widget_area = self.modify_area(area).intersection(buffer.area);

        let delta_x = i32::from(self.x_offset.unsigned_abs() % 2);
        let delta_y = i32::from(self.y_offset.unsigned_abs() % 2);

        let mut start_x = i32::from(widget_area.left()) + i32::from(self.x_offset / 2);
        if self.x_offset < 0 {
            start_x -= 1;
        }

        let mut start_y = i32::from(widget_area.top()) + i32::from(self.y_offset / 2);
        if self.y_offset < 0 {
            start_y -= 1;
        }

        let end_x = start_x + i32::from(widget_area.width) + delta_x;
        let end_y = start_y + i32::from(widget_area.height) + delta_y;

        if start_x < 0 || start_y < 0 {
            return;
        }

        let start_x = u16::try_from(start_x).unwrap_or(u16::MAX);
        let start_y = u16::try_from(start_y).unwrap_or(u16::MAX);
        let end_x = u16::try_from(end_x).unwrap_or(u16::MAX);
        let end_y = u16::try_from(end_y).unwrap_or(u16::MAX);

        for y in start_y..end_y {
            for x in start_x..end_x {
                if widget_area.contains(Position::new(x, y)) {
                    continue;
                }

                let symbol = match (
                    delta_x != 0,
                    delta_y != 0,
                    x == start_x,
                    x == end_x - 1,
                    y == start_y,
                    y == end_y - 1,
                ) {
                    (true, true, true, false, true, false) => '▗',
                    (true, true, false, true, true, false) => '▖',
                    (true, true, true, false, false, true) => '▝',
                    (true, true, false, true, false, true) => '▘',
                    (true, _, true, false, false, false) => '▐',
                    (true, _, false, true, false, false) => '▌',
                    (_, true, false, false, true, false) => '▄',
                    (_, true, false, false, false, true) => '▀',
                    _ => '█',
                };

                buffer[(x, y)].set_char(symbol);
            }
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_serialization() {
        let shadow = Shadow::default();
        let json = serde_json::to_string_pretty(&shadow).unwrap();

        let restored: Shadow = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, shadow);
    }

    #[test]
    fn half_shadow_serialization() {
        let shadow = HalfShadow::default();
        let json = serde_json::to_string_pretty(&shadow).unwrap();

        let restored: HalfShadow = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, shadow);
    }
}
