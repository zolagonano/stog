use clap::{App, Arg, SubCommand};
use lib::{common, config, post_parser::PostParser, templator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("STOG: Static Blog Generator")
        .version("0.1.0")
        .author("Zola Gonano <zolagonano@protonmail.com>")
        .about("Generates a static blog from markdown files.")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize blog directory")
                .arg(Arg::with_name("blog_name").required(true)),
        )
        .subcommand(SubCommand::with_name("build").about("Builds the blog files"))
        .get_matches();

    if matches.is_present("init") {
        let blog_name = matches
            .subcommand_matches("init")
            .unwrap()
            .value_of("blog_name")
            .unwrap();

        initialize(blog_name)?;
        Ok(())
    } else if matches.is_present("build") {
        build()?;
        Ok(())
    } else {
        Err("Use --help to see usage".into())
    }
}

fn initialize(blog_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir_list = vec![
        String::from(blog_name),
        format!("{}/_posts", blog_name),
        format!("{}/_templates", blog_name),
        format!("{}/public", blog_name),
    ];

    common::make_dirs(&dir_list)?;

    let config = config::Config::get_defaults_string();

    let write_list: &[&(String, &str)] = &[
        &(format!("{}/config.toml", blog_name), &config),
        &(
            format!("{}/_posts/lorem1.md", blog_name),
            include_str!("../init_files/posts/lorem1.md"),
        ),
        &(
            format!("{}/_posts/lorem2.md", blog_name),
            include_str!("../init_files/posts/lorem2.md"),
        ),
        &(
            format!("{}/_templates/index.html", blog_name),
            include_str!("../init_files/templates/index.html"),
        ),
        &(
            format!("{}/_templates/post.html", blog_name),
            include_str!("../init_files/templates/post.html"),
        ),
        &(
            format!("{}/_templates/atom.xml", blog_name),
            include_str!("../init_files/templates/atom.xml"),
        ),
        &(
            format!("{}/public/style.css", blog_name),
            include_str!("../init_files/theme/style.css"),
        ),
        &(
            format!("{}/public/skeleton.min.css", blog_name),
            include_str!("../init_files/theme/skeleton.min.css"),
        ),
        &(
            format!("{}/public/normalize.min.css", blog_name),
            include_str!("../init_files/theme/normalize.min.css"),
        ),
    ];

    for write_data in write_list {
        common::write_file(&write_data.0, write_data.1)?;
    }
    Ok(())
}

fn get_post(
    config: config::Config,
    post_path: String,
) -> Result<templator::Post, Box<dyn std::error::Error>> {
    let post_file = common::read_file(&post_path)?;

    let post_parser = PostParser::new(&post_file, &config.post_headers);

    let header = post_parser.parse_header();
    let body = post_parser.parse_md();

    let file_name = post_path.replace("_posts/", "").replace(".md", ".html");

    Ok(templator::Post::new(config, body, header, file_name))
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = common::pwd()?;
    let dir_list = common::list_dir(&current_dir)?;

    let is_blog_dir = dir_list.contains(&"config.toml".to_string())
        && dir_list.contains(&"_posts".to_string())
        && dir_list.contains(&"public".to_string())
        && dir_list.contains(&"_templates".to_string());

    if dir_list.contains(&"_build".to_string()) {
        common::rm_dir("_build")?;
    }

    if is_blog_dir {
        let config_file = common::read_file("config.toml")?;
        let config = config::Config::read_config(&config_file);

        common::make_dirs(&[
            String::from("_build"),
            String::from("_build/posts"),
            String::from("_build/public"),
        ])?;

        common::copy_dir(&["public"], "_build")?;

        let posts: Vec<String> = common::list_dir("_posts")?
            .iter()
            .map(|item| format!("_posts/{}", item))
            .collect();

        let mut index_posts: Vec<templator::Post> = Vec::new();

        let post_template_file = common::read_file("_templates/post.html")?;
        for post in posts {
            let template_post = get_post(config.clone(), post.clone())?;
            common::write_file(
                &format!(
                    "_build/posts/{}",
                    post.replace("_posts/", "").replace(".md", ".html")
                ),
                &template_post.template_text(&post_template_file),
            )?;

            index_posts.push(template_post.clone());
        }

        let index_template_file = common::read_file("_templates/index.html")?;
        let template_index = templator::Index::new(&config, &index_posts);
        common::write_file(
            "_build/index.html",
            &template_index.template_text(&index_template_file),
        )?;

        let feed_template_file = common::read_file("_templates/atom.xml")?;
        let template_feed = templator::Feed::new(&config, &index_posts);
        common::write_file(
            "_build/atom.xml",
            &template_feed.template_text(&feed_template_file),
        )?;
        Ok(())
    } else {
        Err("Could not find necessary directories!!!".into())
    }
}
