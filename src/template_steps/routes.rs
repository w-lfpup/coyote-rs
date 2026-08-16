#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepKind {
    Attr,
    AttrMapInjection,
    AttrSetter,
    AttrValueDoubleQuoteClosed,
    AttrValueDoubleQuoted,
    AttrValueDoubleQuoteOpened,
    AttrValueSingleQuoteClosed,
    AttrValueSingleQuoted,
    AttrValueSingleQuoteOpened,
    AttrValueUnquoted,
    BreakingSpace,
    DescendantInjection,
    Fragment,
    FragmentClosed,
    Initial,
    InjectionConfirmed,
    InjectionSpace,
    NonBreakingSpace,
    Tag,
    TagBreakingSpace,
    TagClosed,
    TagClosedEmpty,
    TagOpened,
    TagSolidus,
    TagNonBreakingSpace,
    TailTag,
    TailTagClosed,
    TailTagSolidus,
    TailTagSpace,
    Text,
    TextAlt,
}

// Names based roughly on:
// https://html.spec.whatwg.org/multipage/parsing.html

pub fn route(glyph: char, prev_kind: &StepKind) -> StepKind {
    match prev_kind {
        StepKind::Attr => get_kind_from_attribute(glyph),
        StepKind::AttrMapInjection => get_kind_from_injection(glyph),
        StepKind::AttrSetter => get_kind_from_attribute_setter(glyph),
        StepKind::AttrValueDoubleQuoteClosed => get_kind_from_attribute_quote_closed(glyph),
        StepKind::AttrValueDoubleQuoted => get_kind_from_attribute_double_quoted(glyph),
        StepKind::AttrValueDoubleQuoteOpened => get_kind_from_attribute_double_quoted(glyph),
        StepKind::AttrValueSingleQuoteClosed => get_kind_from_attribute_quote_closed(glyph),
        StepKind::AttrValueSingleQuoted => get_kind_from_attribute_single_quoted(glyph),
        StepKind::AttrValueSingleQuoteOpened => get_kind_from_attribute_single_quoted(glyph),
        StepKind::AttrValueUnquoted => get_kind_from_attribute_value_unquoted(glyph),
        StepKind::DescendantInjection => get_kind_from_injection(glyph),
        StepKind::InjectionSpace => get_kind_from_injection(glyph),
        StepKind::Tag => get_kind_from_tag(glyph),
        StepKind::TagBreakingSpace => get_kind_from_element_space(glyph),
        StepKind::TagNonBreakingSpace => get_kind_from_element_space(glyph),
        StepKind::TagOpened => get_kind_from_element(glyph),
        StepKind::TagSolidus => get_kind_from_empty_element(glyph),
        StepKind::TailTag => get_kind_from_tail_tag(glyph),
        StepKind::TailTagSolidus => get_kind_from_tail_element_solidus(glyph),
        StepKind::TailTagSpace => get_kind_from_tail_element_space(glyph),
        _ => get_kind_from_text(glyph),
    }
}

fn get_kind_from_attribute(glyph: char) -> StepKind {
    match glyph {
        '=' => StepKind::AttrSetter,
        '{' => StepKind::AttrMapInjection,
        '\n' => StepKind::TagBreakingSpace,
        '>' => StepKind::TagClosed,
        '/' => StepKind::TagSolidus,
        _ => match glyph.is_whitespace() {
            true => StepKind::TagNonBreakingSpace,
            _ => StepKind::Attr,
        },
    }
}

fn get_kind_from_injection(glyph: char) -> StepKind {
    match glyph {
        '}' => StepKind::InjectionConfirmed,
        _ => StepKind::InjectionSpace,
    }
}

fn get_kind_from_attribute_single_quoted(glyph: char) -> StepKind {
    match glyph {
        '\'' => StepKind::AttrValueSingleQuoteClosed,
        _ => StepKind::AttrValueSingleQuoted,
    }
}

fn get_kind_from_attribute_double_quoted(glyph: char) -> StepKind {
    match glyph {
        '"' => StepKind::AttrValueDoubleQuoteClosed,
        _ => StepKind::AttrValueDoubleQuoted,
    }
}

fn get_kind_from_attribute_quote_closed(glyph: char) -> StepKind {
    match glyph {
        '\n' => StepKind::TagBreakingSpace,
        '>' => StepKind::TagClosed,
        '/' => StepKind::TagSolidus,
        _ => match glyph.is_whitespace() {
            true => StepKind::TagNonBreakingSpace,
            _ => StepKind::Attr,
        },
    }
}

fn get_kind_from_attribute_setter(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::AttrSetter;
    }

    match glyph {
        '\'' => StepKind::AttrValueSingleQuoteOpened,
        '"' => StepKind::AttrValueDoubleQuoteOpened,
        _ => StepKind::AttrValueUnquoted,
    }
}

fn get_kind_from_attribute_value_unquoted(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TagClosed,
        '\n' => StepKind::TagBreakingSpace,
        _ => match glyph.is_whitespace() {
            true => StepKind::TagNonBreakingSpace,
            _ => StepKind::AttrValueUnquoted,
        },
    }
}

fn get_kind_from_element(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TagOpened;
    }

    match glyph {
        '>' => StepKind::Fragment,
        '/' => StepKind::TailTagSolidus,
        _ => StepKind::Tag,
    }
}

fn get_kind_from_element_space(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TagClosed,
        '/' => StepKind::TagSolidus,
        '{' => StepKind::AttrMapInjection,
        '\n' => StepKind::TagBreakingSpace,
        _ => match glyph.is_whitespace() {
            true => StepKind::TagNonBreakingSpace,
            _ => StepKind::Attr,
        },
    }
}

fn get_kind_from_empty_element(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TagClosedEmpty,
        _ => StepKind::TagSolidus,
    }
}

fn get_kind_from_tag(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TagClosed,
        '/' => StepKind::TagSolidus,
        '\n' => StepKind::TagBreakingSpace,
        _ => match glyph.is_whitespace() {
            true => StepKind::TagNonBreakingSpace,
            _ => StepKind::Tag,
        },
    }
}

fn get_kind_from_tail_element_solidus(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailTagSolidus;
    }

    match glyph {
        '>' => StepKind::FragmentClosed,
        _ => StepKind::TailTag,
    }
}

fn get_kind_from_tail_tag(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::TailTagSpace;
    }

    match glyph {
        '>' => StepKind::TailTagClosed,
        _ => StepKind::TailTag,
    }
}

fn get_kind_from_tail_element_space(glyph: char) -> StepKind {
    match glyph {
        '>' => StepKind::TailTagClosed,
        _ => StepKind::TailTagSpace,
    }
}

fn get_kind_from_text(glyph: char) -> StepKind {
    match glyph {
        '<' => StepKind::TagOpened,
        '{' => StepKind::DescendantInjection,
        '\n' => StepKind::BreakingSpace,
        _ => match glyph.is_whitespace() {
            true => StepKind::NonBreakingSpace,
            _ => StepKind::Text,
        },
    }
}
