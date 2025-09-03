//! Shadow garnishes
//!
//! A `Shadow` garnish draws a shadow for your widget using
//! the unicode shade characters light '░', medium '▒', dark '▓'
//! or full '█' using the given offset. The color can be set using a
//! `Style` garnish:
//!
//! # Example
//!
//! ```rust
//! use ratatui::{text::Text, style::{Color, Style}};
//! use ratatui_garnish::GarnishableWidget;
//! use ratatui_garnish::shadow::Shadow;
//!
//! let widget = Text::raw("Hello, world")
//!     .garnish(Style::default().fg(Color::Blue))
//!     .garnish(Shadow::new(3,2).light()); // A light blue, shadow
//! ```
//!
//! # `HalfShadow`
//!
//! Half refers to the size: the offset is given in half character
//! lengths. As there are no half versions of the shade characters
//! it is only avaible using the full shade.
//!
//! # Example
//!
//! ```rust
//! use ratatui::text::Text;
//! use ratatui_garnish::GarnishableWidget;
//! use ratatui_garnish::shadow::HalfShadow;
//!
//! let widget = Text::raw("Hello, world")
//!     .garnish(HalfShadow::default()); // A thin shadow
//! ```

use crate::RenderModifier;
use ratatui::layout::{Position, Rect};

/// A Shadow garnish
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Shadow {
    x_offset: i8,
    y_offset: i8,
    symbol: char,
}

impl Shadow {
    /// create a medium `Shadow` with the given offsets.
    #[must_use = "method returns a new instance"]
    pub fn new(x_offset: i8, y_offset: i8) -> Self {
        Self {
            x_offset,
            y_offset,
            ..Default::default()
        }
    }

    /// Change `Shadow` to light: '░'.
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn light(mut self) -> Self {
        self.symbol = '░';
        self
    }

    /// Change `Shadow` to medium: '▒'.
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn medium(mut self) -> Self {
        self.symbol = '▒';
        self
    }

    /// Change `Shadow` to dark: '▓'.
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn dark(mut self) -> Self {
        self.symbol = '▓';
        self
    }

    /// Change `Shadow` to full: '█'.
    #[must_use = "method returns a new instance and doqes not mutate the original"]
    pub const fn full(mut self) -> Self {
        self.symbol = '█';
        self
    }
}

impl Default for Shadow {
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

/// Renders a shadow for your widget using half character
/// offset, only using full type shadow. Default creates
/// a `HalfShadow` with an offset of 1, 1.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HalfShadow {
    x_offset: i8,
    y_offset: i8,
}

impl HalfShadow {
    /// Create a shadow using the offsets given.
    /// The offsets are given in half character lengths:
    /// e.g. an offset of 3 means 1.5 characters.
    #[must_use = "method returns a new instance"]
    pub const fn new(x_offset: i8, y_offset: i8) -> Self {
        Self { x_offset, y_offset }
    }
}

impl Default for HalfShadow {
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
