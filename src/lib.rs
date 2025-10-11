//! <div align="center">
//!
//! *Garnish your Widgets*
//!
//! </div>
//!
//! A powerful composition system for [Ratatui](https://ratatui.rs) widgets.
//!
//! `ratatui-garnish` provides a flexible way to change the rendering of any Ratatui widget with
//! garnishes like borders, titles, padding, shadows, and styling. Garnishes can be layered
//! in any order, applied at runtime, and modified without altering the underlying widget. The
//! `GarnishedWidget` struct wraps a widget and a `Vec` of `Garnish` enums, maintaining zero-cost
//! abstractions and type safety without trait objects.
//!
//! Want a margin outside a border? Garnish with `Padding` before a border. Need multiple borders or
//! titles? Simply add them! Writing custom widgets but want to avoid boilerplate for styling or
//! borders? Use `ratatui-garnish` with any widget implementing `Widget` or `StatefulWidget`.
//!
//! # Example
//!
//! ```rust
//! use ratatui::{text::Text, style::{Color, Style}};
//! use ratatui_garnish::GarnishableWidget;
//! use ratatui_garnish::{border::RoundedBorder, title::{Title, Above}, Padding};
//!
//! // Create a text widget with multiple decorations
//! let widget = Text::raw("Hello, World!\nTasty TUIs from Ratatui")
//!     .garnish(RoundedBorder::default())           // Add a rounded border
//!     .garnish(Title::<Above>::raw("My App"))      // Add a title above
//!     .garnish(Style::default().bg(Color::Blue))   // Set a background color
//!     .garnish(Padding::uniform(1));               // Add padding inside
//!
//! // Garnishes are applied in order during rendering
//! ```
//!
//! # Getting Started
//!
//! Import the `GarnishableWidget` trait to enable the `garnish` method on any Ratatui widget:
//!
//! ```rust
//! use ratatui_garnish::GarnishableWidget;
//! ```
//!
//! This trait extends `Widget`. There is a similar traits `GarnishableStatefulWidget` for `StatefulWidget`.
//! The [`RenderModifier`] trait defines how garnishes modify rendering and layout. Ratatui's
//! `Style` implements `RenderModifier` and [`Padding`] too (which is similair to the
//! `Padding` from `Block` but can be serialized) allowing their use as garnishes:
//!
//! ```rust
//! use ratatui::{style::{Color, Style}, text::Line};
//! use ratatui_garnish::{GarnishableWidget, RenderModifier, Padding};
//!
//! let widget = Line::raw("Hello, World!")
//!     .garnish(Style::default().bg(Color::Blue))   // Background for padded area
//!     .garnish(Padding::horizontal(1))             // Padding on left and right
//!     .garnish(Style::default().bg(Color::Red))    // Background for next padded area
//!     .garnish(Padding::vertical(2))               // Padding on top and bottom
//!     .garnish(Style::default().bg(Color::White)); // Background for the line
//! ```
//!
//! The first call to `garnish()` returns a [`GarnishedWidget`] or [`GarnishedStatefulWidget`], which wraps your
//! widget and a `Vec` of [`Garnish`]. It also has a `garnish()` method so you
//! can keep adding garnishes. At any time you can access the
//! garnishes you've added by treating `GarnishedWidget` like a `Vec` of
//! `Garnish` items, which is an enum that wraps all available garnishes.
//!
//! ```rust
//! # use ratatui_garnish::{GarnishableWidget, RenderModifier, Padding};
//! # use ratatui::{style::{Style, Color}, text::Line};
//! #
//! let widget = Line::raw("Hello, World!")
//!    .garnish(Style::default().bg(Color::Blue))
//!    .garnish(Padding::horizontal(1));
//!
//! assert!(widget[0].is_style()); // The first garnish we added
//! assert_eq!(widget.first_padding(), Some(&Padding::horizontal(1)));
//!
//! // Let's look at all the garnishes
//! for garnish in &widget {
//!     println!("{garnish:?}");
//! }
//! ```
//!
//! Alternatively you can create a `GarnishedWidget` from a widget and a garnish using the `new` constructor
//! or from only a widget using `from`.  Add garnishes with methods like `push` or `extend`.
//!
//! ```rust
//! use ratatui_garnish::{GarnishedWidget, RenderModifier, Padding};
//! use ratatui::{style::{Style, Color}, text::Line};
//!
//! let mut widget = GarnishedWidget::from(Line::raw("Hello, World!"));
//! widget.push(Style::default().bg(Color::Green));
//! ````
//
//! # Available Garnishes
//!
//! ## Borders
//! - Standard: [`PlainBorder`], [`RoundedBorder`], [`DoubleBorder`], [`ThickBorder`]
//! - Dashed variants: [`DashedBorder`], [`RoundedDashedBorder`], [`ThickDashedBorder`],
//! - Custom: [`CharBorder`] (single character, e.g., `****`), [`CustomBorder`] (fully customizable character set)
//! - Specialty: [`QuadrantInsideBorder`], [`QuadrantOutsideBorder`], [`FatInsideBorder`], [`FatOutsideBorder`]
//!
//! ## Titles
//! - Horizontal: [`Title<Top>`] (over top border), [`Title<Bottom>`] (over bottom border), [`Title<Above>`] (reserves space above), [`Title<Below>`] (reserves space below)
//! - Vertical: [`Title<Left>`] (over left border), [`Title<Right>`] (over right border), [`Title<Before>`] (reserves space left), [`Title<After>`] (reserves space right)
//!
//! ## Shadows
//! - [`Shadow`] (light `░`, medium `▒`, dark `▓`, or full `█` shades with full-character offsets)
//! - [`HalfShadow`] (full `█` or quadrant characters with half-character offsets)
//!
//! ## Padding
//! - [`Padding`] (spacing around the widget), same as `Padding` from `ratatui::widgets::Block`
//!
//! ## Built-in Ratatui Support
//! - [`Style`] (background colors, text styling)
//!
//! ## Complex Compositions
//!
//! Combine multiple garnishes for rich widget designs:
//!
//! ```rust
//! use ratatui_garnish::{
//!     GarnishableWidget, RenderModifier,
//!     title::{Title, Top, Bottom,},
//!     border::DoubleBorder, Padding
//! };
//! use ratatui::{
//!     text::Line,
//!     style::{Color, Style, Modifier},
//! };
//!
//! let complex_widget = Line::raw("Important Message")
//!     // Add a margin
//!     .garnish(Padding::uniform(2))
//!     // Set Background color
//!     .garnish(Style::default().bg(Color::DarkGray))
//!     // Border with title
//!     .garnish(Title::<Top>::styled("⚠ WARNING ⚠",
//!         Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)).margin(1))
//!     .garnish(Title::<Bottom>::raw("Status: Active").right_aligned().margin(1))
//!     .garnish(DoubleBorder::default())
//!     .garnish(Padding::uniform(1));
//! ```
//!
//! # Reusing Garnishes
//!
//! Use the [`Garnishes`] vec and `extend_from_slice` or `extend` to apply
//! the same garnishes to multiple widgets:
//!
//! ```rust
//! # use ratatui_garnish::{
//! #     GarnishedWidget, GarnishableWidget, RenderModifier,
//! #     Padding,
//! #     title::{Title, Top},
//! #     border::DoubleBorder, garnishes,
//! # };
//! # use ratatui::{
//! #     text::Line,
//! #     style::{Color, Style, Modifier},
//! # };
//!
//! let garnishes = garnishes![
//!     Style::default().fg(Color::Blue),
//!     DoubleBorder::default(),
//!     Padding::uniform(2),
//!     Style::default().fg(Color::White),
//! ];
//!
//! let mut widget = GarnishedWidget::from(Line::raw("First widget"));
//! widget.extend_from_slice(&garnishes);
//!
//! let mut other_widget = Line::raw("Other widget")
//!     .garnish(Title::<Top>::styled("Second",
//!         Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)).margin(1));
//! other_widget.extend(garnishes);
//! ```
//!
//! The `GarnishableWidget` and `GarnishableStatefulWidget` add the methods
//! `garnishes` and `garnishes_from_slice` to construct `GarnishedWidget`s directly
//! from [`Garnishes`].
//!
//! ```rust
//! # use ratatui_garnish::{
//! #     GarnishedWidget, GarnishableWidget, RenderModifier,
//! #     Padding,
//! #     border::DoubleBorder, garnishes,
//! # };
//! # use ratatui::{
//! #     text::Line,
//! #     style::{Color, Style },
//! # };
//!
//! let widget = Line::raw("Widget")
//!     .garnishes( garnishes![
//!     Style::default().fg(Color::Blue),
//!     DoubleBorder::default(),
//!     Padding::uniform(2),
//!     Style::default().fg(Color::White),
//! ]);
//!
//! // copy garnishes of widget to other_widget
//! let other_widget = Line::raw("Other Widget")
//!     .garnishes_from_slice(widget.as_slice());
//! ```
//!
//! # Features
//!
//! ## Serde support
//!
//! Serialization & deserialization using serde can be enabled using the cargo feature
//! `serde`. When it is enabled all garnishes, the `Garnish` enum and the `Garnishes`
//! `Vec` can be serialized and deserialized. This makes it easy to add theme support
//! to your application.
//!
//! ## Decorated widget
//!
//! The cargo feature `decorated widget` enables `DecoratedWidget` and `DecoratedStatefulWidget`
//! which wrap one widget with one garnish, like the traditional decorator pattern. It
//! offers little benefits over `GarnishedWidget`. It might be slightly faster
//! if you want to use only a small number of garnishes. The `after_render` functions
//! are rendered in reverse order.
//!
//! ```rust
//! use ratatui::{style::{Color, Style}, text::Text};
//! use ratatui_garnish::{
//!     border::PlainBorder,
//!     title::{Title, Top},
//!     GarnishableWidget, Padding,
//! };
//!
//! #[cfg(feature = "decorated_widget")]
//! let widget = Text::raw("Hello World!")
//!     .decorate(Style::default().fg(Color::Red).bg(Color::White))
//!     .decorate(Title::<Top>::raw("Paragraph").margin(1))
//!     .decorate(PlainBorder::default())
//!     .decorate(Padding::horizontal(2));
//! ```
//!
//! # Compatibility
//!
//! `ratatui-garnish` works seamlessly with any Ratatui widget implementing `Widget` or `StatefulWidget`,
//! following Ratatui's conventions.
//!
//! # Contributing
//!
//! This is the first release of `ratatui-garnish`. More garnishes are planned, and contributions are
//! welcome!

