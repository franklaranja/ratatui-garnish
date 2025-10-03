//! Border Garnishes
//!
//! A border is composed of a set of characters used to draw the border,
//! a `BorderSet` and a bitflags struct [`Borders`] (just like in `ratatui`) to
//! configure which borders to render. Borders don't have their own [`Style`](ratatui::style::Style);
//! set the style of your border with the `Style` garnish instead.
//!
//! # Example
//! ```rust
//!
//! use ratatui::{text::Text, style::{Color, Style}};
//! use ratatui_garnish::{ GarnishableWidget };
//! use ratatui_garnish::border::PlainBorder;
//! let widget = Text::raw("Hello, world")
//!     .garnish(Style::default().fg(Color::Blue))
//!     .garnish(PlainBorder::default()); // Blue plain border
//! ```
//!
//! # Standard Borders
//!
//! Standard borders are simple wrappers around [`Borders`]. The
//! `default` implementation returns a border with all borders enabled. Use
//! `new` to select specific borders.
//!
//! ## Example
//!
//! ```rust
//! # use ratatui::{text::Text, style::{Color, Style}};
//! # use ratatui_garnish::{ GarnishableWidget };
//! # use ratatui_garnish::border::PlainBorder;
//! use ratatui_garnish::border::Borders;
//!
//! let widget = Text::raw("Hello, world")
//!     .garnish(PlainBorder::new(Borders::TOP | Borders::BOTTOM));
//! ```
//!
//! Standard borders implement `Deref` and `DerefMut`, allowing
//! direct calls to `Borders` methods:
//!
//! ```rust
//! # use ratatui_garnish::border::{PlainBorder, Borders};
//! let mut border = PlainBorder::new(Borders::TOP);
//! border.insert(Borders::LEFT);
//! border.remove(Borders::TOP);
//! ```
//!
//! There is a standard border for every constructor of `BorderSet`
//! that doesn't need arguments: `PlainBorder`, `DashedBorder`,
//! `DoubleBorder`, `FatInsideBorder`, `FatOutsideBorder`, `QuadrantInsideBorder`,
//! `QuadrantOutsideBorder`, `RoundedBorder`, `RoundedDashedBorder`,
//! `ThickBorder` and `ThickDashedBorder`.
//!
//! # `CharBorder`
//!
//! For creating a border composed of a single character, use
//! `CharBorder`:
//!
//! ```rust
//! # use ratatui_garnish::border::{CharBorder, Borders};
//! let border = CharBorder::new('*')
//!     .borders(Borders::RIGHT);
//! ```
//!
//! # `CustomBorder`
//!
//! With `CustomBorder` you can use your own `BorderSet` to define
//! a border. You can start with one of the standard border constructors
//! and adapt it as needed.
//!
//! ```rust
//! use ratatui_garnish::border::{BorderSet, CustomBorder};
//!
//! let border = CustomBorder::new(BorderSet::plain().corners('*'));
//! ```

use derive_more::{Deref, DerefMut};
use ratatui::{buffer::Buffer, layout::Rect};

/// Trait for rendering borders around ratatui widgets.
///
/// This trait defines methods for retrieving the border configuration
///  ([`Borders`]) and character set ([`BorderSet`]), which are used
///  to render a border. The other methods are used byas well as rendering individual border segments
/// and corners. Implementors can customize rendering behavior if needed,
/// but the default implementations should suffice for most cases.
pub trait Border {
    /// Returns the character set used for rendering.
    fn get_border_set(&self) -> BorderSet;

    /// Returns the border configuration.
    fn get_borders(&self) -> Borders;

    /// Renders the left border.
    fn render_left(&self, area: Rect, buffer: &mut Buffer, symbol: char) {
        for y in area.top()..area.bottom() {
            buffer[(area.left(), y)].set_char(symbol);
        }
    }

    /// Renders the top border.
    fn render_top(&self, area: Rect, buffer: &mut Buffer, symbol: char) {
        for x in area.left()..area.right() {
            buffer[(x, area.top())].set_char(symbol);
        }
    }

    /// Renders the right border.
    fn render_right(&self, area: Rect, buffer: &mut Buffer, symbol: char) {
        let x = area.right().saturating_sub(1);
        for y in area.top()..area.bottom() {
            buffer[(x, y)].set_char(symbol);
        }
    }

