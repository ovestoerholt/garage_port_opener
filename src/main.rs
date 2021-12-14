use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;
//use clap::{Arg, App, SubCommand};
use clap::{App, Arg};

fn main() {
    let matches = App::new("Garage Port Handler")
        .version("0.1")
        .author("Ove Stoerholt <ovestoerholt@gmail.com>")
        .about("Handles garage port")
        .arg(
            Arg::new("toggle")
                .short('t')
                .long("toggle")
                .help("Toggles garage port status. If it is open it closes and vice versa.")
                .takes_value(true),
            )
            .arg(
                Arg::new("status")
                .short('s')
                .long("status")
                .help("Checks status for garage port"),
        )
        .get_matches();

    if let Some(value) = matches.value_of("toggle") {
        // or, to be safe, match the `Err`
        match value.parse::<i8>() {
            Ok(int_value) => toggle_relay(int_value),
            Err(_) => println!("Not a number..."),
        }
    } 
    
    if matches.is_present("status") {
        let closed = is_door_closed();
        println!("Door is closed? {}", closed);
    }
}

/// Will return true magnetic switch is in on (connected) position.
fn is_door_closed() -> bool {
    let button = Button::new(23);
    button.is_active()
}

fn toggle_relay(args: i8) {
    let led = LED::new(18);

    for n in 0..args {
        print!("Blink! {}\n", n);
        led.on();
        sleep(Duration::from_secs(1));
        led.off();
        sleep(Duration::from_secs(1));
    }
}
