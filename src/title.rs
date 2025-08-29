//! A garnish that adds a styled text line to a widget at a specified location.
//!
//! # `Title` comes in four flavors:
//!
//! - **`Title<Top>`**: renders a `Line` *over* the top line of the
//!   widget.
//! - **`Title<Bottom>`** renders a `Line` *over* the bottom line
//!   of the widget.
//! - **`Title<Above>`** renders a `Line` above the widget. The
//!   `Style` of the line is used to set the whole area of
//!   the top line, excluding the margin.
//! - **`Title<Below>`** renders a `Line` below the widget. The
//!   `Style` of the line is used to set the whole area of
//!   the bottom line, excluding the margin.
//!
//! When using either `Top` or `Bottom`, as it renders over the widget, it is a good idea
//! to garnish your widget with a border or padding **after applying the title** otherwise
//! `Title` will render over your widget.
//!
//! # Margin
//!
//! The `margin` field is used to set the left and right margin. It offers precise
//! control over the placement of the text. Use margin to prevent titles from overlapping
//! with the corners of borders.
//!
//! # Style Inheritance
//!
//! Titles implement [`Styled`] and support all standard style operations including
//! patching, resetting, and direct style assignment.
//!
//! # Examples
//!
//! ```rust
//! use ratatui_garnish::title::{Title, Top, Above};
//! use ratatui::{style::{Color, Style}, text::Span};
//!
//! // Basic title with margin
//! let title = Title::<Top>::raw("Hello World")
//!     .margin(2)
//!     .centered();
//!
//! // Styled title
//! let title = Title::<Above>::styled("Error", Style::default().fg(Color::Red))
//!     .left_aligned()
//!     .margin(1);
//!
//! // Create a styled title with spans
//! let title = Title::<Above>::default()
//!     .spans([
//!         Span::raw("Status: "),
//!         Span::styled("OK", Style::default().fg(Color::Green))
//!     ])
//!     .centered();
//!
use std::{borrow::Cow, marker::PhantomData};

use crate::WidgetModifier;
use derive_more::{Deref, DerefMut};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Styled},
    text::{Line, Span},
    widgets::WidgetRef,
};
use unicode_width::UnicodeWidthStr;

/// A wrapper around [`Line`] with additional positioning and margin control.
/// The generic parameter `Location` determines where the title is rendered relative to the widget.
#[derive(Eq, PartialEq, Hash, Deref, DerefMut)]
pub struct Title<'a, Location: TitlePosition> {
    #[deref]
    #[deref_mut]
    line: Line<'a>,
    margin: u8,
    _position: PhantomData<Location>,
}

// ===== Position Marker Types =====

/// Marker trait for title positioning strategies.
///
/// This trait is sealed and can only be implemented by the position types
/// defined in this module.
pub trait TitlePosition: private::Sealed {}

pub struct Top {}

impl TitlePosition for Top {}

pub struct Bottom {}

impl TitlePosition for Bottom {}

pub struct Above {}

impl TitlePosition for Above {}

pub struct Below {}

impl TitlePosition for Below {}

pub struct Left {}

impl TitlePosition for Left {}

pub struct Right {}

impl TitlePosition for Right {}

pub struct Before {}

impl TitlePosition for Before {}

pub struct After {}

impl TitlePosition for After {}

// ===== Core Implementation =====

