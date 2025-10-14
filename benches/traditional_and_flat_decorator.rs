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

use ratatui_garnish::{GarnishableWidget, garnishes};

#[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut buffer = black_box(Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 20,
    }));
    // let mut group = c.benchmark_group("Traditional vs flat decorator 1-10");
    let mut group = c.benchmark_group("count10");
    group.sample_size(500);
    group.measurement_time(std::time::Duration::from_secs(20));
    group.throughput(Throughput::Elements(1));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 1), |b| {
        b.iter(|| trad_01(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 1), |b| {
        b.iter(|| flat_01(&mut buffer));
    });

    group.throughput(Throughput::Elements(2));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 2), |b| {
        b.iter(|| trad_02(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 2), |b| {
        b.iter(|| flat_02(&mut buffer));
    });

    group.throughput(Throughput::Elements(3));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 3), |b| {
        b.iter(|| trad_03(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 3), |b| {
        b.iter(|| flat_03(&mut buffer));
    });

    group.throughput(Throughput::Elements(4));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 4), |b| {
        b.iter(|| trad_04(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 4), |b| {
        b.iter(|| flat_04(&mut buffer));
    });

    group.throughput(Throughput::Elements(5));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 5), |b| {
        b.iter(|| trad_05(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 5), |b| {
        b.iter(|| flat_05(&mut buffer));
    });

    group.throughput(Throughput::Elements(6));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 6), |b| {
        b.iter(|| trad_06(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 6), |b| {
        b.iter(|| flat_06(&mut buffer));
    });

    group.throughput(Throughput::Elements(7));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 7), |b| {
        b.iter(|| trad_07(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 7), |b| {
        b.iter(|| flat_07(&mut buffer));
    });

    group.throughput(Throughput::Elements(8));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 8), |b| {
        b.iter(|| trad_08(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 8), |b| {
        b.iter(|| flat_08(&mut buffer));
    });

    group.throughput(Throughput::Elements(9));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 9), |b| {
        b.iter(|| trad_09(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 9), |b| {
        b.iter(|| flat_09(&mut buffer));
    });

    group.throughput(Throughput::Elements(10));
    #[cfg(feature = "decorated_widget")]
    group.bench_function(BenchmarkId::new("Traditional", 10), |b| {
        b.iter(|| trad_10(&mut buffer));
    });
    group.bench_function(BenchmarkId::new("Flat", 10), |b| {
        b.iter(|| flat_10(&mut buffer));
    });

    group.finish();
}

fn flat_01(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!").garnishes(garnishes![Style::default().bg(Color::White),]),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_02(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_03(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_04(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_05(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_06(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_07(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_08(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
        Style::default().bg(Color::Cyan),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_09(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().bg(Color::White),
        Style::default().bg(Color::Black),
        Style::default().bg(Color::Gray),
        Style::default().bg(Color::Green),
        Style::default().bg(Color::Blue),
        Style::default().bg(Color::Red),
        Style::default().bg(Color::Yellow),
        Style::default().bg(Color::Cyan),
        Style::default().bg(Color::LightGreen),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_10(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
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
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_01(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").decorate(Style::default().bg(Color::White)));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_02(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_03(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_04(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_05(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_06(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_07(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red))
            .decorate(Style::default().bg(Color::Yellow)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_08(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().bg(Color::White))
            .decorate(Style::default().bg(Color::Black))
            .decorate(Style::default().bg(Color::Gray))
            .decorate(Style::default().bg(Color::Green))
            .decorate(Style::default().bg(Color::Blue))
            .decorate(Style::default().bg(Color::Red))
            .decorate(Style::default().bg(Color::Yellow))
            .decorate(Style::default().bg(Color::Cyan)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_09(buffer: &mut Buffer) {
    let widget = black_box(
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
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn trad_10(buffer: &mut Buffer) {
    let widget = black_box(
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
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
