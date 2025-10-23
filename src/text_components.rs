use crate::tag_info::{TagInfo, TextFormat};
use std::cmp;

fn get_index_of_first_char(text: &str) -> usize {
    for (index, glyph) in text.char_indices() {
        if !glyph.is_whitespace() {
            return index;
        }
    }

    text.len()
}

fn get_most_common_space_index(text: &str) -> usize {
    let mut space_index = 0;
    let mut prev_line = "";

    let mut texts = text.split("\n");

    // get the first line with spaces that isn't all spaces
    while let Some(line) = texts.next() {
        if "" == line {
            continue;
        }

        let found_index = get_index_of_first_char(line);
        if found_index < line.len() {
            space_index = found_index;
            prev_line = line;
            break;
        }
    }

    // then get the most common space prefix in the next lines
    while let Some(line) = texts.next() {
        // combine these two to stop doubling up on text char arrays

        let mut prev_line_chars = prev_line.char_indices();
        let mut line_chars = line.char_indices();

        let mut found_index = line.len();
        while let (Some((src_index, src_chr)), Some((_, tgt_chr))) =
            (prev_line_chars.next(), line_chars.next())
        {
            if src_chr == tgt_chr && src_chr.is_whitespace() {
                continue;
            }

            space_index = cmp::min(space_index, src_index);

            break;
        }

        prev_line = line;
    }

    space_index
}

fn all_spaces(line: &str) -> bool {
    line.len() == get_index_of_first_char(line)
}

// add alt text
//
pub fn push_alt_text_component(results: &mut String, text: &str, tag_info: &TagInfo) {
    println!("alt text stuff!:\n{}", text);
    if tag_info.banned_path {
        return;
    }

    if tag_info.preserved_text_path {
        results.push_str(text);
        return;
    }

    let common_space_index = get_most_common_space_index(text);

    // first line in alt text should be zero?

    let mut text_iter = text.split("\n");
    let mut empty_lines = 0;
    let mut last_line_all_spaces = false;
    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            last_line_all_spaces = true;
            empty_lines += 1;
            if 1 < empty_lines {
                results.push('\n');
            }
            continue;
        }

        empty_lines = 1;
        last_line_all_spaces = false;
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line[common_space_index..].trim_end());
    }

    if !last_line_all_spaces {
        results.push('\n');
    }
}

pub fn push_text_component(results: &mut String, text: &str, tag_info: &TagInfo) {
    println!("text stuff!:\n{}", text);
    if tag_info.banned_path {
        return;
    }

    if tag_info.preserved_text_path {
        results.push_str(text);
        return;
    }

    let common_space_index = get_most_common_space_index(text);

    // first line in alt text should be zero?

    let mut text_iter = text.split("\n");
    // first line is ? not "" push line
    // other wise
    //.  Block | Inilne ? push line
    //.

    let mut empty_lines = 0;
    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            empty_lines += 1;
            if 1 < empty_lines {
                results.push('\n');
            }
            continue;
        }

        empty_lines = 1;
        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line[common_space_index..].trim_end());
    }
}