impl<'a, Location: TitlePosition> Title<'a, Location> {
    /// Creates a new title from a string with default style and margin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top, Above};
    /// # let username = "Laranja";
    /// let title = Title::<Top>::raw("My Application");
    /// let title = Title::<Above>::raw(format!("User: {username}"));
    /// ```
    pub fn raw<T>(content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            line: Line::raw(content),
            margin: 0,
            _position: PhantomData,
        }
    }

    /// Creates a new title with the specified style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// # use ratatui::style::{Style, Color, Stylize};
    /// let title = Title::<Top>::styled(
    ///     "Warning",
    ///     Style::default().fg(Color::Yellow).bold()
    /// );
    /// ```
    pub fn styled<T, S>(content: T, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Self {
            line: Line::styled(content, style),
            margin: 0,
            _position: PhantomData,
        }
    }

    /// Sets the left and right margin of the title.
    ///
    /// It affects positioning of the title and prevents text from being rendered over that area.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// let title = Title::<Top>::raw("Title").margin(2);
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn margin(mut self, margin: u8) -> Self {
        self.margin = margin;
        self
    }

    /// Sets the spans of the title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// # use ratatui::style::{Style, Color};
    /// use ratatui::text::Span;
    ///
    /// let title = Title::<Top>::default().spans([
    ///     Span::raw("Status: "),
    ///     Span::styled("Connected", Style::default().fg(Color::Green))
    /// ]);
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'a>>,
    {
        self.line.spans = spans.into_iter().map(Into::into).collect();
        self
    }

    /// Patches the current style by adding modifiers from the given style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// # use ratatui::style::{Style, Color, Stylize};
    /// let title = Title::<Top>::styled("Title", Style::default().fg(Color::Red))
    ///     .patch_style(Style::default().bold()); // Now red and bold
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.line.style = self.line.style.patch(style.into());
        self
    }

    /// Resets the title style to default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// # use ratatui::style::{Style, Color};
    /// let title = Title::<Top>::styled("Title", Style::default().fg(Color::Red))
    ///     .reset_style(); // Now uses default styling
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
    }

    /// Sets the alignment for this title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// use ratatui::layout::Alignment;
    ///
    /// let title = Title::<Top>::raw("Title")
    ///     .alignment(Alignment::Center);
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn alignment(mut self, alignment: Alignment) -> Self {
        self.line.alignment = Some(alignment);
        self
    }

    /// Left-aligns the title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// let title = Title::<Top>::raw("Title").left_aligned();
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Centers the title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// let title = Title::<Top>::raw("Title").centered();
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right-aligns the title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::title::{Title, Top};
    /// let title = Title::<Top>::raw("Title").right_aligned();
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    // /// Converts this title to render on top of the widget.
    // ///
    // /// The title will overlay the top line of the widget area.
    // #[must_use = "method returns a new instance and does not mutate the original"]
    // pub fn top(self) -> Title<'a, Top> {
    //     self.convert_position()
    // }
    //
    // /// Converts this title to render on the bottom of the widget.
    // ///
    // /// The title will overlay the bottom line of the widget area.
    // #[must_use = "method returns a new instance and does not mutate the original"]
    // pub fn bottom(self) -> Title<'a, Bottom> {
    //     self.convert_position()
    // }
    //
    // /// Converts this title to render above the widget.
    // ///
    // /// The title will reserve space above the widget, reducing the widget's area.
    // #[must_use = "method returns a new instance and does not mutate the original"]
    // pub fn above(self) -> Title<'a, Above> {
    //     self.convert_position()
    // }
    //
    // /// Converts this title to render below the widget.
    // ///
    // /// The title will reserve space below the widget, reducing the widget's area.
    // #[must_use = "method returns a new instance and does not mutate the original"]
    // pub fn below(self) -> Title<'a, Below> {
    //     self.convert_position()
    // }
    //
    // /// Generic position conversion helper.
    // fn convert_position<NewLocation: TitlePosition>(self) -> Title<'a, NewLocation> {
    //     Title {
    //         line: self.line,
    //         margin: self.margin,
    //         _position: PhantomData,
    //     }
    // }

    /// Calculates the render area for a title positioned at the top.
    ///
    /// Takes margin into account when determining the available width and position.
    fn calculate_top_area(&self, area: Rect) -> Rect {
        self.calculate_horizontal_area(area, area.y)
    }

    /// Calculates the render area for a title positioned at the bottom.
    ///
    /// Takes margin into account when determining the available width and position.
    fn calculate_bottom_area(&self, area: Rect) -> Rect {
        let y = area.bottom().saturating_sub(1);
        self.calculate_horizontal_area(area, y)
    }

    /// Helper to calculate horizontal positioning with margin consideration.
    fn calculate_horizontal_area(&self, area: Rect, y: u16) -> Rect {
        let margin_u16 = u16::from(self.margin);
        let double_margin = margin_u16.saturating_mul(2);

        if area.width <= double_margin {
            return Rect::ZERO;
        }

        let x = area.x.saturating_add(margin_u16);
        let width = area.width.saturating_sub(double_margin);

        Rect {
            x,
            y,
            width,
            height: 1,
        }
    }

    const fn calculate_left_area(&self, area: Rect) -> Rect {
        self.calculate_vertical_area(area, area.x)
    }

    const fn calculate_right_area(&self, area: Rect) -> Rect {
        let x = area.right().saturating_sub(1);
        self.calculate_vertical_area(area, x)
    }

    const fn calculate_vertical_area(&self, area: Rect, x: u16) -> Rect {
        #[allow(clippy::cast_possible_truncation)]
        let margin_u16 = self.margin as u16;
        let double_margin = margin_u16.saturating_mul(2);

        if area.height <= double_margin {
            return Rect::ZERO;
        }

        let y = area.y.saturating_add(margin_u16);
        let height = area.height.saturating_sub(double_margin);

        Rect {
            x,
            y,
            width: 1,
            height,
        }
    }

    // TODO filter out wide chars?
    fn render_vertical(&self, area: Rect, buffer: &mut Buffer, alignment: Alignment) {
        if area.height == 0 {
            return;
        }

        let line_width: u16 = self.line.width().try_into().expect("Line too long");
        let start_y = match alignment {
            Alignment::Left => area.y, // Top for Left/Before
            Alignment::Center => area.y + area.height.saturating_sub(line_width) / 2,
            Alignment::Right => area.y + area.height.saturating_sub(line_width), // Bottom for Right/After
        };
        let x = area.x;
        let mut y = start_y;

        // adapted from ratatui::text::Span::render_ref()
        for (i, grapheme) in self.line.styled_graphemes(Style::default()).enumerate() {
            let symbol_width = grapheme.symbol.width();
            let next_y = y.saturating_add(1);
            let next_x = x.saturating_add(symbol_width as u16);

            if next_y > area.bottom() {
                break;
            }

            if i == 0 {
                // the first grapheme is always set on the cell
                buffer[(x, y)]
                    .set_symbol(grapheme.symbol)
                    .set_style(grapheme.style);
            } else if y == start_y {
                // there is one or more zero-width graphemes in the first cell, so the first cell
                // must be appended to.
                let mut symbol = buffer[(x, y)].symbol().to_string();
                symbol.push_str(grapheme.symbol);
                buffer[(x, y)].set_symbol(&symbol).set_style(grapheme.style);
            } else if symbol_width == 0 {
                // append zero-width graphemes to the previous cell
                let mut symbol = buffer[(x, y)].symbol().to_string();
                symbol.push_str(grapheme.symbol);
                buffer[(x, y - 1)]
                    .set_symbol(&symbol)
                    .set_style(grapheme.style);
            } else {
                // just a normal grapheme (not first, not zero-width, not overflowing the area)
                buffer[(x, y)]
                    .set_symbol(grapheme.symbol)
                    .set_style(grapheme.style);
            }
            for x_hidden in (x + 1)..next_x {
                buffer[(x_hidden, y)].reset();
            }
            y = next_y;
        }
    }
}

