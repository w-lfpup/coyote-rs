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
        let found_index = get_index_of_first_char(line);
        if 0 == found_index {
            continue;
        }
        if line.len() == found_index {
            continue;
        }

        if found_index < line.len() {
            space_index = found_index;
            prev_line = line;
            break;
        }
    }

    // then get the most common space prefix in the next lines
    while let Some(line) = texts.next() {
        // combine these two to stop doubling up on text char arrays
        let found_index = get_index_of_first_char(line);
        if 0 == found_index {
            continue;
        }
        // if line.len() == found_index {
        //     continue;
        // }

        let mut prev_line_chars = prev_line.char_indices();
        let mut line_chars = line.char_indices();

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

    println!("space index {}", space_index);
    space_index
}

fn get_most_common_middle_space_index(texts: &[&str]) -> usize {
    let mut space_index = 0;
    let mut prev_line = "";

    let mut text_iter = texts.iter();

    // get the first line with spaces that isn't all spaces
    while let Some(line) = text_iter.next() {
        // let space_index = get_index_of_first_char(line);
        // let prev_line = line;
        let found_index = get_index_of_first_char(line);
        if line.len() == found_index {
            continue;
        }
        if 0 == found_index {
            return 0;
        }

        space_index = found_index;
        prev_line = line;
        break;
    }

    // then get the most common space prefix in the next lines
    while let Some(line) = text_iter.next() {
        // combine these two to stop doubling up on text char arrays
        let found_index = get_index_of_first_char(line);
        if line.len() == found_index {
            continue;
        }
        if 0 == found_index {
            return 0;
        }

        let mut prev_line_chars = prev_line.char_indices();
        let mut line_chars = line.char_indices();

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
    if tag_info.banned_path {
        return;
    }

    if tag_info.preserved_text_path {
        results.push_str(text);
        return;
    }

    let texts: Vec<&str> = text.split("\n").collect();
    if 0 == texts.len() {
        return;
    }
    if 1 == texts.len() {
        results.push_str(&texts[0]);
        return;
    }

    let first = texts[0];
    let middle = &texts[1..texts.len() - 1];
    let last = texts[texts.len() - 1];

    let common_middle_space_index = get_most_common_middle_space_index(middle);

    // first
    results.push_str(first.trim());

    // middle
    for line in middle {
        results.push('\n');

        let found_index = get_index_of_first_char(line);

        match found_index {
            0 => {
                results.push_str(first.trim_end());
            }
            _ => {
                results.push_str(&"\t".repeat(tag_info.indent_count));
                results.push_str(line[common_middle_space_index..].trim_end());
            }
        }
    }

    // last
    results.push('\n');
    results.push_str(&"\t".repeat(tag_info.indent_count - 1));
    results.push_str(last.trim())
}

pub fn push_text_component(results: &mut String, text: &str, tag_info: &TagInfo) {
    if tag_info.banned_path {
        return;
    }

    if tag_info.preserved_text_path {
        results.push_str(text);
        return;
    }

    let common_space_index = get_most_common_space_index(text);

    let mut text_iter = text.split("\n");

    if let Some(line) = text_iter.next() {
        if !all_spaces(line) {
            match tag_info.text_format {
                TextFormat::LineSpace => {
                    results.push('\n');
                    results.push_str(&"\t".repeat(tag_info.indent_count));
                    results.push_str(line[common_space_index..].trim_end());
                }
                _ => {
                    results.push_str(line.trim());
                }
            }
        }
    }

    while let Some(line) = text_iter.next() {
        results.push('\n');
        if all_spaces(line) {
            continue;
        }

        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line[common_space_index..].trim_end());
    }
}
