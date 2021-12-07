use serde_yaml::{from_str, Value};
use std::collections::HashMap;

pub struct Yaml {
    key_list: Vec<String>,
    yaml_text: String,
}

impl Yaml {
    pub fn new(key_list: Vec<String>, yaml_text: String) -> Yaml {
        Yaml {
            key_list,
            yaml_text,
        }
    }

    pub fn parse(&self) -> HashMap<String, String> {
        let yaml_data: Value = from_str(&self.yaml_text).expect("could not parse this yaml header");

        let mut parsed_yaml: HashMap<String, String> = HashMap::new();

        for key in &self.key_list {
            parsed_yaml.insert(key.to_string(), Yaml::get_value_by_key(key, &yaml_data));
        }

        parsed_yaml
    }

    fn get_value_by_key(key: &str, yaml_data: &Value) -> String {
        yaml_data[key]
            .as_str()
            .map(|value| value.to_string())
            .unwrap_or_else(|| panic!("could not find key {} in your yaml header", key))
    }
}

#[cfg(test)]
mod tests {
    use super::Yaml;
    use std::collections::HashMap;

    #[test]
    fn yaml_parser_test() {
        let yaml = Yaml::new(
            vec![
                String::from("title"),
                String::from("date"),
                String::from("author"),
            ],
            String::from(
                r#"title: The first!!
date: 2027-12-27
author: Somebody"#,
            ),
        );

        let mut expected_result: HashMap<String, String> = HashMap::new();
        expected_result.insert(String::from("title"), String::from("The first!!"));
        expected_result.insert(String::from("date"), String::from("2027-12-27"));
        expected_result.insert(String::from("author"), String::from("Somebody"));

        assert_eq!(expected_result, yaml.parse());
    }
}
