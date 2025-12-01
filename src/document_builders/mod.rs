mod flyweight;
mod html;
mod html_only;
mod template_builder;
mod xml;

pub use flyweight::DocumentParams;
pub use html::{Html, HtmlRules};
pub use html_only::{HtmlOnly, HtmlOnlyRules};
pub use xml::{Xml, XmlRules};