    /// Renders the bottom border.
    fn render_bottom(&self, area: Rect, buffer: &mut Buffer, symbol: char) {
        let y = area.bottom().saturating_sub(1);
        for x in area.left()..area.right() {
            buffer[(x, y)].set_char(symbol);
        }
    }

    /// Renders the corner characters if adjacent sides are present.
    fn render_corners(&self, area: Rect, buffer: &mut Buffer, charset: &BorderSet) {
        let borders = self.get_borders();
        let (right, bottom) = (
            area.right().saturating_sub(1),
            area.bottom().saturating_sub(1),
        );

        if borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buffer[(right, bottom)].set_char(charset.bottom_right);
        }
        if borders.contains(Borders::RIGHT | Borders::TOP) {
            buffer[(right, area.top())].set_char(charset.top_right);
        }
        if borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buffer[(area.left(), bottom)].set_char(charset.bottom_left);
        }
        if borders.contains(Borders::LEFT | Borders::TOP) {
            buffer[(area.left(), area.top())].set_char(charset.top_left);
        }
    }
}

impl<T: Border> crate::RenderModifier for T {
    fn before_render(&self, area: Rect, buffer: &mut Buffer) {
        let borders = self.get_borders();
        let border_set = self.get_border_set();

        if borders.contains(Borders::LEFT) {
            self.render_left(area, buffer, border_set.left);
        }
        if borders.contains(Borders::TOP) {
            self.render_top(area, buffer, border_set.top);
        }
        if borders.contains(Borders::RIGHT) {
            self.render_right(area, buffer, border_set.right);
        }
        if borders.contains(Borders::BOTTOM) {
            self.render_bottom(area, buffer, border_set.bottom);
        }

        self.render_corners(area, buffer, &border_set);
    }

    fn modify_area(&self, area: Rect) -> Rect {
        let mut inner = area;
        let borders = self.get_borders();

        if borders.contains(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1).min(inner.right());
            inner.width = inner.width.saturating_sub(1);
        }
        if borders.contains(Borders::TOP) {
            inner.y = inner.y.saturating_add(1).min(inner.bottom());
            inner.height = inner.height.saturating_sub(1);
        }
        if borders.contains(Borders::RIGHT) {
            inner.width = inner.width.saturating_sub(1);
        }
        if borders.contains(Borders::BOTTOM) {
            inner.height = inner.height.saturating_sub(1);
        }
        inner
    }
}

// ===== Character Sets =====

/// Character set for rendering borders with different visual styles.
///
/// Provides various predefined styles and methods to customize individual characters.
///
/// # Examples
///
/// ```rust
/// use ratatui_garnish::border::BorderSet;
///
/// // Use a predefined set
/// let rounded = BorderSet::rounded();
///
/// // Customize a set
/// let custom = BorderSet::plain()
///     .corners('*')
///     .horizontals('=');
///
/// // Create from a single character
/// let simple = BorderSet::new('#');
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BorderSet {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub left: char,
    pub right: char,
    pub top: char,
    pub bottom: char,
}

impl BorderSet {
    /// Creates a new `BorderSet` with all characters set to the same symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_garnish::border::BorderSet;
    ///
    /// let charset = BorderSet::new('*');
    /// assert_eq!(charset.top_left, '*');
    /// assert_eq!(charset.bottom, '*');
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn new(symbol: char) -> Self {
        Self {
            top: symbol,
            bottom: symbol,
            left: symbol,
            right: symbol,
            top_left: symbol,
            top_right: symbol,
            bottom_left: symbol,
            bottom_right: symbol,
        }
    }

