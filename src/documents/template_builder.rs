use crate::template_steps::{compose, RulesetImpl, TemplateSteps};

pub trait BuilderImpl {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps;
}
