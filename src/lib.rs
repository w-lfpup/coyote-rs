mod documents;
mod template_steps;

mod components;
mod document_builders;
mod rulesets;

pub use documents::{compose_string, BuilderImpl};
pub use template_steps::RulesetImpl;

pub use crate::components::{
    attr, attr_val, list, text, tmpl, tmpl_string, unescaped_text, vlist, Component,
};
pub use crate::document_builders::{Html, HtmlOnly, Xml};
