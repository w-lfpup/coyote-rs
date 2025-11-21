use crate::component_string::compose_string;
use crate::components::Component;
use crate::rulesets::{HtmlOnlyRules, HtmlRules, XmlRules};
use crate::template_builder::Builder;

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