    /// Creates a plain border set.
    ///
    /// ```text
    /// ┌───────┐
    /// │       │
    /// └───────┘
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn plain() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            left: '│',
            right: '│',
            top: '─',
            bottom: '─',
        }
    }

    /// Creates a dashed border set.
    ///
    /// ```text
    /// ┌╌╌╌╌╌╌╌┐
    /// ┆       ┆
    /// └╌╌╌╌╌╌╌┘
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn dashed() -> Self {
        Self::plain().horizontals('╌').verticals('┊')
    }

    /// Creates a plain border set with rounded corners.
    ///
    /// ```text
    /// ╭───────╮
    /// │       │
    /// ╰───────╯
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn rounded() -> Self {
        Self {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            left: '│',
            right: '│',
            top: '─',
            bottom: '─',
        }
    }

    /// Creates a rounded dashed border set.
    ///
    /// ```text
    /// ╭╌╌╌╌╌╌╌╮
    /// ┆       ┆
    /// ╰╌╌╌╌╌╌╌╯
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn rounded_dashed() -> Self {
        Self::rounded().horizontals('╌').verticals('┊')
    }

    /// Creates a double border set.
    ///
    /// ```text
    /// ╔═══════╗
    /// ║       ║
    /// ╚═══════╝
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn double() -> Self {
        Self {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            left: '║',
            right: '║',
            top: '═',
            bottom: '═',
        }
    }

    /// Creates a thick border set.
    ///
    /// ```text
    /// ┏━━━━━━━┓
    /// ┃       ┃
    /// ┗━━━━━━━┛
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn thick() -> Self {
        Self {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            left: '┃',
            right: '┃',
            top: '━',
            bottom: '━',
        }
    }

    /// Creates a thick dashed border set.
    ///
    /// ```text
    /// ┏╍╍╍╍╍╍╍┓
    /// ┇       ┇
    /// ┗╍╍╍╍╍╍╍┛
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn thick_dashed() -> Self {
        Self::rounded().horizontals('╍').verticals('┋')
    }

    /// Create a quadrant inside border set.
    ///
    /// ```text
    /// ▗▄▄▄▄▄▄▄▖
    /// ▐       ▌
    /// ▐       ▌
    /// ▝▀▀▀▀▀▀▀▘
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn quadrant_inside() -> Self {
        Self {
            top_left: '▗',
            top_right: '▖',
            bottom_left: '▝',
            bottom_right: '▘',
            left: '▐',
            right: '▌',
            top: '▄',
            bottom: '▀',
        }
    }

    /// Create a quadrant outside border set.
    ///
    /// ```text
    /// ▛▀▀▀▀▀▀▀▜
    /// ▌       ▐
    /// ▌       ▐
    /// ▙▄▄▄▄▄▄▄▟
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn quadrant_outside() -> Self {
        Self {
            top_left: '▛',
            top_right: '▜',
            bottom_left: '▙',
            bottom_right: '▟',
            left: '▌',
            right: '▐',
            top: '▀',
            bottom: '▄',
        }
    }

    /// Create a fat inside border set.
    ///
    /// ```text
    /// ▄▄▄▄▄▄▄▄▄
    /// █       █
    /// █       █
    /// ▀▀▀▀▀▀▀▀▀
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn fat_inside() -> Self {
        Self {
            top_left: '▄',
            top_right: '▄',
            bottom_left: '▀',
            bottom_right: '▀',
            left: '█',
            right: '█',
            top: '▄',
            bottom: '▀',
        }
    }

    /// Create a fat outside border set.
    ///
    /// ```text
    /// █▀▀▀▀▀▀▀█
    /// █       █
    /// █       █
    /// █▄▄▄▄▄▄▄█
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub const fn fat_outside() -> Self {
        Self {
            top_left: '█',
            top_right: '█',
            bottom_left: '█',
            bottom_right: '█',
            left: '█',
            right: '█',
            top: '▀',
            bottom: '▄',
        }
    }

    /// Sets all corner characters to the specified symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::border::BorderSet;
    /// let border = BorderSet::plain().corners('*');
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn corners(mut self, symbol: char) -> Self {
        self.top_left = symbol;
        self.top_right = symbol;
        self.bottom_left = symbol;
        self.bottom_right = symbol;
        self
    }

    /// Sets vertical border characters (left and right) to the specified symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::border::BorderSet;
    /// let border = BorderSet::plain().verticals('|');
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn verticals(mut self, symbol: char) -> Self {
        self.left = symbol;
        self.right = symbol;
        self
    }

    /// Sets horizontal border characters (top and bottom) to the specified symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::border::BorderSet;
    /// let border = BorderSet::plain().horizontals('-');
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn horizontals(mut self, symbol: char) -> Self {
        self.top = symbol;
        self.bottom = symbol;
        self
    }

    /// Sets all side characters (non-corner) to the specified symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_garnish::border::BorderSet;
    /// let border = BorderSet::plain().sides('=');
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn sides(mut self, symbol: char) -> Self {
        self.top = symbol;
        self.bottom = symbol;
        self.left = symbol;
        self.right = symbol;
        self
    }
}

