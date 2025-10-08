#[derive(Debug, Eq, Clone, PartialEq)]
pub enum StepKind {
    Attr,
    AttrDoubleQuote,
    AttrDoubleQuoteClosed,
    AttrMapInjection,
    AttrSetter,
    AttrSingleQuote,
    AttrSingleQuoteClosed,
    AttrValueDoubleQuoted,
    AttrValueSingleQuoted,
    AttrValueUnquoted,
    DescendantInjection,
    Element,
    ElementClosed,
    ElementSpace,
    EmptyElement,
    EmptyElementClosed,
    Fragment,
    FragmentClosed,
    Initial,
    InjectionConfirmed,
    InjectionSpace,
    Tag,
    TailElementClosed,
    TailElementSolidus,
    TailElementSpace,
    TailTag,
    Text,
    TextSpace,
    TextAlt,
}

// Names based roughly on:
// https://html.spec.whatwg.org/multipage/parsing.html

pub fn route(glyph: char, prev_kind: &StepKind) -> StepKind {
    match prev_kind {
        StepKind::Attr => get_kind_from_attribute(glyph),
        StepKind::AttrDoubleQuote => get_kind_from_attribute_double_quote(glyph),
        StepKind::AttrDoubleQuoteClosed => get_kind_from_attribute_quote_closed(glyph),
        StepKind::AttrMapInjection => get_kind_from_injection(glyph),
        StepKind::AttrSetter => get_kind_from_attribute_setter(glyph),
        StepKind::AttrSingleQuote => get_kind_from_attribute_single_quote(glyph),
        StepKind::AttrSingleQuoteClosed => get_kind_from_attribute_quote_closed(glyph),
        StepKind::AttrValueDoubleQuoted => get_kind_from_attribute_double_quote(glyph),
        StepKind::AttrValueSingleQuoted => get_kind_from_attribute_single_quote(glyph),
        StepKind::AttrValueUnquoted => get_kind_from_attribute_value_unquoted(glyph),
        StepKind::DescendantInjection => get_kind_from_injection(glyph),
        StepKind::Element => get_kind_from_element(glyph),
        StepKind::ElementSpace => get_kind_from_element_space(glyph),
        StepKind::EmptyElement => get_kind_from_empty_element(glyph),
        StepKind::InjectionSpace => get_kind_from_injection(glyph),
        StepKind::Tag => get_kind_from_tag(glyph),
        StepKind::TailElementSolidus => get_kind_from_tail_element_solidus(glyph),
        StepKind::TailElementSpace => get_kind_from_tail_element_space(glyph),
        StepKind::TailTag => get_kind_from_tail_tag(glyph),
        _ => get_kind_from_text(glyph),
    }
}

fn get_kind_from_attribute(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '=' => StepKind::AttrSetter,
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        '{' => StepKind::AttrMapInjection,
        _ => StepKind::Attr,
    }
}

fn get_kind_from_injection(glyph: char) -> StepKind {
    match glyph {
        '}' => StepKind::InjectionConfirmed,
        _ => StepKind::InjectionSpace,
    }
}

fn get_kind_from_attribute_single_quote(glyph: char) -> StepKind {
    match glyph {
        '\'' => StepKind::AttrSingleQuoteClosed,
        _ => StepKind::AttrValueSingleQuoted,
    }
}

fn get_kind_from_attribute_double_quote(glyph: char) -> StepKind {
    match glyph {
        '"' => StepKind::AttrDoubleQuoteClosed,
        _ => StepKind::AttrValueDoubleQuoted,
    }
}

fn get_kind_from_attribute_quote_closed(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        _ => StepKind::ElementSpace,
    }
}

fn get_kind_from_attribute_setter(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::AttrSetter;
    }

    match glyph {
        '"' => StepKind::AttrDoubleQuote,
        '\'' => StepKind::AttrSingleQuote,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_kind_from_attribute_value_unquoted(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_kind_from_element(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::Element;
    }

    match glyph {
        '>' => StepKind::Fragment,
        '/' => StepKind::TailElementSolidus,
        _ => StepKind::Tag,
    }
}

fn get_kind_from_element_space(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        '{' => StepKind::AttrMapInjection,
        _ => StepKind::Attr,
    }
}

fn get_kind_from_empty_element(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::EmptyElementClosed,
        _ => StepKind::EmptyElement,
    }
}

fn get_kind_from_tag(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ElementSpace;
    }

    match glyph {
        '>' => StepKind::ElementClosed,
        '/' => StepKind::EmptyElement,
        _ => StepKind::Tag,
    }
}

fn get_kind_from_tail_element_solidus(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailElementSolidus;
    }

    match glyph {
        '>' => StepKind::FragmentClosed,
        _ => StepKind::TailTag,
    }
}

fn get_kind_from_tail_tag(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailElementSpace;
    }

    match glyph {
        '>' => StepKind::TailElementClosed,
        _ => StepKind::TailTag,
    }
}

fn get_kind_from_tail_element_space(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TailElementClosed,
        _ => StepKind::TailElementSpace,
    }
}

fn get_kind_from_text(glyph: char) -> StepKind {
    // is white space
    // return StepKind::TextSpace
    if glyph.is_whitespace() {
        return StepKind::TextSpace;
    }

    match glyph {
        '<' => StepKind::Element,
        '{' => StepKind::DescendantInjection,
        _ => StepKind::Text,
    }
}
