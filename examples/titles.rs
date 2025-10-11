use crossterm::event::{self, Event};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
};

use ratatui_garnish::{
    GarnishableWidget, Padding,
    border::RoundedBorder,
    garnishes,
    title::{Above, After, Before, Below, Bottom, Left, Right, Title, Top},
};

use palette::{
    BLUE200, BLUE400, BLUE500, BLUE900, GREEN200, GREEN500, ORANGE50, ORANGE200, ORANGE400,
    ORANGE500, PURPLE200, PURPLE500,
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    use Constraint::{Length, Min};
    let screen_area = frame.area();
    let buffer = frame.buffer_mut();
    buffer.set_style(screen_area, Style::default().bg(BLUE900).fg(ORANGE50));

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, help_area] = vertical.areas(frame.area());

    // render title & help
    frame.render_widget(title("ratatui-garnish  Titles Demo"), title_area);
    frame.render_widget(help(), help_area);

    let widget = Line::styled("The compiler is your friend", Style::default().fg(BLUE400))
        .centered()
        .garnishes(garnishes![
            Title::<Above>::styled("▲", Style::default().fg(ORANGE500)).centered(),
            Title::<Below>::styled("▼", Style::default().fg(BLUE500)).centered(),
            Title::<Before>::styled("◀", Style::default().fg(PURPLE500)).centered(),
            Title::<After>::styled("▶", Style::default().fg(GREEN500)).centered(),
            Padding::horizontal(1),
            Title::<Top>::styled(" top ", Style::default().fg(ORANGE200)).centered(),
            Title::<Bottom>::styled(" bottom ", Style::default().fg(BLUE200)).centered(),
            Title::<Left>::styled(" left ", Style::default().fg(PURPLE200)).centered(),
            Title::<Right>::styled(" right ", Style::default().fg(GREEN200)).centered(),
            RoundedBorder::default(),
            Padding::top(4),
        ]);

    frame.render_widget(widget, center_area(45, 13, main_area));
}

fn title(s: &str) -> Paragraph<'_> {
    Paragraph::new(s)
        .style(
            Style::default()
                .fg(BLUE900)
                .bg(ORANGE400)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
}

fn help() -> Paragraph<'static> {
    Paragraph::new("press any key to quit")
        .style(Style::default().fg(ORANGE500))
        .alignment(Alignment::Center)
}

const fn center_area(width: u16, height: u16, area: Rect) -> Rect {
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}

// Palette Laranja's Rust Shop website
#[allow(dead_code)]
pub(crate) mod palette {
    use ratatui::style::Color;

    pub(crate) const ORANGE50: Color = Color::Rgb(252, 242, 232);
    pub(crate) const ORANGE100: Color = Color::Rgb(247, 215, 186);
    pub(crate) const ORANGE200: Color = Color::Rgb(242, 188, 140);
    pub(crate) const ORANGE300: Color = Color::Rgb(237, 161, 95);
    pub(crate) const ORANGE400: Color = Color::Rgb(232, 135, 49);
    pub(crate) const ORANGE500: Color = Color::Rgb(206, 109, 23);
    pub(crate) const ORANGE600: Color = Color::Rgb(160, 85, 18);
    pub(crate) const ORANGE700: Color = Color::Rgb(115, 61, 13);
    pub(crate) const ORANGE800: Color = Color::Rgb(69, 36, 8);
    pub(crate) const ORANGE900: Color = Color::Rgb(23, 12, 3);
    pub(crate) const BLUE50: Color = Color::Rgb(229, 250, 255);
    pub(crate) const BLUE100: Color = Color::Rgb(178, 240, 255);
    pub(crate) const BLUE200: Color = Color::Rgb(128, 229, 255);
    pub(crate) const BLUE300: Color = Color::Rgb(77, 219, 255);
    pub(crate) const BLUE400: Color = Color::Rgb(26, 209, 255);
    pub(crate) const BLUE500: Color = Color::Rgb(0, 184, 230);
    pub(crate) const BLUE600: Color = Color::Rgb(0, 143, 178);
    pub(crate) const BLUE700: Color = Color::Rgb(0, 102, 128);
    pub(crate) const BLUE800: Color = Color::Rgb(0, 61, 76);
    pub(crate) const BLUE900: Color = Color::Rgb(0, 20, 26);
    pub(crate) const GREEN50: Color = Color::Rgb(234, 250, 241);
    pub(crate) const GREEN100: Color = Color::Rgb(193, 241, 213);
    pub(crate) const GREEN200: Color = Color::Rgb(151, 232, 185);
    pub(crate) const GREEN300: Color = Color::Rgb(109, 222, 157);
    pub(crate) const GREEN400: Color = Color::Rgb(68, 213, 129);
    pub(crate) const GREEN500: Color = Color::Rgb(42, 187, 104);
    pub(crate) const GREEN600: Color = Color::Rgb(33, 146, 81);
    pub(crate) const GREEN700: Color = Color::Rgb(23, 104, 58);
    pub(crate) const GREEN800: Color = Color::Rgb(14, 62, 35);
    pub(crate) const GREEN900: Color = Color::Rgb(5, 21, 12);
    pub(crate) const PURPLE50: Color = Color::Rgb(244, 237, 247);
    pub(crate) const PURPLE100: Color = Color::Rgb(223, 202, 232);
    pub(crate) const PURPLE200: Color = Color::Rgb(202, 166, 216);
    pub(crate) const PURPLE300: Color = Color::Rgb(180, 131, 200);
    pub(crate) const PURPLE400: Color = Color::Rgb(159, 96, 185);
    pub(crate) const PURPLE500: Color = Color::Rgb(133, 70, 159);
    pub(crate) const PURPLE600: Color = Color::Rgb(104, 55, 124);
    pub(crate) const PURPLE700: Color = Color::Rgb(74, 39, 89);
    pub(crate) const PURPLE800: Color = Color::Rgb(44, 23, 53);
    pub(crate) const PURPLE900: Color = Color::Rgb(15, 8, 18);
}
