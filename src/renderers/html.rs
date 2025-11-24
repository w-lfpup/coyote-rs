// Just make a BuilderImpl so others downstream can use it

pub trait RenderImpl {
    fn render(&mut self, component: &Component) -> Result<String, String>;
}

use crate::template_steps::RulesetImpl;

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

pub struct HtmlRules {}

impl HtmlRules {
    pub fn new() -> HtmlRules {
        HtmlRules {}
    }
}

impl RulesetImpl for HtmlRules {
    fn get_initial_namespace(&self) -> &str {
        "html"
    }

    fn tag_prefix_of_contentless(&self, tag: &str) -> Option<&str> {
        if tag.starts_with("!--") {
            return Some("!--");
        }

        return None;
    }

    fn get_close_sequence_from_contentless_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "!--" => Some("-->"),
            _ => None,
        }
    }

    fn get_contentless_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "--" => Some("!--"),
            _ => None,
        }
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "script" => Some("</script"),
            "style" => Some("</style"),
            _ => None,
        }
    }

    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "</script" => Some("script"),
            "</style" => Some("style"),
            _ => None,
        }
    }

    fn respect_indentation(&self) -> bool {
        true
    }

    fn tag_is_banned_el(&self, tag: &str) -> bool {
        is_banned_el(tag)
    }

    fn tag_is_void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }

    fn tag_is_namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }

    fn tag_is_preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }

    fn tag_is_inline_el(&self, tag: &str) -> bool {
        is_inline_el(tag)
    }
}

pub struct HtmlOnlyRules {}

impl HtmlOnlyRules {
    pub fn new() -> HtmlOnlyRules {
        HtmlOnlyRules {}
    }
}

impl RulesetImpl for HtmlOnlyRules {
    fn get_initial_namespace(&self) -> &str {
        "html"
    }

    fn get_close_sequence_from_contentless_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "!--" => Some("-->"),
            _ => None,
        }
    }

    fn get_contentless_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "--" => Some("!--"),
            _ => None,
        }
    }

    fn tag_prefix_of_contentless(&self, tag: &str) -> Option<&str> {
        if tag.starts_with("!--") {
            return Some("!--");
        }

        return None;
    }

    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str> {
        match tag {
            "script" => Some("</script"),
            "style" => Some("</style"),
            _ => None,
        }
    }

    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str> {
        match tag {
            "</script" => Some("script"),
            "</style" => Some("style"),
            _ => None,
        }
    }

    fn respect_indentation(&self) -> bool {
        false
    }

    fn tag_is_banned_el(&self, tag: &str) -> bool {
        match tag {
            "link" => true,
            "script" => true,
            "style" => true,
            _ => is_banned_el(tag),
        }
    }

    fn tag_is_void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }

    fn tag_is_namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }

    fn tag_is_preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }

    fn tag_is_inline_el(&self, _tag: &str) -> bool {
        true
    }
}

// deprecated elements
fn is_banned_el(tag: &str) -> bool {
    match tag {
        "acronym" => true,
        "big" => true,
        "center" => true,
        "content" => true,
        "dir" => true,
        "font" => true,
        "frame" => true,
        "frameset" => true,
        "image" => true,
        "marquee" => true,
        "menuitem" => true,
        "nobr" => true,
        "noembed" => true,
        "noframes" => true,
        "param" => true,
        "plaintext" => true,
        "rb" => true,
        "rtc" => true,
        "shadow" => true,
        "strike" => true,
        "tt" => true,
        "xmp" => true,
        _ => false,
    }
}

fn is_void_el(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
        "area" => true,
        "base" => true,
        "br" => true,
        "col" => true,
        "embed" => true,
        "hr" => true,
        "img" => true,
        "input" => true,
        "link" => true,
        "meta" => true,
        "param" => true,
        "source" => true,
        "track" => true,
        "wbr" => true,
        _ => false,
    }
}

fn is_namespace_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "math" => true,
        "svg" => true,
        _ => false,
    }
}

pub fn is_preserved_text_el(tag: &str) -> bool {
    return "pre" == tag;
}

pub fn is_inline_el(tag: &str) -> bool {
    match tag {
        "a" => true,
        "abbr" => true,
        "b" => true,
        "bdi" => true,
        "bdo" => true,
        "cite" => true,
        "code" => true,
        "data" => true,
        "dfn" => true,
        "em" => true,
        "i" => true,
        "kbd" => true,
        "mark" => true,
        "q" => true,
        "rp" => true,
        "rt" => true,
        "ruby" => true,
        "s" => true,
        "samp" => true,
        "small" => true,
        "span" => true,
        "strong" => true,
        "sub" => true,
        "sup" => true,
        "time" => true,
        "u" => true,
        "var" => true,
        _ => false,
    }
}
