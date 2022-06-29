use fs_extra::{copy_items, dir};
use std::env::current_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, write};
use std::path::Path;

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    read_to_string(path)
        .expect("could not read this file")
        .parse()
        .expect("could not parse this file")
}

pub fn list_dir(path: &str) -> Result<Vec<String>, &'static str> {
    let path = Path::new(path);
    if path.is_dir() {
        let mut file_list: Vec<String> = Vec::new();
        for file in read_dir(path).expect("could not read this directory") {
            file_list.push(file.unwrap().file_name().into_string().unwrap());
        }

        Ok(file_list)
    } else {
        Err("not a directory")
    }
}

pub fn rm_dir(path: &str) {
    remove_dir_all(path).expect("could not remove this directory");
}

pub fn make_dirs(dirs: &[String]) {
    for dir in dirs {
        create_dir_all(dir).expect("could not create directory");
    }
}

pub fn pwd() -> String {
    current_dir()
        .expect("could not get current directory")
        .display()
        .to_string()
}

pub fn write_file(path: &str, text: &str) {
    let path = Path::new(path);
    write(path, text).expect("could not write file");
}

pub fn copy_dir(from: &[&str], to: &str) {
    let options = dir::CopyOptions::new();
    copy_items(from, to, &options).expect("could not copy this directory");
}
