use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[arg(long)]
    pub(crate) shader: Option<String>,

    #[arg(long)]
    pub(crate) speed: Option<f32>,

    #[arg(long)]
    pub(crate) intensity: Option<f32>,

    #[arg(long)]
    color_intensity: Option<f32>,
}


#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Run {
        shader: String,
    },
    List {
        #[command(subcommand)]
        list_what: ListWhat,
    },
    Save {
        name: String,
    }
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum ListWhat {
    Shaders {},
    History {}
}

// • Amaan: CLI, Config & Systems:
// CLI with  Run directly with flags like genshade run --shader plasma --speed 1.5 or genshade list.
// Preset System ( Save and load user custom presets to a config file so people can save their favorite visual tweaks.
// Multithreading with  Parallelize the pixel loops across CPU cores so it hits 60 FPS without stuttering.
// Engine Controls: Delta-time loop, FPS counter, pause/resume, and frame-stepping.
// This way, nobody is blocked, everyone has 10+ hours of real code, and I can start translating my OpenGL shader into Rust today.

