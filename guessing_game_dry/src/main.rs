use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::process::exit;
use rand::Rng;

fn main() {

    println!("Pick a game");
    println!("1. I guess your number");
    println!("2. We guess our number");
    println!("3. Compare and get idea");
    println!("4. Keep guessing and comparing. No Exit");
    println!("5. Keep guessing and comparing. Exit when correct");

    let input = get_input_as_number();
    println!("You entered: {}", input);

    match input {
        1 => your_guessing_game(),
        2 => ran_guessing_game(),
        3 => guess_and_compare(),
        4 => keep_guessing_and_compare_no_exit(),
        5 => keep_guessing_and_compare_exit_when_correct(),
        _ => println!("Invalid input")
    }
}

fn your_guessing_game() {
    println!("Guess the number");

    let number = get_input_as_number();

    println!("You guessed: {}", number);
}

fn ran_guessing_game() {
    let secret_number = get_random_number();

    println!("Guess your number!");

    let number = get_input_as_number();
    println!("You guessed the number {number}");

    if secret_number == number {
        println!("We guessed the number same!");
    } else {
        println!("You guessed {number}");
        println!("I guessed {secret_number}")
    }
}

fn guess_and_compare() {
    let secret_number = get_random_number();

    let number = get_input_as_number();

    println!("You guessed: {}", number);

    compare_number(number, secret_number);
}

fn keep_guessing_and_compare_no_exit() {
    let secret_number = get_random_number();

    loop {
        let number = get_input_as_number();

        println!("You guessed: {}", number);

        compare_number(number, secret_number);
    }
}

fn keep_guessing_and_compare_exit_when_correct() {
    let secret_number = get_random_number();

    let mut total_iteration = 0;

    loop {
        let number = get_input_as_number();

        let result = compare_number(number, secret_number);

        if result == true {
            break;
        }
        total_iteration = total_iteration + 1;
    }
    println!("Took total {total_iteration} tries");
}

fn get_random_number() -> i32 {
    rand::rng().random_range(1..=100)
}

fn get_input_as_number() -> i32 {

    let mut input = String::new();
    print!("Enter your choice = ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut input).expect(
        "Failed to get number");

    to_i32(input)
}

fn to_i32(input: String) -> i32 {
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {            println!("That was not a number");
            exit(1);
        },
    }
}

fn compare_number(number: i32, secret_number: i32) -> bool {
    match number.cmp(&secret_number) {
        Ordering::Less => {
            println!("Small Number");
            false
        },
        Ordering::Greater => {
            println!("Big Number");
            false
        },
        Ordering::Equal => {
            println!("You guessed the number");
            true
        },
    }
}

