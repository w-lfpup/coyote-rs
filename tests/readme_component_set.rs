use coyotes::{Component, attr, list, text, tmpl};

pub fn no_added_spaces() -> Component {
    tmpl("<p>hai :3</p>", [])
}

pub fn collapse_spaces() -> Component {
    tmpl("<p>   hai   :3   </p>", [])
}

pub fn attribute_collapse_spaces() -> Component {
    tmpl("<p    attr    attr2    att3    ></p>", [])
}

pub fn attribute_preserve_new_lines() -> Component {
    tmpl(
        "
		<p

			attr

			attr2

			attr3>

		</p>
		",
        [],
    )
}

pub fn attribute_values_preserve_new_lines() -> Component {
    tmpl(
        "
		<p
			attr='

			hai   :3    hello!

			'
		></p>
		",
        [],
    )
}

pub fn attribute_injections() -> Component {
    let descendants = list([attr("hai"), attr("hello")]);

    tmpl("<p {}></p>", [descendants])
}

pub fn attribute_injections_with_new_lines() -> Component {
    let descendants = list([attr("hai"), attr("hello")]);

    tmpl(
        "
        <p
            {}></p>
        ",
        [descendants],
    )
}

pub fn component_injections() -> Component {
    let descendants = list([
        tmpl(" <span>hai :3</span> ", []),
        tmpl(
            "

			<span>hello</span>
			
			",
            [],
        ),
    ]);

    tmpl("<p>{}</p>", [descendants])
}

pub fn text_component_injections() -> Component {
    let descendants = text(
        "

		hai    :3

		",
    );

    tmpl(
        "
		<p>{}</p>
		",
        [descendants],
    )
}
