# ratatui-garnish

[![Crates.io](https://img.shields.io/crates/v/ratatui-garnish.svg)](https://crates.io/crates/ratatui-garnish)
[![Docs.rs](https://docs.rs/ratatui-garnish/badge.svg)](https://docs.rs/ratatui-garnish)
[![GitHub Issues](https://img.shields.io/github/issues/franklaranja/ratatui-garnish)](https://github.com/franklaranja/ratatui-garnish/issues)
[![CI](https://github.com/franklaranja/ratatui-garnish/actions/workflows/ci.yml/badge.svg)](https://github.com/franklaranja/ratatui-garnish/actions/workflows/ci.yml)
[![Maintenance](https://img.shields.io/badge/maintenance-actively%20developed-brightgreen.svg)](https://github.com/franklaranja/ratatui-garnish)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<div align="center">

*Garnish your Ratatui Widgets*

</div>

`ratatui-garnish` is a powerful composition system for [Ratatui](https://ratatui.rs) widgets,
allowing you to change de rendering of any widget with garnishes like borders, titles, shadows,
padding, and styles in a flexible, type-safe way. Layer multiple
garnishes in any order, apply them at runtime, and modify them
without altering the underlying widget. With zero-cost abstractions
and no trait objects, `ratatui-garnish` ensures performance
and type safety.

Want a margin outside a border? Add `Padding` before a border. Need
multiple titles or a shadow effect? Just garnish! Writing custom
widgets but dreading the boilerplate for styling or borders?
Use `ratatui-garnish` with any widget implementing
`Widget` or `StatefulWidget`.

## Features

- **Composable Modifications**: Layer borders, titles, shadows, padding,
  and styles in any order.
- **Type-Safe**: No trait objects or dynamic dispatch, preserving
  Rust's type system.
- **Zero-Cost Abstractions**: Efficient rendering with no overhead
  for unused garnishes.
- **Flexible**: Apply or modify garnishes at runtime.
- **Seamless Integration**: Works with any Ratatui widget.

## Installation

Add `ratatui-garnish` to your `Cargo.toml`:

```toml
[dependencies]
ratatui-garnish = "0.1.0"
ratatui = "0.29"
```

## Usage

Import the `GarnishableWidget` trait to add the `garnish` method to
any Ratatui widget:

```rust
use ratatui::{style::{Color, Style}, text::Text, widgets::Padding};
use ratatui_garnish::{border::RoundedBorder, title::{Title, Above}, GarnishableWidget};

// Create a text widget with multiple decorations
let widget = Text::raw("Hello, World!\nTasty TUIs from Ratatui")
    .garnish(RoundedBorder::default())           // Add a rounded border
    .garnish(Title::<Above>::raw("My App"))      // Add a title above
    .garnish(Style::default().bg(Color::Blue))   // Set a background color
    .garnish(Padding::uniform(1));               // Add padding inside
```

Garnishes are applied in the order they are added, with `before_render`
effects (e.g., styles, shadows) applied first, followed by the widget,
and then `after_render` effects (e.g., titles over borders).

### Complex Example

Combine multiple garnishes for a polished UI:

```rust
use ratatui::{style::{Color, Style, Modifier}, text::Line, widgets::Padding};
use ratatui_garnish::{border::DoubleBorder, shadow::Shadow, title::{Title, Top, Bottom}, GarnishableWidget};

let widget = Line::raw("Important Message")
    .garnish(Padding::uniform(2))                       // Margin outside
    .garnish(Style::default().bg(Color::DarkGray))      // Background color
    .garnish(Title::<Top>::styled("⚠ WARNING ⚠",        // Styled title
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)).margin(1))
    .garnish(Title::<Bottom>::raw("Status: Active").right_aligned().margin(1))
    .garnish(DoubleBorder::default());                   // Double border
```


### Reusing Garnishes

Use the `Garnishes` vec to apply the same garnishes to multiple widgets:

```rust
use ratatui::{style::{Color, Style, Modifier}, text::Line, widgets::Padding};
use ratatui_garnish::{border::DoubleBorder, title::{Title, Top}, garnishes, GarnishableWidget};

let garnishes = garnishes![
    Style::default().fg(Color::Blue),
    DoubleBorder::default(),
    Padding::uniform(2),
    Style::default().fg(Color::White),
];

let mut widget1 = Line::raw("First Widget")
    .garnish(Title::<Top>::styled("First", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)).margin(1));
widget1.extend_from_slice(&garnishes);

let mut widget2 = Line::raw("Second Widget")
    .garnish(Title::<Top>::styled("Second", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)).margin(1));
widget2.extend(garnishes);
```

### Accessing Garnishes

Treat a `GarnishedWidget` like a `Vec` to inspect or modify its garnishes:

```rust
let widget = Line::raw("Test")
    .garnish(Style::default().bg(Color::Blue))
    .garnish(Padding::uniform(1));
assert!(widget[0].is_style());
assert_eq!(widget.first_padding(), Some(&Padding::uniform(1)));
```

## Available Garnishes

### Borders
- **Standard**: `PlainBorder`, `RoundedBorder`, `DoubleBorder`, `ThickBorder`
- **Dashed Variants**: `DashedBorder`, `RoundedDashedBorder`, `ThickDashedBorder`,
- **Custom**: `CharBorder` (single character, e.g., `****`), `CustomBorder` (custom character set)
- **Specialty**: `QuadrantInsideBorder`, `QuadrantOutsideBorder`, `FatInsideBorder`, `FatOutsideBorder`

### Titles
- **Horizontal**: `Title<Top>` (over top border), `Title<Bottom>` (over bottom border),
  `Title<Above>` (reserves space above), `Title<Below>` (reserves space below)
- **Vertical**: `Title<Left>` (over left border), `Title<Right>` (over right border),
  `Title<Before>` (reserves space left), `Title<After>` (reserves space right)

### Shadows
- `Shadow`: Light (`░`), medium (`▒`), dark (`▓`), or full (`█`) shades with full-character offsets
- `HalfShadow`: Full (`█`) or quadrant characters (e.g., `▗`) with half-character offsets

### Built-in Ratatui Support
- `Style`: Background colors, text styling
- `Padding`: Spacing around the widget

## Performance

- Garnishes are applied in order, allowing precise control over rendering.
- Area modifications are accumulated efficiently.
- Zero-cost abstractions ensure no runtime overhead for unused garnishes.
- No dynamic dispatch or type erasure, preserving type safety and performance.

## Compatibility

`ratatui-garnish` integrates seamlessly with Ratatui widgets implementing
`Widget` or `StatefulWidget` following Ratatui's conventions.

## Contributing

This is the first release of `ratatui-garnish`, and more garnishes are
planned! Contributions are welcome.

## License

Licensed under the [MIT License](LICENSE). See the [LICENSE](https://github.com/username/ratatui-garnish/blob/main/LICENSE) file for details.

## Acknowledgements

Built with love for the Rust and Ratatui communities. Inspired by the
need for flexible, reusable widget modification.
