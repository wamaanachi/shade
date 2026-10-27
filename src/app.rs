use std::io;

pub fn run(){

    loop {
    crate::ui::draw_menu();

    println!("pick a shader:  1, 2, 3, 4 or q");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("smthin went wrong");

    let choice = input.trim();

    match choice {
        "1" => println!("selected smthin1"),
        "2" => println!("selected smthin2"),
        "3" => println!("selected smthin3"),
        "4" => println!("selected smthin4"),
        "q" => break,
        _ => println!("pick one of em bruh"),
    }
  }    
}