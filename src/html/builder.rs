use crate::components::Component;
use crate::documents::{compose_string, BuilderImpl};
use crate::rulesets::{HtmlOnlyRules, HtmlRules, XmlRules};
use crate::template_steps::{compose, RulesetImpl, TemplateSteps};
use std::collections::HashMap;

// default cache params
struct BuilderParams {
    max_memory: usize,
}

struct Builder {
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