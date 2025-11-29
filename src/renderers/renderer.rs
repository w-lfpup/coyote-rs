#[derive(Clone)]
pub struct RendererParams {
    pub cache_memory_limit: usize,
    pub document_memory_limit: usize,
    pub respect_indentation: bool,
}
