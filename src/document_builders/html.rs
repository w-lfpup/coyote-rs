use crate::components::Component;
use crate::document_builders::flyweight as fw;
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

    pub fn from(params: &fw::DocumentParams) -> Html {
        Html {
            rules: HtmlRules::from(params),
            builder: Builder::new(),
        }
    }

    pub fn render(&mut self, component: &Component) -> Result<String, Errors> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

pub struct HtmlRules {
    params: fw::DocumentParams,
}

impl HtmlRules {
    pub fn new() -> HtmlRules {
        let params = fw::DocumentParams {
            cache_memory_limit: fw::FALLBACK_CACHE_MEMORY_LIMIT,
            document_memory_limit: fw::FALLBACK_DOCUMENT_MEMORY_LIMIT,
            embedded_content: String::from("html"),
            respect_indentation: true,
        };

        HtmlRules { params }
    }

    pub fn from(params: &fw::DocumentParams) -> HtmlRules {
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

    fn get_initial_embedded_content(&self) -> &str {
        &self.params.embedded_content
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
        fw::is_banned_el(tag)
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
