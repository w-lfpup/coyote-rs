use crate::components::Component;

pub struct Params {
    respect_indentation: bool,
    max_cache_memory: usize,
    max_document_memory: usize,
}

pub trait RendererImpl {
    fn from(params: Params) -> Self;
    fn render(&mut self, component: &Component) -> Result<String, String>;
}
