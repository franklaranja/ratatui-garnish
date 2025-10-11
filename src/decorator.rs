use derive_more::{Deref, DerefMut};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, StatefulWidgetRef, Widget, WidgetRef},
};

use crate::RenderModifier;

/// A widget that wraps another widget and a garnish.
///
/// This struct implements `Deref` and `DerefMut` to the inner widget,
/// allowing you to access the original widget's methods while adding
/// garnish functionality.
#[derive(Debug, Deref, DerefMut)]
pub struct DecoratedWidget<W, G> {
    #[deref]
    #[deref_mut]
    pub widget: W,
    pub garnish: G,
}

impl<W: Widget, G: RenderModifier> DecoratedWidget<W, G> {
    /// creates a new `DecoratedWidget`.
    ///
    /// # example
    ///
    /// ```rust
    /// use ratatui::{style::Style, text::Line};
    /// use ratatui_garnish::DecoratedWidget;
    ///
    /// let widget = DecoratedWidget::new(Line::raw("Test"), Style::default());
    /// ```
    pub const fn new(widget: W, garnish: G) -> Self {
        Self { widget, garnish }
    }

    /// add a garnish.
    ///
    /// # example
    ///
    /// ```rust
    /// use ratatui::{style::Style, text::Line};
    /// use ratatui_garnish::{border::PlainBorder, DecoratedWidget};
    ///
    /// let widget = DecoratedWidget::new(Line::raw("Test"), Style::default())
    ///     .decorate(PlainBorder::default());
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn decorate<R: RenderModifier>(self, garnish: R) -> DecoratedWidget<Self, R> {
        DecoratedWidget {
            widget: self,
            garnish,
        }
    }
}

impl<W: Widget, G: RenderModifier> Widget for DecoratedWidget<W, G> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.garnish.before_render(area, buf);
        self.widget.render(self.garnish.modify_area(area), buf);
        self.garnish.after_render(area, buf);
    }
}

impl<W: WidgetRef, G: RenderModifier> WidgetRef for DecoratedWidget<W, G> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.garnish.before_render(area, buf);
        self.widget.render_ref(self.garnish.modify_area(area), buf);
        self.garnish.after_render(area, buf);
    }
}

/// A stateful widget that wraps another stateful widget and a garnish.
///
/// This struct implements `Deref` and `DerefMut` to the inner widget,
/// allowing you to access the original widget's methods while adding
/// garnish functionality.
#[derive(Debug, Deref, DerefMut)]
pub struct DecoratedStatefulWidget<W, G> {
    #[deref]
    #[deref_mut]
    pub widget: W,
    pub garnish: G,
}

impl<W: StatefulWidget, G: RenderModifier> DecoratedStatefulWidget<W, G> {
    /// creates a new `DecoratedStatefullWidget`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::{style::Style, widgets::List, text::Line};
    /// use ratatui_garnish::DecoratedStatefulWidget;
    ///
    /// let widget = DecoratedStatefulWidget::new(List::new::<Vec<Line>>(vec![]), Style::default());
    /// ```
    pub const fn new(widget: W, garnish: G) -> Self {
        Self { widget, garnish }
    }

    /// add a garnish.
    ///
    /// # example
    ///
    /// ```rust
    /// use ratatui::{style::Style, widgets::List, text::Line};
    /// use ratatui_garnish::{border::PlainBorder, DecoratedStatefulWidget};
    ///
    /// let widget = DecoratedStatefulWidget::new(List::new::<Vec<Line>>(vec![]), Style::default())
    ///     .decorate(PlainBorder::default());
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original"]
    pub const fn decorate<R: RenderModifier>(self, garnish: R) -> DecoratedWidget<Self, R> {
        DecoratedWidget {
            widget: self,
            garnish,
        }
    }
}

impl<W, G> StatefulWidget for DecoratedStatefulWidget<W, G>
where
    W: StatefulWidget,
    G: RenderModifier,
{
    type State = W::State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.garnish.before_render(area, buf);
        self.widget
            .render(self.garnish.modify_area(area), buf, state);
        self.garnish.after_render(area, buf);
    }
}

impl<W, G> StatefulWidgetRef for DecoratedStatefulWidget<W, G>
where
    W: StatefulWidgetRef,
    G: RenderModifier,
{
    type State = W::State;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.garnish.before_render(area, buf);
        self.widget
            .render_ref(self.garnish.modify_area(area), buf, state);
        self.garnish.after_render(area, buf);
    }
}
