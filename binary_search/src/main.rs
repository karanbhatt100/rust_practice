use std;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::process::exit;
use rand::{rng, Rng};

fn main() {

    println!("Let's do binary search");

    let total_element = get_input(String::from("Give me total element to generate array = "));

    let max_value = get_input(String::from("What should be the max value = "));


    let vector = create_sorted_array(total_element, max_value);
    let value = get_input(String::from("Enter a number to search between 1 and max_value = "));
    search_value(value, &vector);
}

fn search_value(value: i32, vec_array: &[i32]) {
    if vec_array.len() == 1 && value.ne(vec_array.get(0).unwrap()) || vec_array.is_empty() {
        println!("Value not found. Sorry");
        return;
    }


    let middle_point = vec_array.len() / 2;
    let v = vec_array.get(middle_point).unwrap();
    println!("Length {}, Value in middle {}", vec_array.len(), v);

    match value.cmp(v) {
        Ordering::Less => search_value(value, &vec_array[0..middle_point]),
        Ordering::Greater => search_value(value, &vec_array[middle_point+1..]),
        Ordering::Equal => {
            println!("Value {} found and exist in vector", value);
        }
    }
}

fn get_input(message: String) -> i32 {
    let mut input = String::new();
    print!("{message}");
    io::stdout().flush().expect("Failed to flush stdout1");
    io::stdin().read_line(&mut input).expect("Failed to get input");

    let input = to_i32(input);
    if input.eq(&0) {
        println!("Invalid input. Enter some number");
        exit(1);
    }

    input
}

fn create_sorted_array(size: i32, max_value: i32) -> Vec<i32> {
    let mut vector  =(0..size)
        .map(|_| rng().random_range(1..=max_value))
        .collect::<Vec<i32>>();
    vector.sort();

    vector.iter().for_each(|num| print!("{num} "));
    println!("");

    vector
}

fn to_i32(input: String) -> i32 {
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Enter some number");
            exit(1);
        }
    }
}
