mod sliders;
mod value_input;
mod footer;
mod gallery;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    app.slider_areas = [Rect::default(); 4];

    app.gallery_area = Rect::default();

    if app.editing {
        value_input::draw(frame, app);
        return;
    }

    if app.fullscreen || frame.area().width < 80 || frame.area().height < 20 {
        app.dragging = None;
    }

    let area = frame.area();

    if area.width < 80 || area.height < 20 {
        let message = Paragraph::new(
            "Terminal is too small, resize it"
        );

        frame.render_widget(message, area);

        return;
    }

    if app.fullscreen {
        let area = frame.area();

        draw_preview(frame, area, app);

        return;
    }

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
        Constraint::Length(15),
        Constraint::Min(0),
    ])
        .split(rows[1]);

    let controls_height =  if app.show_controls { 6 } else { 0 };
        let right = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(controls_height),
        ])
        .split(columns[1]);

    let title = Paragraph::new("Shade")
        .block(Block::bordered());

    frame.render_widget(title, rows[0]);

    // umm deletd all that beacuse i wrote gallery.rs with the search box

            gallery::draw(frame, columns[0], app);

    draw_preview(frame, right[0], app);

        if app.show_controls {
            sliders::draw(frame, right[1], app);
        }

    // check ui/sliders.rs, i decided to replace the plain texts with sliders which look so cool
    // so umm this is just no longer required

    // let labels = ["Speed", "Zoom", "Color shift", "Intensity"];
    // let mut text = String::new();
    //
    // for index in 0..4 {
    //     let marker = if index == app.control { ">" } else { " " };
    //
    //     let line = format!(
    //         "{} {}: {:.1}\n",
    //         marker,
    //         labels[index],
    //         app.values[index],
    //     );
    //
    //     text.push_str(&line);
    // }

    // // the paragraph thing is getting used so often
    // let controls = Paragraph::new(text)
    //     .block(Block::bordered().title(" Controls: Tab / Left / Right "));
    //
    // frame.render_widget(controls, right[1]);


    // so im going to replace he plain text with sliders, might delete the code soon

    let area = frame.area();

// so im going to replace he plain text with sliders, might delete the code soon

// deleted all that because i just wrote footer.rs which does the same thing but better

footer::draw(frame, rows[2], app);
}

fn draw_preview(frame: &mut Frame, area: Rect, app: &App) {
    let names = crate::shaders::name();

    let status = if app.paused { "Paused" } else { "Running" };

    let text = format!(
        // this shi is really confusing me
        "Selected: {}\n\nStatus: {}\nTime: {:.1}s\n\nSpace: pause/resume | f: toggle fullscreen | q: quit",
        names[app.selected],
        status,
        app.time,
    );

    let preview = Paragraph::new(text)
        .block(Block::bordered().title(" Preview "));

    frame.render_widget(preview, area);
}
