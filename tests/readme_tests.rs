use coyotes::{Html, tmpl, text, list, tmpl_string};

#[test]
fn no_added_spaces() {
    let template = tmpl("<p>hai :3</p>", []);
    let expected = "<p>hai :3</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn collapse_spaces() {
    let template = tmpl("<p>   hai   :3   </p>", []);
    let expected = "<p> hai :3 </p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_collapse_spaces() {
    let template = tmpl("<p    attr    attr2    att3    ></p>", []);
    let expected = "<p attr attr2 att3></p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn attribute_preserve_new_lines() {
    let template = tmpl(
		"
		<p

			attr

			attr2

			attr3>

		</p>
		",
		[]
	);
    let expected = "<p\n\tattr\n\tattr2\n\tattr3>\n</p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}


#[test]
fn attribute_values_preserve_new_lines() {
    let template = tmpl("
		<p
			attr='

			hai   :3    hello!

			'
		></p>
		",
		[]
	);
    
	let expected = "<p\n\tattr='\n\n\thai :3 hello!\n\n\t'></p>";

    let mut html = Html::new();
    let results = html.render(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

// #[test]
// fn preserve_new_lines() {
//     let component = text(
// 		"
		
// 		hai    :3
		
// 		"
// 	);

//     let expected = "\n\n\thai :3\n";

//     let mut html = Html::new();
//     let results = html.render(&component);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn descendant_injections() {
//     let descendants = list([
// 		tmpl(" <span>hai :3</span> ", []),
// 		tmpl(
// 			"
// 			<span>hello</span>
// 			", []),
// 	]);

// 	let component = tmpl(
// 		"<p>{}</p>",
// 		[descendants]
// 	);

//     let expected = "<p><span>hai :3</span><span>hello</span></p>";

//     let mut html = Html::new();
//     let results = html.render(&component);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn descendant_injections_with_space() {
//     let descendants = list([
// 		tmpl("<span>hai :3</span> ", []),
// 		tmpl("<span>hello</span>", []),
// 	]);

// 	let component = tmpl(
// 		"<p> {}</p>",
// 		[descendants]
// 	);

//     let expected = "<p><span>hai :3</span><span>hello</span></p>";

//     let mut html = Html::new();
//     let results = html.render(&component);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn descendant_injections_with_new_lines() {
//     let descendants = list([
// 		tmpl(
// 			"
// 			<span>hai :3</span>
// 			", []),
// 		tmpl(
// 			"
// 			<span>hello</span>
// 			", []),
// 	]);

// 	let component = tmpl(
// 		"<p>
// 			{}
// 		</p>",
// 		[descendants]
// 	);

//     let expected = "<p>\n\t<span>hai :3</span>\n\t<span>hello</span>\n</p>";

//     let mut html = Html::new();
//     let results = html.render(&component);

//     assert_eq!(Ok(expected.to_string()), results);
// }