use derive_more::{Deref, DerefMut};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{StatefulWidget, StatefulWidgetRef, Widget, WidgetRef},
};

pub mod border;
#[cfg(feature = "decorated_widget")]
mod decorator;
mod padding;
pub mod shadow;
pub mod title;

#[cfg(feature = "decorated_widget")]
pub use decorator::{DecoratedStatefulWidget, DecoratedWidget};
pub use padding::Padding;

use border::{
    CharBorder, CustomBorder, DashedBorder, DoubleBorder, FatInsideBorder, FatOutsideBorder,
    PlainBorder, QuadrantInsideBorder, QuadrantOutsideBorder, RoundedBorder, RoundedDashedBorder,
    ThickBorder, ThickDashedBorder,
};
use shadow::{HalfShadow, Shadow};
use title::{Above, After, Before, Below, Bottom, Left, Right, Title, Top};

/// A trait that can modify the rendering of a widget.
pub trait RenderModifier {
    /// Modifies the widget's rendering area.
    ///
    /// Returns the adjusted area, typically reduced to account for borders, padding, or shadows.
    /// Default implementation returns the input area unchanged.
    fn modify_area(&self, area: Rect) -> Rect {
        area
    }

    /// Executes before the widget is rendered.
    ///
    /// Used for pre-rendering effects like setting background styles or drawing shadows.
    /// Default implementation does nothing.
    fn before_render(&self, _area: Rect, _buf: &mut Buffer) {}

