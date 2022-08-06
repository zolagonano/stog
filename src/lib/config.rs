use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub site_name: String,
    pub site_author: String,
    pub site_description: String,
    pub build_dirs: Vec<(String, String, bool)>,
    pub public_dir: String,
    pub templates_dir: String,
    pub output_dir: String,
}

impl Config {
    pub fn read_config(toml_text: &str) -> Config {
        let config: Config = toml::from_str(toml_text).expect("could not parse the config file");
        config
    }
    pub fn get_defaults_string() -> String {
        let default_config = Config {
            base_url: String::from(""),
            site_name: String::from("STOG"),
            site_author: String::from("Somebody"),
            site_description: String::from("generated with STOG"),
            build_dirs: vec![(String::from("_posts"), String::from("posts"), true)],
            public_dir: String::from("public"),
            templates_dir: String::from("_templates"),
            output_dir: String::from("_build"),
        };

        toml::to_string_pretty(&default_config).unwrap()
    }
}
