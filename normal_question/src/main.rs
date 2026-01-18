mod util_func;
mod string_op;

use std::io;
use std::io::Write;
use util_func::*;
use string_op::check_if_palindrome;

fn main() {

    println!("1. Check if string palindrome");

    print!("Enter your choice = ");
    io::stdout().flush().expect("Failed flush");

    let input = get_input_as_u32();

    match input {
        1 => check_if_palindrome(),
        _ => println!("Invalid input")
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for n in nums.repeat(nums.len()) {
        if n >= target {
            continue;
        }
        

    }

    vec![]
}