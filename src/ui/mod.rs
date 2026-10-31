mod sliders;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph},
};

use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, app: &mut App){

    app.slider_areas = [Rect::default(); 4];

    app.gallery_area = Rect::default();

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

        let right = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(6), // might change this again tbh
        ])
        .split(columns[1]);

    let title = Paragraph::new("Shade")
            .block(Block::bordered());

    frame.render_widget(title, rows[0]);

    let names = crate::shaders::name();

    // gallery is the thing with the sahders'names btw
    let gallery_color = if app.focus == Focus::Gallery {
        Color::Cyan
    }
    else {
        Color::Gray
    };

    let gallery_border = Block::bordered()
        .title(" Shaders ")
        .border_style(Style::default().fg(gallery_color));

        app.gallery_area = gallery_border.inner(columns[0]);

    let shader_list = List::new(names)
            .block(gallery_border)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                .fg(Color::Black)
                .bg(gallery_color),
            );
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    frame.render_stateful_widget(shader_list, columns[0], &mut list_state);

            draw_preview(frame, right[0], app);

            sliders::draw(frame, right[1], app);

            // check ui/sliders.rs, i decided to replace the plain texts with sliders which look so cool
            // so umm this is just no longer required

//         let labels = ["Speed", "Zoom", "Color shift", "Intensity"];
//         let mut text = String::new();
        
//         for index in 0..4 {
//             let marker = if index == app.control { ">" } else { " " };

//             let line = format!(
//                 "{} {}: {:.1}\n",
//                 marker,
//                 labels[index],
//                 app.values[index],
//             );

//             text.push_str(&line);
//         }
// // the paragraph thing is getting used so often
//         let controls = Paragraph::new(text)
//             .block(Block::bordered().title(" Controls: Tab / Left / Right "));

//             frame.render_widget(controls, right[1]);


// so im going to replace he plain text with sliders, might delete the code soon

        let area = frame.area();

        let footer_text = format!(
            "Tab: panel | Up/Down: select | Left/Right: adjust | r: reset | f: fullscreen | q: quit\nFPS: {:.1} | Terminal: {} x {} cells",
            app.fps,
            area.width,
            area.height,
        );

    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Cyan)); // cyan is cool!

    frame.render_widget(footer, rows[2]);
}

fn draw_preview(frame: &mut Frame, area: Rect, app: &App){
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