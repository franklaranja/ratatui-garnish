//! <div align="center">
//!
//! *Garnish your Widgets*
//!
//! </div>
//!
//! A powerful composition system for [Ratatui](https://ratatui.rs) widgets.
//!
//! Ratatui Garnish provides a composable way to modify any widget,
//! including borders, titles, padding and styling. You can layer multiple garnishes
//! on any widget without modifying the widget itself. The garnishes and the order they
//! applied in can be changed at runtime whilst maintaining type safety (no trait objects).
//! a `GarnisedWidget` acts like the original widget with a `Vec` of Garnishes spliced in.
//!
//! Do you wish `Block` has a margin? Just garnish your widget with `Padding` before you
//! add a border. You want to have multiple borders? Just add them!
//!
//! Are you writing widgets, but don't want to add all that boiler plate for `Style` and `Border`?
//! Use ratatui-garnish, it works with any widget that implements `Widget`, `StatefulWidget`.
//! `WidgetRef` or `StatefulWidgetRef`.
//!
//! ## Example
//!
//! ```rust
//! use ratatui::{text::Text, widgets::Padding, style::{Color, Style}};
//! use ratatui_garnish::GarnishableWidget;
//! use ratatui_garnish::{border::RoundedBorder, title::{Title, Above}};
//!
//! // Create a paragraph with multiple decorations
//! let widget = Text::raw("Hello, World!\nTasty TUIs from Ratatui")
//!     .garnish(RoundedBorder::default())           // Add a rounded border
//!     .garnish(Title::<Above>::raw("My App"))      // Add a title above
//!     .garnish(Style::default().bg(Color::Blue))   // Add a background color
//!     .garnish(Padding::uniform(1));               // Add padding inside
//!
//! // The garnishes will be applied in order when rendering
//! ```
//!
//! # Getting started
//!
//! ```rust
//! use ratatui_garnish::GarnishableWidget;
//! ```
//!
//! This trait adds the garnish function to any ratatui [`Widget`], there are similar traits
//! for [`WidgetRef`], [`StatefulWidget`] and [`StatefulWidgetRef`].
//!
//! The [`WidgetModifier`] trait specifies what a garnish does. Ratatui-garnish implements
//! this trait for [`Style`] and [`Padding`] from ratatui. So you can use them
//! as garlnishes:
//!
//! ```rust
//! use ratatui_garnish::{GarnishableWidget, WidgetModifier};
//! use ratatui::{style::{Style, Color}, text::Line, widgets::Padding};
//!
//! let widget = Line::raw("Hello, World!")         // create widget
//!    .garnish(Style::default().bg(Color::Blue))   // set background for padded area
//!    .garnish(Padding::horizontal(1))             // add a padding to the left and right side.
//!    .garnish(Style::default().bg(Color::Red))    // set background for next padded area
//!    .garnish(Padding::vertical(2))               // add a padding to the top and bottom.
//!    .garnish(Style::default().bg(Color::White)); // set background for the line
//! ```
//!
//! The first call to `garnish()` returns a [`GarnishedWidget`], which wraps your
//! widget and a `Vec` of the [`Garnishes`]. It also has a `garnish()` method so you
//! just can keep on adding garnishes. At any time you can access the
//! garnishes you've added by treating `GarnishedWidget` like a `Vec` of
//! `Garnishes` which is an enum that wraps all available garnishes.
//!
//! ```rust
//! # use ratatui_garnish::{GarnishableWidget, WidgetModifier};
//! # use ratatui::{style::{Style, Color}, text::Line, widgets::Padding};
//! #
//! # let widget = Line::raw("Hello, World!")
//! #   .garnish(Style::default().bg(Color::Blue))
//! #   .garnish(Padding::horizontal(1))
//! #   .garnish(Style::default().bg(Color::Red))
//! #   .garnish(Padding::vertical(2))
//! #   .garnish(Style::default().bg(Color::White));
//! assert!(widget[0].is_style()); // The first garnish we added
//! assert_eq!(widget.first_padding(), Some(&Padding::horizontal(1)));
//!
//! // let's have a look at all the garnishes
//! for garnish in &widget {
//!     println!("{garnish:?}");
//! }
//! ```
//!
//! ## Available Garnishes
//!
//! ### Borders
//! - Standard borders like [`PlainBorder`], [`RoundedBorder`], [`DoubleBorder`] and [`ThickBorder`]
//! - [`CharBorder`] - Custom single character: `****`
//! - [`CustomBorder`] - Fully customizable character set
//! - Specialty borders: [`QuadrantInsideBorder`], [`QuadrantOutsideBorder`], [`FatInsideBorder`], [`FatOutsideBorder`]
//!
//! ### Titles
//! - [`Title<Top>`] - Renders over the top border/line
//! - [`Title<Bottom>`] - Renders over the bottom border/line  
//! - [`Title<Above>`] - Renders above widget (reserves space)
//! - [`Title<Below>`] - Renders below widget (reserves space)
//! - There are similar positions for rendering over the left or
//!   right border.
//!
//! ### Built-in Ratatui Support
//! - [`Style`] - Apply background colors, text styling
//! - [`Padding`] - Add spacing around the widget
//!
//! ## Complex Compositions
//!
//! ```rust
//! use ratatui_garnish::{
//!     GarnishableWidget, WidgetModifier,
//!     title::{Title, Top, Bottom,},
//!     border::DoubleBorder,
//! };
//! use ratatui::{
//!     text::Line,
//!     style::{Color, Style, Modifier},
//!     widgets::Padding,
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
//! ## Applying the same garnishes to multiple widgets.
//!
//! There is a regular polymorphic `Vec`, [`Garnishes`] that can be
//! used in combination with `extend_from_slice` from [`GarnishedWidget`].
//!
//! ```rust
//! use ratatui_garnish::{
//!     GarnishableWidget, WidgetModifier,
//!     title::{Title, Top},
//!     border::DoubleBorder, garnishes,
//! };
//! use ratatui::{text::Line, widgets::Padding};
//! use ratatui::style::{Color, Style, Modifier};
//!
//! let garnishes = garnishes![
//!     Style::default().fg(Color::Blue),
//!     DoubleBorder::default(),
//!     Padding::uniform(2),
//!     Style::default().fg(Color::White),
//! ];
//!
//! let mut widget = Line::raw("First widget")
//!     .garnish(Title::<Top>::styled("First",
//!         Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)).margin(1));
//! widget.extend_from_slice(&garnishes);
//!
//! let mut other_widget = Line::raw("Other widget")
//!     .garnish(Title::<Top>::styled("Second",
//!         Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)).margin(1));
//! other_widget.extend(garnishes);
//! ```
//!
//! ## Performance Notes
//!
//! - Garnishes are applied in the order they're added
//! - Area modifications are accumulated efficiently  
//! - Zero-cost abstractions - no runtime overhead for unused garnishes
//! - No dynamic dispatch and no type erasure
//!
//! ## Compatibility
//!
//! Any Ratatui Garnish is designed to work seamlessly with Ratatui widgets and follows the same
//! patterns and conventions. Any widget implementing [`Widget`], [`StatefulWidget`], [`WidgetRef`]
//! or [`StatefulWidgetRef`] can be garnished.
//!
//! ## More garnishes
//!
//! This is just the first release of ratatui-garnish. More garnishes are planned!
//! Contributions are welcome!

