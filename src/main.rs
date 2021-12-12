fn main() {
    let hellos = std::env::args().nth(1).expect("You have to specify number of hello's...");
    let args = Cli{
        hellos: hellos.parse().expect("Not a number")
    };
    for n in 0..args.hellos {
        print!("Hello {}\n", n);
    }
}


struct Cli {
    hellos: i8,
}