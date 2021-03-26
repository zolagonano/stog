use regex::Regex;
use std::env::current_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, write};
use std::path::Path;

pub fn split_header_from_markdown(markdown: &str) -> (String, String) {
    let re = Regex::new(r"\A---\n((.|\n)*?)---\n((.|\n)*)").expect("There is a problem in regex");

    let caps = re
        .captures(markdown)
        .expect("Could not split header and body");

    (caps[1].to_string(), caps[3].to_string())
}

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

pub fn make_dirs(dirs: &Vec<String>) {
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

#[cfg(test)]
mod tests {
    use super::split_header_from_markdown;
    #[test]
    fn split_header_test() {
        let expected_result = (
            include_str!("../test_includes/markdown_lorem_ipsum_with_header_header.md").to_string(),
            include_str!("../test_includes/markdown_lorem_ipsum_with_header_body.md").to_string(),
        );

        assert_eq!(
            expected_result,
            split_header_from_markdown(include_str!(
                "../test_includes/markdown_lorem_ipsum_with_header.md"
            ))
        );
    }
}
