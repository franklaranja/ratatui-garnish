//! This benchmark compares the rendering of a `Paragraph`
//! to a `GarnishedWidget` and `DecoratedWidget` with similar
//! garnishes. Run with `--features=decorated_widget` to enable
//! the `DecoratedWidget` (traditional decorater).

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Padding as RatPadding, Paragraph, Widget},
};

use ratatui_garnish::{
    GarnishableWidget, Padding,
    border::PlainBorder,
    garnishes,
    title::{Title, Top},
};

#[allow(clippy::missing_panics_doc)]
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut buffer = black_box(Buffer::empty(Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 20,
    }));
    let mut group = c.benchmark_group("Compare Composition Approaches");
    group.sample_size(500);
    group.measurement_time(std::time::Duration::from_secs(20));
    group.bench_function("Ratatui Way", |b| b.iter(|| ratatui_way(&mut buffer)));
    #[cfg(feature = "decorated_widget")]
    group.bench_function("Traditional Decorator", |b| {
        b.iter(|| traditional_decorator(&mut buffer));
    });
    group.bench_function("Flat Decorator", |b| b.iter(|| flat_decorator(&mut buffer)));
    group.finish();
}

fn ratatui_way(buffer: &mut Buffer) {
    let widget = black_box(
        Paragraph::new("Hello World!")
            .block(
                Block::bordered()
                    .title("Paragraph")
                    .padding(RatPadding::horizontal(2)),
            )
            .style(Style::default().fg(Color::Red).bg(Color::White)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

fn flat_decorator(buffer: &mut Buffer) {
    let widget = black_box(Text::raw("Hello World!").garnishes(garnishes![
        Style::default().fg(Color::Red).bg(Color::White),
        Title::<Top>::raw("Paragraph").margin(1),
        PlainBorder::default(),
        Padding::horizontal(2),
    ]));
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

#[cfg(feature = "decorated_widget")]
fn traditional_decorator(buffer: &mut Buffer) {
    let widget = black_box(
        Text::raw("Hello World!")
            .decorate(Style::default().fg(Color::Red).bg(Color::White))
            .decorate(Title::<Top>::raw("Paragraph").margin(1))
            .decorate(PlainBorder::default())
            .decorate(Padding::horizontal(2)),
    );
    buffer.reset();
    widget.render(*buffer.area(), buffer);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
