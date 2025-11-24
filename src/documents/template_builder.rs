use crate::template_steps::{RulesetImpl, TemplateSteps};

pub trait TemplateBuilderImpl {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps;
}
