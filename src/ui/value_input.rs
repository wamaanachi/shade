use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App){
    let screen = frame.area();

    let width = screen.width.min(48);
    let height = screen.height.min(8);

        // looks tuff but it is really simple i'd say
    let area = Rect::new(
        screen.x + (screen.width - width) / 2,
        screen.y + (screen.height - height) / 2,
        width,
        height,
    );

    let names = ["Speed", "Zoom", "Color shift", "Intensity"];

    let text = format!(
        "{}: {}_ \n\nEnter: apply | Esc: cancel\n{}",
        names[app.control],
        app.input,
        app.input_error,
    );

    let input_box = Paragraph::new(text)
        .block(Block::bordered().title(" Enter a value "));

        frame.render_widget(input_box, area);
}