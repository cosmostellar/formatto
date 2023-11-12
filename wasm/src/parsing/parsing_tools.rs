use crate::{
    console_error,
    types::{
        setting_types::MainPluginSettings,
        token_types::{HeadingLevel, MarkdownSection},
    },
};
use std::{error::Error, vec};

/// Return a String value that is replacing the entire document.
pub fn get_formatted_string(
    sections: Vec<MarkdownSection>,
    settings: &MainPluginSettings,
) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    let mut right_after_properties = false;

    let mut right_after_heading = false;
    let mut right_after_code_block = false;

    let after_properties_gap = parse_str_to_usize(&settings.other_gaps.after_properties)? + 1;

    for section in sections {
        match section {
            MarkdownSection::Property(content) => {
                output.push_str(&content);

                right_after_properties = true;
                right_after_heading = false;
                right_after_code_block = false;
            }
            MarkdownSection::Heading(heading_level) => {
                match heading_level {
                    HeadingLevel::Top(content) => {
                        if output.is_empty() {
                            output.push_str(&insert_line_breaks(
                                &content,
                                if right_after_properties {
                                    after_properties_gap
                                } else {
                                    0
                                },
                                0,
                            ));
                        } else {
                            output.push_str(&insert_line_breaks(
                                &content,
                                if right_after_properties {
                                    after_properties_gap
                                } else {
                                    parse_str_to_usize(
                                        &settings.heading_gaps.before_top_level_headings,
                                    )? + 1
                                },
                                0,
                            ));
                        }
                    }
                    HeadingLevel::FirstSub(content) => {
                        let formatted = insert_line_breaks(
                            &content,
                            if right_after_properties {
                                after_properties_gap
                            } else {
                                parse_str_to_usize(&settings.heading_gaps.before_first_sub_heading)?
                                    + 1
                            },
                            0,
                        );
                        output.push_str(&formatted);
                    }
                    HeadingLevel::Sub(content) => {
                        output.push_str(&insert_line_breaks(
                            &content,
                            if right_after_properties {
                                after_properties_gap
                            } else {
                                parse_str_to_usize(&settings.heading_gaps.before_sub_headings)? + 1
                            },
                            0,
                        ));
                    }
                }

                right_after_properties = false;
                right_after_heading = true;
                right_after_code_block = false;
            }
            MarkdownSection::Content(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if right_after_properties {
                        after_properties_gap
                    } else if right_after_code_block {
                        parse_str_to_usize(&settings.other_gaps.before_contents_after_code_blocks)?
                            + 1
                    } else {
                        parse_str_to_usize(&settings.other_gaps.before_contents)? + 1
                    },
                    0,
                ));

                right_after_properties = false;
                right_after_heading = false;
                right_after_code_block = false;
            }
            MarkdownSection::Code(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if right_after_properties {
                        after_properties_gap
                    } else if right_after_heading {
                        parse_str_to_usize(&settings.other_gaps.before_code_blocks_after_headings)?
                            + 1
                    } else {
                        parse_str_to_usize(&settings.other_gaps.before_code_blocks)? + 1
                    },
                    0,
                ));

                right_after_properties = false;
                right_after_heading = false;
                right_after_code_block = true
            }
        }
    }

    Ok(output)
}

