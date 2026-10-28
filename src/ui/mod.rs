use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph},
};

pub fn draw(frame: &mut Frame, selected: usize){
    
    let rows = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    // distributin the code among all the files lowk :teehee:
    
    // the responsive thingy
    let columns = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ])
    .split([rows[1]]);

        let right = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(columns[1]);

    let title = Paragraph::new("Shade")
            .block(Block::bordered());

    frame.render_widget(title, rows([0]));

    let names = crate:shaders::name();

    let shader_list = List::new(names)
            .block(bordered().title(" Shaders "))
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan),
            );
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

}