use std::io;

fn main() {
    println!("Welcome to Guessing number game");
    
    println!("Would you like to start the game? (y/n)");
    let mut is_start = String::new();
    
    io::stdin()
        .read_line(&mut is_start)
        .expect("Failed to read line.");
    let is_start = is_start.trim_end();
    
    if is_start == "y" {
        println!("Please input your guess");
    } else if is_start == "n" {
        println!("Successfully quit the game");
        println!("If you want to start game again, please restart your terminal");
    } else {
        println!("Invalid command");
    };
}
