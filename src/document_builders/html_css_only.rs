use crate::components::Component;
use crate::document_builders::{TemplateBuilder, flyweight as fw};
use crate::documents::compose_string;
use crate::errors::Errors;
use crate::template_steps::RulesetImpl;

pub struct HtmlCssOnly {
    rules: HtmlCssOnlyRules,
    builder: TemplateBuilder,
}

impl HtmlCssOnly {
    pub fn new() -> HtmlCssOnly {
        HtmlCssOnly {
            rules: HtmlCssOnlyRules::new(),
            builder: TemplateBuilder::new(),
        }
    }

    pub fn from(params: &fw::DocumentParams) -> HtmlCssOnly {
        HtmlCssOnly {
            rules: HtmlCssOnlyRules::from(params),
            builder: TemplateBuilder::new(),
        }
    }

    pub fn render(&mut self, component: &Component) -> Result<String, Errors> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

pub struct HtmlCssOnlyRules {
    params: fw::DocumentParams,
}

impl HtmlCssOnlyRules {
    pub fn new() -> HtmlCssOnlyRules {
        let params = fw::DocumentParams {
            cache_memory_limit: fw::FALLBACK_CACHE_MEMORY_LIMIT,
            document_memory_limit: fw::FALLBACK_DOCUMENT_MEMORY_LIMIT,
            embedded_content: String::from("html"),
            respect_indentation: true,
        };

        HtmlCssOnlyRules { params }
    }

    pub fn from(params: &fw::DocumentParams) -> HtmlCssOnlyRules {
        HtmlCssOnlyRules {
            params: params.clone(),
        }
    }
}

impl RulesetImpl for HtmlCssOnlyRules {
    fn attr_is_banned(&self, attr: &str) -> bool {
        attr.starts_with("on")
    }

    fn get_document_memory_limit(&self) -> usize {
        self.params.document_memory_limit
    }

    fn get_cache_memory_limit(&self) -> usize {
        self.params.cache_memory_limit
    }

    fn get_initial_embedded_content(&self) -> &str {
        &self.params.embedded_content
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
            _ => fw::is_banned_el(tag),
        }
    }

    fn tag_is_void_el(&self, tag: &str) -> bool {
        fw::is_void_el(tag)
    }

    fn tag_is_embedded_content_el(&self, tag: &str) -> bool {
        fw::is_embedded_el(tag)
    }

    fn tag_is_preformatted_text_el(&self, tag: &str) -> bool {
        fw::is_preformatted_text_el(tag)
    }

    fn tag_is_inline_el(&self, tag: &str) -> bool {
        fw::is_inline_el(tag)
    }
}
