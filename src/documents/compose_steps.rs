use crate::documents::tag_info::{TagInfo, TextFormat};
use crate::documents::text_components::{push_alt_text_component, push_multiline_attributes};
use crate::template_steps::{RulesetImpl, Step, StepKind, get_text_from_step};

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
            StepKind::TailElementClosed => close_tail_tag(results, tag_info_stack),
            StepKind::Text => push_text(results, tag_info_stack, rules, template_str, step),
            StepKind::TextAlt => push_alt_text(results, tag_info_stack, rules, template_str, step),
            StepKind::TextLineSpace => {
                push_text_space(results, tag_info_stack, rules, template_str, step)
            }
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
                push_text_space(results, tag_info_stack, rules, template_str, step)
            }
            StepKind::ElementLineSpace => {
                push_text_space(results, tag_info_stack, rules, template_str, step)
            }
            StepKind::InjectionConfirmed => {
                push_injection_confirmed(results, tag_info_stack, rules, template_str, step)
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
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    let text = get_text_from_step(template_str, step);
    push_space_on_text(results, &tag_info);
    results.push_str(text);

    tag_info.text_format = TextFormat::Text;
}

// SET SOME KIND OF TEXT FORMAT
fn push_alt_text(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
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
    push_alt_text_component(results, rules, text, tag_info);

    tag_info.text_format = TextFormat::Text;
}

fn push_injection_confirmed(
    _results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    _template_str: &str,
    _step: &Step,
) {
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        // this should never happen
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    // This feels heavy handed and suspect.
    // More interesting logic might be necessary
    tag_info.text_format = TextFormat::Text;
}

fn push_text_space(
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

    if tag_info.preserved_text_path {
        let text = get_text_from_step(template_str, step);
        results.push_str(text);
        return;
    }

    if TextFormat::Initial == tag_info.text_format || TextFormat::LineSpace == tag_info.text_format
    {
        return;
    }

    match step.kind {
        StepKind::ElementLineSpace => {
            tag_info.text_format = TextFormat::LineSpace;
        }
        StepKind::TextLineSpace => {
            tag_info.text_format = TextFormat::LineSpace;
        }
        _ => {
            tag_info.text_format = TextFormat::Space;
        }
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
        push_space_on_text(results, &tag_info);
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

    if !tag_info.banned_path {
        results.push_str(">");
        tag_info.text_format = TextFormat::Text;
    }

    if !tag_info.void_el {
        return;
    }

    if let Some(_) = stack.pop() {
        let prev_tag_info = match stack.last_mut() {
            Some(tag_info) => tag_info,
            _ => return,
        };

        prev_tag_info.text_format = TextFormat::Text;
    };
}

fn close_empty_element(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.pop() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    match "html" != tag_info.namespace {
        true => results.push_str("/>"),
        _ => match tag_info.void_el {
            true => results.push('>'),
            _ => {
                results.push_str("></");
                results.push_str(&tag_info.tag);
                results.push('>');
            }
        },
    }

    let prev_tag_info = match stack.last_mut() {
        Some(prev_tag_info) => prev_tag_info,
        _ => return,
    };

    prev_tag_info.text_format = TextFormat::Text;
}

fn pop_element(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    template_str: &str,
    step: &Step,
) {
    if stack.len() < 2 {
        return;
    }

    let tag_info = match stack.last() {
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

    if let Some(close_tag) = rules.get_contentless_tag_from_close_sequence(tag) {
        closed_tag = close_tag;
    }

    // mismatched tags? bail
    if closed_tag != tag_info.tag {
        return;
    }

    if tag_info.void_el {
        return;
    }

    if let (None, None) = (
        rules.get_alt_text_tag_from_close_sequence(tag),
        rules.get_contentless_tag_from_close_sequence(tag),
    ) {
        if let Some(prev_tag_info) = stack.get(stack.len() - 2) {
            push_space_on_pop(results, &prev_tag_info, &tag_info);
        };
    }

    // better if else here
    match tag == closed_tag {
        true => {
            results.push_str("</");
            results.push_str(tag);
        }
        _ => results.push_str(tag),
    }
}

fn close_tail_tag(results: &mut String, stack: &mut Vec<TagInfo>) {
    let tag_info = match stack.pop() {
        Some(tag_info) => tag_info,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str(">");

    let prev_tag_info = match stack.last_mut() {
        Some(tag_info) => tag_info,
        _ => return,
    };

    prev_tag_info.text_format = TextFormat::Text;
}

fn push_attr(results: &mut String, stack: &mut Vec<TagInfo>, template_str: &str, step: &Step) {
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    if !tag_info.preserved_text_path {
        match tag_info.text_format {
            TextFormat::Space => results.push(' '),
            TextFormat::LineSpace => {
                results.push('\n');
                results.push_str(&"\t".repeat(tag_info.indent_count))
            }
            _ => {}
        }
    }

    let attr = get_text_from_step(template_str, step);
    results.push_str(attr.trim());

    tag_info.text_format = TextFormat::Text
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

    let text = get_text_from_step(template_str, step);
    results.push('=');
    results.push_str(text);
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

    let text = get_text_from_step(template_str, step);
    results.push_str("='");
    push_multiline_attributes(results, &text, tag_info);
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

    let text = get_text_from_step(template_str, step);
    results.push_str("=\"");
    push_multiline_attributes(results, &text, tag_info);
    results.push('"');
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

fn push_space_on_pop(results: &mut String, prev_tag_info: &TagInfo, tag_info: &TagInfo) {
    if tag_info.preserved_text_path {
        return;
    }

    match tag_info.text_format {
        TextFormat::Space => results.push(' '),
        TextFormat::LineSpace => {
            results.push('\n');
            results.push_str(&"\t".repeat(prev_tag_info.indent_count))
        }
        _ => {}
    }
}
