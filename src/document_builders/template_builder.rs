use crate::documents::TemplateBuilderImpl;
use crate::template_steps::{RulesetImpl, TemplateSteps, compose};
use std::collections::HashMap;

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
        // obliterate cache if memory limit exceeded
        if rules.get_cache_memory_limit() < self.memory_footprint {
            self.memory_footprint = 0;
            self.results_cache = HashMap::new();
        }

        if let Some(steps) = self.results_cache.get(template_str) {
            return steps.clone();
        }

        self.memory_footprint += template_str.len();

        let steps = compose(rules, template_str);

        self.results_cache
            .insert(template_str.to_string(), steps.clone());

        steps
    }
}
