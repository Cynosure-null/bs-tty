use std::io;
use std::io::{stdout, BufWriter};
use std::process::Command;
use ferris_says;

fn start_plasma_wayland() {
    print(String::from("Starting KDE Plasma with the Wayland display manager!"));
    Command::new("startplasma-wayland").spawn();
}
fn start_plasma_xorg() {
    print(String::from("Starting KDE Plasma with X!"));
    Command::new("startplasma-x11").spawn();
}

fn start_xorg() {
    print(String::from("Starting X server"));
    Command::new("startx").spawn();
}

fn print(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(message.as_bytes(), width, &mut writer).unwrap();
}

fn main() {
    static DEFAULT_WAYLAND: bool = true;
    print(String::from("Welcome to the bloody stupid session manager \n
What would you like to do?  \n
Options include: n: new, k: kde, x: xorg, w: wayland, c: clean(NYI)"));


    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim_end();

    if input == "n" {
        println!("here");
        if DEFAULT_WAYLAND{
            start_plasma_wayland();
        }
        else {
            start_xorg();
        }
    }

    else if input == "k"
    {
        if DEFAULT_WAYLAND{
            start_plasma_wayland();
        }
        else {
            start_plasma_xorg();
        }
    }
    else if input == "x"
    {
        start_xorg();
    }
    else if input == "w" {
        start_plasma_wayland();
    }
    else {
        print(String::from("No option specified"));
    }
}
