use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use ratatui::layout::Rect;
use crossterm::event::{
    EnableMouseCapture, DisableMouseCapture,
    MouseButton, MouseEvent, MouseEventKind,
};

#[derive(PartialEq)]
pub enum Focus {
    Gallery,
    Controls,
}

pub struct App {
    pub selected: usize,
    pub control: usize,
    pub values: [f32; 4],
    pub fps: f32,
    pub fullscreen: bool,
    pub paused: bool,
    pub time: f32,
    pub focus: Focus,
    pub slider_areas: [Rect; 4],
    pub dragging: Option<usize>,
}

// im lowk gonna be building the importtant things now
pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let result = match crossterm::execute!(
        io::stdout(),
        EnableMouseCapture
    ) {
        Ok(()) => run_loop(&mut terminal),
        Err(error) => Err(error),
    };

    let cleanup = crossterm::execute!(
        io::stdout(),
        DisableMouseCapture
    );

    ratatui::restore();

    result.and(cleanup)
}

fn run_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App {
        selected: 0,
        control: 0,
        values: [1.0, 1.0, 0.0, 1.0],  // these are the things i wrote in control btw
        fps: 0.0,
        fullscreen: false,
        paused: false,
        time: 0.0,
        focus: Focus::Gallery,
        slider_areas: [Rect::default(); 4],
        dragging: None,
    };

    let shader_count = crate::shaders::name().len();

    let frame_time = Duration::from_secs_f64(1.0 / 60.0);

    let mut fps_timer = Instant::now();
    let mut frames = 0;

    let mut previous_frame = Instant::now();

    loop {
        let frame_start = Instant::now();

        let listening_to_drake_rn = frame_start.duration_since(previous_frame).as_secs_f32();

        previous_frame = frame_start;

        if !app.paused {
            app.time += listening_to_drake_rn;
        }

        terminal.draw(|frame| {
            crate::ui::draw(frame, &mut app);
        })?;

        frames += 1;

        let elapsed = fps_timer.elapsed().as_secs_f32();

        if elapsed >= 1.0 {
            app.fps = frames as f32 / elapsed;
            frames = 0;
            fps_timer = Instant::now();
        }

        if event::poll(Duration::ZERO)? {
            let input = event::read()?;

            if let Event::Mouse(mouse) = &input {
                handle_mouse(&mut app, mouse);
            }

            if let Event::Key(key) = input {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),

                        // gonna have to rerwite all of those, because of the feature im making
                        // it is js focusing on one thing, contols or shaders

                        KeyCode::Tab => {
                            app.focus = if app.focus == Focus::Gallery {
                                Focus::Controls
                            } else {
                                Focus::Gallery
                            };
                        }

                        KeyCode::Down => {
                            if app.focus == Focus::Gallery {
                                if app.selected + 1 < shader_count {
                                    app.selected += 1;
                                }
                            } else {
                                // umm gonna make it so that the last control will take you to the first one when you press the up/down and vice versa.
                                app.control = (app.control + 1) % 4;
                            }
                        }

                        KeyCode::Up => {
                            if app.focus == Focus::Gallery {
                                if app.selected > 0 {
                                    app.selected -= 1;
                                }
                            } else {
                                app.control = (app.control + 3) % 4;
                            }
                        }
                        KeyCode::Left => {
                            if app.focus == Focus::Controls {
                                change_control(&mut app, -0.1);
                            }
                        }

                        KeyCode::Right => {
                            if app.focus == Focus::Controls {
                                change_control(&mut app, 0.1);
                            }
                        }

                        KeyCode::Char('r') => {
                            if app.focus == Focus::Controls {
                                app.values = [1.0, 1.0, 0.0, 1.0];
                            }
                        }

                        KeyCode::Char('f') => {
                            app.fullscreen = !app.fullscreen;
                        }

                        KeyCode::Char(' ') => {
                            app.paused = !app.paused;
                        }
                        _ => {}
                    }
                }
            }
        }
        let remaining = frame_time.saturating_sub(frame_start.elapsed());
        std::thread::sleep(remaining);
    }
}

fn change_control(app: &mut App, amount: f32) {
    // lowk im not sure about these, but too tired too ask rn will change em later ig

    let minimums = [0.5, 0.5, 0.0, 0.0];
    let maximums = [3.0, 5.0, 1.0, 2.0];

    let index = app.control;

    app.values[index] = (app.values[index] + amount)
        .clamp(minimums[index], maximums[index]);
}

fn handle_mouse(app: &mut App, mouse: &MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            app.dragging = None;

            for index in 0..4 {
                let area = app.slider_areas[index];

                // mehh the code is like similar to the once i wrote previously (when it used keys to control the sliders) so nothing new much
                if mouse.column >= area.x
                    && mouse.column < area.right()
                    && mouse.row >= area.y
                    && mouse.row < area.bottom()
                {
                    app.focus = Focus::Controls;
                    app.control = index;
                    app.dragging = Some(index);

                    the_function_for_the_functioning_of_mouse_from_the_slider(app, index, mouse.column);
                    break;
                }
            }
        }
        // bruhh this shi is taking so much time.
        // doesnt even have proper documentantions

        MouseEventKind::Drag(MouseButton::Left) => {
            if let Some(index) = app.dragging {
                the_function_for_the_functioning_of_mouse_from_the_slider(app, index, mouse.column);
            }
        }

        MouseEventKind::Up(MouseButton::Left) => {
            app.dragging = None;
        }
        _ => {}
    }
}

// lol

fn the_function_for_the_functioning_of_mouse_from_the_slider(app: &mut App, index: usize, column: u16) {
    let area = app.slider_areas[index];

    if area.width < 2 {
        return;
    }

    let minimums = [0.5, 0.5, 0.0, 0.0];
    let maximums = [3.0, 5.0, 1.0, 2.0];

    // tuff shit
    let position = column as f32 - area.x as f32;
    let ratio = (position / (area.width - 1) as f32)
        .clamp(0.0, 1.0);

    // bruh fuck this shit
    // why doesnt this shit work
    let value = minimums[index] + ratio * (maximums[index] - minimums[index]);

    app.values[index] = (value * 10.0).round() / 10.0;
}
