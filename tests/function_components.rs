use coyotes::{Component, Html, attr, attr_val, list, text, tmpl, vlist};

fn submit_button() -> Component {
    tmpl(
        "
		<input type=submit value=\"yus -_-\">
		",
        [],
    )
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
    let results = html.render(&template);

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
    let results = html.render(&template);

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

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

fn el_hai() -> Component {
    tmpl("<span> hai :3 </span>", [])
}

#[test]
fn element_components_retain_spacing() {
    let template = lil_divs(el_hai);

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

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

fn el_hai_extra_spacey() -> Component {
    tmpl(
        "
        <span> hai :3 </span>
        ",
        [],
    )
}

#[test]
fn element_components_retain_extra_spacey_spacing() {
    let template = lil_divs(el_hai_extra_spacey);

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

    let mut html = Html::new();
    let results = html.render(&template);

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
    let results = html.render(&template);

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
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

// list of attribute injections
fn attributes() -> Component {
    list([
        attr("hai"),
        attr("hello"),
        attr_val("yo", "what's good!"),
        attr_val(
            "hey",
            "
			howdy!

			howdy!

			hurray!
			",
        ),
    ])
}

fn lil_attributes(hai: fn() -> Component) -> Component {
    tmpl(
        "
		<p {}>
		</p>
		<p
			{}>
		</p>
		<span {}></span>
		<span {}></span>
		",
        [hai(), hai(), hai(), hai()],
    )
}

#[test]
fn attribute_component_injections_retain_spacing() {
    let template = lil_attributes(attributes);

    let expected = "<p hai hello yo=\"what's good!\" hey=\"
	howdy!
	\"
>
</p>
<p
	hai
	hello
	yo=\"what's good!\"
	hey=\"
	howdy!
	\">
</p>
<span hai hello yo=\"what's good!\" hey=\"
howdy!
\"></span>
<span hai hello yo=\"what's good!\" hey=\"
howdy!
\"></span>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
// attributes with line

// attribute with space inline

// attribute with lines