use derive_more::{Deref, DerefMut};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Padding, StatefulWidget, StatefulWidgetRef, Widget, WidgetRef},
};

pub mod border;
mod garnishable;
pub mod shadow;
pub mod title;

use border::{
    CharBorder, CustomBorder, DashedBorder, DoubleBorder, DoubleDashedBorder, FatInsideBorder,
    FatOutsideBorder, PlainBorder, QuadrantInsideBorder, QuadrantOutsideBorder, RoundedBorder,
    RoundedDashedBorder, RoundedDoubleDashedBorder, ThickBorder, ThickDashedBorder,
    ThickDoubleDashedBorder,
};

pub use garnishable::{
    GarnishableStatefulWidget, GarnishableStatefulWidgetRef, GarnishableWidget,
    GarnishableWidgetRef,
};

use shadow::{HalfShadow, Shadow};
use title::{Above, After, Before, Below, Bottom, Left, Right, Title, Top};

/// A trait for widget garnishes that can transform rendering and layout.
pub trait WidgetModifier {
    /// Modifies the rendering area for the widget.
    /// Default implementation returns the input area unchanged.
    fn modify_area(&self, area: Rect) -> Rect {
        area
    }

    /// Runs before the main widget is rendered.
    /// Default implementation does nothing.
    fn before_render(&self, _area: Rect, _buf: &mut Buffer) {}

    /// Runs after the main widget is rendered.
    /// Default implementation does nothing.
    fn after_render(&self, _area: Rect, _buf: &mut Buffer) {}
}

