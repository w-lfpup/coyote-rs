use crate::components::Component;
use crate::document_builders::renderer::{
    FALLBACK_CACHE_MEMORY_LIMIT, FALLBACK_DOCUMENT_MEMORY_LIMIT, RendererParams,
};
use crate::document_builders::template_builder::Builder;
use crate::documents::compose_string;
use crate::errors::Errors;
use crate::template_steps::RulesetImpl;

pub struct Html {
    rules: HtmlRules,
    builder: Builder,
}

impl Html {
    pub fn new() -> Html {
        Html {
            rules: HtmlRules::new(),
            builder: Builder::new(),
        }
    }

    pub fn from(params: &RendererParams) -> Html {
        Html {
            rules: HtmlRules::from(params),
            builder: Builder::new(),
        }
    }

    pub fn render(&mut self, component: &Component) -> Result<String, Errors> {
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

    pub fn from(params: &RendererParams) -> HtmlOnly {
        HtmlOnly {
            rules: HtmlOnlyRules::from(params),
            builder: Builder::new(),
        }
    }

    pub fn render(&mut self, component: &Component) -> Result<String, Errors> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

pub struct HtmlRules {
    params: RendererParams,
}

impl HtmlRules {
    pub fn new() -> HtmlRules {
        let params = RendererParams {
            cache_memory_limit: FALLBACK_CACHE_MEMORY_LIMIT,
            document_memory_limit: FALLBACK_DOCUMENT_MEMORY_LIMIT,
            respect_indentation: true,
        };

        HtmlRules { params }
    }

    pub fn from(params: &RendererParams) -> HtmlRules {
        HtmlRules {
            params: params.clone(),
        }
    }
}

impl RulesetImpl for HtmlRules {
    fn get_document_memory_limit(&self) -> usize {
        self.params.document_memory_limit
    }

    fn get_cache_memory_limit(&self) -> usize {
        self.params.cache_memory_limit
    }

    fn get_initial_namespace(&self) -> &str {
        "html"
    }

    fn tag_is_prefix_of_contentless_el(&self, tag: &str) -> Option<&str> {
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
        self.params.respect_indentation
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

pub struct HtmlOnlyRules {
    params: RendererParams,
}

impl HtmlOnlyRules {
    pub fn new() -> HtmlOnlyRules {
        let params = RendererParams {
            cache_memory_limit: FALLBACK_CACHE_MEMORY_LIMIT,
            document_memory_limit: FALLBACK_DOCUMENT_MEMORY_LIMIT,
            respect_indentation: false,
        };

        HtmlOnlyRules { params }
    }

    pub fn from(params: &RendererParams) -> HtmlOnlyRules {
        HtmlOnlyRules {
            params: params.clone(),
        }
    }
}

impl RulesetImpl for HtmlOnlyRules {
    fn get_document_memory_limit(&self) -> usize {
        self.params.document_memory_limit
    }

    fn get_cache_memory_limit(&self) -> usize {
        self.params.cache_memory_limit
    }

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

    fn tag_is_prefix_of_contentless_el(&self, tag: &str) -> Option<&str> {
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
        self.params.respect_indentation
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
