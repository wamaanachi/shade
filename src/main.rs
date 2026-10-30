use std::{
    io::{Write, stdout},
    time::Instant,
};

use crate::shaders::{Shader, ShaderParams, plasma::NeonPlasma};

pub mod shaders;
pub mod config;
pub mod cli;

fn main() {
    cli::parse::parse();
    println!("Hello, world!");
}
