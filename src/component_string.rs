use crate::components::Component;
use crate::compose_steps::compose_steps;
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
                    let escaped = text.replace("<", "&lt;").replace("{", "&quot;");
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
                            if let Err(e) =
                                add_attr_inj(&mut tag_info_stack, &mut template_results, inj)
                            {
                                return Err(e);
                            };
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

fn add_attr_inj(
    stack: &mut Vec<TagInfo>,
    template_str: &mut String,
    cmpnt: &Component,
) -> Result<(), String> {
    match cmpnt {
        Component::Attr(attr) => {
            if let Err(e) = push_attr_component(template_str, stack, attr) {
                return Err(e);
            }
        }
        Component::AttrVal(attr, val) => {
            if let Err(e) = push_attr_component(template_str, stack, attr) {
                return Err(e);
            }

            push_attr_value_component(template_str, stack, val)
        }
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        if let Err(e) = push_attr_component(template_str, stack, attr) {
                            return Err(e);
                        }
                    }
                    Component::AttrVal(attr, val) => {
                        if let Err(e) = push_attr_component(template_str, stack, attr) {
                            return Err(e);
                        }
                        push_attr_value_component(template_str, stack, val)
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };

    Ok(())
}

fn push_attr_component(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    attr: &str,
) -> Result<(), String> {
    if !attr_is_valid(attr) {
        return Err("invalid attribute: ".to_string() + attr);
    }

    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return Ok(()),
    };

    if tag_info.banned_path {
        return Ok(());
    }

    results.push(' ');
    results.push_str(attr);

    Ok(())
}

fn attr_is_valid(attr: &str) -> bool {
    for glyph in attr.chars() {
        if forbidden_attr_glyph(glyph) {
            return false;
        }
    }

    true
}

// https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
fn forbidden_attr_glyph(glyph: char) -> bool {
    match glyph {
        '<' => true,
        '=' => true,
        '"' => true,
        '\'' => true,
        '/' => true,
        '>' => true,
        '{' => true, // this ones for coyote, reserved char
        _ => false,
    }
}

fn push_attr_value_component(results: &mut String, stack: &mut Vec<TagInfo>, val: &str) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    if tag_info.banned_path {
        return;
    }

    results.push_str("=\"");
    let escaped = val.replace("\"", "&quot;");
    results.push_str(&escaped);
    results.push('"');
}

fn push_text_component(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    rules: &dyn RulesetImpl,
    text: &str,
) {
    // if all_spaces(text) {
    //     return;
    // }

    // let tag_info = match stack.last_mut() {
    //     Some(curr) => curr,
    //     // this should never happen
    //     _ => return,
    // };

    // if tag_info.banned_path || tag_info.void_el {
    //     return;
    // }

    // if tag_info.preserved_text_path {
    //     results.push_str(text);
    //     tag_info.text_format = TextFormat::Inline;
    //     return;
    // }

    // // if alt text
    // if let Some(_) = rules.get_close_sequence_from_alt_text_tag(&tag_info.tag) {
    //     add_alt_element_text(results, text, tag_info);
    //     tag_info.text_format = TextFormat::Inline;
    //     return;
    // }

    // // if unformatted
    // if !rules.respect_indentation() {
    //     add_inline_text(results, text, &tag_info);
    //     tag_info.text_format = TextFormat::Inline;
    //     return;
    // }

    // // formatted text
    // if TextFormat::Inline == tag_info.text_format {
    //     results.push(' ');
    // }

    // if tag_info.inline_el || TextFormat::Inline == tag_info.text_format {
    //     add_first_line_text(results, text, tag_info);
    // } else {
    //     add_text(results, text, tag_info);
    // }

    // tag_info.text_format = TextFormat::Inline;
}
