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
    .split(rows[1]);

        let right = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(6), // might change this again tbh
        ])
        .split(columns[1]);

    let title = Paragraph::new("Shade")
            .block(Block::bordered());

    frame.render_widget(title, rows[0]);

    let names = crate::shaders::name();

    let shader_list = List::new(names)
            .block(Block::bordered().title(" Shaders "))
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan),
            );
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    frame.render_stateful_widget(shader_list, columns[0], &mut list_state);

    let the_text = format!("Selcted: {}\n\n", names[selected],);

    let preview = Paragraph::new(the_text)
            .block(Block::bordered().title(" Preview "));

        frame.render_widget(preview, right[0]);

        let controls = Paragraph::new("Speed: 1.0\nColor shift: 0.0\nIntensity: 1.0",)
        .block(Block::bordered().title(" Controls "));

        frame.render_widget(controls, right[1]);

    let footer = Paragraph::new("Up/Down: select |  q: quit").style(Style::default().fg(Color::Cyan)); // cyan is cool!

    frame.render_widget(footer, rows[2]);
}