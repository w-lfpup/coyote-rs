use crate::parse::{get_text_from_step, Step};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::{TagInfo, TextFormat};

pub fn compose_steps(
    rules: &dyn RulesetImpl,
    results: &mut String,
    tag_info_stack: &mut Vec<TagInfo>,
    template_str: &str,
    steps: &Vec<Step>,
) {
    for step in steps {
        match step.kind {
            StepKind::Tag => push_element(results, tag_info_stack, rules, template_str, step),
            StepKind::ElementClosed => close_element(results, tag_info_stack),
            StepKind::EmptyElementClosed => close_empty_element(results, tag_info_stack),
            StepKind::TailTag => pop_element(results, tag_info_stack, rules, template_str, step),
            StepKind::Text => push_text(results, tag_info_stack, rules, template_str, step),
            StepKind::TextAlt => push_alt_text(results, tag_info_stack, rules, template_str, step),
            StepKind::TextSpace => {
                push_text_space(results, tag_info_stack, rules, template_str, step)
            }
            StepKind::Attr => push_attr(results, tag_info_stack, template_str, step),
            StepKind::AttrValueSingleQuoted => {
                push_attr_value_single_quoted(results, tag_info_stack, template_str, step)
            }
            StepKind::AttrValueDoubleQuoted => {
                push_attr_value_double_quoted(results, tag_info_stack, template_str, step)
            }
            StepKind::AttrValueUnquoted => {
                push_attr_value_unquoted(results, tag_info_stack, template_str, step)
            }
            _ => {}
        }
    }
}

fn push_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let stack_len = stack.len();
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    push_space_acordingly(results, stack_len, tag_info);

    tag_info.text_format = TextFormat::Text;

    let text = get_text_from_step(template_str, step);
    results.push_str(text);
}

// SET SOME KIND OF TEXT FORMAT
fn push_alt_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let text = get_text_from_step(template_str, step);
    results.push_str(text);
}

// THIS DEPENDS whether the space is INLINE or in an ALT TEXT
fn push_text_space(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let stack_len = stack.len();
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    if stack_len < 2 && TextFormat::Initial == tag_info.text_format {
        return;
    }

    let text = get_text_from_step(template_str, step);

    tag_info.text_format = TextFormat::Space;
    if (text.contains("\n")) {
        tag_info.text_format = TextFormat::LineSpace;
    }

    // preserved text
    if tag_info.preserved_text_path {
        results.push_str(text);
    }
}

fn push_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last() {
        Some(tag_info) => tag_info,
        _ => {
            // this never happens
            return;
        }
    };

    let tag = get_text_from_step(template_str, step);
    let next_tag_info = TagInfo::from(rules, tag_info, tag);

    // banned path
    if !next_tag_info.banned_path {
        push_space_acordingly(results, stack.len(), &next_tag_info);
        results.push('<');
        results.push_str(tag);
    }

    stack.push(next_tag_info);
}

fn close_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.last() {
        Some(tag_info) => tag_info,
        _ => return,
    };

    if !tag_info.banned_path {
        results.push_str(">");
    }

    // DONT NEED THIS BECAUSE IT"S ALWAYS INITIAL INITIALLY

    if tag_info.void_el && "html" == tag_info.namespace {
        stack.pop();
    }

    let next_tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    next_tag_info.text_format = TextFormat::Text;
}

fn close_empty_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.pop() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    if "html" != tag_info.namespace {
        results.push_str("/>");
    } else {
        if !tag_info.void_el {
            results.push_str("></");
            results.push_str(&tag_info.tag);
        }

        results.push('>');
    }

    let next_tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    next_tag_info.text_format = TextFormat::Text;
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.pop() {
        Some(ti) => ti,
        _ => {
            // never happens
            return;
        }
    };

    if tag_info.banned_path {
        return;
    }

    let mut tag = get_text_from_step(template_str, step);
    let mut closed_tag = tag.clone();
    if let Some(close_tag) = rules.get_alt_text_tag_from_close_sequence(tag) {
        closed_tag = close_tag;
    }

    if closed_tag != tag_info.tag {
        return;
    }

    push_space_acordingly(results, stack.len(), &tag_info);

    if !tag_info.void_el {
        if let None = rules.get_close_sequence_from_alt_text_tag(closed_tag) {
            results.push_str("</");
        }

        results.push_str(tag);
    }

    results.push('>');

    // Reset text formating
    let next_tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    next_tag_info.text_format = TextFormat::Text;
}

fn push_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: &Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push(' ');
    let attr = get_text_from_step(template_str, step);
    results.push_str(attr.trim());
}

fn push_attr_value_single_quoted(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str("='");
    let val = get_text_from_step(template_str, step);
    results.push_str(val);
    results.push('\'');
}

fn push_attr_value_double_quoted(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str("=\"");
    let val = get_text_from_step(template_str, step);
    results.push_str(val);
    results.push('"');
}

fn push_attr_value_unquoted(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let val = get_text_from_step(template_str, step);
    results.push('=');
    results.push_str(val);
}

// fn all_spaces(line: &str) -> bool {
//     line.len() == get_index_of_first_char(line)
// }

fn get_index_of_first_char(text: &str) -> usize {
    for (index, glyph) in text.char_indices() {
        if !glyph.is_whitespace() {
            return index;
        }
    }

    text.len()
}

fn push_space_regardless(results: &mut String, stack_len: usize, tag_info: &TagInfo) {
    if tag_info.preserved_text_path {
        return;
    }

    if TextFormat::Space == tag_info.text_format {
        results.push(' ');
    }

    if TextFormat::LineSpace == tag_info.text_format {
        results.push('\n');
    }
}

fn push_space_acordingly(results: &mut String, stack_len: usize, tag_info: &TagInfo) {
    if tag_info.preserved_text_path {
        return;
    }

    if TextFormat::Space == tag_info.text_format {
        results.push(' ');
    }

    if TextFormat::LineSpace == tag_info.text_format {
        results.push('\n');
        if stack_len > 1 {
            results.push_str(&"\t".repeat(tag_info.indent_count + 1))
        }
    }
}
