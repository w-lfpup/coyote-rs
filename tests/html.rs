mod html_component_set;

use coyotes::Html;
use html_component_set as hcs;

#[test]
fn empty_element_retains_spacing() {
    let template = hcs::empty_element_retains_spacing();
    let expected = "<p></p>\n<p> </p><p>\n</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn fragments_dont_exist() {
    let template = hcs::fragments_dont_exist();
    let expected = "";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn block_element_with_text_retains_spacing() {
    let template = hcs::block_element_with_text_retains_spacing();
    let expected = "<p>hello!</p>\n<p> hello! </p>\n<p>\n\thello\n</p><p>\n\thello\n</p>\n<p>hello\n</p>\n<p>\n\thello</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_element_with_text_retains_spacing() {
    let template = hcs::inline_element_with_text_retains_spacing();
    let expected = "<b>hello!</b>\n<b> hello! </b>\n<b> hello\n</b>\n<b>\nhello </b>\n<b>\nhello\n</b>\n<b>\nhello\n</b>\n<b>hello\n</b>\n<b>\nhello</b>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn comment_element_retains_spacing() {
    let template = hcs::comment_element_retains_spacing();
    let expected = "<!---->
<!--Hello!-->
<!-- Hello! -->
<!--Hello! -->
<!-- Hello!-->
<!--Hello!
-->
<!--
Hello!-->
<!--

	Hello!

-->";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn empty_element_stays_empty() {
    let template = hcs::empty_element_stays_empty();
    let expected = "<html></html>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn unbalanced_empty_elemen_errors_out() {
    let template = hcs::unbalanced_empty_elemen_errors_out();

    let mut html = Html::new();
    let results = html.render(&template);

    if let Err(_) = results {
        return;
    }

    assert!(false, "unbalanced template failed to error",);
}

#[test]
fn forbidden_attribute_injection_glyph_errors_out() {
    let template = hcs::forbidden_attribute_injection_glyph_errors_out();

    let mut html = Html::new();
    let results = html.render(&template);

    if let Err(_) = results {
        return;
    }

    assert!(false, "forbidden attribute glyph failed to error",);
}

#[test]
fn mozilla_spacing_example_passes() {
    let template = hcs::mozilla_spacing_example_passes();

    let expected = "<h1> Hello\n\t<span> World!</span> </h1>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_value_retains_spacing() {
    let template = hcs::attribute_value_retains_spacing();

    let expected = "<h1\n\toh\n\tyikes='woah!'\n\toh-no='\n\t\tit goes bye bye\n\t'\n\twow='People use\n\t\tattributes in some very\n\twild ways but thats okay'\n> Hello\n\t<span> World!</span> </h1>\n<h1 oh yikes='woah!' oh-no='\n\t\tit goes bye bye\n\t' wow='\n\n\t\tPeople use attributes in some very\n\n\t\twild ways but thats okay\n\n\t'>\n\tHello! <span> World!</span>\n</h1>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_retain_spacing() {
    let template = hcs::void_elements_retain_spacing();

    let expected = "<input> <input>\n<input><input>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_with_inline_elements_retain_spacing() {
    let template = hcs::text_with_inline_elements_retain_spacing();

    let expected = "beasts <span> tread </span> softly <span> underfoot </span> .";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_with_block_elements_retain_spacing() {
    let template = hcs::text_with_block_elements_retain_spacing();

    let expected = "beasts <p> tread </p> softly <p> underfoot </p> .";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_can_have_attributes() {
    let template = hcs::void_elements_can_have_attributes();
    let expected =
        "<!DOCTYPE html><input type=checkbox> <input woof=\"bark\">\n<input grrr><input>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element_with_sibling() {
    let template = hcs::void_element_with_sibling();
    let expected = "<input><p>hai :3</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_void_element_with_siblings_retains_spacing() {
    let template = hcs::nested_void_element_with_siblings_retains_spacing();

    let expected = "<section>\n\t<input><p>hai :3</p>\n</section>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_elements_and_text_retain_spacing() {
    let template = hcs::nested_elements_and_text_retain_spacing();
    let expected = "<a><label><input type=woofer>bark!</label><img></a>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_retains_spacing() {
    let template = hcs::document_retains_spacing();

    let expected = "<!DOCTYPE>\n<html>\n\t<head>\n\t</head>\n\t<body>\n\t\t<article>\n\t\t\tYou're a <span>boy kisser</span> aren't you?\n\t\t\tClick <a>here</a> and go somewhere else.\n\t\t</article>\n\t\t<footer></footer>\n\t</body>\n</html>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_with_alt_text_elements_retains_spacing() {
    let template = hcs::document_with_alt_text_elements_retains_spacing();

    let expected = "<!DOCTYPE>\n<html>\n\t<head>\n\t\t<style>\n\t\t\t#woof .bark {\n\t\t\t\tcolor: doggo;\n\t\t\t}\n\t\t</style>\n\t\t<script>\n\t\t\tif 2 < 3 {\n\t\t\t\tconsole.log();\n\t\t\t}\n\t\t</script>\n\t</head>\n\t<body>\n\t\t<article></article>\n\t\t<footer></footer>\n\t</body>\n</html>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
