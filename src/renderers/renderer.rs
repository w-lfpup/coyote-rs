use crate::components::Component;

pub trait RendererImpl {
    fn render(&mut self, component: &Component) -> Result<String, String>;
}
