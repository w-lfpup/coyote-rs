pub const MEGABYTE: usize = 1024 * 1024;
pub const FALLBACK_CACHE_MEMORY_LIMIT: usize = 16 * MEGABYTE;
pub const FALLBACK_DOCUMENT_MEMORY_LIMIT: usize = 32 * MEGABYTE;

#[derive(Clone)]
pub struct RendererParams {
    pub cache_memory_limit: usize,
    pub document_memory_limit: usize,
    pub respect_indentation: bool,
}