// ===== Trait Implementations =====

impl<Location: TitlePosition> Styled for Title<'_, Location> {
    type Item = Self;

    fn style(&self) -> Style {
        self.line.style
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        Self {
            line: self.line.set_style(style),
            margin: self.margin,
            _position: PhantomData,
        }
    }
}

impl<Location: TitlePosition> core::fmt::Debug for Title<'_, Location> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Title")
            .field("line", &self.line)
            .field("margin", &self.margin)
            .field("position", &std::any::type_name::<Location>())
            .finish()
    }
}

impl<Location: TitlePosition> Clone for Title<'_, Location> {
    fn clone(&self) -> Self {
        Self {
            line: self.line.clone(),
            margin: self.margin,
            _position: PhantomData,
        }
    }
}

impl<Location: TitlePosition> Default for Title<'_, Location> {
    fn default() -> Self {
        Self {
            line: Line::default(),
            margin: 0,
            _position: PhantomData,
        }
    }
}

// ===== From Implementations =====

impl<'a, Location: TitlePosition> From<Line<'a>> for Title<'a, Location> {
    fn from(line: Line<'a>) -> Self {
        Self {
            line,
            margin: 0,
            _position: PhantomData,
        }
    }
}

impl<Location: TitlePosition> From<String> for Title<'_, Location> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a, Location: TitlePosition> From<&'a str> for Title<'a, Location> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl<'a, Location: TitlePosition> From<Cow<'a, str>> for Title<'a, Location> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::raw(s)
    }
}

