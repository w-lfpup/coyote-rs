use crate::template_steps::{RulesetImpl, TemplateSteps};

pub trait BuilderImpl {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps;
}