    /// Executes after the widget is rendered.
    ///
    /// Used for post-rendering effects like drawing titles over borders.
    /// Default implementation does nothing.
    fn after_render(&self, _area: Rect, _buf: &mut Buffer) {}
}

nodyn::nodyn! {
    /// Enum wrapping all available garnishes.
    #[module_path = "ratatui_garnish"]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Clone)]
    pub enum Garnish<'a> {
        CharBorder,
        CustomBorder,
        DashedBorder,
        DoubleBorder,
        FatInsideBorder,
        FatOutsideBorder,
        HalfShadow,
        Padding,
        PlainBorder,
        QuadrantInsideBorder,
        QuadrantOutsideBorder,
        RoundedBorder,
        RoundedDashedBorder,
        Shadow,
        Style,
        ThickBorder,
        ThickDashedBorder,
        Title<'a, Above>,
        Title<'a, After>,
        Title<'a, Before>,
        Title<'a, Below>,
        Title<'a, Bottom>,
        Title<'a, Left>,
        Title<'a, Right>,
        Title<'a, Top>,
    }

    impl is_as;

    impl RenderModifier {
        fn before_render(&self, area: Rect, buf: &mut Buffer);
        fn modify_area(&self, area: Rect) -> Rect;
        fn after_render(&self, area: Rect, buf: &mut Buffer);
    }

    /// A `Vec` of `Garnish` for applying multiple garnishes to widgets.
    ///
    /// Useful for reusing a set of garnishes across multiple widgets.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::{style::{Color, Style}, text::Line};
    /// use ratatui_garnish::{
    ///     border::DoubleBorder, garnishes, GarnishableWidget,
    ///     title::{Title, Top},
    /// };
    ///
    /// let garnishes = garnishes![
    ///     Style::default().fg(Color::Blue),
    ///     DoubleBorder::default(),
    ///     Style::default().fg(Color::White),
    /// ];
    ///
    /// let widget = Line::raw("Garnished Widget")
    ///     .garnish(Title::<Top>::raw("Blue Border"))
    ///     .extend_from_slice(&garnishes);
    /// ```
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    vec Garnishes;

    /// A widget that wraps another widget with a vector of garnishes.
    ///
    /// This struct implements `Deref` and `DerefMut` to the inner widget,
    /// allowing you to access the original widget's methods while adding
    /// garnish functionality.
    #[vec(garnishes)]
    #[derive(Debug, Deref, DerefMut)]
    pub struct GarnishedWidget<W> {
        #[deref]
        #[deref_mut]
        pub widget: W,
    }

    /// A widget that wraps another stateful widget with a vec of garnishes.
    ///
    /// This struct implements `Deref` and `DerefMut` to the inner widget,
    /// allowing you to access the original widget's methods while adding
    /// garnish functionality.
    #[vec(garnishes)]
    #[derive(Debug, Deref, DerefMut)]
    pub struct GarnishedStatefulWidget<W> {
        #[deref]
        #[deref_mut]
        pub widget: W,
    }
}

