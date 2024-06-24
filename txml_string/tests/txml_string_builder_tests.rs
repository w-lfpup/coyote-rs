use parsley::{get_text_from_step, parse_template_str, Step, StepKind};
use txml::{attr_val, list, text, tmpl, Component};
use txml_string::{TxmlBuilder, TxmlBuilderResults};

// Test will not build if Function Components do not build

fn woof() -> Component {
    tmpl("<input type=submit value=\"yus -_-\">", [])
}

fn woof_woof() -> Component {
    let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

    let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

    tmpl("<form {}>{}</form>", [attributes, descendants])
}

#[test]
fn test_txml_builder() {
    let template = woof_woof();
    let builder = TxmlBuilder::new();

    if let Component::Tmpl(tmpl) = template {
        let results = builder.build(&tmpl.template_str);
        println!("{:?}", results);
    }
}
