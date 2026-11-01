use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};
// so this file lowk has the same thing that i wrote before but the text will just change
// wait it already wrote this below but yeah you got it
use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, area: Rect, app: &App){
    // pretty easy to understand but lemme tell you, its just what text to show depending
    // on the thing ur focused
    let hints = if app.focus == Focus::Gallery {
        "Up/Down: shader | Scroll: browse | Tab: controls"
    }
    else {
        "Up/Down: slider | Left/Right: adjust | Enter: type | r: reset | Tab: gallery"
    };

    let screen = frame.area();

    let text = format!(
    // the samee thing that i wrote before, but now it will switch
        "{}\nf: fullscreen | Space: pause | q: quit | UI FPS: {:.1} | {}x{} cells",
        hints,
        app.fps,
        screen.width,
        screen.height,
    );

    let footer = Paragraph::new(text)
        .style(Style::default().fg(Color::Cyan));

        frame.render_widget(footer, area);
}