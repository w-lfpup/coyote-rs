mod flyweight;
mod html;
mod html_css_only;
mod html_only;
mod template_builder;
mod xml;

pub use flyweight::DocumentParams;
pub use html::{Html, HtmlRules};
pub use html_css_only::{HtmlCssOnly, HtmlCssOnlyRules};
pub use html_only::{HtmlOnly, HtmlOnlyRules};
pub use template_builder::TemplateBuilder;
pub use xml::{Xml, XmlRules};
