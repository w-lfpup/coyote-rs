// BuilderImpl

// Enum Builders { Html, HtmlOnly, Xml }

// Builders / BuilderImpl

// Self match { Html, HtmlOnly, Xml }

// They all follow BuilderImpl

mod html;
mod renderer;
mod template_builder;
mod xml;

pub use html::{Html, HtmlOnly, Xml};
pub use renderer::RendererImpl;
