use std::io;
use std::process::exit;

pub(crate) fn get_input_as_string() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read from stdin");
    s.trim().to_string()
}

pub(crate) fn get_input_as_u32() -> u32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read from stdin");

    let input = line.trim().parse().unwrap_or_else(|_| {
        println!("Not a number");
        exit(1);
    });

    input
}