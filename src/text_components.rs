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

fn get_largest_common_space_index(texts: &[&str]) -> usize {
    let mut space_index = 0;
    let mut prev_line = "";

    let mut text_iter = texts.iter();

    // get the first line with spaces that isn't all spaces
    while let Some(line) = text_iter.next() {
        let found_index = get_index_of_first_char(line);
        if line.len() == found_index {
            continue;
        }
        if 0 == found_index {
            continue;
        }

        space_index = found_index;
        prev_line = line;
        break;
    }

    // then get the most common space prefix in the next lines
    while let Some(line) = text_iter.next() {
        let found_index = get_index_of_first_char(line);
        if line.len() == found_index {
            continue;
        }
        if 0 == found_index {
            continue;
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

    // first
    results.push_str(&texts[0]);
    if 1 == texts.len() {
        return;
    }

    // middle
    let middle = &texts[1..texts.len() - 1];
    let common_space_index = get_largest_common_space_index(middle);
    for line in middle {
        results.push('\n');

        match get_index_of_first_char(line) {
            0 => {
                if 0 < line.len() {
                    results.push_str(&"\t".repeat(tag_info.indent_count));
                    results.push_str(line.trim_end());
                }
            }
            _ => {
                results.push_str(&"\t".repeat(tag_info.indent_count));
                results.push_str(line[common_space_index..].trim_end());
            }
        }
    }

    // last
    let last = texts[texts.len() - 1];
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

    let texts: Vec<&str> = text.split("\n").collect();
    if 0 == texts.len() {
        return;
    }

    let first_line = texts[0];
    match tag_info.text_format {
        TextFormat::LineSpace => {
            results.push('\n');
            if 0 < first_line.len() {
                results.push_str(&"\t".repeat(tag_info.indent_count));
            }
        }
        TextFormat::Space => {
            if 0 < first_line.len() {
                results.push(' ');
            }
        }
        _ => {}
    }
    push_line_of_text(results, first_line);

    let middle_lines = &texts[1..texts.len()];
    let common_space_index = get_largest_common_space_index(middle_lines);
    for line in middle_lines {
        results.push('\n');
        if line.len() == get_index_of_first_char(line) {
            continue;
        }

        results.push_str(&"\t".repeat(tag_info.indent_count));
        push_line_of_text(results, &line[common_space_index..]);
    }
}

pub fn push_multiline_attributes(results: &mut String, text: &str, tag_info: &TagInfo) {
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

    // first
    push_line_of_text(results, texts[0]);
    if 1 == texts.len() {
        return;
    }

    // middle
    let middle_lines = &texts[1..texts.len() - 1];
    let common_space_index = get_largest_common_space_index(middle_lines);
    for line in middle_lines {
        results.push('\n');
        if line.len() == get_index_of_first_char(line) {
            continue;
        }

        match get_index_of_first_char(line) {
            0 => {
                if 0 < line.len() {
                    results.push_str(&"\t".repeat(tag_info.indent_count));
                    push_line_of_text(results, line);
                }
            }
            _ => {
                results.push_str(&"\t".repeat(tag_info.indent_count));
                push_line_of_text(results, &line[common_space_index..])
            }
        }
    }

    // last
    let last = texts[texts.len() - 1];
    results.push('\n');
    results.push_str(&"\t".repeat(tag_info.indent_count));
    push_line_of_text(results, last.trim())
}

fn push_line_of_text(results: &mut String, line: &str) {
    let mut state = TextFormat::Text;

    for glyph in line.chars() {
        if glyph.is_whitespace() {
            state = TextFormat::Space;
            continue;
        }

        if state == TextFormat::Space {
            results.push(' ')
        }

        state = TextFormat::Text;
        results.push(glyph)
    }
}
