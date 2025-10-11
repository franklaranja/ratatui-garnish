//! This benchmark compares the rendering of a `GarnishedWidget`
//! to a `DecoratedWidget` with 1 to 10 garnishes.
//!
//! Run with `--features=decorated_widget` to enable
//! the `DecoratedWidget` (traditional decorater).
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::Widget,
};

use ratatui_garnish::{GarnishableWidget, GarnishedWidget, garnishes};

#[cfg(feature = "decorated_widget")]
use ratatui_garnish::DecoratedWidget;

#[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut buffer = black_box(Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: 80,
        height: 50,
    }));
    let mut group = c.benchmark_group("Compare decorators 1-10");

    group.throughput(Throughput::Elements(1));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 1), |b| {
        b.iter(|| {
            let widget = trad_01();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 1), |b| {
        b.iter(|| {
            let widget = flat_01();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(2));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 2), |b| {
        b.iter(|| {
            let widget = trad_02();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 2), |b| {
        b.iter(|| {
            let widget = flat_02();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(3));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 3), |b| {
        b.iter(|| {
            let widget = trad_03();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 3), |b| {
        b.iter(|| {
            let widget = flat_03();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(4));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 4), |b| {
        b.iter(|| {
            let widget = trad_04();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 4), |b| {
        b.iter(|| {
            let widget = flat_04();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(5));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 5), |b| {
        b.iter(|| {
            let widget = trad_05();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 5), |b| {
        b.iter(|| {
            let widget = flat_05();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(6));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 6), |b| {
        b.iter(|| {
            let widget = trad_06();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 6), |b| {
        b.iter(|| {
            let widget = flat_06();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(7));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 7), |b| {
        b.iter(|| {
            let widget = trad_07();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 7), |b| {
        b.iter(|| {
            let widget = flat_07();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(8));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 8), |b| {
        b.iter(|| {
            let widget = trad_08();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 8), |b| {
        b.iter(|| {
            let widget = flat_08();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(9));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 9), |b| {
        b.iter(|| {
            let widget = trad_09();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 9), |b| {
        b.iter(|| {
            let widget = flat_09();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.throughput(Throughput::Elements(10));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 10), |b| {
        b.iter(|| {
            let widget = trad_10();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });
    group.bench_function(BenchmarkId::new("Flat", 10), |b| {
        b.iter(|| {
            let widget = flat_10();
            buffer.reset();
            widget.render(*buffer.area(), &mut buffer);
        });
    });

    group.finish();
}

fn flat_01() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![Style::default().bg(Color::White),]))
}

fn flat_02() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
    ]))
}

fn flat_03() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
    ]))
}

fn flat_04() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
    ]))
}

fn flat_05() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
    ]))
}

fn flat_06() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
    ]))
}

fn flat_07() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
    ]))
}

fn flat_08() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
        Style::default().bg(Color::Cyan),
    ]))
}

fn flat_09() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
        Style::default().bg(Color::Cyan),
        Style::default().bg(Color::LightGreen),
    ]))
}

fn flat_10() -> GarnishedWidget<'static, Text<'static>> {
    black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
        Style::default().bg(Color::Cyan),
        Style::default().bg(Color::LightGreen),
        Style::default().bg(Color::LightRed),
    ]))
}

#[cfg(feature = "decorated_widget")]
fn trad_01() -> DecoratedWidget<Text<'static>, Style> {
    black_box(Text::raw("Hello World!").decorate(Style::default().bg(Color::White)))
}

#[cfg(feature = "decorated_widget")]
fn trad_02() -> DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_03() -> DecoratedWidget<DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>, Style>
{
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_04() -> DecoratedWidget<
    DecoratedWidget<DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>, Style>,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_05() -> DecoratedWidget<
    DecoratedWidget<
        DecoratedWidget<DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>, Style>,
        Style,
    >,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_06() -> DecoratedWidget<
    DecoratedWidget<
        DecoratedWidget<
            DecoratedWidget<DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>, Style>,
            Style,
        >,
        Style,
    >,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_07() -> DecoratedWidget<
    DecoratedWidget<
        DecoratedWidget<
            DecoratedWidget<
                DecoratedWidget<
                    DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>,
                    Style,
                >,
                Style,
            >,
            Style,
        >,
        Style,
    >,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red))
            .decorate(Style::default().bg(Color::Yellow)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_08() -> DecoratedWidget<
    DecoratedWidget<
        DecoratedWidget<
            DecoratedWidget<
                DecoratedWidget<
                    DecoratedWidget<
                        DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>,
                        Style,
                    >,
                    Style,
                >,
                Style,
            >,
            Style,
        >,
        Style,
    >,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red))
            .decorate(Style::default().bg(Color::Yellow))
            .decorate(Style::default().bg(Color::Cyan)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_09() -> DecoratedWidget<
    DecoratedWidget<
        DecoratedWidget<
            DecoratedWidget<
                DecoratedWidget<
                    DecoratedWidget<
                        DecoratedWidget<
                            DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>,
                            Style,
                        >,
                        Style,
                    >,
                    Style,
                >,
                Style,
            >,
            Style,
        >,
        Style,
    >,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red))
            .decorate(Style::default().bg(Color::Yellow))
            .decorate(Style::default().bg(Color::Cyan))
            .decorate(Style::default().bg(Color::LightGreen)),
    )
}

#[cfg(feature = "decorated_widget")]
fn trad_10() -> DecoratedWidget<
    DecoratedWidget<
        DecoratedWidget<
            DecoratedWidget<
                DecoratedWidget<
                    DecoratedWidget<
                        DecoratedWidget<
                            DecoratedWidget<
                                DecoratedWidget<DecoratedWidget<Text<'static>, Style>, Style>,
                                Style,
                            >,
                            Style,
                        >,
                        Style,
                    >,
                    Style,
                >,
                Style,
            >,
            Style,
        >,
        Style,
    >,
    Style,
> {
    black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red))
            .decorate(Style::default().bg(Color::Yellow))
            .decorate(Style::default().bg(Color::Cyan))
            .decorate(Style::default().bg(Color::LightGreen))
            .decorate(Style::default().bg(Color::LightRed)),
    )
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
