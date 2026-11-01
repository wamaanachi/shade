use crate::shaders::mountains::Mountains;
use crate::shaders::plasma::NeonPlasma;
use crate::shaders::{Shader, ShaderParams};


use crate::shaders::show::show;
use crate::shaders::tunnel::CyberTunnel;

const MOUNTAINS: Mountains = Mountains;
const NEONPLASMA: NeonPlasma = NeonPlasma;
const CYBERTUNNEL: CyberTunnel = CyberTunnel;


pub(crate) fn run(shader: String) {
    match shader.to_lowercase().as_str() {
        "mountains" => show(MOUNTAINS, ShaderParams::default()),
        "plasma" => show(NEONPLASMA, ShaderParams::default()),
        "neonplasma" => show(NEONPLASMA, ShaderParams::default()),
        "tunnel" => show(CYBERTUNNEL, ShaderParams::default()),
        "cybertunner" => show(CYBERTUNNEL, ShaderParams::default()),

        _ => show(MOUNTAINS, ShaderParams::default())
    }
}

pub(crate) fn list() {
    println!("Shaders Available: ");
    println!("- Mountains");
    println!("- NeonPlasma");
    println!("- CyberTunnel");
}
//
// pub(crate) fn save(save_what: ShaderArgs) {
//
// }
//
// pub(crate) fn genshade() {
//
// }
//
//
// // --- Helper Functions ---
//
//
// fn get_saved_shaders() -> HashMap<String, ShaderParams> {
//
// }

