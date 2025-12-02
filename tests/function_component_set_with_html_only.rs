mod function_component_set;

use coyotes::HtmlOnly;
use function_component_set as fcs;

#[test]
fn form_component_retains_spacing() {
    let template = fcs::form_component_retains_spacing();
    let expected = "<form action=\"/uwu\" method=\"post\">\nyou're a boy kisser aren't you >:3\n<input type=submit value=\"yus -_-\">\n</form>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn elememt_and_text_components_retains_spacing() {
    let template = fcs::elememt_and_text_components_retains_spacing();
    let expected = "<div>hai :3hai :3</div>
<div>
hai :3hai :3
</div>
<div>hai :3 hai :3</div>
<div>
hai :3 hai :3
</div>
<div>
hai :3
hai :3
</div>
<div>
hai :3
hai :3
</div>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_and_text_components_retain_extra_spacey_spacing() {
    let template = fcs::element_and_text_components_retain_extra_spacey_spacing();
    let expected = "<div>
hai :3

hai :3
</div>
<div>

hai :3

hai :3

</div>
<div>
hai :3

hai :3
</div>
<div>

hai :3

hai :3

</div>
<div>

hai :3


hai :3

</div>
<div>

hai :3


hai :3

</div>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_components_retain_spacing() {
    let template = fcs::element_components_retain_spacing();
    let expected = "<div><span> hai :3 </span><span> hai :3 </span></div>
<div>
<span> hai :3 </span><span> hai :3 </span>
</div>
<div><span> hai :3 </span> <span> hai :3 </span></div>
<div>
<span> hai :3 </span> <span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_components_retain_extra_spacey_spacing() {
    let template = fcs::element_components_retain_extra_spacey_spacing();
    let expected = "<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>
<div>
<span> hai :3 </span>
<span> hai :3 </span>
</div>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attributes_retain_spacing() {
    let template = fcs::attributes_retain_spacing();
    let expected = "<p hai></p>\n<p hai\n></p>\n<p\nhai></p>\n<p\nhai\n>\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_component_injections_retain_spacing() {
    let template = fcs::attribute_component_injections_retain_spacing();
    let expected = "<p hai hello yo=\"what's good!\" hey=\"\nhowdy!\n\nhowdy!\n\nhurray!\n\">\n</p>\n<p\nhai\nhello\nyo=\"what's good!\"\nhey=\"\nhowdy!\n\nhowdy!\n\nhurray!\n\"\n>\n</p>\n<span hai hello yo=\"what's good!\" hey=\"\nhowdy!\n\nhowdy!\n\nhurray!\n\"></span>\n<span hai hello yo=\"what's good!\" hey=\"\nhowdy!\n\nhowdy!\n\nhurray!\n\"></span>";

    let mut html = HtmlOnly::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
