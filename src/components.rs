#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Component {
    UnescapedText(String),
    Text(String),
    Attr(String),
    AttrVal(String, String),
    Tmpl(Template),
    List(Vec<Component>),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub template_str: &'static str,
    pub injections: Vec<Component>,
}

// ergonomic functions to quickly create componets without the typical rust verbosity
// (considerably improves readability of component code)
pub fn tmpl<const N: usize>(template_str: &'static str, injections: [Component; N]) -> Component {
    Component::Tmpl(Template {
        template_str: template_str,
        injections: Vec::from(injections),
    })
}

pub fn text(txt: &str) -> Component {
    Component::Text(txt.to_string())
}

pub fn unescaped_text(txt: &str) -> Component {
    Component::UnescapedText(txt.to_string())
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
