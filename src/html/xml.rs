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

pub struct XmlRules {}

impl XmlRules {
    pub fn new() -> XmlRules {
        XmlRules {}
    }
}

impl RulesetImpl for XmlRules {
    fn get_initial_namespace(&self) -> &str {
        "xml"
    }

    fn get_close_sequence_from_contentless_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "?" => Some("?"),
            "!--" => Some("-->"),
            "![CDATA[" => Some("]]>"),
            _ => None,
        }
    }

    fn get_contentless_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "?" => Some("?"),
            "--" => Some("!--"),
            "]]" => Some("![CDATA["),
            _ => None,
        }
    }

    fn tag_prefix_of_contentless(&self, tag: &str) -> Option<&str> {
        if tag.starts_with("?") {
            return Some("?");
        }

        if tag.starts_with("!--") {
            return Some("!--");
        }

        if tag.starts_with("![CDATA[") {
            return Some("![CDATA[");
        }

        return None;
    }

    fn get_close_sequence_from_alt_text_tag(&self, _tag: &str) -> Option<&str> {
        None
    }

    fn get_alt_text_tag_from_close_sequence(&self, _tag: &str) -> Option<&str> {
        None
    }

    fn respect_indentation(&self) -> bool {
        true
    }

    fn tag_is_banned_el(&self, _tag: &str) -> bool {
        false
    }

    fn tag_is_void_el(&self, _tag: &str) -> bool {
        false
    }

    fn tag_is_namespace_el(&self, _tag: &str) -> bool {
        false
    }

    fn tag_is_preserved_text_el(&self, tag: &str) -> bool {
        "!CDATA[[" == tag
    }

    fn tag_is_inline_el(&self, _tag: &str) -> bool {
        false
    }
}
