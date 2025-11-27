use crate::components::Component;
use crate::documents::compose_string;
use crate::errors::Errors;
use crate::renderers::renderer::{RendererImpl, RendererParams};
use crate::renderers::template_builder::Builder;
use crate::template_steps::RulesetImpl;

const MEGABYTE: usize = 1048576;
const FALLBACK_CACHE_MEMORY_LIMIT: usize = 16 * MEGABYTE;
const FALLBACK_DOCUMENT_MEMORY_LIMIT: usize = 32 * MEGABYTE;

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
}

impl RendererImpl for Xml {
    fn render(&mut self, component: &Component) -> Result<String, Errors> {
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
