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