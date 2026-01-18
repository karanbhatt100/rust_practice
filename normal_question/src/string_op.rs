use std::io;
use std::io::Write;
use crate::util_func::get_input_as_string;

pub(crate) fn check_if_palindrome() {
    print!("Enter string you want to check - ");
    io::stdout().flush().expect("Failed flush");

    let input = get_input_as_string();

    let output = reverse_string(&input);

    let input: String = input.trim().parse().expect("failed to remove \n");

    print!("Original String = {}", input);
    print!("\nReversed String = {}", output);

    if input == output {
        println!("\nIt is palindrome")
    } else {
        println!("\nIt is not the palindrome")
    }
}

fn reverse_string(input: &String) -> String {
    let mut output = String::new();
    for num in (0..input.len()).rev() {
        output.push(input.chars().nth(num).expect("Out of bound"))
    }
    output.trim().parse().expect("Failed to remove \n")
}
