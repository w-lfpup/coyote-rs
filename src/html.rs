use crate::component_string::{compose_string, Builder};
use crate::components::Component;
use crate::rulesets::{ClientRules, ServerRules};

pub struct Html {
    rules: ServerRules,
    builder: Builder,
}

impl Html {
    pub fn new() -> Html {
        Html {
            rules: ServerRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> String {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

// CLIENT HTML
// safer without styles, scripts, or links
pub struct ClientHtml {
    rules: ClientRules,
    builder: Builder,
}

impl ClientHtml {
    pub fn new() -> ClientHtml {
        ClientHtml {
            rules: ClientRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn build(&mut self, component: &Component) -> String {
        compose_string(&mut self.builder, &self.rules, component)
    }
}
