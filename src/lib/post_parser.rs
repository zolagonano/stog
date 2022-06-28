use regex::Regex;

pub struct Post {
    markdown_text: String,
}

impl Post {
    pub fn new(markdown_text: &str) -> Post {
        Post {
            markdown_text: markdown_text.to_string(),
        }
    }

    fn seperate_header(&self) -> (String, String) {
        let re = Regex::new(r"\A--\n((.|\n)*?)---\n((.|\n)*)").unwrap();

        let captures = re
            .captures(&self.markdown_text)
            .expect("Could not split header and body");

        (captures[1].to_string(), captures[3].to_string())
    }
}
