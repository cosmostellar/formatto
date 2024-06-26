use crate::{
    testing::{get_example_preferences, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

/// Property section only.
#[test]
fn case_1() {
    setup();

    let input = r#"---
aliases:
- Test
---
"#;

    let expected_output = vec![MarkdownSection::Property(
        "---\naliases:\n- Test\n---".to_string(),
    )];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

/// Properties with other sections.
#[test]
fn case_2() {
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

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

/// Invalid property syntax.
/// It gets read as a content section.
#[test]
fn invalid_input_1() {
    setup();

    let input = r#"---INVALID
aliases:
---
- Test
---INVALID
---INVALID
---INVALID

## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Content(
            "---INVALID\naliases:\n---\n- Test\n---INVALID\n---INVALID\n---INVALID".to_string(),
        ),
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

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

/// Invalid property syntax.
/// It gets read as a content section.
#[test]
fn invalid_input_2() {
    setup();

    let input = r#"Text

---

## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Content("Text\n\n---".to_string()),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}
