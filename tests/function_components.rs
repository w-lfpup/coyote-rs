// Test will not build if function components do not build

use coyote::{attr_val, list, text, tmpl, vlist, Component, Html};

fn submit_button() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn form() -> Component {
    let attributes = [attr_val("action", "/uwu"), attr_val("method", "post")];

    let mut descendants: Vec<Component> = Vec::new();
    descendants.push(text("you're a boy kisser aren't you >:3"));
    descendants.push(submit_button());

    tmpl(
        "
		<form {}>
			{}
		</form>
		",
        [list(attributes), vlist(descendants)],
    )
}

#[test]
fn form_component_retains_spacing() {
    let template = form();

    let expected = "<form action=\"/uwu\" method=\"post\">\n\tyou're a boy kisser aren't you >:3\n\t<input type=submit value=\"yus -_-\">\n</form>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

fn hai() -> Component {
    text("hai :3")
}

fn lil_divs(hai: fn() -> Component) -> Component {
    tmpl(
        "

		<div>{}{}</div>
		<div>
			{}{}
		</div>
		<div>{} {}</div>
		<div>
			{} {}
		</div>
		<div>
			{}
			{}
		</div>
		<div>
			{}

			{}
		</div>
		",
        [
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
            hai(),
        ],
    )
}

#[test]
fn elememt_and_text_components_retains_spacing() {
    let template = lil_divs(hai);

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

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

fn spacey_hai() -> Component {
    text(
        "
		hai :3
		",
    )
}

#[test]
fn element_and_text_components_retain_extra_spacey_spacing() {
    let template = lil_divs(spacey_hai);

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

    let expected2 = "<div>hai :3hai :3</div>
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
    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

fn lots_of_attributes() -> Component {
    tmpl(
        "
		<p hai></p>
		<p hai
		></p>
		<p
		hai ></p>
		<p
		hai
		>
		</p>
		",
        [],
    )
}

#[test]
fn attributes_retain_spacing() {
    let template = lots_of_attributes();

    let expected = "<p hai></p>
<p hai></p>
<p
	hai></p>
<p
	hai>
</p>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_components_retain_spacing() {
    let template = lots_of_attributes();

    let expected = "<p hai></p>
<p hai></p>
<p
	hai></p>
<p
	hai>
</p>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
