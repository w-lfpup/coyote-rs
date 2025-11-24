/*
    INTERMEDIATE RENDER FORMAT

    Templates are converted to an array of template steps[][] and and injections[].

    Coyote is focused on text / strings
*/

use crate::template_steps::parse::{Step, parse_str};
use crate::template_steps::routes::StepKind;
use crate::template_steps::rulesets::RulesetImpl;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TemplateSteps {
    pub steps: Vec<Vec<Step>>,
    pub injs: Vec<Step>,
}

impl TemplateSteps {
    pub fn new() -> TemplateSteps {
        TemplateSteps {
            steps: Vec::from([Vec::new()]),
            injs: Vec::new(),
        }
    }
}

pub fn compose(ruleset: &dyn RulesetImpl, template_str: &str) -> TemplateSteps {
    let mut template_steps = TemplateSteps::new();

    for step in parse_str(ruleset, template_str, StepKind::Initial) {
        match step.kind {
            StepKind::AttrMapInjection => push_injection(&mut template_steps, step),
            StepKind::DescendantInjection => push_injection(&mut template_steps, step),
            _ => push_step(&mut template_steps, step),
        }
    }

    template_steps
}

fn push_injection(template_steps: &mut TemplateSteps, step: Step) {
    template_steps.steps.push(Vec::new());
    template_steps.injs.push(step);
}

fn push_step(template_steps: &mut TemplateSteps, step: Step) {
    if let Some(last) = template_steps.steps.last_mut() {
        last.push(step);
    }
}
