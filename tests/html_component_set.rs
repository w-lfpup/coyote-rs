use coyotes::{Component, attr, tmpl};

pub fn empty_element_retains_spacing() -> Component {
    tmpl(
        "
		<p></p>
		<p> </p><p>
		</p>
		",
        [],
    )
}

pub fn fragments_dont_exist() -> Component {
    tmpl(
        "
		<><>
		</></>
		",
        [],
    )
}

pub fn block_element_with_text_retains_spacing() -> Component {
    tmpl(
        "
		<p>hello!</p>
		<p> hello! </p>
		<p>
			hello
		</p><p>
hello
		</p>
		<p>hello
		</p>
		<p>
		hello</p>
		",
        [],
    )
}

pub fn inline_element_with_text_retains_spacing() -> Component {
    tmpl(
        "
		<b>hello!</b>
		<b> hello! </b>
		<b> hello
		</b>
		<b>
			hello </b>
		<b>
hello
		</b>
		<b>
			hello
		</b>
		<b>hello
		</b>
		<b>
		hello</b>
		",
        [],
    )
}

pub fn comment_element_retains_spacing() -> Component {
    tmpl(
        "
		<!---->
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

		-->
		",
        [],
    )
}

pub fn empty_element_stays_empty() -> Component {
    tmpl("<html></html>", [])
}

pub fn unbalanced_empty_elemen_errors_out() -> Component {
    tmpl("<html>", [])
}

pub fn forbidden_attribute_injection_glyph_errors_out() -> Component {
    tmpl("<p {}></p>", [attr("a<b/c'd=e>f")])
}

pub fn mozilla_spacing_example_passes() -> Component {
    tmpl(
        "
		<h1>   Hello
				<span> World!</span>   </h1>",
        [],
    )
}

pub fn attribute_value_retains_spacing() -> Component {
    tmpl(
        "
		<h1 
			oh=''
			yikes='woah!'
			oh-no='
				it goes bye bye
			'
			wow='People use
			attributes in some very
			wild ways but thats okay'
		> Hello
				<span> World!</span>   </h1>
		<h1 oh='' yikes='woah!' oh-no='
				it goes bye bye
			' wow='

			People use attributes in some very

			wild ways but thats okay

		'>
			Hello! <span> World!</span>
		</h1>
		",
        [],
    )
}

pub fn void_elements_retain_spacing() -> Component {
    tmpl(
        "<input>   <input>
			<input><input> ",
        [],
    )
}

pub fn text_with_inline_elements_retain_spacing() -> Component {
    tmpl(
        "beasts <span>	tread		</span>	 softly <span>	underfoot </span>	  .",
        [],
    )
}

pub fn text_with_block_elements_retain_spacing() -> Component {
    tmpl("beasts <p>	tread		</p>	 softly <p>	underfoot </p>	  .", [])
}

pub fn void_elements_can_have_attributes() -> Component {
    tmpl(
        "
		<!DOCTYPE html><input type=checkbox>   <input woof=\"bark\">
			<input grrr><input> ",
        [],
    )
}

pub fn void_element_with_sibling() -> Component {
    tmpl(
        "
			<input><p>hai :3</p>	",
        [],
    )
}

pub fn nested_void_element_with_siblings_retains_spacing() -> Component {
    tmpl(
        "
		<section>
			<input><p>hai :3</p>
		</section>
	",
        [],
    )
}

pub fn nested_elements_and_text_retain_spacing() -> Component {
    tmpl("<a><label><input type=woofer>bark!</label><img></a>", [])
}

pub fn document_retains_spacing() -> Component {
    tmpl(
        "		<!DOCTYPE>
	<html>
	<head>

	</head>
		<body>
			<article>
				You're a <span>boy kisser</span> aren't you?
				Click <a>here</a> and go somewhere else.
			</article>
			<footer/>
		</body>
</html>",
        [],
    )
}

pub fn document_with_alt_text_elements_retains_spacing() -> Component {
    tmpl(
        "		<!DOCTYPE>
	<html>
	<head>
		<style>
			#woof .bark {
				color: doggo;
			}
		</style>
		<script>
			if 2 < 3 {
				console.log();
			}
		</script>
	</head>
		<body>
			<article></article>
			<footer/>
		</body>
</html>",
        [],
    )
}