impl<'a, Location: TitlePosition> From<Vec<Span<'a>>> for Title<'a, Location> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self {
            line: Line {
                spans,
                ..Default::default()
            },
            margin: 0,
            _position: PhantomData,
        }
    }
}

impl<'a, Location: TitlePosition> From<Span<'a>> for Title<'a, Location> {
    fn from(span: Span<'a>) -> Self {
        Self::from(vec![span])
    }
}

// ===== WidgetModifier Implementations =====

impl WidgetModifier for Title<'_, Top> {
    fn after_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_top_area(area);
        if !render_area.is_empty() {
            self.line.render_ref(render_area, buffer);
        }
    }
}

impl WidgetModifier for Title<'_, Bottom> {
    fn after_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_bottom_area(area);
        if !render_area.is_empty() {
            self.line.render_ref(render_area, buffer);
        }
    }
}

impl WidgetModifier for Title<'_, Above> {
    fn before_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_top_area(area);
        if !render_area.is_empty() {
            buffer.set_style(render_area, self.line.style);
            self.line.render_ref(render_area, buffer);
        }
    }

    fn modify_area(&self, area: Rect) -> Rect {
        Rect {
            y: area.y.saturating_add(1),
            height: area.height.saturating_sub(1),
            ..area
        }
    }
}

impl WidgetModifier for Title<'_, Below> {
    fn before_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_bottom_area(area);
        if !render_area.is_empty() {
            buffer.set_style(render_area, self.line.style);
            self.line.render_ref(render_area, buffer);
        }
    }

    fn modify_area(&self, area: Rect) -> Rect {
        Rect {
            height: area.height.saturating_sub(1),
            ..area
        }
    }
}

impl WidgetModifier for Title<'_, Left> {
    fn after_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_left_area(area);
        if !render_area.is_empty() {
            self.render_vertical(
                render_area,
                buffer,
                self.line.alignment.unwrap_or(Alignment::Left),
            );
        }
    }
}

impl WidgetModifier for Title<'_, Right> {
    fn after_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_right_area(area);
        if !render_area.is_empty() {
            self.render_vertical(
                render_area,
                buffer,
                self.line.alignment.unwrap_or(Alignment::Left),
            );
        }
    }
}

impl WidgetModifier for Title<'_, Before> {
    fn before_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_left_area(area);
        if !render_area.is_empty() {
            buffer.set_style(render_area, self.line.style);
            self.render_vertical(
                render_area,
                buffer,
                self.line.alignment.unwrap_or(Alignment::Left),
            );
        }
    }

    fn modify_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x.saturating_add(1),
            width: area.width.saturating_sub(1),
            ..area
        }
    }
}

