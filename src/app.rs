use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

pub struct App {
    pub selected: usize,
    pub control: usize,
    pub values: [f32; 4],
    pub fps: f32,
}

// im lowk gonna be building the importtant things now
pub fn run() -> io::Result<()>{

    let mut terminal = ratatui::init();

    let result = run_loop(&mut terminal);

    ratatui::restore();

    result
} 

fn run_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {

    let mut app = App {
        selected: 0,
        control: 0,
        values: [1.0, 1.0, 0.0, 1.0],  // these are the things i wrote in control btw
        fps: 0.0,
    };

    let shader_count = crate::shaders::name().len();

    let frame_time = Duration::from_secs_f64(1.0 / 60.0);

    let mut fps_timer = Instant::now();
    let mut frames = 0;

    loop {

        let frame_start = Instant::now();

        terminal.draw(|frame| {
            crate::ui::draw(frame, app.selected);
        })?;

        frames +=1;

        let elapsed = fps_timer.elapsed().as_secs_f32();

        if elapsed >= 1.0 {
            app.fps = frames as f32 / elapsed;
            frames = 0;
            fps_timer = Instant::now();
        }

    if event::poll(Duration::ZERO)?{
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Ok(()),

                KeyCode::Down => {
                    if app.selected + 1 < shader_count {
                        app.selected += 1;
                    }
                }

                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }

                KeyCode::Tab  => {
                    app.control = (app.control + 1) % 4;
                }

                KeyCode::Left => change_control(&mut app, -0.1),
                KeyCode::Right => change_control(&mut app, 0.1),

                KeyCode::Char('r') => {
                    app.values = [1.0, 1.0, 0.0, 1.0];
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

fn change_control(app: &mut App, amount: f32){
    // lowk im not sure about these, but too tired too ask rn will change em later ig

    let minimums = [0.5, 0.5. 0.0, 2.0];
    let maximums = [3.0, 5.0, 1.0, 2.0]; 

    let index = app.control;

    app.values[index] = (app.values[index] + amoutn)
        .clamp(minimums[index], maximums[index]);
}