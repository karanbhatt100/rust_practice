
use std::fs::ReadDir;
use std::process::exit;
use crate::file_info::FileInfo;

pub(crate) fn get_files_list(path: String) -> Vec<FileInfo> {
    let entries = std::fs::read_dir(path).unwrap_or_else( |_| {
        println!("Failed to read file names");
        exit(1);
    });

    get_file_name(entries)
}

pub(crate) fn get_file_name(dir: ReadDir) -> Vec<FileInfo> {
    let mut vec: Vec<FileInfo> = Vec::new();
    for entry in dir {
        let e = &entry.unwrap();
        let file_info = FileInfo {
            name: e.file_name().to_str().unwrap().to_string(),
            is_dir: !e.file_type().unwrap().is_file()
        };
        vec.push(file_info);
    }

    vec
}
