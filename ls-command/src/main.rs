mod dir_operation;
mod file_info;

use std::env;
use crate::dir_operation::get_files_list;

fn main() {

    const DEFAULT_DIRECTORY: &str = ".";
    const DEFAULT_SEPARATOR: &str = " ";
    const DEFAULT_SHOW_HIDDEN_FILE: bool = false;

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let directory = get_arg_if_exist(&args, 1, DEFAULT_DIRECTORY);
    let separator = get_arg_if_exist(&args, 2, DEFAULT_SEPARATOR);

    if args[0] == "ls" {
        let list_of_files = get_files_list(directory);

        for x in list_of_files {
            x.print_name();
            print!("{}", separator.replace("\\n","\n").replace("\\t","\t"));
        }
        println!()
    }
}

fn get_arg_if_exist(mut args: &Vec<String>, index: usize, default: &str) -> String {
    args.iter().nth(index).unwrap_or(&String::from(default)).to_string()
}
