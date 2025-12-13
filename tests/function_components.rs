mod function_component_set;

use coyotes::Html;
use function_component_set as fcs;

#[test]
fn form_component_retains_spacing() {
    let template = fcs::form_component_retains_spacing();
    let expected = "<form action=\"/uwu\" method=\"post\">\n\tyou're a boy kisser aren't you >:3\n\t<input type=submit value=\"yus -_-\">\n</form>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn elememt_and_text_components_retains_spacing() {
    let template = fcs::elememt_and_text_components_retains_spacing();
    let expected = "<div>hai :3hai :3</div>\n<div>\n\thai :3hai :3\n</div>\n<div>hai :3 hai :3</div>\n<div>\n\thai :3 hai :3\n</div>\n<div>\n\thai :3\n\thai :3\n</div>\n<div>\n\thai :3\n\thai :3\n</div>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_and_text_components_retain_extra_spacey_spacing() {
    let template = fcs::element_and_text_components_retain_extra_spacey_spacing();
    let expected = "<div>\n\thai :3\n\n\thai :3\n</div>\n<div>\n\n\thai :3\n\n\thai :3\n\n</div>\n<div>\n\thai :3\n\n\thai :3\n</div>\n<div>\n\n\thai :3\n\n\thai :3\n\n</div>\n<div>\n\n\thai :3\n\n\n\thai :3\n\n</div>\n<div>\n\n\thai :3\n\n\n\thai :3\n\n</div>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_components_retain_spacing() {
    let template = fcs::element_components_retain_spacing();
    let expected = "<div><span> hai :3 </span><span> hai :3 </span></div>\n<div>\n\t<span> hai :3 </span><span> hai :3 </span>\n</div>\n<div><span> hai :3 </span> <span> hai :3 </span></div>\n<div>\n\t<span> hai :3 </span> <span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_components_retain_extra_spacey_spacing() {
    let template = fcs::element_components_retain_extra_spacey_spacing();
    let expected = "<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>\n<div>\n\t<span> hai :3 </span>\n\t<span> hai :3 </span>\n</div>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attributes_retain_spacing() {
    let template = fcs::attributes_retain_spacing();
    let expected = "<p hai></p>\n<p hai\n></p>\n<p\n\thai></p>\n<p\n\thai\n>\n</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_component_injections_retain_spacing() {
    let template = fcs::attribute_component_injections_retain_spacing();
    let expected = "<p hai hello yo=\"what's good!\" hey=\"\n\t\thowdy!\n\n\t\thowdy!\n\n\t\thurray!\n\t\">\n</p>\n<p\n\thai\n\thello\n\tyo=\"what's good!\"\n\they=\"\n\t\thowdy!\n\n\t\thowdy!\n\n\t\thurray!\n\t\">\n</p>\n<span hai hello yo=\"what's good!\" hey=\"\nhowdy!\n\nhowdy!\n\nhurray!\n\"></span>\n<span hai hello yo=\"what's good!\" hey=\"\nhowdy!\n\nhowdy!\n\nhurray!\n\"></span>";
    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
