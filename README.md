# ratatui-garnish

[![CI](https://github.com/franklaranja/ratatui-garnish/actions/workflows/ci.yml/badge.svg)](https://github.com/franklaranja/ratatui-garnish/actions/workflows/ci.yml)
[![Maintenance](https://img.shields.io/badge/maintenance-actively%20developed-brightgreen.svg)](https://github.com/franklaranja/ratatui-garnish)
[![GitHub Issues](https://img.shields.io/github/issues/franklaranja/ratatui-garnish)](https://github.com/franklaranja/ratatui-garnish/issues)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<div align="center">

*Garnish your Ratatui Widgets*

</div>

Ratatui Garnish provides a composable way to modify any widget,
including borders, titles, padding and styling. You can layer multiple garnishes
on any widget without modifying the widget itself. The garnishes and the order they
applied in can be changed at runtime whilst maintaining type safety (no trait objects).
a `GarnisedWidget` acts like the original widget with a `Vec` of `Garnishes` spliced in.

Don't want to add all that boiler plate for `Style` and `Border` to your widgets?
Use ratatui-garnish, it works with any widget that implements `Widget` or `WidgetRef`

Do you wish `Block` has a margin? Just garnish your widget with `Padding` before you
add a border. You want to have multiple borders? Just add them!

## Quick Start

```rust
use ratatui::{text::Text, widgets::Padding};
use ratatui::style::{Color, Style};
use ratatui_garnish::{RoundedBorder, Title, Above, GarnishableWidgetRef, GarnishableWidget};

// Create a paragraph with multiple decorations
let widget = Text::raw("Hello, World!\nTasty TUIs from Ratatui")
    .garnish(RoundedBorder::default())           // Add a rounded border
    .garnish(Title::<Above>::raw("My App"))      // Add a title above
    .garnish(Style::default().bg(Color::Blue))   // Add a background color
    .garnish(Padding::uniform(1));               // Add padding inside

// The garnishes will be applied in order when rendering
```
## Core Concepts

### Garnishes

A **garnish** can be applied to any widget. Garnishes can:
- **Transform the rendering area** (e.g., padding reduces available space)
- **Render before the widget** (e.g., backgrounds, borders)  
- **Render after the widget** (e.g., overlays, titles on top of content)

A Rust library for decorating `ratatui` widgets with visual garnishes like borders, titles, padding, styles, and shadows.

`tui-garnish` enhances `ratatui` widgets by providing a flexible, trait-based system for applying decorators (garnishes) that modify appearance or layout. Garnishes can add borders, titles, shadows, or adjust padding without altering the underlying widget's logic. The library supports chaining multiple garnishes and is extensible for custom decorations.

## Features
- **Garnishes**: Add borders (`PlainBorder`, `RoundedBorder`, etc.), titles (`Title<Top>`, `Title<Bottom>`), shadows (`SolidShadow`, `LightShadow`), padding, or styles.
- **Chaining**: Apply multiple garnishes in a specific order using a builder-like pattern.
- **Seamless Integration**: Works with any `ratatui` widget implementing `Widget` or `WidgetRef`.
- **Type-Safe**: Leverages Rust's type system, no type erasure.

## Installation
Add `tui-garnish` to your `Cargo.toml`:

```toml
[dependencies]
tui-garnish = "0.1.0"
ratatui = "0.29" # Required for widget rendering
```


## Available Garnishes
- **Borders**: `PlainBorder`, `RoundedBorder`, `DoubleBorder`, `ThickBorder`, `QuadrantInsideBorder`, `QuadrantOutsideBorder`, `FatInsideBorder`, `FatOutsideBorder`, `CharBorder`, `CustomBorder`.
- **Titles**: `Title<Top>`, `Title<Bottom>`, `Title<Above>`, `Title<Below>` with customizable alignment, style, and margins.
- **Padding**: `ratatui::widgets::Padding` for adjusting widget margins.
- **Style**: `ratatui::style::Style` for applying colors and modifiers.

## Contributing
Contributions are welcome!

## License
`ratatui-garnish` is licensed under the MIT License. See [LICENSE](LICENSE) for details.
```

