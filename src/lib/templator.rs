use crate::config::Config;
use std::collections::HashMap;
use tera::{Context, Tera};

fn template_one(text: &str, context: &Context, auto_escape: bool) -> String {
    Tera::one_off(text, context, auto_escape).expect("could not render this template")
}

pub struct Post<'a> {
    config: Config,
    body: String,
    header: HashMap<&'a str, String>,
}

pub struct Index<'a> {
    config: Config,
    headers: Vec<HashMap<&'a str, String>>,
    descriptions: Vec<String>,
}

pub struct Feed<'a> {
    config: Config,
    headers: Vec<HashMap<&'a str, String>>,
    bodies: Vec<String>,
}

pub struct InitConfig {
    config: Config,
}

impl Post<'_> {
    pub fn new(config: Config, body: String, header: HashMap<&str, String>) -> Post {
        Post {
            config: config,
            body: body,
            header: header,
        }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("post", &(&self.body, &self.header));

        template_one(file_content, &context, false)
    }
}

impl Index<'_> {
    pub fn new(
        config: Config,
        headers: Vec<HashMap<&str, String>>,
        descriptions: Vec<String>,
    ) -> Index {
        Index {
            config: config,
            headers: headers,
            descriptions: descriptions,
        }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("posts", &(&self.descriptions, &self.headers));

        template_one(file_content, &context, false)
    }
}

impl Feed<'_> {
    pub fn new(
        config: Config,
        headers: Vec<HashMap<&str, String>>,
        bodies: Vec<String>,
    ) -> Feed {
        Feed {
            config: config,
            headers: headers,
            bodies: bodies,
        }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("post", &(&self.bodies, &self.headers));

        template_one(file_content, &context, true)
    }
}

impl InitConfig {
    pub fn new(config: Config) -> InitConfig {
        InitConfig { config: config }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);

        template_one(file_content, &context, false)
    }
}
