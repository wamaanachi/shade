use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph},
};

use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App){

    // making that box for the search

    let sections = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .split(area);

    let marker = if app.searching { "_" } else { "" };

    let search_box = Paragraph::new(
        format!("{}{}", app.search, marker),
    )
    .block(Block::bordered().title(" / Search "));

    frame.render_widget(search_box, sections[0]);
    let names = crate::shaders::name();

    let color = if app.focus == Focus::Gallery {
        Color::Cyan
    }
    else {
        Color::Gray
    };

    let border = Block::bordered()  
        .title(" Shaders ")
        .border_style(Style::default().fg(color));

        app.gallery_area = border.inner(sections[1]);

        let list = List::new(names)
            .block(border)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(color),
            );

        let mut state = ListState::default();
        state.select(Some(app.selected));

        frame.render_stateful_widget(list, sections[1], &mut state);
}