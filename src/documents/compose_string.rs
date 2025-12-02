use crate::components::Component;
use crate::documents::compose_steps::compose_steps;
use crate::documents::tag_info::{TagInfo, TextFormat};
use crate::documents::template_builder::TemplateBuilderImpl;
use crate::documents::text_components::{push_multiline_attributes, push_text_component};
use crate::errors::Errors;
use crate::template_steps::{RulesetImpl, StepKind, TemplateSteps};

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

pub fn compose_string(
    builder: &mut dyn TemplateBuilderImpl,
    rules: &dyn RulesetImpl,
    component: &Component,
) -> Result<String, Errors> {
    let mut document_results = "".to_string();

    let mut tag_info_stack: Vec<TagInfo> = Vec::from([TagInfo::get_root(rules)]);
    let mut component_stack: Vec<StackBit> = Vec::from([get_bit_from_component_stack(
        &mut tag_info_stack,
        builder,
        rules,
        component,
    )]);

    while let Some(mut cmpnt_bit) = component_stack.pop() {
        // check document length
        if rules.get_document_memory_limit() < document_results.len() {
            return Err(Errors::DocumentMemoryLimitExceeded(
                rules.get_document_memory_limit(),
                document_results.len(),
            ));
        }

        match cmpnt_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => {
                    let escaped_text = remove_template_glyphs(text);
                    push_text_component_injection(
                        &mut document_results,
                        &mut tag_info_stack,
                        &escaped_text,
                    );
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
            StackBit::Tmpl(cmpnt, ref template_steps, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                let tmpl_str = match cmpnt {
                    Component::Tmpl(template, _) => template.template_str,
                    Component::TmplString(tmpl_string, _) => tmpl_string,
                    _ => continue,
                };

                // template chunk
                match template_steps.steps.get(index) {
                    Some(chunk) => {
                        compose_steps(
                            rules,
                            &mut document_results,
                            &mut tag_info_stack,
                            tmpl_str,
                            chunk,
                        );
                    }
                    _ => {
                        // at the end of template
                        // if stack depth does not match tag_infor_stack depth
                        if bit.stack_depth != tag_info_stack.len() {
                            return Err(Errors::UnbalancedTemplate(tmpl_str.to_string()));
                        }
                    }
                }

                // add injections
                let injections = match cmpnt {
                    Component::Tmpl(_, injections) => injections,
                    Component::TmplString(_, injections) => injections,
                    _ => continue,
                };

                if let (Some(inj_step), Some(inj)) =
                    (template_steps.injs.get(index), injections.get(index))
                {
                    match inj_step.kind {
                        StepKind::AttrMapInjection => {
                            if let Err(e) =
                                add_attr_inj(&mut tag_info_stack, &mut document_results, rules, inj)
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

                if index < template_steps.steps.len() {
                    component_stack.push(cmpnt_bit);
                }
            }
            _ => {}
        }
    }

    Ok(document_results)
}

fn get_bit_from_component_stack<'a>(
    stack: &mut Vec<TagInfo>,
    builder: &mut dyn TemplateBuilderImpl,
    rules: &dyn RulesetImpl,
    cmpnt: &'a Component,
) -> StackBit<'a> {
    match cmpnt {
        Component::Text(_) => StackBit::Cmpnt(cmpnt),
        Component::List(_) => StackBit::Cmpnt(cmpnt),
        Component::Tmpl(tmpl, _) => {
            let template_steps = builder.build(rules, tmpl.template_str);
            StackBit::Tmpl(
                cmpnt,
                template_steps,
                TemplateBit {
                    inj_index: 0,
                    stack_depth: stack.len(),
                },
            )
        }
        Component::TmplString(tmpl_string, _) => {
            let template_steps = builder.build(rules, tmpl_string);
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
    document_results: &mut String,
    rules: &dyn RulesetImpl,
    cmpnt: &Component,
) -> Result<(), Errors> {
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        _ => return Ok(()),
    };

    if tag_info.banned_path {
        return Ok(());
    }

    match cmpnt {
        Component::Attr(attr) => {
            if let Err(e) = push_attr_component(document_results, tag_info, attr) {
                return Err(e);
            }
        }
        Component::AttrVal(attr, val) => {
            if let Err(e) = push_attr_component(document_results, tag_info, attr) {
                return Err(e);
            }

            push_attr_value_component(document_results, rules, tag_info, val)
        }
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        if let Err(e) = push_attr_component(document_results, tag_info, attr) {
                            return Err(e);
                        }
                    }
                    Component::AttrVal(attr, val) => {
                        if let Err(e) = push_attr_component(document_results, tag_info, attr) {
                            return Err(e);
                        }
                        push_attr_value_component(document_results, rules, tag_info, val)
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };

    tag_info.text_format = TextFormat::Text;

    Ok(())
}

fn remove_template_glyphs(text: &str) -> String {
    let mut safer_text = String::from("");
    for glyph in text.chars() {
        match glyph {
            '<' => safer_text.push_str("&lt;"),
            '{' => safer_text.push_str("&123;"),
            _ => safer_text.push(glyph),
        }
    }

    safer_text
}

fn push_attr_component(results: &mut String, tag_info: &TagInfo, attr: &str) -> Result<(), Errors> {
    if let Err(e) = attr_is_valid(attr) {
        return Err(e);
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

fn attr_is_valid(attr: &str) -> Result<(), Errors> {
    for (index, glyph) in attr.char_indices() {
        if forbidden_attr_glyph(glyph) {
            return Err(Errors::InvalidAttribute(attr.to_string(), index, glyph));
        }
    }

    Ok(())
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

fn push_attr_value_component(
    results: &mut String,
    rules: &dyn RulesetImpl,
    tag_info: &TagInfo,
    val: &str,
) {
    results.push_str("=\"");
    let escaped = val.replace("\"", "&quot;");
    push_multiline_attributes(results, rules, &escaped, tag_info);
    results.push('"');
}

fn push_text_component_injection(results: &mut String, stack: &mut Vec<TagInfo>, text: &str) {
    let tag_info = match stack.last_mut() {
        Some(curr) => curr,
        _ => return,
    };

    push_text_component(results, text, tag_info);

    tag_info.text_format = TextFormat::Text;
}
