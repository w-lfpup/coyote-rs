pub const MEGABYTE: usize = 1024 * 1024;
pub const FALLBACK_CACHE_MEMORY_LIMIT: usize = 16 * MEGABYTE;
pub const FALLBACK_DOCUMENT_MEMORY_LIMIT: usize = 32 * MEGABYTE;

#[derive(Clone, Debug)]
pub struct DocumentParams {
    pub cache_memory_limit: usize,
    pub document_memory_limit: usize,
    pub respect_indentation: bool,
    pub embedded_content: String,
}

// deprecated elements
pub fn is_banned_el(tag: &str) -> bool {
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

pub fn is_void_el(tag: &str) -> bool {
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

pub fn is_embedded_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "math" => true,
        "svg" => true,
        _ => false,
    }
}

pub fn is_preformatted_text_el(tag: &str) -> bool {
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
