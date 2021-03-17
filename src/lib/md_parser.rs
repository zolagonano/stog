use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};

pub struct Html<'a> {
    markdown_text: &'a str,
}

impl Html<'_> {
    pub fn new(markdown_text: &str) -> Html {
        Html {
            markdown_text: markdown_text,
        }
    }

    pub fn parse(&self) -> String {
        let options = Options::all();

        let parser = Parser::new_ext(self.markdown_text, options);

        let mut html_output = String::new();

        push_html(&mut html_output, parser);

        html_output
    }
}

#[cfg(test)]
mod tests {
    use super::Html;

    #[test]
    fn markdown_to_html_test() {
        let html = Html::new(include_str!("../test_includes/markdown_lorem_ipsum.md"));
        assert_eq!(
            include_str!("../test_includes/markdown_lorem_ipsum.html"),
            html.parse()
        );
    }
}
