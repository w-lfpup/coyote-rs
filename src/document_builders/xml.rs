use crate::components::Component;
use crate::document_builders::renderer::{
    FALLBACK_CACHE_MEMORY_LIMIT, FALLBACK_DOCUMENT_MEMORY_LIMIT, RendererParams,
};
use crate::document_builders::template_builder::Builder;
use crate::documents::compose_string;
use crate::errors::Errors;
use crate::template_steps::RulesetImpl;

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

    pub fn from(params: &RendererParams) -> Xml {
        Xml {
            rules: XmlRules::from(params.clone()),
            builder: Builder::new(),
        }
    }

    pub fn render(&mut self, component: &Component) -> Result<String, Errors> {
        compose_string(&mut self.builder, &self.rules, component)
    }
}

pub struct XmlRules {
    params: RendererParams,
}

impl XmlRules {
    pub fn new() -> XmlRules {
        let params = RendererParams {
            cache_memory_limit: FALLBACK_CACHE_MEMORY_LIMIT,
            document_memory_limit: FALLBACK_DOCUMENT_MEMORY_LIMIT,
            respect_indentation: false,
        };

        XmlRules { params }
    }

    pub fn from(params: RendererParams) -> XmlRules {
        XmlRules {
            params: params.clone(),
        }
    }
}

impl RulesetImpl for XmlRules {
    fn get_document_memory_limit(&self) -> usize {
        self.params.document_memory_limit
    }

    fn get_cache_memory_limit(&self) -> usize {
        self.params.cache_memory_limit
    }

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

    fn tag_is_prefix_of_contentless_el(&self, tag: &str) -> Option<&str> {
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
        self.params.respect_indentation
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
