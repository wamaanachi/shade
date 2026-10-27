use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

// im lowk gonna be building the importtant things now
pub fn run() -> io::Result<()>{

    let mut terminal = ratatui::init();

    let result = run_loop(&mut terminal);

    ratatui::restore();

    result
} 

fn run_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut selected: usize  = 0;
    let shader_count = crate::shaders::name().len();

    loop {
        terminal.draw(|frame| {
            crate::ui::draw(frame, selected);
        })?;

        let event = event::read()?;

        if let Event::Key(key) = event {
            if key.kind != ?KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),

                KeyCode:Down => {
                    if selected + 1 < shader_count {
                        selected += 1;
                    }
                }

                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                _ = {}
            }
        }
    }
}