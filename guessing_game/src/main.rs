use std::cmp::Ordering;
use std::io;
use std::process::exit;
use rand::Rng;

fn main() {

    println!("Pick a game");
    println!("1. I guess your number");
    println!("2. We guess our number");
    println!("3. Compare and get idea");
    println!("4. Keep guessing and comparing. No Exit");
    println!("5. Keep guessing and comparing. Exit when correct");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let input = guess.chars().next().unwrap_or('0');
    println!("You entered: {}", input);

    if input == '1' {
        your_guessing_game();
    } else if input == '2' {
        ran_guessing_game();
    } else if input == '3' {
        guess_and_compare();
    } else if input == '4' {
        keep_guessing_and_compare_no_exit();
    } else if input == '5' {
        keep_guessing_and_compare_exit_when_correct();
    } else { println!("Not a valid input"); }

    match input {
        '1' => your_guessing_game(),

        '3' => guess_and_compare(),
        '4' => keep_guessing_and_compare_no_exit(),
        '5' => keep_guessing_and_compare_exit_when_correct(),
        _ => println!("Invalid input"),
    }
}

fn your_guessing_game() {
    println!("Guess the number");
    println!("Input your guess");

    let mut your_number = String::new();

    io::stdin().read_line(&mut your_number).expect("Failed to read line");

    println!("You guessed: {}", your_number);
}

fn ran_guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess your number!");

    let mut your_number = String::new();

    io::stdin().read_line(&mut your_number).expect("Failed to read line");

    println!("You guessed the number {your_number}");

    let your_number: i32 = match your_number.trim().parse() {
        Ok(your_number) => your_number,
        Err(_) => {
            println!("That was not a number");
            exit(1);
        }
    };

    if secret_number == your_number {
        println!("We guessed the number same!");
    } else {
        println!("You guessed {your_number}");
        println!("I guessed {secret_number}")
    }
}

fn guess_and_compare() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess your number!");
    let mut your_number = String::new();

    io::stdin().read_line(&mut your_number).expect("Failed to read line");

    let your_number:i32 = match your_number.trim().parse() {
        Ok(your_number) => your_number,
        Err(_) => {
            println!("That was not a number");
            exit(1);
        }
    };

    println!("You guessed: {}", your_number);

    match your_number.cmp(&secret_number) {
        Ordering::Less => println!("Small Number"),
        Ordering::Greater => println!("Big Number"),
        Ordering::Equal => println!("You guessed the number"),
    }
}

fn keep_guessing_and_compare_no_exit() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number = ");
        let mut your_number = String::new();

        io::stdin().read_line(&mut your_number).expect("Failed to read line");

        let your_number: i32 = match your_number.trim().parse() {
            Ok(your_number) => your_number,
            Err(_) => {
                println!("That was not a number");
                exit(1);
            }
        };

        println!("You guessed: {}", your_number);

        match your_number.cmp(&secret_number) {
            Ordering::Less => println!("Small Number"),
            Ordering::Greater => println!("Big Number"),
            Ordering::Equal => println!("HURRAY!!! Correct number"),
        }
    }
}

fn keep_guessing_and_compare_exit_when_correct() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number = ");
        let mut your_number = String::new();

        io::stdin().read_line(&mut your_number).expect("Failed to read line");

        let your_number: i32 = match your_number.trim().parse() {
            Ok(your_number) => your_number,
            Err(_) => {
                println!("That was not a number");
                exit(1);
            }
        };

        println!("You guessed: {}", your_number);

        match your_number.cmp(&secret_number) {
            Ordering::Less => println!("Small Number"),
            Ordering::Greater => println!("Big Number"),
            Ordering::Equal => {
                println!("HURRAY!!! Correct number");
                break;
            }
        }
    }
}

fn to_i3(input: &str) -> i32 {
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That was not a number");
            exit(1);
        },
    }
}