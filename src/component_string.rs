use crate::components::Component;
use crate::compose_steps::{compose_steps, push_attr, push_text};
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::tag_info::TagInfo;
use crate::template_steps::{compose, Results as TemplateSteps};

struct TemplateBit {
    pub inj_index: usize,
}

enum StackBit<'a> {
    Tmpl(&'a Component, TemplateSteps, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

pub trait BuilderImpl {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps;
}

pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps {
        // chance to cache templates here
        compose(rules, template_str)
    }
}

pub fn compose_string(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &Component,
) -> String {
    let mut tmpl_str = "".to_string();

    let component_bit = get_bit_from_component_stack(builder, rules, component);
    let mut component_stack: Vec<StackBit> = Vec::from([component_bit]);
    let mut tag_info_stack: Vec<TagInfo> = Vec::from([TagInfo::new(rules, ":root")]);

    while let Some(mut component_bit) = component_stack.pop() {
        match component_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => {
                    push_text(&mut tmpl_str, &mut tag_info_stack, rules, text);
                }
                Component::List(list) => {
                    for cmpnt in list.iter().rev() {
                        let bit = get_bit_from_component_stack(builder, rules, cmpnt);
                        component_stack.push(bit);
                    }
                }
                _ => {}
            },
            StackBit::Tmpl(component, ref template, ref mut bit) => {
                let index = bit.inj_index;
                bit.inj_index += 1;

                let tmpl_component = match component {
                    Component::Tmpl(cmpnt) => cmpnt,
                    _ => continue,
                };

                // add current template chunk
                if let Some(chunk) = template.steps.get(index) {
                    compose_steps(
                        rules,
                        &mut tmpl_str,
                        &mut tag_info_stack,
                        &tmpl_component.template_str,
                        chunk,
                    );
                }

                // add injections
                if let (Some(inj_step), Some(inj)) = (
                    template.injs.get(index),
                    tmpl_component.injections.get(index),
                ) {
                    match inj_step.kind {
                        StepKind::AttrMapInjection => {
                            add_attr_inj(&mut tag_info_stack, &mut tmpl_str, inj);
                        }
                        StepKind::DescendantInjection => {
                            // push template back and bail early
                            component_stack.push(component_bit);

                            let bit = get_bit_from_component_stack(builder, rules, inj);
                            component_stack.push(bit);

                            continue;
                        }
                        _ => {}
                    }
                }

                // don't forget the last part of the templates!
                if index < template.steps.len() {
                    component_stack.push(component_bit);
                }
            }
            _ => {}
        }
    }

    tmpl_str
}

fn get_bit_from_component_stack<'a>(
    builder: &mut dyn BuilderImpl,
    rules: &dyn RulesetImpl,
    component: &'a Component,
) -> StackBit<'a> {
    match component {
        Component::Text(_text) => StackBit::Cmpnt(component),
        Component::List(_list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            let template_steps = builder.build(rules, &tmpl.template_str);
            StackBit::Tmpl(component, template_steps, TemplateBit { inj_index: 0 })
        }
        _ => StackBit::None,
    }
}

fn add_attr_inj(stack: &mut Vec<TagInfo>, template_str: &mut String, component: &Component) {
    match component {
        Component::Attr(attr) => push_attr(template_str, stack, attr),
        Component::AttrVal(attr, val) => add_attr_val(template_str, attr, val),
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        push_attr(template_str, stack, &attr);
                    }
                    Component::AttrVal(attr, val) => {
                        add_attr_val(template_str, &attr, &val);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn add_attr_val(tmpl_str: &mut String, attr: &str, val: &str) {
    tmpl_str.push_str(" ");
    tmpl_str.push_str(attr);
    tmpl_str.push_str("=\"");
    tmpl_str.push_str(val);
    tmpl_str.push_str("\"");
}
