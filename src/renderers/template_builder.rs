use crate::documents::TemplateBuilderImpl;
use crate::template_steps::{RulesetImpl, Step, TemplateSteps, compose};

use std::collections::HashMap;
use std::mem::size_of;

pub struct Builder {
    memory_footprint: usize,
    results_cache: HashMap<String, TemplateSteps>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            memory_footprint: 0,
            results_cache: HashMap::new(),
        }
    }
}

impl TemplateBuilderImpl for Builder {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps {
        // cache template steps here
        if rules.get_cache_memory_limit() > self.memory_footprint {
            self.results_cache = HashMap::new();
        }

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
