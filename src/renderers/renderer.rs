use crate::components::Component;
use crate::errors::Errors;

#[derive(Clone)]
pub struct RendererParams {
    pub cache_memory_limit: usize,
    pub document_memory_limit: usize,
    pub respect_indentation: bool,
}

impl RendererParams {
    pub fn default() -> RendererParams {
        RendererParams {
            cache_memory_limit: 32 * 1024 * 1024,
            document_memory_limit: 32 * 1024 * 1024,
            respect_indentation: true,
        }
    }
}

pub trait RendererImpl {
    fn render(&mut self, component: &Component) -> Result<String, Errors>;
}
