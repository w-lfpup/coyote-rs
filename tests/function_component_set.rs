use coyotes::{Component, attr, attr_val, list, text, tmpl, vlist};

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

pub fn form_component_retains_spacing() -> Component {
    form()
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

pub fn elememt_and_text_components_retains_spacing() -> Component {
    lil_divs(hai)
}

fn spacey_hai() -> Component {
    text(
        "
		hai :3
		",
    )
}

pub fn element_and_text_components_retain_extra_spacey_spacing() -> Component {
    lil_divs(spacey_hai)
}

fn el_hai() -> Component {
    tmpl("<span> hai :3 </span>", [])
}

pub fn element_components_retain_spacing() -> Component {
    lil_divs(el_hai)
}

fn el_hai_extra_spacey() -> Component {
    tmpl(
        "
        <span> hai :3 </span>
        ",
        [],
    )
}

pub fn element_components_retain_extra_spacey_spacing() -> Component {
    lil_divs(el_hai_extra_spacey)
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

pub fn attributes_retain_spacing() -> Component {
    lots_of_attributes()
}

fn attribute_list() -> Component {
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

pub fn attribute_component_injections_retain_spacing() -> Component {
    lil_attributes(attribute_list)
}
