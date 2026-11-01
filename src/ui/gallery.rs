use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListState},
};

use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, area: Rect, app: &mut App){
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

        app.gallery_area = border.inner(area);

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

        frame.render_stateful_widget(list, area, &mut state);
}