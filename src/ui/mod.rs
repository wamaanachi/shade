use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App){

    let selected = app.selected;
    
    let rows = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(2),
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

        let labels = ["Speed", "Zoom", "Color shift", "Intensity"];
        let mut text = String::new();
        
        for index in 0..4 {
            let marker = if index == app.control { ">" } else { " " };

            let line = format!(
                "{} {}: {:.1}\n",
                marker,
                labels[index],
                app.values[index],
            );

            text.push_str(&line);
        }
// the paragraph thing is getting used so often
        let controls = Paragraph::new(text)
            .block(Block::bordered().title(" Controls: Tab / Left / Right "));

            frame.render_widget(controls, right[1]);

        let area = frame.area();

        let footer_text = format!(
            "Up/Down: shader | Tab: controls | Left/Right: adjust | r: reset | q: quit\nFPS: {:.1} | Terminal: {} x {} cells",
            app.fps,
            area.width,
            area.height,
        );

    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Cyan)); // cyan is cool!

    frame.render_widget(footer, rows[2]);
}