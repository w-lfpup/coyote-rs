use crate::rulesets::RulesetImpl;

// describes how to handle elements and spacing
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextFormat {
    Initial,
    Inline,
    InlineClose,
    Block,
    BlockClose,
    Space,
    LineSpace,
    Text,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagInfo {
    pub namespace: String,
    pub tag: String,
    pub text_format: TextFormat,
    pub indent_count: usize,
    pub void_el: bool,
    pub inline_el: bool,
    pub preserved_text_path: bool,
    pub banned_path: bool,
}

impl TagInfo {
    pub fn get_root(rules: &dyn RulesetImpl) -> TagInfo {
        TagInfo {
            namespace: rules.get_initial_namespace().to_string(),
            tag: ":root".to_string(),
            indent_count: 0,
            void_el: false,
            inline_el: false,
            text_format: TextFormat::Initial,
            preserved_text_path: false,
            banned_path: false,
        }
    }

    pub fn from(rules: &dyn RulesetImpl, prev_tag_info: &TagInfo, tag: &str) -> TagInfo {
        let mut tag_info = prev_tag_info.clone();

        tag_info.tag = tag.to_string();
        tag_info.void_el = rules.tag_is_void_el(tag);
        tag_info.inline_el = rules.tag_is_inline_el(tag);

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

        if !rules.tag_is_void_el(&prev_tag_info.tag) && !rules.tag_is_inline_el(tag) {
            tag_info.indent_count += 1;
        }

        tag_info
    }
}
