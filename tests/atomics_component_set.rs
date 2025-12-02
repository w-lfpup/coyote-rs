use coyotes::{Component, tmpl, tmpl_string};

pub fn text_element() -> Component {
    tmpl(
        "

			Beasts tread
				softly   underfoot.

		",
        [],
    )
}

pub fn empty_element() -> Component {
    tmpl(
        "
		<p>
		</p>
		",
        [],
    )
}

pub fn fragment() -> Component {
    tmpl(
        "
		<>
		</>
		",
        [],
    )
}

pub fn block_element_with_text() -> Component {
    tmpl(
        "
		<p>
			hello!
		</p>
		",
        [],
    )
}

pub fn block_element_with_text_for_string() -> Component {
    tmpl_string(
        "
		<p>
			hello!
		</p>
		",
        [],
    )
}

pub fn inline_element_with_text() -> Component {
    tmpl(
        "
		<b> hello! </b>
		",
        [],
    )
}

pub fn void_element() -> Component {
    tmpl(
        "
		<input>
		",
        [],
    )
}

pub fn void_element_with_self_closing() -> Component {
    tmpl(
        "
		<input />
		",
        [],
    )
}

pub fn non_void_element() -> Component {
    tmpl(
        "
		<p />
		",
        [],
    )
}

// needs updating
pub fn comment_element() -> Component {
    tmpl(
        "
		<!-- Hello! -->
		",
        [],
    )
}

pub fn alt_text_element() -> Component {
    tmpl(
        "<style>#woof .bark {
			color: doggo;
		}</style>",
        [],
    )
}

pub fn alt_element_has_no_descendants() -> Component {
    tmpl(
        "
		<script>
			{}
		</script>
		",
        [],
    )
}

pub fn preserved_text_element_retains_spacing() -> Component {
    tmpl(
        "
<pre>
	U w U
	  woof woof!
</pre>
		",
        [],
    )
}

pub fn attribute() -> Component {
    tmpl("<span hai>UwU</span>", [])
}

pub fn attribute_with_single_quote() -> Component {
    tmpl("<span hai=''>UwU</span>", [])
}

pub fn attribute_with_double_quote() -> Component {
    tmpl("<span hai=\"\">UwU</span>", [])
}

pub fn attribute_with_single_quote_value() -> Component {
    tmpl("<span hai='hewoo'>UwU</span>", [])
}

pub fn attribute_with_double_quote_value() -> Component {
    tmpl("<span hai=\"hewoo\">UwU</span>", [])
}
