use std::collections::HashMap;

use std::path::PathBuf;
use shaders::ShaderParams;

use crate::config::config::{get_app_dir, SHADER_LIST};
use crate::shaders;

fn get_shaders() -> Result<HashMap<String, ShaderParams>, u8> {
    let app_dir: PathBuf = get_app_dir()?;

    let shaders_file = app_dir.join(SHADER_LIST);

    let raw_shaders: String = if shaders_file.exists() {
        match std::fs::read_to_string(shaders_file) {
            Ok(s) => {s},
            Err(_) => {
                return Err(0);
            }
        }
    } else { return Err(0) };
    

    let parsed_shaders = match serde_json::from_str(&raw_shaders) {
        Ok(s) => s,
        Err(_) => {
            return Err(0);
        }
    };

    Ok(parsed_shaders)
}

/// Save the inputted flags to a new shader
pub(crate) fn save() {

}

pub(crate) fn load() {

}
