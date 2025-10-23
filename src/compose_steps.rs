use crate::parse::{get_text_from_step, Step};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::{TagInfo, TextFormat};
use crate::text_components::push_alt_text_component;

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
            StepKind::ElementSpace => {
                push_element_space(results, tag_info_stack, rules, template_str, step)
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

    // push_space_for_text(results, stack_len, tag_info);
    push_space_on_text(results, &tag_info);

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
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let text = get_text_from_step(template_str, step);
    push_alt_text_component(results, text, tag_info);

    tag_info.text_format = TextFormat::LineSpace;
    // prev_tag_info.text_format = TextFormat::BlockClose;
    if tag_info.inline_el {
        tag_info.text_format = TextFormat::Text;
    }
}

fn push_element_space(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let text = get_text_from_step(template_str, step);

    if tag_info.preserved_text_path {
        results.push_str(text);
    }

    match text.contains("\n") {
        true => tag_info.text_format = TextFormat::ElementLineSpace,
        _ => tag_info.text_format = TextFormat::ElementSpace,
    }
}

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

    let text = get_text_from_step(template_str, step);

    if tag_info.preserved_text_path {
        results.push_str(text);
    }

    if TextFormat::Initial == tag_info.text_format {
        return;
    }

    tag_info.text_format = TextFormat::Space;
    if text.contains("\n") {
        tag_info.text_format = TextFormat::LineSpace;
    }
}

fn push_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    let tag_info = match stack.last_mut() {
        Some(tag_info) => tag_info,
        _ => {
            // this never happens
            return;
        }
    };

    let tag = get_text_from_step(template_str, step);
    let next_tag_info = TagInfo::from(rules, tag_info, tag);

    if !next_tag_info.banned_path {
        push_space_on_push(results, &tag_info, &next_tag_info);
        results.push('<');
        results.push_str(tag);
    }

    stack.push(next_tag_info);
}

fn close_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.last_mut() {
        Some(tag_info) => tag_info,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str(">");

    if tag_info.void_el {
        if let Some(info) = stack.pop() {
            let prev_tag_info = match stack.last_mut() {
                Some(tag_info) => tag_info,
                _ => return,
            };

            prev_tag_info.text_format = TextFormat::BlockClose;
            if info.inline_el {
                prev_tag_info.text_format = TextFormat::InlineClose;
            }
        };
    }
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

    let prev_tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    prev_tag_info.text_format = TextFormat::BlockClose;
    if tag_info.inline_el {
        prev_tag_info.text_format = TextFormat::InlineClose;
    }
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

    let tag = get_text_from_step(template_str, step);
    let mut closed_tag = tag;
    if let Some(close_tag) = rules.get_alt_text_tag_from_close_sequence(tag) {
        closed_tag = close_tag;
    }

    if closed_tag != tag_info.tag {
        return;
    }

    // push_space_for_pop_element(results, stack.len(), &tag_info);
    let prev_tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    if !tag_info.void_el {
        push_space_on_pop(results, &prev_tag_info, &tag_info);
        if let None = rules.get_close_sequence_from_alt_text_tag(closed_tag) {
            results.push_str("</");
        }

        results.push_str(tag);
    }

    results.push('>');

    prev_tag_info.text_format = TextFormat::BlockClose;
    if tag_info.inline_el {
        prev_tag_info.text_format = TextFormat::InlineClose;
    }
}

fn push_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: &Step) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    match tag_info.text_format {
        TextFormat::ElementSpace => results.push(' '),
        TextFormat::ElementLineSpace => {
            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count))
        }
        _ => {}
    }

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

fn push_space_on_text(results: &mut String, tag_info: &TagInfo) {
    if tag_info.preserved_text_path {
        return;
    }

    match tag_info.text_format {
        TextFormat::Space => results.push(' '),
        TextFormat::LineSpace => {
            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count))
        }
        _ => {}
    }
}

fn push_space_on_push(results: &mut String, prev_tag_info: &TagInfo, tag_info: &TagInfo) {
    if tag_info.preserved_text_path {
        return;
    }

    if tag_info.inline_el {
        match prev_tag_info.text_format {
            TextFormat::Space => results.push(' '),
            TextFormat::LineSpace => {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count))
            }
            _ => {}
        }
    } else {
        match prev_tag_info.text_format {
            TextFormat::Space => {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count))
            }
            TextFormat::LineSpace => {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count))
            }
            _ => {}
        }
    }
}

// need popped element
fn push_space_on_pop(results: &mut String, prev_tag_info: &TagInfo, tag_info: &TagInfo) {
    if tag_info.preserved_text_path {
        return;
    }

    if tag_info.inline_el {
        match tag_info.text_format {
            TextFormat::Space => results.push(' '),
            TextFormat::LineSpace => {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count))
            }
            _ => {}
        }
    } else {
        match tag_info.text_format {
            TextFormat::Space => {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count))
            }
            TextFormat::LineSpace => {
                results.push('\n');
                results.push_str(&"\t".repeat(prev_tag_info.indent_count))
            }
            _ => {}
        }
    }
}
