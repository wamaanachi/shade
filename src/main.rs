mod app;
mod shaders;
mod ui;

use clap::Parser;
use crate::cli::parse;
use crate::shaders::{Shader, ShaderParams, mountains::Mountains};
use crate::shaders::show::show;

pub mod config;
pub mod cli;


fn main() -> std::io::Result<()> {
    if !parse::parse() {
        app::run()?;
    }

    Ok(())
}


fn run_demo() {
    let shader = Mountains;
    let params = ShaderParams::default();
    
    show(shader, params);
}
