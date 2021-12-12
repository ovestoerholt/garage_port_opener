use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let blinks = std::env::args()
        .nth(1)
        .expect("You have to specify number of hello's...");
    let args = Cli {
        hellos: blinks.parse().expect("Not a number"),
    };

    let led = LED::new(18);

    for n in 0..args.hellos {
        print!("Blink! {}\n", n);
        led.on();
        sleep(Duration::from_secs(1));
        led.off();
        sleep(Duration::from_secs(1));
    }
}

struct Cli {
    hellos: i8,
}
