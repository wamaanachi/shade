use std::collections::HashMap;
use crate::cli::cli::ShaderArgs;
use crate::shaders::ShaderParams;

pub(crate) fn run(shader: String) {
    let shaders = get_saved_shaders().keys();
    todo!()
}

pub(crate) fn list() {
    todo!()
}

pub(crate) fn save(save_what: ShaderArgs) {
    todo!()
}

pub(crate) fn genshade() {
    todo!()
}


// --- Helper Functions ---


fn get_saved_shaders() -> HashMap<String, ShaderParams> {
    todo!()
}
