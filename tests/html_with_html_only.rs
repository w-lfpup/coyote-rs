use coyote::{attr, tmpl, HtmlOnly};

#[test]
fn empty_element_retains_spacing() {
    let template = tmpl(
        "
        <p></p>
		<p> </p><p>
        </p>
		",
        [],
    );
    let expected = "<p></p>\n<p> </p><p>\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn fragments_dont_exist() {
    let template = tmpl(
        "
		<><>
        </></>
		",
        [],
    );
    let expected = "";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn block_element_with_text_retains_spacing() {
    let template = tmpl(
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
    );
    let expected = "<p>hello!</p>\n<p> hello! </p>\n<p>\nhello\n</p><p>\nhello\n</p>\n<p>hello\n</p>\n<p>\nhello</p>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_element_with_text_retains_spacing() {
    let template = tmpl(
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
    );

    let expected = "<b>hello!</b>\n<b> hello! </b>\n<b> hello\n</b>\n<b>\nhello </b>\n<b>\nhello\n</b>\n<b>\nhello\n</b>\n<b>hello\n</b>\n<b>\nhello</b>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn comment_element_retains_spacing() {
    let template = tmpl(
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
    );
    let expected = "<!---->\n<!--Hello!-->\n<!-- Hello! -->\n<!--Hello! -->\n<!-- Hello!-->\n<!--Hello!\n-->\n<!--\nHello!-->\n<!--\n\nHello!\n\n-->";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn empty_element_stays_empty() {
    let template = tmpl("<html></html>", []);
    let expected = "<html></html>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn unbalanced_empty_elemen_errors_out() {
    let template = tmpl("<html>", []);

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    if let Err(_) = results {
        return;
    }

    assert!(false, "unbalanced template failed to error",);
}

#[test]
fn forbidden_attribute_injection_glyph_errors_out() {
    let template = tmpl("<p {}></p>", [attr("a<b/c'd=e>f")]);

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    if let Err(_) = results {
        return;
    }

    assert!(false, "forbidden attribute glyph failed to error",);
}

#[test]
fn mozilla_spacing_example_passes() {
    let template = tmpl(
        "
        <h1>   Hello
                <span> World!</span>   </h1>",
        [],
    );

    let expected = "<h1> Hello\n<span> World!</span> </h1>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_value_retains_spacing() {
    let template = tmpl(
        "
		<h1 wow='People use
			attributes in some very
			wild ways but thats okay'>   Hello
				<span> World!</span>   </h1>
		<h1 wow='

			People use attributes in some very

			wild ways but thats okay
	
			'>
			Hello! <span> World!</span>
		</h1>
		",
        [],
    );

    let expected = "<h1 wow='People use
attributes in some very
wild ways but thats okay'> Hello
<span> World!</span> </h1>
<h1 wow='

People use attributes in some very

wild ways but thats okay

'>
Hello! <span> World!</span>
</h1>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_retain_spacing() {
    let template = tmpl(
        "<input>   <input>
            <input><input> ",
        [],
    );

    let expected = "<input> <input>\n<input><input>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_with_inline_elements_retain_spacing() {
    let template = tmpl(
        "beasts <span>    tread		</span>     softly <span>    underfoot </span>      .",
        [],
    );

    let expected = "beasts <span> tread </span> softly <span> underfoot </span> .";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_with_block_elements_retain_spacing() {
    let template = tmpl(
        "beasts <p>    tread		</p>     softly <p>    underfoot </p>      .",
        [],
    );

    let expected = "beasts <p> tread </p> softly <p> underfoot </p> .";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_can_have_attributes() {
    let template = tmpl(
        "
        <!DOCTYPE html><input type=checkbox>   <input woof=\"bark\">
            <input grrr><input> ",
        [],
    );
    let expected =
        "<!DOCTYPE html><input type=checkbox> <input woof=\"bark\">\n<input grrr><input>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element_with_sibling() {
    let template = tmpl(
        "
            <input><p>hai :3</p>    ",
        [],
    );
    let expected = "<input><p>hai :3</p>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_void_element_with_siblings_retains_spacing() {
    let template = tmpl(
        "
        <section>
            <input><p>hai :3</p>
        </section>
    ",
        [],
    );

    let expected = "<section>\n<input><p>hai :3</p>\n</section>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_elements_and_text_retain_spacing() {
    let template = tmpl("<a><label><input type=woofer>bark!</label><img></a>", []);
    let expected = "<a><label><input type=woofer>bark!</label><img></a>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_retains_spacing() {
    let template = tmpl(
        "        <!DOCTYPE>
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
    );

    let expected =
    "<!DOCTYPE>\n<html>\n<head>\n</head>\n<body>\n<article>\nYou're a <span>boy kisser</span> aren't you?\nClick <a>here</a> and go somewhere else.\n</article>\n<footer></footer>\n</body>\n</html>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_with_alt_text_elements_retains_spacing() {
    let template = tmpl(
        "        <!DOCTYPE>
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
    );

    let expected =
        "<!DOCTYPE>\n<html>\n<head>\n</head>\n<body>\n<article></article>\n<footer></footer>\n</body>\n</html>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
