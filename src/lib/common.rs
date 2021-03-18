use regex::Regex;

pub fn split_header_from_markdown(markdown: &str) -> Vec<String> {
    let re = Regex::new(r"\A---\n((.|\n)*?)---\n((.|\n)*)").expect("There is a problem in regex");

    let caps = re
        .captures(markdown)
        .expect("Could not split header and body");

    vec![caps[1].to_string(), caps[3].to_string()]
}

#[cfg(test)]
mod tests {

    use super::split_header_from_markdown;
    #[test]
    fn split_header_test() {
        let expected_result = vec![
            include_str!("../test_includes/markdown_lorem_ipsum_with_header_header.md"),
            include_str!("../test_includes/markdown_lorem_ipsum_with_header_body.md"),
        ];

        assert_eq!(
            expected_result,
            split_header_from_markdown(include_str!(
                "../test_includes/markdown_lorem_ipsum_with_header.md"
            ))
        );
    }
}
