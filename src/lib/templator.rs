use crate::config::Config;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use tera::{Context, Tera};

fn template_one(text: &str, context: &Context, auto_escape: bool) -> String {
    Tera::one_off(text, context, auto_escape).expect("could not render this template")
}

#[derive(Serialize, Debug, Clone)]
pub struct MetaData {
    file_name: String,
    file_id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Post {
    config: Config,
    body: String,
    header: HashMap<String, String>,
    metadata: MetaData,
}

#[derive(Serialize, Debug)]
pub struct Index<'a> {
    config: &'a Config,
    posts: &'a Vec<Post>,
}

#[derive(Serialize, Debug)]
pub struct Feed<'a> {
    config: &'a Config,
    posts: &'a Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitConfig {
    config: Config,
}

impl MetaData {
    pub fn new(file_name: String, file_id: String) -> MetaData {
        MetaData {
            file_name: file_name,
            file_id: file_id,
        }
    }
}

impl Post {
    pub fn new(
        config: Config,
        body: String,
        header: HashMap<String, String>,
        metadata: MetaData,
    ) -> Post {
        Post {
            config: config,
            body: body,
            header: header,
            metadata: metadata,
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
    pub fn new<'a>(config: &'a Config, posts: &'a Vec<Post>) -> Index<'a> {
        Index {
            config: config,
            posts: posts,
        }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("posts", &self.posts);

        template_one(file_content, &context, false)
    }
}

impl Feed<'_> {
    pub fn new<'a>(config: &'a Config, posts: &'a Vec<Post>) -> Feed<'a> {
        Feed {
            config: config,
            posts: posts,
        }
    }

    pub fn template_text(&self, file_content: &str) -> String {
        let mut context = Context::new();

        context.insert("config", &self.config);
        context.insert("posts", &self.posts);

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
