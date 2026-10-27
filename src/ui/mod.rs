pub fn draw_menu(){
    // lowk i donno what to put here
    println!("Shade");

    // distributin the code among all the files lowk :teehee:
    let names = crate::shaders::name();

    for name in names {
        println!("- {}", name);
    }

    println!();
    println!("lowk");
}