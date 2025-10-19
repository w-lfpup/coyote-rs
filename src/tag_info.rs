use crate::rulesets::RulesetImpl;

// describes how to handle elements and spacing
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextFormat {
    Block,
    BlockClose,
    Initial,
    Inline,
    InlineClose,
    LineSpace,
    Space,
    Text,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagInfo {
    pub banned_path: bool,
    pub indent_count: usize,
    pub inline_el: bool,
    pub namespace: String,
    pub preserved_text_path: bool,
    pub tag: String,
    pub text_format: TextFormat,
    pub void_el: bool,
}

impl TagInfo {
    pub fn get_root(rules: &dyn RulesetImpl) -> TagInfo {
        TagInfo {
            banned_path: false,
            indent_count: 0,
            inline_el: true,
            namespace: rules.get_initial_namespace().to_string(),
            preserved_text_path: false,
            tag: ":root".to_string(),
            text_format: TextFormat::Initial,
            void_el: false,
        }
    }

    pub fn from(rules: &dyn RulesetImpl, prev_tag_info: &TagInfo, tag: &str) -> TagInfo {
        let mut tag_info = prev_tag_info.clone();

        tag_info.tag = tag.to_string();
        tag_info.void_el = rules.tag_is_void_el(tag);

        tag_info.inline_el = rules.tag_is_inline_el(tag);

        // if void el
        // if void el and inline
        tag_info.text_format = TextFormat::Block;
        if tag_info.inline_el {
            tag_info.text_format = TextFormat::Inline;
        }

        if rules.tag_is_namespace_el(tag) {
            tag_info.namespace = tag.to_string();
        }

        if rules.tag_is_preserved_text_el(&tag_info.tag) {
            tag_info.preserved_text_path = true;
        }

        if rules.tag_is_banned_el(tag) {
            tag_info.banned_path = true;
        }

        if !rules.tag_is_void_el(&tag_info.tag) && !rules.tag_is_inline_el(tag) {
            tag_info.indent_count += 1;
        }

        tag_info
    }
}
