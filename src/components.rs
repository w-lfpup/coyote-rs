#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Component {
    Attr(String),
    AttrVal(String, String),
    List(Vec<Component>),
    Text(String),
    Tmpl(Template, Vec<Component>),
    TmplString(String, Vec<Component>),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub template_str: &'static str,
}

// ergonomic functions to quickly create componets without the typical rust verbosity
// (considerably improves readability of component code)
pub fn tmpl<const N: usize>(template_str: &'static str, injections: [Component; N]) -> Component {
    Component::Tmpl(Template { template_str }, Vec::from(injections))
}

pub fn tmpl_string<const N: usize>(template: &str, injections: [Component; N]) -> Component {
    Component::TmplString(template.to_string(), Vec::from(injections))
}

pub fn text(txt: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn attr(attr_str: &str) -> Component {
    Component::Attr(attr_str.to_string())
}

pub fn attr_val(attr_str: &str, value_txt: &str) -> Component {
    Component::AttrVal(attr_str.to_string(), value_txt.to_string())
}

pub fn list<const N: usize>(components: [Component; N]) -> Component {
    Component::List(Vec::from(components))
}

pub fn vlist(components: Vec<Component>) -> Component {
    Component::List(components)
}
