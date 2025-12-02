mod atomics_component_set;

use coyotes::Html;
use atomics_component_set as acs;

#[test]
fn text_element() {
    let template = acs::text_element();
    let expected = "Beasts tread\nsoftly underfoot.";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn empty_element() {
    let template = acs::empty_element();
    let expected = "<p>\n</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn fragment() {
    let template = acs::fragment();
    let expected = "";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn block_element_with_text() {
    let template = acs::block_element_with_text();
    let expected = "<p>\n\thello!\n</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn block_element_with_text_for_string() {
    let template = acs::block_element_with_text_for_string();
    let expected = "<p>\n\thello!\n</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_element_with_text() {
    let template = acs::inline_element_with_text();
    let expected = "<b> hello! </b>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element() {
    let template = acs::void_element();
    let expected = "<input>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element_with_self_closing() {
    let template = acs::void_element_with_self_closing();
    let expected = "<input>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn non_void_element() {
    let template = acs::non_void_element();
    let expected = "<p></p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

// needs updating
#[test]
fn comment_element() {
    let template = acs::comment_element();
    let expected = "<!-- Hello! -->";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn alt_text_element() {
    let template = acs::alt_text_element();
    let expected = "<style>#woof .bark {\n\tcolor: doggo;\n}</style>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn alt_element_has_no_descendants() {
    let template = acs::alt_element_has_no_descendants();
    let expected = "<script>\n\t{}\n</script>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn preserved_text_element_retains_spacing() {
    let template = acs::preserved_text_element_retains_spacing();

    let expected = "<pre>\n\tU w U\n\t  woof woof!\n</pre>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute() {
    let template = acs::attribute();
    let expected = "<span hai>UwU</span>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_single_quote() {
    let template = acs::attribute_with_single_quote();
    let expected = "<span hai>UwU</span>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_double_quote() {
    let template = acs::attribute_with_double_quote();
    let expected = "<span hai>UwU</span>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_single_quote_value() {
    let template = acs::attribute_with_single_quote_value();
    let expected = "<span hai='hewoo'>UwU</span>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_double_quote_value() {
    let template = acs::attribute_with_double_quote_value();
    let expected = "<span hai=\"hewoo\">UwU</span>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
