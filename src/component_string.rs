use crate::components::Component;
use crate::compose_steps::compose_steps;
use crate::tag_info::{TagInfo, TextFormat};
use crate::template_builder::BuilderImpl;
use crate::template_steps::{RulesetImpl, StepKind, TemplateSteps};
use crate::text_components::{
    push_multiline_attributes, push_text_component as push_that_text_component,
};

#[derive(Debug)]
struct TemplateBit {
    pub inj_index: usize,
    pub stack_depth: usize,
}

// Needed to track iteration across template steps and injections
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
            // could be used for String too
            StackBit::Tmpl(cmpnt, ref template, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                let tmpl_str = match cmpnt {
                    Component::Tmpl(cmpnt) => cmpnt.template,
                    Component::TmplString(cmpnt) => &cmpnt.template,
                    _ => continue,
                };

                // template chunk
                match template.steps.get(index) {
                    Some(chunk) => {
                        compose_steps(
                            rules,
                            &mut template_results,
                            &mut tag_info_stack,
                            tmpl_str,
                            chunk,
                        );
                    }
                    _ => {
                        // at the end of template
                        // if stack depth does not match tag_infor_stack depth
                        if bit.stack_depth != tag_info_stack.len() {
                            return Err(
                                "Coyote Err: the following template component is imbalanced:\n{:?}"
                                    .to_string()
                                    + tmpl_str,
                            );
                        }
                    }
                }

                // add injections
                let injections = match cmpnt {
                    Component::Tmpl(cmpnt) => &cmpnt.injections,
                    Component::TmplString(cmpnt) => &cmpnt.injections,
                    _ => continue,
                };

                if let (Some(inj_step), Some(inj)) =
                    (template.injs.get(index), injections.get(index))
                {
                    match inj_step.kind {
                        StepKind::AttrMapInjection => {
                            if let Err(e) =
                                add_attr_inj(&mut tag_info_stack, &mut template_results, inj)
                            {
                                return Err(e);
                            };
                        }

                        // push template injection and bail early
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
            let template_steps = builder.build(rules, &tmpl.template);
            StackBit::Tmpl(
                cmpnt,
                template_steps,
                TemplateBit {
                    inj_index: 0,
                    stack_depth: stack.len(),
                },
            )
        }
        Component::TmplString(tmpl) => {
            let template_steps = builder.build(rules, &tmpl.template);
            StackBit::Tmpl(
                cmpnt,
                template_steps,
                TemplateBit {
                    inj_index: 0,
                    stack_depth: stack.len(),
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

            push_attr_value_component(template_str, tag_info, val)
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
                        push_attr_value_component(template_str, tag_info, val)
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
    if glyph.is_whitespace() {
        return true;
    }

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

fn push_attr_value_component(results: &mut String, tag_info: &TagInfo, val: &str) {
    results.push_str("=\"");
    let escaped = val.replace("\"", "&quot;");
    push_multiline_attributes(results, &escaped, tag_info);
    results.push('"');
}

fn push_text_component(
    results: &mut String,
    stack: &mut Vec<TagInfo>,
    _rules: &dyn RulesetImpl,
    text: &str,
) {
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        _ => return,
    };

    push_that_text_component(results, text, tag_info);
}
