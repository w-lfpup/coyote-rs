use crate::components::Component;
use crate::compose_steps::compose_steps;
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::{TagInfo, TextFormat};
use crate::template_builder::BuilderImpl;
use crate::template_steps::Results as TemplateSteps;
use crate::text_components::push_text_component as push_that_text_component;

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

// needs to be a concept of a "remainder"
// the last empty string of the previous step,
// provided to the next step

pub fn compose_string(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &Component,
) -> Result<String, String> {
    let mut template_results = "".to_string();

    let mut tag_info_stack: Vec<TagInfo> = Vec::from([TagInfo::get_root(rules)]);
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
                        // returns "remainder str"
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
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return Ok(()),
    };

    if tag_info.banned_path {
        return Ok(());
    }

    match cmpnt {
        Component::Attr(attr) => {
            if let Err(e) = push_attr_component(template_str, tag_info, attr) {
                return Err(e);
            }
        }
        Component::AttrVal(attr, val) => {
            if let Err(e) = push_attr_component(template_str, tag_info, attr) {
                return Err(e);
            }

            push_attr_value_component(template_str, val)
        }
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        if let Err(e) = push_attr_component(template_str, tag_info, attr) {
                            return Err(e);
                        }
                    }
                    Component::AttrVal(attr, val) => {
                        if let Err(e) = push_attr_component(template_str, tag_info, attr) {
                            return Err(e);
                        }
                        push_attr_value_component(template_str, val)
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };

    Ok(())
}

fn push_attr_component(results: &mut String, tag_info: &TagInfo, attr: &str) -> Result<(), String> {
    if !attr_is_valid(attr) {
        return Err("invalid attribute: ".to_string() + attr);
    }

    match tag_info.text_format {
        TextFormat::Space => results.push(' '),
        TextFormat::LineSpace => {
            results.push('\n');
            results.push_str(&"\t".repeat(tag_info.indent_count));
        }
        _ => {}
    }

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

fn push_attr_value_component(results: &mut String, val: &str) {
    results.push_str("=\"");
    let escaped = val.replace("\"", "&quot;");
    results.push_str(&escaped);
    results.push('"');
}

fn push_text_component(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    text: &str,
) {
    let tag_info = match stack.last() {
        Some(curr) => curr,
        _ => return,
    };

    push_that_text_component(results, text, tag_info)
}
