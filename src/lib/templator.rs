use crate::config::Config;
use serde_derive::Serialize;
use std::collections::HashMap;
use tera::{Context, Tera};

fn template_one(text: &str, context: &Context, auto_escape: bool) -> String {
    Tera::one_off(text, context, auto_escape).expect("could not render this template")
}

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    config: Config,
    body: String,
    header: HashMap<String, String>,
    file_name: String,
}

#[derive(Serialize, Debug)]
pub struct Index<'a> {
    config: &'a Config,
    posts: &'a [Post],
}

#[derive(Serialize, Debug)]
pub struct Feed<'a> {
    config: &'a Config,
    posts: &'a [Post],
}

impl Post {
    pub fn new(
        config: Config,
        body: String,
        header: HashMap<String, String>,
        file_name: String,
    ) -> Post {
        Post {
            config,
            body,
            header,
            file_name,
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
    pub fn new<'a>(config: &'a Config, posts: &'a [Post]) -> Index<'a> {
        Index { config, posts }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("posts", &self.posts);

        template_one(file_content, &context, false)
    }
}

impl Feed<'_> {
    pub fn new<'a>(config: &'a Config, posts: &'a [Post]) -> Feed<'a> {
        Feed { config, posts }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("posts", &self.posts);

        template_one(file_content, &context, true)
    }
}
