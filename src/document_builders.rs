use crate::components::Component;
use crate::documents::{compose_string, BuilderImpl};
use crate::rulesets::{HtmlOnlyRules, HtmlRules, XmlRules};
use crate::template_steps::{compose, RulesetImpl, TemplateSteps};
use std::collections::HashMap;

// Enum
// Html::default()
// Html::with_cache
// html::html_only()
// html::html_only_with_cache()
// html::html_with_styles()
//

/*
Naw that sucks.



*/

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

pub struct Html {
    rules: HtmlRules,
    builder: Builder,
    // template_builder: TemplateBuilder,
}

impl Html {
    pub fn new() -> Html {
        Html {
            rules: HtmlRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> Result<String, String> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

// HTML ONLY without styles, scripts, or links
pub struct HtmlOnly {
    rules: HtmlOnlyRules,
    builder: Builder,
}

impl HtmlOnly {
    pub fn new() -> HtmlOnly {
        HtmlOnly {
            rules: HtmlOnlyRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> Result<String, String> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

pub struct Xml {
    rules: XmlRules,
    builder: Builder,
}

impl Xml {
    pub fn new() -> Xml {
        Xml {
            rules: XmlRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> Result<String, String> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}
