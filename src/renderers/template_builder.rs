use crate::documents::TemplateBuilderImpl;
use crate::template_steps::{RulesetImpl, TemplateSteps, compose};
use std::collections::HashMap;

pub struct Builder {
    results_cache: HashMap<String, TemplateSteps>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            results_cache: HashMap::new(),
        }
    }
}

impl TemplateBuilderImpl for Builder {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps {
        // cache template steps here

        if let Some(steps) = self.results_cache.get(template_str) {
            return steps.clone();
        }

        // sizeof step * len of steps
        // + len of template string
        //
        // = memory footprint

        // += curr_bytes

        // if bytes is > max memory
        // create new cache

        let steps = compose(rules, template_str);

        // check if step count is above threshold

        // obliterate cache if step count + new step count > threshold

        self.results_cache
            .insert(template_str.to_string(), steps.clone());

        steps
    }
}