impl<'a, W> GarnishedWidget<'a, W> {
    /// creates a new `garnishedwidget` with a single garnish.
    ///
    /// # example
    ///
    /// ```rust
    /// use ratatui::{style::Style, text::Line};
    /// use ratatui_garnish::GarnishedWidget;
    ///
    /// let widget = GarnishedWidget::new(Line::raw("Test"), Style::default());
    /// ```
    pub fn new<G: Into<Garnish<'a>>>(widget: W, garnish: G) -> Self {
        Self {
            widget,
            garnishes: vec![garnish.into()],
        }
    }

    /// Adds an additional garnish to the widget.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::{style::Style, text::Line};
    /// use ratatui_garnish::{GarnishableWidget, Padding};
    ///
    /// let widget = Line::raw("Test")
    ///     .garnish(Style::default())
    ///     .garnish(Padding::uniform(1));
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn garnish<G: Into<Garnish<'a>>>(mut self, garnish: G) -> Self {
        self.push(garnish);
        self
    }
}

impl<W: Widget> From<W> for GarnishedWidget<'_, W> {
    fn from(value: W) -> Self {
        Self {
            widget: value,
            garnishes: Vec::new(),
        }
    }
}

impl<W: Widget> Widget for GarnishedWidget<'_, W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut render_area = area;
        for g in &self.garnishes {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.widget.render(render_area, buf);

        let mut render_area = area;
        for g in &self.garnishes {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

impl<W: WidgetRef> WidgetRef for GarnishedWidget<'_, W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut render_area = area;
        for g in &self.garnishes {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.widget.render_ref(render_area, buf);

        let mut render_area = area;
        for g in &self.garnishes {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

impl<'a, W> GarnishedStatefulWidget<'a, W> {
    /// Creates a new `GarnishedStatefulWidget` with a single garnish.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::{style::Style, widgets::List, text::Line};
    /// use ratatui_garnish::GarnishedStatefulWidget;
    ///
    /// let widget = GarnishedStatefulWidget::new(List::new::<Vec<Line>>(vec![]), Style::default());
    /// ```
    pub fn new<G: Into<Garnish<'a>>>(widget: W, garnish: G) -> Self {
        Self {
            widget,
            garnishes: vec![garnish.into()],
        }
    }

    /// Adds an additional garnish to the widget.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::{style::Style, widgets::List, text::Line};
    /// use ratatui_garnish::{GarnishableWidget, Padding};
    ///
    /// let widget = List::new::<Vec<Line>>(vec![])
    ///     .garnish(Style::default())
    ///     .garnish(Padding::uniform(1));
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn garnish<G: Into<Garnish<'a>>>(mut self, garnish: G) -> Self {
        self.push(garnish);
        self
    }
}

impl<W: StatefulWidget> From<W> for GarnishedStatefulWidget<'_, W> {
    fn from(value: W) -> Self {
        Self {
            widget: value,
            garnishes: Vec::new(),
        }
    }
}

impl<W> StatefulWidget for GarnishedStatefulWidget<'_, W>
where
    W: StatefulWidget,
{
    type State = W::State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut render_area = area;
        for g in &self.garnishes {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.widget.render(render_area, buf, state);

        let mut render_area = area;
        for g in &self.garnishes {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

impl<W> StatefulWidgetRef for GarnishedStatefulWidget<'_, W>
where
    W: StatefulWidgetRef,
{
    type State = W::State;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut render_area = area;
        for g in &self.garnishes {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.widget.render_ref(render_area, buf, state);

        let mut render_area = area;
        for g in &self.garnishes {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

/// A trait for widgets that can be garnished.
pub trait GarnishableWidget: Widget + Sized {
    /// Applies a garnish to the widget, wrapping it in a `GarnishedWidget`.
    fn garnish<'a, G: Into<Garnish<'a>>>(self, garnish: G) -> GarnishedWidget<'a, Self> {
        GarnishedWidget::new(self, garnish)
    }

    /// Applies a Vec<Garnish>to the widget, wrapping it in a `GarnishedWidget`.
    fn garnishes<'a, G: Into<Vec<Garnish<'a>>>>(self, garnishes: G) -> GarnishedWidget<'a, Self> {
        GarnishedWidget {
            widget: self,
            garnishes: garnishes.into(),
        }
    }

    /// Applies a copy of `&[Garnish]` to the widget, wrapping it in a `GarnishedWidget`.
    fn garnishes_from_slice<'a>(self, garnishes: &[Garnish<'a>]) -> GarnishedWidget<'a, Self> {
        GarnishedWidget {
            widget: self,
            garnishes: garnishes.to_vec(),
        }
    }

    /// Applies a garnish to the widget, wrapping it in a `DecoratedWidget`.
    #[cfg(feature = "decorated_widget")]
    fn decorate<R: RenderModifier>(self, garnish: R) -> DecoratedWidget<Self, R> {
        DecoratedWidget::new(self, garnish)
    }
}

// Blanket implementation for all widgets that implement `Widget`.
impl<W: Widget> GarnishableWidget for W {}

/// A trait for stateful widgets that can be garnished.
pub trait GarnishableStatefulWidget: StatefulWidget + Sized {
    /// Applies a garnish to the widget, wrapping it in a `GarnishedStatefulWidget`.
    fn garnish<'a, G: Into<Garnish<'a>>>(self, garnish: G) -> GarnishedStatefulWidget<'a, Self> {
        GarnishedStatefulWidget::new(self, garnish)
    }

    /// Applies a Vec<Garnish>to the widget, wrapping it in a `GarnishedStatefulWidget`.
    fn garnishes<'a, G: Into<Vec<Garnish<'a>>>>(
        self,
        garnishes: G,
    ) -> GarnishedStatefulWidget<'a, Self> {
        GarnishedStatefulWidget {
            widget: self,
            garnishes: garnishes.into(),
        }
    }

    /// Applies a copy of `&[Garnish]` to the widget, wrapping it in a `GarnishedStatefulWidget`.
    fn garnishes_from_slice<'a>(
        self,
        garnishes: &[Garnish<'a>],
    ) -> GarnishedStatefulWidget<'a, Self> {
        GarnishedStatefulWidget {
            widget: self,
            garnishes: garnishes.to_vec(),
        }
    }

    /// Applies a garnish to the widget, wrapping it in a `DecoratedStatefulWidget`.
    #[cfg(feature = "decorated_widget")]
    fn decorate<R: RenderModifier>(self, garnish: R) -> DecoratedStatefulWidget<Self, R> {
        DecoratedStatefulWidget::new(self, garnish)
    }
}

// Blanket implementation for all widgets that implement `StatefulWidget`.
impl<W: StatefulWidget> GarnishableStatefulWidget for W {}

// RenderModifier implementations for ratatui `Style` & `Padding`

impl RenderModifier for Style {
    fn before_render(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, *self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{
        style::{Color, Style},
        text::Line,
    };

    #[test]
    fn garnish_chain() {
        let widget = Line::raw("Test")
            .garnish(Style::default().bg(Color::Blue))
            .garnish(Padding::uniform(1));

        assert_eq!(widget.len(), 2);
        assert!(widget[0].is_style());
        assert_eq!(widget.first_padding(), Some(&Padding::uniform(1)));
    }

    #[test]
    fn widget_rendering() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let widget = Line::raw("Test")
            .garnish(Padding::uniform(1))
            .garnish(Style::default().bg(Color::Blue));

        widget.render(Rect::new(0, 0, 10, 10), &mut buffer);

        // Check padding reduced the area
        assert_eq!(buffer[(1, 1)].style().bg, Some(Color::Blue)); // Inside padded area
        assert_eq!(buffer[(0, 0)].style().bg, Some(Color::Reset)); // Outside padded area
    }
}
