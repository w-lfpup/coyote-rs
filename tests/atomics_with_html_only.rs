use coyote::{tmpl, HtmlOnly};

#[test]
fn text_element() {
    let template = tmpl(
        "

            Beasts tread
                softly     underfoot.

		",
        [],
    );
    let expected = "Beasts tread\nsoftly underfoot.";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn empty_element() {
    let template = tmpl(
        "
		<p>
		</p>
		",
        [],
    );
    let expected = "<p>\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn fragment() {
    let template = tmpl(
        "
		<>
		</>
		",
        [],
    );
    let expected = "";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn block_element_with_text() {
    let template = tmpl(
        "
		<p>
            hello!
        </p>
		",
        [],
    );
    let expected = "<p>\nhello!\n</p>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_element_with_text() {
    let template = tmpl(
        "
		<b> hello! </b>
		",
        [],
    );
    let expected = "<b> hello! </b>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element() {
    let template = tmpl(
        "
		<input>
		",
        [],
    );
    let expected = "<input>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element_with_self_closing() {
    let template = tmpl(
        "
		<input />
		",
        [],
    );
    let expected = "<input>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn non_void_element() {
    let template = tmpl(
        "
		<p />
		",
        [],
    );
    let expected = "<p></p>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

// needs updating
#[test]
fn comment_element() {
    let template = tmpl(
        "
		<!-- Hello! -->
		",
        [],
    );
    let expected = "";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn alt_text_element() {
    let template = tmpl(
        "<style>#woof .bark {
	color: doggo;
}</style>",
        [],
    );
    let expected = "";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn alt_element_has_no_descendants() {
    let template = tmpl(
        "
		<script>
			{}
		</script>
		",
        [],
    );
    let expected = "";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn preserved_text_element_retains_spacing() {
    let template = tmpl(
        "
<pre>
	U w U
	  woof woof!
</pre>
		",
        [],
    );

    let expected = "<pre>\n\tU w U\n\t  woof woof!\n</pre>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute() {
    let template = tmpl("<span hai>UwU</span>", []);
    let expected = "<span hai>UwU</span>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_single_quote() {
    let template = tmpl("<span hai=''>UwU</span>", []);
    let expected = "<span hai>UwU</span>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_double_quote() {
    let template = tmpl("<span hai=\"\">UwU</span>", []);
    let expected = "<span hai>UwU</span>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_single_quote_value() {
    let template = tmpl("<span hai='hewoo'>UwU</span>", []);
    let expected = "<span hai='hewoo'>UwU</span>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_with_double_quote_value() {
    let template = tmpl("<span hai=\"hewoo\">UwU</span>", []);
    let expected = "<span hai=\"hewoo\">UwU</span>";

    let mut html = HtmlOnly::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
