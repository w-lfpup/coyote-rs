pub trait RulesetImpl {
    fn get_document_memory_limit(&self) -> usize;
    fn get_cache_memory_limit(&self) -> usize;
    fn get_alt_text_tag_from_close_sequence(&self, tag: &str) -> Option<&str>;
    fn get_close_sequence_from_alt_text_tag(&self, tag: &str) -> Option<&str>;
    fn get_close_sequence_from_contentless_tag(&self, tag: &str) -> Option<&str>;
    fn get_contentless_tag_from_close_sequence(&self, tag: &str) -> Option<&str>;
    fn get_initial_namespace(&self) -> &str;
    fn respect_indentation(&self) -> bool;
    fn tag_is_banned_el(&self, tag: &str) -> bool;
    fn tag_is_inline_el(&self, tag: &str) -> bool;
    fn tag_is_namespace_el(&self, tag: &str) -> bool;
    fn tag_is_prefix_of_contentless_el(&self, tag: &str) -> Option<&str>;
    fn tag_is_preformatted_text_el(&self, tag: &str) -> bool;
    fn tag_is_void_el(&self, tag: &str) -> bool;
}
