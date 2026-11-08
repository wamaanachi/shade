use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Gauge, Paragraph},
};

use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, area: Rect,  app: &mut App) {
    let focused = app.focus == Focus::Controls;

    let border_color = if focused {
        Color::Cyan
    } else {
        Color::Gray
    };

    let border = Block::bordered()
        .title(" Controls: Up/Down selects | Left/Righ: to change values")
        .border_style(Style::default().fg(border_color));

    let inside = border.inner(area);
    frame.render_widget(border, area);

    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
        .split(inside);

    let labels = ["Speed", "Zoom", "Color shift", "Intensity"];

    let minimums = [0.5, 0.5, 0.0, 0.0];
    let maximums = [3.0, 5.0, 1.0, 2.0];

    for index in 0..4 {
        let columns = Layout::horizontal([
            Constraint::Length(15),
            Constraint::Min(0),
        ])
            .split(rows[index]);

        app.slider_areas[index] = columns[1];

        let selected = focused && index == app.control;

        let color = if selected {
            Color::Cyan
        } else {
            Color::Gray
        };

        let marker = if selected { ">" } else { " " };

        let name = Paragraph::new(
            format!("{} {}", marker, labels[index]),
        )
            .style(Style::default().fg(color));

        frame.render_widget(name, columns[0]);

        // umm so this is the thing to calculate the value from the slider, lil complicated lol. looked it up
        let ratio = (app.values[index] - minimums[index]) / (maximums[index] - minimums[index]);

        let slider = Gauge::default()
            .ratio(ratio.clamp(0.0, 1.0) as f64)
            .label(format!("{:.1}", app.values[index]))
            .gauge_style(
                Style::default()
                    .fg(color)
                    .bg(Color::DarkGray),
            );
        frame.render_widget(slider, columns[1]);
    }
}