// ===== Border Implementations =====

/// Macro to generate standard border types with predefined character sets.
macro_rules! standard_border {
    ($name:ident, $border_set_fn:ident, $doc:literal) => {
        #[doc = $doc]
        ///
        /// This is a wrapper around `Borders` with `Deref` and `DerefMut` implementations
        /// for convenient access to the underlying `Borders` configuration.
        ///
        /// # Examples
        ///
        /// ```rust
        #[doc = concat!("use ratatui_garnish::border::{Borders, ", stringify!($name), "};")]
        ///
        #[doc = concat!("let border = ", stringify!($name), "::default();")]
        #[doc = concat!("let custom = ", stringify!($name), "::new(Borders::TOP | Borders::BOTTOM);")]
        /// ```
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deref, DerefMut)]
        pub struct $name(
            pub Borders
        );

        impl $name {
            /// Creates a new border with the specified configuration.
            #[must_use = "constructor returns a new instance"]
            pub const fn new(borders: Borders) -> Self {
                Self(borders)
            }
        }

        impl Default for $name {
            /// Creates a border with all sides enabled.
            fn default() -> Self {
                Self(Borders::ALL)
            }
        }

        impl Border for $name {
            fn get_borders(&self) -> Borders {
                self.0
            }

            fn get_border_set(&self) -> BorderSet {
                BorderSet::$border_set_fn()
            }
        }
    };
}

standard_border!(
    PlainBorder,
    plain,
    "A plain border with standard box-drawing characters."
);
standard_border!(DashedBorder, dashed, "A dashed border.");
standard_border!(
    RoundedDashedBorder,
    rounded_dashed,
    "A dashed border with rounded corners."
);
standard_border!(ThickDashedBorder, thick_dashed, "A thick dashed border.");
standard_border!(RoundedBorder, rounded, "A border with rounded corners.");
standard_border!(DoubleBorder, double, "A double-line border.");
standard_border!(ThickBorder, thick, "A thick border.");
standard_border!(
    QuadrantInsideBorder,
    quadrant_inside,
    "A quadrant-style inside border."
);
standard_border!(
    QuadrantOutsideBorder,
    quadrant_outside,
    "A quadrant-style outside border."
);
standard_border!(FatInsideBorder, fat_inside, "A fat inside border.");
standard_border!(FatOutsideBorder, fat_outside, "A fat outside border.");

/// A border rendered with a single character for all sides.
///
/// # Examples
///
/// ```rust
/// use ratatui_garnish::border::{Borders, CharBorder};
///
/// let star_border = CharBorder::new('*'); // All sides with '*'
/// let hash_border = CharBorder::new('#').borders(Borders::TOP | Borders::BOTTOM);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CharBorder {
    pub symbol: char,
    pub borders: Borders,
}

impl Default for CharBorder {
    /// Creates a border with all sides enabled (`Borders::ALL`) and space as the symbol.
    fn default() -> Self {
        Self {
            symbol: ' ',
            borders: Borders::ALL,
        }
    }
}

impl CharBorder {
    /// Creates a new border using the specified symbol for all sides.
    ///
    /// By default, uses `Borders::ALL`.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_garnish::border::CharBorder;
    ///
    /// let border = CharBorder::new('*');
    /// ```
    #[must_use = "constructor returns a new instance"]
    pub fn new(symbol: char) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }

    /// Sets which borders to render.
    ///
    /// # Example
    ///
    /// ```
    /// use ratatui_garnish::border::{CharBorder, Border, Borders};
    ///
    /// let border = CharBorder::new('*').borders(Borders::TOP);
    /// assert_eq!(border.get_borders(), Borders::TOP);
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }
}

impl Border for CharBorder {
    fn get_borders(&self) -> Borders {
        self.borders
    }

    fn get_border_set(&self) -> BorderSet {
        BorderSet::new(self.symbol)
    }
}

/// A border with a fully customizable character set.
///
/// Provides maximum flexibility for creating unique border styles by allowing
/// specification of individual characters for each border element.
///
/// # Example
///
/// ```rust
/// use ratatui_garnish::border::{Borders, BorderSet, CustomBorder};
///
/// let custom_set = BorderSet::plain().corners('*').horizontals('=');
/// let border = CustomBorder::new(custom_set).borders(Borders::ALL);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CustomBorder {
    /// The custom character set used to draw the border.
    pub char_set: BorderSet,
    /// Which borders to draw, defaults to `Borders::ALL`.
    pub borders: Borders,
}

