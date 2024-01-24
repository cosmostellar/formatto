#[cfg(test)]
mod get_sections {
    use crate::{
        testing::setup,
        tools::{
            parsing::get_sections,
            tokens::{HeadingLevel, MarkdownSection},
        },
    };

    #[test]
    fn invalid_input() {
        setup();

        let input = r#"```
code
SPACE```"#;
        let sections = get_sections(input);
        assert!(sections.is_err());
    }

    #[test]
    fn non_headings() {
        setup();

        let input = r#"##Heading 2
###Heading 3
####Heading 4"#;

        let expected_output = vec![MarkdownSection::Content(
            r#"##Heading 2
###Heading 3
####Heading 4"#
                .to_string(),
        )];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn only_headings_1() {
        setup();

        let input = r#"## Heading 2
## Heading 2
## Heading 2"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn only_headings_2() {
        setup();

        let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
            MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn only_content() {
        setup();

        let input = r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

        let expected_output = vec![MarkdownSection::Content(
            r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
                .to_string(),
        )];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn sub_heading() {
        setup();

        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::Sub("### Heading 3".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn properties_1() {
        setup();

        let input = r#"---
aliases:
- Test
---

## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

        let expected_output = vec![
            MarkdownSection::Property("---\naliases:\n- Test\n---".to_string()),
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn properties_2() {
        setup();

        let input = r#"---
aliases:
- Test
---
"#;

        let expected_output = vec![MarkdownSection::Property(
            "---\naliases:\n- Test\n---".to_string(),
        )];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn random_line_breaks() {
        setup();

        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn no_subheadings() {
        setup();

        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
## Heading 2"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn two_headings_with_each_content() {
        setup();

        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn empty_input() {
        setup();

        let input = "";
        let expected_output = vec![];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn code_block() {
        setup();

        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
println!(\"Hello World\");
) {}
```"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
            MarkdownSection::Code(
                r#"```rust
fn main(
println!(\"Hello World\");
) {}
```"#
                    .to_string(),
            ),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn headings_without_letters() {
        setup();

        let input = r#"#
##
##
##
##
###
###
####
####
##
#
"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
            MarkdownSection::Heading(HeadingLevel::FirstSub("##".to_string())),
            MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
            MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
            MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
            MarkdownSection::Heading(HeadingLevel::FirstSub("###".to_string())),
            MarkdownSection::Heading(HeadingLevel::Sub("###".to_string())),
            MarkdownSection::Heading(HeadingLevel::FirstSub("####".to_string())),
            MarkdownSection::Heading(HeadingLevel::Sub("####".to_string())),
            MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
            MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }

    #[test]
    fn contents_with_line_breaks() {
        setup();

        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

\
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
"#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
            MarkdownSection::Content(
                r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.

\
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
                    .to_string(),
            ),
        ];

        assert_eq!(get_sections(input).unwrap(), expected_output);
    }
}