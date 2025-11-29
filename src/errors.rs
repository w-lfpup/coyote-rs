use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Errors {
    InvalidAttribute(String, usize, char),
    UnbalancedTemplate(String),
    DocumentMemoryLimitExceeded(usize, usize),
}

impl error::Error for Errors {}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::InvalidAttribute(attribute, index, glyph) => {
                write!(
                    f,
                    "The following attribute: {}\ncontains the invalid glyph: *{}*\nat index: {}",
                    attribute, glyph, index
                )
            }
            Errors::UnbalancedTemplate(template_str) => {
                write!(f, "The following template is unbalanced:\n{}", template_str)
            }
            Errors::DocumentMemoryLimitExceeded(memory_limit, document_length) => {
                write!(
                    f,
                    "Document exceeded memory limit: {}/{}",
                    document_length, memory_limit
                )
            }
        }
    }
}