impl WidgetModifier for Title<'_, After> {
    fn before_render(&self, area: Rect, buffer: &mut Buffer) {
        let render_area = self.calculate_right_area(area);
        if !render_area.is_empty() {
            buffer.set_style(render_area, self.line.style);
            self.render_vertical(
                render_area,
                buffer,
                self.line.alignment.unwrap_or(Alignment::Left),
            );
        }
    }

    fn modify_area(&self, area: Rect) -> Rect {
        Rect {
            width: area.width.saturating_sub(1),
            ..area
        }
    }
}

// ===== Private Module for Sealed Trait =====

mod private {
    pub trait Sealed {}
    impl Sealed for super::Top {}
    impl Sealed for super::Bottom {}
    impl Sealed for super::Above {}
    impl Sealed for super::Below {}
    impl Sealed for super::Left {}
    impl Sealed for super::Right {}
    impl Sealed for super::Before {}
    impl Sealed for super::After {}
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{
        buffer::Buffer,
        layout::Rect,
        style::{Color, Style},
        text::Span,
    };

    fn create_test_buffer(width: u16, height: u16) -> Buffer {
        Buffer::empty(Rect::new(0, 0, width, height))
    }

    #[test]
    fn title_creation_from_various_types() {
        // From string
        let title1 = Title::<Top>::from("Hello World".to_string());
        assert_eq!(title1.line.spans[0].content, "Hello World");

        // From str
        let title2 = Title::<Top>::from("Hello World");
        assert_eq!(title2.line.spans[0].content, "Hello World");

        // From Cow
        let cow = Cow::Borrowed("Hello World");
        let title3 = Title::<Top>::from(cow);
        assert_eq!(title3.line.spans[0].content, "Hello World");

        // From spans
        let spans = vec![
            Span::raw("Hello "),
            Span::styled("World", Style::default().fg(Color::Red)),
        ];
        let title4 = Title::<Top>::from(spans);
        assert_eq!(title4.line.spans.len(), 2);

        // From single span
        let span = Span::styled("Single", Style::default().fg(Color::Blue));
        let title5 = Title::<Top>::from(span);
        assert_eq!(title5.line.spans.len(), 1);
    }

    #[test]
    fn title_raw_and_styled_constructors() {
        let raw_title = Title::<Top>::raw("Raw Title");
        assert_eq!(raw_title.line.spans[0].content, "Raw Title");
        assert_eq!(raw_title.margin, 0);

        let styled_title = Title::<Top>::styled("Styled Title", Style::default().fg(Color::Red));
        assert_eq!(styled_title.line.spans[0].content, "Styled Title");
        assert_eq!(styled_title.line.style.fg, Some(Color::Red));
    }

    #[test]
    fn title_configuration_methods() {
        let title = Title::<Top>::raw("Test")
            .margin(2)
            .centered()
            .set_style(Style::default().bg(Color::Blue));

        assert_eq!(title.margin, 2);
        assert_eq!(title.line.alignment, Some(Alignment::Center));
        assert_eq!(title.line.style.bg, Some(Color::Blue));
    }

    #[test]
    fn title_alignment_methods() {
        let left_title = Title::<Top>::raw("Left").left_aligned();
        assert_eq!(left_title.line.alignment, Some(Alignment::Left));

        let center_title = Title::<Top>::raw("Center").centered();
        assert_eq!(center_title.line.alignment, Some(Alignment::Center));

        let right_title = Title::<Top>::raw("Right").right_aligned();
        assert_eq!(right_title.line.alignment, Some(Alignment::Right));
    }

    #[test]
    fn title_style_operations() {
        let base_style = Style::default().fg(Color::Red);
        let title = Title::<Top>::styled("Test", base_style);

        // Test style patching
        let patched_title = title.clone().patch_style(Style::default().bg(Color::Blue));
        assert_eq!(patched_title.line.style.fg, Some(Color::Red));
        assert_eq!(patched_title.line.style.bg, Some(Color::Blue));

        // Test style reset
        let reset_title = title.reset_style();
        assert_eq!(reset_title.line.style, Style::reset());
    }