impl Default for CustomBorder {
    /// Creates a plain border with all sides enabled (`Borders::ALL`).
    fn default() -> Self {
        Self {
            char_set: BorderSet::plain(),
            borders: Borders::ALL,
        }
    }
}

impl CustomBorder {
    /// Creates a new custom border with the specified character set.
    ///
    /// By default, uses `Borders::ALL`.
    #[must_use = "constructor returns a new instance"]
    pub fn new(char_set: BorderSet) -> Self {
        Self {
            char_set,
            ..Default::default()
        }
    }

    /// Sets which borders to render.
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }
}

impl Border for CustomBorder {
    fn get_borders(&self) -> Borders {
        self.borders
    }

    fn get_border_set(&self) -> BorderSet {
        self.char_set
    }
}

bitflags::bitflags! {
    /// Bitflags that can be composed to set the visible borders essentially on any border garnish.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct Borders: u8 {
        const NONE   = 0b0000;
        const TOP    = 0b0001;
        const RIGHT  = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT   = 0b1000;
        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RenderModifier;

    fn create_test_buffer(width: u16, height: u16) -> Buffer {
        Buffer::empty(Rect::new(0, 0, width, height))
    }

    #[test]
    fn border_set_new_creates_uniform_set() {
        let border_set = BorderSet::new('*');

        assert_eq!(border_set.top_left, '*');
        assert_eq!(border_set.top_right, '*');
        assert_eq!(border_set.bottom_left, '*');
        assert_eq!(border_set.bottom_right, '*');
        assert_eq!(border_set.left, '*');
        assert_eq!(border_set.right, '*');
        assert_eq!(border_set.top, '*');
        assert_eq!(border_set.bottom, '*');
    }

    #[test]
    fn border_set_modifiers_work_correctly() {
        let border_set = BorderSet::plain()
            .corners('*')
            .verticals('|')
            .horizontals('-');

        assert_eq!(border_set.top_left, '*');
        assert_eq!(border_set.top_right, '*');
        assert_eq!(border_set.bottom_left, '*');
        assert_eq!(border_set.bottom_right, '*');
        assert_eq!(border_set.left, '|');
        assert_eq!(border_set.right, '|');
        assert_eq!(border_set.top, '-');
        assert_eq!(border_set.bottom, '-');
    }

    #[test]
    fn char_border_creates_correct_border() {
        let border = CharBorder::new('*');
        assert_eq!(border.symbol, '*');
        assert_eq!(border.borders, Borders::ALL);

        let partial_border = CharBorder::new('#').borders(Borders::TOP | Borders::BOTTOM);
        assert_eq!(partial_border.symbol, '#');
        assert_eq!(partial_border.borders, Borders::TOP | Borders::BOTTOM);
    }

    #[test]
    fn standard_borders_have_correct_defaults() {
        assert_eq!(PlainBorder::default().get_border_set(), BorderSet::plain());
        assert_eq!(PlainBorder::default().get_borders(), Borders::ALL);
        assert_eq!(
            RoundedBorder::default().get_border_set(),
            BorderSet::rounded()
        );
        assert_eq!(RoundedBorder::default().get_borders(), Borders::ALL);
        assert_eq!(
            DoubleBorder::default().get_border_set(),
            BorderSet::double()
        );
        assert_eq!(DoubleBorder::default().get_borders(), Borders::ALL);
        assert_eq!(ThickBorder::default().get_border_set(), BorderSet::thick());
        assert_eq!(ThickBorder::default().get_borders(), Borders::ALL);
    }

    #[test]
    fn complete_border_renders_correctly() {
        let mut buffer = create_test_buffer(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let border = PlainBorder::default();

        border.before_render(area, &mut buffer);

        // Verify corners
        assert_eq!(buffer[(0, 0)].symbol(), "┌");
        assert_eq!(buffer[(4, 0)].symbol(), "┐");
        assert_eq!(buffer[(0, 4)].symbol(), "└");
        assert_eq!(buffer[(4, 4)].symbol(), "┘");

        // Verify sides
        assert_eq!(buffer[(0, 1)].symbol(), "│"); // left
        assert_eq!(buffer[(4, 1)].symbol(), "│"); // right
        assert_eq!(buffer[(1, 0)].symbol(), "─"); // top
        assert_eq!(buffer[(1, 4)].symbol(), "─"); // bottom
    }

    #[test]
    fn partial_border_renders_only_specified_sides() {
        let mut buffer = create_test_buffer(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let border = PlainBorder::new(Borders::TOP | Borders::LEFT);

        border.before_render(area, &mut buffer);

        // Should render top-left corner and associated sides
        assert_eq!(buffer[(0, 0)].symbol(), "┌");
        assert_eq!(buffer[(0, 1)].symbol(), "│");

        // Should not render other borders
        assert_eq!(buffer[(0, 4)].symbol(), "│");
        assert_eq!(buffer[(4, 0)].symbol(), "─");
        assert_eq!(buffer[(4, 1)].symbol(), " ");
        assert_eq!(buffer[(1, 4)].symbol(), " ");
    }

    #[test]
    fn area_modification_accounts_for_all_borders() {
        let area = Rect::new(0, 0, 10, 10);
        let border = PlainBorder::default();

        let inner_area = border.modify_area(area);

        assert_eq!(inner_area.x, 1);
        assert_eq!(inner_area.y, 1);
        assert_eq!(inner_area.width, 8);
        assert_eq!(inner_area.height, 8);
    }

    #[test]
    fn area_modification_accounts_for_partial_borders() {
        let area = Rect::new(0, 0, 10, 10);
        let border = PlainBorder::new(Borders::TOP | Borders::LEFT);

        let inner_area = border.modify_area(area);

        assert_eq!(inner_area.x, 1); // Left border reduces x
        assert_eq!(inner_area.y, 1); // Top border reduces y
        assert_eq!(inner_area.width, 9); // Only left border reduces width
        assert_eq!(inner_area.height, 9); // Only top border reduces height
    }

    #[test]
    fn border_deref_provides_border_access() {
        let mut border = PlainBorder::new(Borders::TOP);
        assert_eq!(*border, Borders::TOP);

        // Test DerefMut
        *border |= Borders::LEFT;
        assert_eq!(border.get_borders(), Borders::TOP | Borders::LEFT);
    }

    #[test]
    fn custom_border_works_with_modified_border_set() {
        let custom_border_set = BorderSet::plain().corners('*').horizontals('=');
        let custom_border =
            CustomBorder::new(custom_border_set).borders(Borders::TOP | Borders::LEFT);

        assert_eq!(custom_border.get_borders(), Borders::TOP | Borders::LEFT);

        let border_set = custom_border.get_border_set();
        assert_eq!(border_set.top_left, '*');
        assert_eq!(border_set.top, '=');
        assert_eq!(border_set.left, '│'); // Unchanged from plain()
    }

    #[test]
    fn edge_case_single_cell_area() {
        let mut buffer = create_test_buffer(1, 1);
        let area = Rect::new(0, 0, 1, 1);
        let border = PlainBorder::default();

        border.before_render(area, &mut buffer);

        // In a 1x1 area, only the top-left corner should be rendered
        // since all borders converge to the same cell
        assert_eq!(buffer[(0, 0)].symbol(), "┌");
    }

    #[test]
    fn zero_area_handling() {
        let area = Rect::new(0, 0, 0, 0);
        let border = PlainBorder::default();

        let inner_area = border.modify_area(area);

        // Should handle zero-sized areas gracefully
        assert_eq!(inner_area.width, 0);
        assert_eq!(inner_area.height, 0);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn plain_border_serialization() {
        let border = PlainBorder::default();
        let json = serde_json::to_string_pretty(&border).unwrap();

        let restored: PlainBorder = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, border);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn char_border_serialization() {
        let border = CharBorder::new('*');
        let json = serde_json::to_string_pretty(&border).unwrap();

        let restored: CharBorder = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, border);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn custom_border_serialization() {
        let custom_border_set = BorderSet::plain().corners('*').horizontals('=');
        let border = CustomBorder::new(custom_border_set).borders(Borders::TOP | Borders::LEFT);
        let json = serde_json::to_string_pretty(&border).unwrap();

        let restored: CustomBorder = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, border);
    }
}
