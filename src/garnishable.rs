use crate::{Garnish, GarnishedStatefulWidget, GarnishedWidget};
use ratatui::widgets::{StatefulWidget, StatefulWidgetRef, Widget, WidgetRef};

/// A trait for widgets that can be garnished with decorators.
pub trait GarnishableWidget: Widget + Sized {
    /// Applies a garnish to the widget, wrapping it in a `GarnishedWidget`.
    fn garnish<'a, G: Into<Garnish<'a>>>(self, garnish: G) -> GarnishedWidget<'a, Self> {
        GarnishedWidget::new(self, garnish)
    }
}

// Blanket implementation for all widgets that implement `Widget`.
impl<W: Widget> GarnishableWidget for W {}

/// A trait for widgets that can be garnished with decorators.
pub trait GarnishableWidgetRef: WidgetRef + Sized {
    /// Applies a garnish to the widget, wrapping it in a `GarnishedWidget`.
    fn garnish_ref<'a, G: Into<Garnish<'a>>>(self, garnish: G) -> GarnishedWidget<'a, Self> {
        GarnishedWidget::new(self, garnish)
    }
}

// Blanket implementation for all widgets that implement `WidgetRef`.
impl<W: WidgetRef> GarnishableWidgetRef for W {}

/// A trait for widgets that can be garnished with decorators.
pub trait GarnishableStatefulWidget: StatefulWidget + Sized {
    /// Applies a garnish to the widget, wrapping it in a `GarnishedWidget`.
    fn garnish<'a, G: Into<Garnish<'a>>>(self, garnish: G) -> GarnishedStatefulWidget<'a, Self> {
        GarnishedStatefulWidget::new(self, garnish)
    }
}

// Blanket implementation for all widgets that implement `StatefulWidget`.
impl<W: StatefulWidget> GarnishableStatefulWidget for W {}

/// A trait for widgets that can be garnished with decorators.
pub trait GarnishableStatefulWidgetRef: StatefulWidgetRef + Sized {
    /// Applies a garnish to the widget, wrapping it in a `GarnishedWidget`.
    fn garnish_ref<'a, G: Into<Garnish<'a>>>(
        self,
        garnish: G,
    ) -> GarnishedStatefulWidget<'a, Self> {
        GarnishedStatefulWidget::new(self, garnish)
    }
}

// Blanket implementation for all widgets that implement `WidgetRef` and `Clone`.
impl<W: StatefulWidgetRef> GarnishableStatefulWidgetRef for W {}
