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
    pub gallery_area: Rect,
    pub editing: bool,
    pub input: String,
    pub input_error: String,
}

// im lowk gonna be building the importtant things now
pub fn run() -> io::Result<()>{

    let mut terminal = ratatui::init();

    let result = match crossterm::execute!(
        io::stdout(),
        EnableMouseCapture
    ){
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
        gallery_area: Rect::default(),
        editing: false,
        input: String::new(),
        input_error: String::new(),
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

        frames +=1;

        let elapsed = fps_timer.elapsed().as_secs_f32();

        if elapsed >= 1.0 {
            app.fps = frames as f32 / elapsed;
            frames = 0;
            fps_timer = Instant::now();
        }

    if event::poll(Duration::ZERO)?{
        
        let input = event::read()?;

        if let Event::Mouse(mouse) = &input {
            handle_mouse(&mut app, mouse);
        }

        if let Event::Key(key) = input {
            if key.kind == KeyEventKind::Press {
                if app.editing {
                    edit_value(&mut app, key.code);
                }
                else {
            match key.code {
                KeyCode::Char('q') => return Ok(()),

                // gonna have to rerwite all of those, because of the feature im making
                // it is js focusing on one thing, contols or shaders

                KeyCode::Tab => {
                    app.focus = if app.focus == Focus::Gallery {
                        Focus::Controls
                    }
                    else {
                        Focus::Gallery
                    };
                }

                KeyCode::Down => {
                    if app.focus == Focus::Gallery {
                        if app.selected + 1 < shader_count {
                            app.selected += 1;
                        }
                    }
                        else {
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
                    if app.focus  == Focus::Controls {
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

                KeyCode::Enter => {
                    if app.focus == Focus::Controls && !app.fullscreen {
                        app.editing = true;
                        app.input.clear();
                        app.input_error.clear();
                        app.dragging = None;
                    }
                }
                _ => {}
                }
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

    let minimums = [0.5, 0.5, 0.0, 0.0];
    let maximums = [3.0, 5.0, 1.0, 2.0]; 

    let index = app.control;

    app.values[index] = (app.values[index] + amount)
        .clamp(minimums[index], maximums[index]);
}

fn handle_mouse(app: &mut App, mouse: &MouseEvent){

    if app.editing {
        return;
    }
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            app.dragging = None;

            let area = app.gallery_area;

            // umm its like almost the same code i wrote for the slider thing or mouse
            if mouse.column >= area.x
                && mouse.column < area.right()
                && mouse.row >= area.y
                && mouse.row < area.bottom() {
                    app.focus = Focus::Gallery;

                    let index = (mouse.row - area.y) as usize;

                    if index < crate::shaders::name().len(){
                        app.selected = index;
                    }

                    return;
                }

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

    // so im gonna add scrolling for all the shaders cuz why not
    // i tested the scrolling with fake shaders and it worked but deleted em later so rn its not umm doing anything as there is like 1 shader
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
            let area = app.gallery_area;

        // same ahh code repeatin
            if mouse.column >= area.x
                && mouse.column < area.right()
                && mouse.row >= area.y
                && mouse.row < area.bottom(){
                    app.focus = Focus::Gallery;

                    let count = crate::shaders::name().len();

                    if mouse.kind == MouseEventKind::ScrollDown {
                        if app.selected + 1 < count {
                            app.selected += 1;
                        }
                    }
                    else {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                }

        }
        _ => {}
    }
}

// lol

fn the_function_for_the_functioning_of_mouse_from_the_slider(app: &mut App, index: usize, column: u16){
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

fn edit_value(app: &mut App, key: KeyCode){
    match key {
        KeyCode::Esc => {
            app.editing = false;
        }

        KeyCode::Backspace => {
            app.input.pop();
            app.input_error.clear();
        }

        KeyCode::Char(character) => {
            if app.input.len() < 12 {
                app.input.push(character);
                app.input_error.clear();
            }
        }

        KeyCode::Enter => {
            let minimums = [0.5, 0.5, 0.0, 0.0];
            let maximums = [3.0, 5.0, 1.0, 2.0];
            let index = app.control;

            match app.input.parse::<f32>() {
                Ok(value) => {
                    if value.is_finite()
                        && value >= minimums[index]
                        && value <= maximums[index]
                        {
                            app.values[index] = value;
                            app.editing = false;
                        }
                        else {
                            app.input_error = format!(
                                "Use a value from {} to {}",
                                minimums[index], maximums[index],
                            );
                        }
            }
            Err(_)  => {
                app.input_error = String::from("type a number");
            }
        }
    }
    _ => {}
}
}