use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config{
    pub base_url : String,
    pub site_name : String,
    pub site_author : String,
    pub site_description : String,
    pub post_headers : Vec<String>,
}

impl Config{
    pub fn get_defaults() -> Config{
        Config{
            base_url : String::from(""),
            site_name : String::from("STOG"),
            site_author : String::from("Somebody"),
            site_description : String::from("generated with STOG"),
            post_headers: vec![String::from("title"), String::from("date"), String::from("author")],
        }
    }
}
