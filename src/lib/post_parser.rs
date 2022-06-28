use pulldown_cmark::{html::push_html, Options, Parser};
use regex::Regex;
use std::collections::HashMap;

pub struct PostParser {
    markdown_text: String,
    key_list: Vec<String>,
}

impl PostParser {
    pub fn new(markdown_text: &str, key_list: &[String]) -> PostParser {
        PostParser {
            markdown_text: markdown_text.to_string(),
            key_list: key_list.to_vec(),
        }
    }

    fn seperate_header(&self) -> (String, String) {
        let re = Regex::new(r"\A--\n((.|\n)*?)---\n((.|\n)*)").unwrap();

        let captures = re
            .captures(&self.markdown_text)
            .expect("Could not split header and body");

        (captures[1].to_string(), captures[3].to_string())
    }

    pub fn parse_header(&self) -> HashMap<String, String> {
        let yaml: serde_yaml::Value = serde_yaml::from_str(&self.seperate_header().0)
            .expect("Could not parse the yaml header");

        let mut parsed_yaml: HashMap<String, String> = HashMap::new();

        for key in &self.key_list {
            parsed_yaml.insert(
                key.to_string(),
                yaml[key]
                    .as_str()
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| panic!("could not find key {} in your yaml header", key)),
            );
        }

        parsed_yaml
    }

    pub fn parse_md(&self) -> String {
        let parser = Parser::new_ext(&self.markdown_text, Options::all());

        let mut html = String::new();

        push_html(&mut html, parser);

        html
    }

}
