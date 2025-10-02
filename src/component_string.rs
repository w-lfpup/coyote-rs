use crate::components::Component;
use crate::compose_steps::{compose_steps, push_text_component};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::TagInfo;
use crate::template_builder::BuilderImpl;
use crate::template_steps::Results as TemplateSteps;

#[derive(Debug)]
struct TemplateBit {
    pub inj_index: usize,
    pub stack_depth: isize,
}

enum StackBit<'a> {
    Tmpl(&'a Component, TemplateSteps, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

pub fn compose_string(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &Component,
) -> Result<String, String> {
    let mut template_results = "".to_string();

    let mut tag_info_stack: Vec<TagInfo> = Vec::from([TagInfo::new(rules, ":root")]);
    let mut component_stack: Vec<StackBit> = Vec::from([get_bit_from_component_stack(
        &mut tag_info_stack,
        builder,
        rules,
        component,
    )]);

    while let Some(mut cmpnt_bit) = component_stack.pop() {
        match cmpnt_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => {
                    // escape text here
                    let escaped = text.replace("<", "&lt;");
                    push_text_component(
                        &mut template_results,
                        &mut tag_info_stack,
                        rules,
                        &escaped,
                    );
                }
                Component::UnescapedText(text) => {
                    push_text_component(&mut template_results, &mut tag_info_stack, rules, text);
                }
                Component::List(list) => {
                    for cmpnt in list.iter().rev() {
                        let bit = get_bit_from_component_stack(
                            &mut tag_info_stack,
                            builder,
                            rules,
                            cmpnt,
                        );
                        component_stack.push(bit);
                    }
                }
                _ => {}
            },
            // template chunk and possible injection
            StackBit::Tmpl(cmpnt, ref template, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                // Should always be a template
                let tmpl_cmpnt = match cmpnt {
                    Component::Tmpl(cmpnt) => cmpnt,
                    _ => continue,
                };

                // add current template chunk
                match template.steps.get(index) {
                    Some(chunk) => {
                        compose_steps(
                            rules,
                            &mut template_results,
                            &mut tag_info_stack,
                            &tmpl_cmpnt.template_str,
                            chunk,
                        );
                    }
                    _ => {
                        if bit.stack_depth != tag_info_stack.len() as isize {
                            return Err(
                                "Coyote Err: the following template component is imbalanced:\n{:?}"
                                    .to_string()
                                    + tmpl_cmpnt.template_str,
                            );
                        }
                    }
                }

                // add injections
                if let (Some(inj_step), Some(inj)) =
                    (template.injs.get(index), tmpl_cmpnt.injections.get(index))
                {
                    match inj_step.kind {
                        StepKind::AttrMapInjection => {
                            // should return error
                            add_attr_inj(&mut tag_info_stack, &mut template_results, inj);
                            // if let Err(e) = sdfsdf { return Err }
                        }
                        // push template back and bail early
                        StepKind::DescendantInjection => {
                            component_stack.push(cmpnt_bit);

                            let bit = get_bit_from_component_stack(
                                &mut tag_info_stack,
                                builder,
                                rules,
                                inj,
                            );
                            component_stack.push(bit);

                            continue;
                        }
                        _ => {}
                    }
                }

                if index < template.steps.len() {
                    component_stack.push(cmpnt_bit);
                }
            }
            _ => {}
        }
    }

    Ok(template_results)
}

fn get_bit_from_component_stack<'a>(
    stack: &mut Vec<TagInfo>,
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    cmpnt: &'a Component,
) -> StackBit<'a> {
    match cmpnt {
        Component::Text(_) => StackBit::Cmpnt(cmpnt),
        Component::List(_) => StackBit::Cmpnt(cmpnt),
        Component::Tmpl(tmpl) => {
            let template_steps = builder.build(rules, &tmpl.template_str);
            StackBit::Tmpl(
                cmpnt,
                template_steps,
                TemplateBit {
                    inj_index: 0,
                    stack_depth: stack.len() as isize,
                },
            )
        }
        _ => StackBit::None,
    }
}

// https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
// return error if attributes are not valid
fn add_attr_inj(stack: &mut Vec<TagInfo>, template_str: &mut String, cmpnt: &Component) {
    match cmpnt {
        Component::Attr(attr) => push_attr_component(template_str, stack, attr),
        Component::AttrVal(attr, val) => {
            push_attr_component(template_str, stack, attr);
            push_attr_value_component(template_str, stack, val);
        }
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        push_attr_component(template_str, stack, attr);
                    }
                    Component::AttrVal(attr, val) => {
                        push_attr_component(template_str, stack, attr);
                        push_attr_value_component(template_str, stack, val);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn check_attr_validity(attr: &str) -> bool {
    for glyph in attr.chars() {
        if forbidden_glyph(glyph) {
            return false;
        }
    }
    // if attribute does not have
    // <
    // "
    // '
    // {
    // >
    // /
    // =
    // is_whitespace

    true
}

fn forbidden_glyph(glyph: char) -> bool {
    match glyph {
        '<' => true,
        '"' => true,
        '\'' => true,
        '{' => true, // this onees for coyote, reserved char
        '>' => true,
        '/' => true,
        '=' => true,
        _ => false,
    }
}

pub fn push_attr_component(results: &mut String, stack: &mut Vec<TagInfo>, attr: &str) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push(' ');
    results.push_str(attr.trim());
}

pub fn push_attr_value_component(results: &mut String, stack: &mut Vec<TagInfo>, val: &str) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str("=\"");
    let escaped = val.trim().replace("\"", "&quot;");
    results.push_str(&escaped);
    results.push('"');
}
