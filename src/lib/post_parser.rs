use pulldown_cmark::{html::push_html, Options, Parser};
use regex::Regex;

pub struct PostParser {
    markdown_text: String,
}

impl PostParser {
    pub fn new(markdown_text: &str) -> PostParser {
        PostParser {
            markdown_text: markdown_text.to_string(),
        }
    }

    fn seperate_header(&self) -> (String, String) {
        let re = Regex::new(r"\A---\n((.|\n)*?)---\n((.|\n)*)").unwrap();

        let captures = re
            .captures(&self.markdown_text)
            .expect("Could not split header and body");

        (captures[1].to_string(), captures[3].to_string())
    }

    pub fn parse_header(&self) -> serde_yaml::Value {
        let yaml: serde_yaml::Value = serde_yaml::from_str(&self.seperate_header().0)
            .expect("Could not parse the yaml header");

        yaml
    }

    pub fn parse_md(&self) -> String {
        let md = self.seperate_header().1;
        let parser = Parser::new_ext(&md, Options::all());

        let mut html = String::new();

        push_html(&mut html, parser);

        html
    }
}
