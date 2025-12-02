mod html_component_set;

use coyotes::HtmlOnly;
use html_component_set as hcs;

#[test]
fn empty_element_retains_spacing() {
    let template = hcs::empty_element_retains_spacing();
    let expected = "<p></p>\n<p> </p><p>\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn fragments_dont_exist() {
    let template = hcs::fragments_dont_exist();
    let expected = "";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn block_element_with_text_retains_spacing() {
    let template = hcs::block_element_with_text_retains_spacing();
    let expected = "<p>hello!</p>\n<p> hello! </p>\n<p>\nhello\n</p><p>\nhello\n</p>\n<p>hello\n</p>\n<p>\nhello</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_element_with_text_retains_spacing() {
    let template = hcs::inline_element_with_text_retains_spacing();
    let expected = "<b>hello!</b>\n<b> hello! </b>\n<b> hello\n</b>\n<b>\nhello </b>\n<b>\nhello\n</b>\n<b>\nhello\n</b>\n<b>hello\n</b>\n<b>\nhello</b>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn comment_element_retains_spacing() {
    let template = hcs::comment_element_retains_spacing();
    let expected = "<!---->\n<!--Hello!-->\n<!-- Hello! -->\n<!--Hello! -->\n<!-- Hello!-->\n<!--Hello!\n-->\n<!--\nHello!-->\n<!--\n\nHello!\n\n-->";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn empty_element_stays_empty() {
    let template = hcs::empty_element_stays_empty();
    let expected = "<html></html>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn unbalanced_empty_elemen_errors_out() {
    let template = hcs::unbalanced_empty_elemen_errors_out();

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    if let Err(_) = results {
        return;
    }

    assert!(false, "unbalanced template failed to error",);
}

#[test]
fn forbidden_attribute_injection_glyph_errors_out() {
    let template = hcs::forbidden_attribute_injection_glyph_errors_out();

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    if let Err(_) = results {
        return;
    }

    assert!(false, "forbidden attribute glyph failed to error",);
}

#[test]
fn mozilla_spacing_example_passes() {
    let template = hcs::mozilla_spacing_example_passes();

    let expected = "<h1> Hello\n<span> World!</span> </h1>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_value_retains_spacing() {
    let template = hcs::attribute_value_retains_spacing();

    let expected = "<h1\noh\nyikes='woah!'\noh-no='\nit goes bye bye\n'\nwow='People use\nattributes in some very\nwild ways but thats okay'\n> Hello\n<span> World!</span> </h1>\n<h1 oh yikes='woah!' oh-no='\nit goes bye bye\n' wow='\n\nPeople use attributes in some very\n\nwild ways but thats okay\n\n'>\nHello! <span> World!</span>\n</h1>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_retain_spacing() {
    let template = hcs::void_elements_retain_spacing();

    let expected = "<input> <input>\n<input><input>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_with_inline_elements_retain_spacing() {
    let template = hcs::text_with_inline_elements_retain_spacing();

    let expected = "beasts <span> tread </span> softly <span> underfoot </span> .";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_with_block_elements_retain_spacing() {
    let template = hcs::text_with_block_elements_retain_spacing();

    let expected = "beasts <p> tread </p> softly <p> underfoot </p> .";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_can_have_attributes() {
    let template = hcs::void_elements_can_have_attributes();
    let expected =
        "<!DOCTYPE html><input type=checkbox> <input woof=\"bark\">\n<input grrr><input>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element_with_sibling() {
    let template = hcs::void_element_with_sibling();
    let expected = "<input><p>hai :3</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_void_element_with_siblings_retains_spacing() {
    let template = hcs::nested_void_element_with_siblings_retains_spacing();

    let expected = "<section>\n<input><p>hai :3</p>\n</section>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_elements_and_text_retain_spacing() {
    let template = hcs::nested_elements_and_text_retain_spacing();
    let expected = "<a><label><input type=woofer>bark!</label><img></a>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_retains_spacing() {
    let template = hcs::document_retains_spacing();

    let expected = "<!DOCTYPE>\n<html>\n<head>\n</head>\n<body>\n<article>\nYou're a <span>boy kisser</span> aren't you?\nClick <a>here</a> and go somewhere else.\n</article>\n<footer></footer>\n</body>\n</html>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_with_alt_text_elements_retains_spacing() {
    let template = hcs::document_with_alt_text_elements_retains_spacing();

    let expected = "<!DOCTYPE>\n<html>\n<head>\n</head>\n<body>\n<article></article>\n<footer></footer>\n</body>\n</html>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
