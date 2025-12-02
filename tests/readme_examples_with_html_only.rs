mod readme_component_set;

use coyotes::HtmlOnly;

use readme_component_set as rcs;

#[test]
fn no_added_spaces() {
    let template = rcs::no_added_spaces();
    let expected = "<p>hai :3</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn collapse_spaces() {
    let template = rcs::collapse_spaces();
    let expected = "<p> hai :3 </p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_collapse_spaces() {
    let template = rcs::attribute_collapse_spaces();
    let expected = "<p attr attr2 att3></p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_preserve_new_lines() {
    let template = rcs::attribute_preserve_new_lines();
    let expected = "<p\nattr\nattr2\nattr3>\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_values_preserve_new_lines() {
    let template = rcs::attribute_values_preserve_new_lines();
    let expected = "<p\nattr='\n\nhai :3 hello!\n\n'\n></p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn component_injections() {
    let template = rcs::component_injections();
    let expected = "<p> <span>hai :3</span>\n<span>hello</span>\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_component_injections() {
    let template = rcs::text_component_injections();
    let expected = "<p>\n\nhai :3\n\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
