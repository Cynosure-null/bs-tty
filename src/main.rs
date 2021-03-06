use std::io;
use std::io::{stdout, BufWriter};
use std::process::Command;
use ferris_says;

/* ROADMAP */
/* This project is less important than many of my other ones but a few small changes need to be made:
* 1. Add support for Gnome desktops
* 2. Enable cleanup
* 3. Add startup scripts
* 12. Cleanup codebase
*/


fn start_plasma_wayland() {
    println!("Starting KDE Plasma with the Wayland display manager!");
    Command::new("startplasma-wayland").spawn();
}
fn start_plasma_xorg() {
    println!("Starting KDE Plasma with X!");
    Command::new("startx /usr/bin/plasmashell").spawn();
}

fn start_xorg() {
    println!("Starting X server");
    Command::new("startx").spawn();
}
fn start_gnome_wayland() {
    println!("Starting GNOME with Wayland!");
    Command::new("dbus-run-session -- gnome-shell --display-server --wayland").spawn();
    //IN DEVELOPMENT. Currently unstable
}
fn start_gnome_xorg() {
    Command::new("startx /usr/bin/gnome-shell").spawn();
}
fn start_i3(){
    Command::new("startx /usr/bin/i3").spawn();
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
Options include: n: new, k: kde, kx: kde as x, x: xorg, w: wayland, g: gnome, gx: gnome as x, i: i3 c: clean(NYI)"));

while true {
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
    else if input == "kx" {
        start_plasma_xorg();
    }
    else if input == "x"
    {
        start_xorg();
    }
    else if input == "w" {
        start_plasma_wayland();
    }
    else if input == "g" {
        start_gnome_wayland();
    }
    else if input == "gx" {
        start_gnome_xorg();
    }
    else if input == "i" {
        start_i3();
    }
    else if input == "c" {
        // How to clean up
        // A. Kill all proceses started by $USER
        // This has the slight issue of 
         
    }
    else if input == "q" {
        break
    }
    else {
        print(String::from("No option specified"));
    }
  }
}
