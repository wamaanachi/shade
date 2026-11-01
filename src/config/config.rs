use std::path::{Path, PathBuf};
use directories::BaseDirs;


// --- MAIN THINGS ---
pub(crate) const GROUP_NAME: &str = "wamaanachi";
pub(crate) const APP_NAME: &str = "shade";

// --- FILES ---
pub(crate) const SHADER_LIST: &str = "shaders.json";


pub(crate) fn load_config() {
    todo!(
        "load config from the config file"
    )
}

pub(crate) fn save_config() {
    todo!(
        "save config to the config file"
    )
}

pub(crate) fn test_config() {
    println!("testing config");
    println!("get_base_dir(): \n{:?}\n", get_base_dir());
    println!("get_app_dir(): \n{:?}\n", get_app_dir());
    println!("get_config_dir(): \n{:?}", get_config_dir());
}


fn get_base_dir() -> Result<BaseDirs, u8> {
    let dirs: Option<BaseDirs> = BaseDirs::new();

    if dirs.is_some() {
        Ok(dirs.unwrap())
    } else {
        Err(0)
    }
}


// returns like
pub(crate) fn get_app_dir() -> Result<PathBuf, u8> {
    let dirs: BaseDirs = get_base_dir()?;

    // Linux:   /home/user/.local/share
    // Windows: C:\Users\user\AppData\Local
    // macOS:   /Users/user/Library/Application Support
    let data_dir: &Path = dirs.data_local_dir();

    let app_dir: PathBuf = data_dir.join(GROUP_NAME).join(APP_NAME);
    if !app_dir.exists() {
        std::fs::create_dir_all(app_dir.clone()).map_err(|_| {0})?;
    }

    Ok(app_dir)
}

fn get_config_dir() -> Result<PathBuf, u8> {
    let dirs: BaseDirs = get_base_dir()?;
    let conf_base_dir: PathBuf = dirs.config_dir().to_path_buf();

    Ok(conf_base_dir.join(GROUP_NAME).join(APP_NAME))
}
