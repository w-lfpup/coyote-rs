use crate::tag_info::{TagInfo, TextFormat};

fn get_index_of_first_char(text: &str) -> usize {
    for (index, glyph) in text.char_indices() {
        if !glyph.is_whitespace() {
            return index;
        }
    }

    text.len()
}

fn get_most_common_space_index(text: &str) -> usize {
    let mut space_index = text.len();
    let mut prev_line = "";

    let mut texts = text.split("\n");

    while let Some(line) = texts.next() {
        let found_index = get_index_of_first_char(line);
        if found_index < line.len() {
            space_index = found_index;
            prev_line = line;
            break;
        }
    }

    while let Some(line) = texts.next() {
        // combine these two to stop doubling up on text char arrays
        let found_index = get_most_common_space_index_between_two_strings(prev_line, line);
        if found_index < space_index {
            space_index = curr_index
        }

        prev_line = line;
    }

    space_index
}

fn get_most_common_space_index_between_two_strings(source: &str, target: &str) -> usize {
    let mut source_chars = source.char_indices();
    let mut target_chars = target.chars();

    let mut prev_index = 0;
    while let (Some((src_index, src_chr)), Some(tgt_chr)) =
        (source_chars.next(), target_chars.next())
    {
        if src_chr == tgt_chr && src_chr.is_whitespace() {
            prev_index = src_index;
            continue;
        }

        return src_index;
    }

    prev_index
}

fn all_spaces(line: &str) -> bool {
    line.len() == get_index_of_first_char(line)
}

fn add_text(results: &mut String, text: &str, tag_info: &TagInfo) {
    let mut text_iter = text.split("\n");

    // add space

    // if inline
    // add text without new line

    // then add new lines

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line.trim());
        break;
    }

    while let Some(line) = text_iter.next() {
        if all_spaces(line) {
            continue;
        }

        results.push('\n');
        results.push_str(&"\t".repeat(tag_info.indent_count));
        results.push_str(line.trim());
    }
}
