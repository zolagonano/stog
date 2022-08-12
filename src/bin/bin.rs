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
    let dir_list = [
        blog_name,
        &format!("{}/_posts", blog_name),
        &format!("{}/_templates", blog_name),
        &format!("{}/public", blog_name),
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
    config: &config::Config,
    post_path: &str,
    build_dir: &(String, String, bool),
) -> Result<templator::Post, Box<dyn std::error::Error>> {
    let post_file = common::read_file(post_path)?;

    let post_parser = PostParser::new(&post_file);

    let header = post_parser.parse_header();
    let body = post_parser.parse_md();

    let file_name = post_path.replace(&build_dir.0, "").replace(".md", ".html");

    Ok(templator::Post::new(
        config.clone(),
        body,
        header,
        format!("{}{}", build_dir.1, file_name),
    ))
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = common::read_file("config.toml")?;
    let config = config::Config::read_config(&config_file);

    common::rm_dir(&config.output_dir)?; // Removes the build dir if exists

    common::make_dirs(&[
        &config.output_dir,
        &format!("{}/{}", &config.output_dir, &config.public_dir),
    ])?;

    common::copy_dir(&[&config.public_dir], &config.output_dir)?;

    let index_template_file = common::read_file(&format!("{}/index.html", &config.templates_dir))?;

    let post_template_file = common::read_file(&format!("{}/post.html", &config.templates_dir))?;
    let mut index_posts: Vec<templator::Post> = Vec::new();
    for build_dir in &config.build_dirs {
        common::make_dirs(&[&format!("{}/{}", &config.output_dir, &build_dir.1)])?;
        let posts: Vec<String> = common::list_dir(&build_dir.0)?
            .iter()
            .map(|item| format!("{}/{}", &build_dir.0, item))
            .collect();

        for post in posts {
            let template_post = get_post(&config, &post, build_dir)?;
            common::write_file(
                &format!(
                    "{}/{}/{}",
                    &config.output_dir,
                    &build_dir.1,
                    post.replace(&build_dir.0, "").replace(".md", ".html")
                ),
                &template_post.template_text(&post_template_file)?,
            )?;
            if build_dir.2 {
                index_posts.push(template_post);
            }
        }
    }

    let template_index = templator::Index::new(&config, &index_posts);
    common::write_file(
        &format!("{}/index.html", &config.output_dir),
        &template_index.template_text(&index_template_file)?,
    )?;

    let feed_template_file = common::read_file(&format!("{}/atom.xml", &config.templates_dir))?;
    let template_feed = templator::Feed::new(&config, &index_posts);
    common::write_file(
        &format!("{}/atom.xml", &config.output_dir),
        &template_feed.template_text(&feed_template_file)?,
    )?;
    Ok(())
}