pub fn get_sections(input: &str) -> Vec<MarkdownSection> {
    if input.is_empty() {
        return vec![];
    }

    let mut sections = Vec::<MarkdownSection>::new();

    let input_lines = input.trim().split('\n');
    let input_lines_vec = input_lines.clone().collect::<Vec<&str>>();

    let mut md_properties = String::new();
    let mut is_reading_md_properties = false;

    let mut md_code_block = String::new();
    let mut is_reading_md_code_block = false;

    let md_top_heading_level = get_top_heading_level(&input_lines_vec);
    let md_top_heading_literal = "#".repeat(md_top_heading_level);

    let mut is_reading_content = false;
    let mut current_heading_level = 0;

    // Everything goes into `MarkdownSection::Content` type,
    // unless it detects a markdown syntax that needs to be handled.
    let mut md_content = String::new();

    for (index, line) in input_lines.enumerate() {
        if line.is_empty() && !is_reading_content && !is_reading_md_code_block {
            continue;
        }

        is_reading_content = true;

        // Reads Properties.
        if sections.is_empty() && (line == "---" || is_reading_md_properties) {
            is_reading_content = false;
            push_content_section(&mut sections, &mut md_content);

            let is_first_line = sections.is_empty() && md_properties.is_empty();

            // Enter and exit properties section.
            if line == "---" {
                if is_first_line {
                    md_properties.push_str(line);
                    is_reading_md_properties = true;
                    continue;
                } else if is_reading_md_properties {
                    md_properties.push('\n');
                    md_properties.push_str(line);
                    is_reading_md_properties = false;

                    sections.push(MarkdownSection::Property(md_properties.clone()));
                    continue;
                }
            }

            // Keep reading properties.
            if is_reading_md_properties {
                if !is_first_line {
                    md_properties.push('\n');
                }

                md_properties.push_str(line);
                continue;
            }
        }

        // * Read code blocks.
        if line.starts_with("```") || is_reading_md_code_block {
            is_reading_content = false;
            push_content_section(&mut sections, &mut md_content);

            // Enter and exit a code block.
            if line.starts_with("```") {
                if !is_reading_md_code_block {
                    md_code_block.push_str(line);
                    is_reading_md_code_block = true;
                    continue;
                } else {
                    md_code_block.push_str(format!("\n{}", line).as_str());
                    sections.push(MarkdownSection::Code(md_code_block.clone()));

                    // Clear the temporary code block.
                    md_code_block.clear();

                    is_reading_md_code_block = false;
                    continue;
                }
            }

            // Keep reading the code block.
            if is_reading_md_code_block {
                if !md_code_block.is_empty() {
                    md_code_block.push('\n');
                }

                md_code_block.push_str(line);
                continue;
            }
        }

        // * Read headings.
        if line.starts_with('#') {
            let is_top_heading = line.starts_with(&md_top_heading_literal)
                && !line.starts_with(format!("{}#", md_top_heading_literal).as_str());

            if is_top_heading {
                is_reading_content = false;
                push_content_section(&mut sections, &mut md_content);

                sections.push(MarkdownSection::Heading(HeadingLevel::Top(
                    line.to_string(),
                )));

                current_heading_level = md_top_heading_level;
            } else {
                // `take_while` Stops as soon as the predicate is false.
                let filtered_string = line
                    .chars()
                    .take_while(|&c| c == '#' || c == ' ')
                    .collect::<Vec<char>>();

                // `map_or` is used to handle `Option<T>` value.
                let is_sub_heading = filtered_string.last().map_or(false, |last_char| {
                    *last_char == ' ' && filtered_string.len() > 1
                });

                if is_sub_heading {
                    is_reading_content = false;
                    push_content_section(&mut sections, &mut md_content);

                    if filtered_string.len() - 1 > current_heading_level {
                        sections.push(MarkdownSection::Heading(HeadingLevel::FirstSub(
                            line.to_string(),
                        )));
                    } else {
                        sections.push(MarkdownSection::Heading(HeadingLevel::Sub(
                            line.to_string(),
                        )));
                    }

                    current_heading_level = filtered_string.len() - 1;
                }
            }
        }

        // * Read contents.
        if is_reading_content {
            append_string_with_line_breaks(&mut md_content, line);
        }

        // run this when it's the last line.
        if index == input_lines_vec.len() - 1 {
            push_content_section(&mut sections, &mut md_content);
        }
    }

    sections
}

fn push_content_section(sections: &mut Vec<MarkdownSection>, content: &mut String) {
    // Check if "content" is empty.
    // Because this function is also called with empty values.
    if content.is_empty() {
        return;
    }

    sections.push(MarkdownSection::Content(content.trim_end().to_string()));
    content.clear();
}

fn append_string_with_line_breaks(string: &mut String, line: &str) {
    // Break lines unless it's the first line.
    if !string.is_empty() {
        string.push('\n');
    }

    string.push_str(line);
}

pub fn get_top_heading_level(input_lines: &[&str]) -> usize {
    let mut top_heading_level: usize = usize::MAX;
    let mut is_reading_md_code_block = false;

    for line in input_lines {
        // Skip code blocks.
        if line.starts_with("```") {
            is_reading_md_code_block = !is_reading_md_code_block;
        }
        if is_reading_md_code_block {
            continue;
        }

        let current_line_level = line.chars().take_while(|&c| c == '#').count();

        if line.starts_with('#') && top_heading_level > current_line_level {
            top_heading_level = current_line_level;
        }
    }

    top_heading_level
}

/// Insert line breaks before and after an input.
pub fn insert_line_breaks(input: &str, before: usize, after: usize) -> String {
    let line_breaks_before = "\n".repeat(before);
    let line_breaks_after = "\n".repeat(after);

    format!("{}{}{}", line_breaks_before, input, line_breaks_after)
}

/// Parse a usize value from a &str type argument.
/// Also return an `Error` to handle exceptions.
fn parse_str_to_usize(input: &Option<String>) -> Result<usize, Box<dyn Error>> {
    match input {
        Some(input) => match input.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(err) => {
                console_error!("{}", err);
                Err(String::from(
                    "Failed to read settings. Some of them are possibly not positive number values.",
                )
                .into())
            }
        },
        None => Err(String::from("Failed to access setting properties.").into()),
    }
}
