use fs_extra::{copy_items, dir};
use std::env::current_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, write};
use std::path::Path;

pub fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    Ok(read_to_string(path)?.parse()?)
}

pub fn list_dir(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    if path.is_dir() {
        let mut file_list: Vec<String> = Vec::new();
        for file in read_dir(path)? {
            file_list.push(file?.file_name().into_string().unwrap());
        }

        Ok(file_list)
    } else {
        Err("Not a directory".into())
    }
}

pub fn rm_dir(path: &str) -> std::io::Result<()> {
    remove_dir_all(path)?;
    Ok(())
}

pub fn make_dirs(dirs: &[String]) -> std::io::Result<()> {
    for dir in dirs {
        create_dir_all(dir)?;
    }
    Ok(())
}

pub fn pwd() -> std::io::Result<String> {
    Ok(current_dir()?.display().to_string())
}

pub fn write_file(path: &str, text: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    write(path, text)?;
    Ok(())
}

pub fn copy_dir(from: &[&str], to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let options = dir::CopyOptions::new();
    copy_items(from, to, &options)?;
    Ok(())
}