nodyn::nodyn! {
    /// Wrapper enum for all garnishes.
    #[module_path = "ratatui_garnish"]
    #[derive(Debug, Clone)]
    pub enum Garnish<'a> {
        CharBorder,
        CustomBorder,
        DashedBorder,
        DoubleBorder,
        DoubleDashedBorder,
        FatInsideBorder,
        FatOutsideBorder,
        HalfShadow,
        Padding,
        PlainBorder,
        QuadrantInsideBorder,
        QuadrantOutsideBorder,
        RoundedBorder,
        RoundedDashedBorder,
        RoundedDoubleDashedBorder,
        Shadow,
        Style,
        ThickBorder,
        ThickDashedBorder,
        ThickDoubleDashedBorder,
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

    impl WidgetModifier {
        fn before_render(&self, area: Rect, buf: &mut Buffer);
        fn modify_area(&self, area: Rect) -> Rect;
        fn after_render(&self, area: Rect, buf: &mut Buffer);
    }

    /// A wrapper around `Vec<Garnish>`. Convenient for
    /// creating `GarnisedWidget`s with the same garnishes
    ///
    /// ```rust
    /// use ratatui_garnish::{
    ///     GarnishableWidget, WidgetModifier,
    ///     title::{Title, Top},
    ///     border::DoubleBorder, garnishes,
    /// };
    /// use ratatui::{text::Line, style::{Color, Style, Modifier}};
    ///
    /// let garnishes = garnishes![
    ///     Style::default().fg(Color::Blue),
    ///     DoubleBorder::default(),
    ///     Style::default().fg(Color::White),
    /// ];
    ///
    /// let mut widget = Line::raw("A widget with a blue double border")
    ///     .garnish(Title::<Top>::raw("First"));
    /// widget.extend_from_slice(&garnishes);
    /// ```
    vec Garnishes;

    /// A widget that wraps another widget with a vec of garnishes
    /// (decorators).
    #[vec(garnish)]
    #[derive(Debug, Deref, DerefMut)]
    pub struct GarnishedWidget<W> {
        #[deref]
        #[deref_mut]
        pub inner: W,
    }

    /// A widget that wraps another stateful widget with a vec of garnishes
    /// (decorators).
    #[vec(garnish)]
    #[derive(Debug, Deref, DerefMut)]
    pub struct GarnishedStatefulWidget<W> {
        #[deref]
        #[deref_mut]
        pub inner: W,
    }
}

impl<'a, W> GarnishedWidget<'a, W> {
    /// Creates a new `GarnishedWidget` with a single garnish.
    pub fn new<G: Into<Garnish<'a>>>(widget: W, garnish: G) -> Self {
        Self {
            inner: widget,
            garnish: vec![garnish.into()],
        }
    }

    /// Adds an additional garnish to the widget.
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn garnish<G: Into<Garnish<'a>>>(mut self, garnish: G) -> Self {
        self.push(garnish);
        self
    }
}

impl<'a, W: Widget> Widget for GarnishedWidget<'a, W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut render_area = area;
        for g in &self.garnish {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.inner.render(render_area, buf);

        let mut render_area = area;
        for g in &self.garnish {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

impl<'a, W: WidgetRef> WidgetRef for GarnishedWidget<'a, W> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut render_area = area;
        for g in &self.garnish {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.inner.render_ref(render_area, buf);

        let mut render_area = area;
        for g in &self.garnish {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

impl<'a, W> GarnishedStatefulWidget<'a, W> {
    /// Creates a new `GarnishedWidget` with a single garnish.
    pub fn new<G: Into<Garnish<'a>>>(widget: W, garnish: G) -> Self {
        Self {
            inner: widget,
            garnish: vec![garnish.into()],
        }
    }

    /// Adds an additional garnish to the widget.
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub fn garnish<G: Into<Garnish<'a>>>(mut self, garnish: G) -> Self {
        self.push(garnish);
        self
    }
}

impl<'a, W> StatefulWidget for GarnishedStatefulWidget<'a, W>
where
    W: StatefulWidget,
{
    type State = W::State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut render_area = area;
        for g in &self.garnish {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.inner.render(render_area, buf, state);

        let mut render_area = area;
        for g in &self.garnish {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

impl<'a, W> StatefulWidgetRef for GarnishedStatefulWidget<'a, W>
where
    W: StatefulWidgetRef,
{
    type State = W::State;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut render_area = area;
        for g in &self.garnish {
            g.before_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }

        self.inner.render_ref(render_area, buf, state);

        let mut render_area = area;
        for g in &self.garnish {
            g.after_render(render_area, buf);
            render_area = g.modify_area(render_area);
        }
    }
}

// WidgetModifier implementations for ratatui `Style` & `Padding`

impl WidgetModifier for Style {
    fn before_render(&self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, *self);
    }
}

impl WidgetModifier for Padding {
    fn modify_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x + self.left,
            y: area.y + self.top,
            width: area.width.saturating_sub(self.left + self.right),
            height: area.height.saturating_sub(self.top + self.bottom),
        }
    }
}