    #[test]
    fn area_calculation_with_margin() {
        let title = Title::<Top>::raw("Test").margin(2);
        let area = Rect::new(0, 0, 10, 5);

        let top_area = title.calculate_top_area(area);
        assert_eq!(top_area, Rect::new(2, 0, 6, 1)); // x=2 (margin), width=6 (10-4)

        let bottom_area = title.calculate_bottom_area(area);
        assert_eq!(bottom_area, Rect::new(2, 4, 6, 1)); // y=4 (bottom-1)
    }

    #[test]
    fn area_calculation_edge_cases() {
        let title = Title::<Top>::raw("Test").margin(5);

        // Area too small for margin
        let tiny_area = Rect::new(0, 0, 8, 3);
        let result = title.calculate_top_area(tiny_area);
        assert_eq!(result, Rect::ZERO);

        // Exact margin size
        let exact_area = Rect::new(0, 0, 10, 3);
        let result = title.calculate_top_area(exact_area);
        assert_eq!(result, Rect::ZERO); // 10 - (5*2) = 0 width
    }

    #[test]
    fn title_rendering_with_garnish() {
        let mut buffer = create_test_buffer(10, 3);
        let area = Rect::new(0, 0, 10, 3);

        // Test Top title rendering
        let top_title = Title::<Top>::raw("TOP");
        top_title.after_render(area, &mut buffer);

        // Should render on the first line
        assert_eq!(buffer[(0, 0)].symbol(), "T");
        assert_eq!(buffer[(1, 0)].symbol(), "O");
        assert_eq!(buffer[(2, 0)].symbol(), "P");

        // Test Above title with area modification
        let above_title = Title::<Above>::raw("ABOVE");
        let modified_area = above_title.modify_area(area);

        assert_eq!(modified_area.y, 1); // Moved down by 1
        assert_eq!(modified_area.height, 2); // Reduced by 1
    }

    #[test]
    fn styled_trait_implementation() {
        let title = Title::<Top>::raw("Test");
        let original_style = title.style();

        let new_style = Style::default().fg(Color::Green);
        let styled_title = title.set_style(new_style);

        assert_eq!(styled_title.style().fg, Some(Color::Green));
        assert_ne!(styled_title.style(), original_style);
    }

    #[test]
    fn deref_functionality() {
        let mut title = Title::<Top>::raw("Test");

        // Test Deref
        assert_eq!(title.spans[0].content, "Test");

        // Test DerefMut
        title.spans.push(Span::raw(" Added"));
        assert_eq!(title.spans.len(), 2);
    }

    #[test]
    fn spans_method_replaces_content() {
        let title = Title::<Top>::raw("Original").spans([
            Span::raw("New "),
            Span::styled("Content", Style::default().fg(Color::Blue)),
        ]);

        assert_eq!(title.line.spans.len(), 2);
        assert_eq!(title.line.spans[0].content, "New ");
        assert_eq!(title.line.spans[1].content, "Content");
        assert_eq!(title.line.spans[1].style.fg, Some(Color::Blue));
    }

    #[test]
    fn area_modification_for_positioning() {
        let area = Rect::new(5, 5, 20, 10);

        // Above should reduce height and move y down
        let above_title = Title::<Above>::raw("Test");
        let above_modified = above_title.modify_area(area);
        assert_eq!(above_modified.y, 6);
        assert_eq!(above_modified.height, 9);
        assert_eq!(above_modified.x, area.x);
        assert_eq!(above_modified.width, area.width);

        // Below should only reduce height
        let below_title = Title::<Below>::raw("Test");
        let below_modified = below_title.modify_area(area);
        assert_eq!(below_modified.y, area.y);
        assert_eq!(below_modified.height, 9);
        assert_eq!(below_modified.x, area.x);
        assert_eq!(below_modified.width, area.width);
    }
}
