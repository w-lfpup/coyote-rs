use crate::template_steps::{compose, RulesetImpl, TemplateSteps};
use std::collections::HashMap;

// Build Enum
// default()
// with_cache()

// enum TemplateBuilder {
//     Builder(Builder),
//     BuilderWithCache(BuilderWithCache),
// }

pub trait BuilderImpl {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps;
}

pub struct Builder {
    step_count: usize,
    results_cache: HashMap<String, TemplateSteps>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            step_count: 0,
            results_cache: HashMap::new(),
        }
    }
}

impl BuilderImpl for Builder {
    fn build(&mut self, rules: &dyn RulesetImpl, template_str: &str) -> TemplateSteps {
        // cache template steps here

        if let Some(steps) = self.results_cache.get(template_str) {
            return steps.clone();
        }

        let steps = compose(rules, template_str);

        // check if step count is above threshold

        // obliterate cache if step count + new step count > threshold

        self.results_cache
            .insert(template_str.to_string(), steps.clone());

        steps
    }
}
