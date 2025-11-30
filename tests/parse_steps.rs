use coyote_rs::{Component, HtmlRules, Step, StepKind, TemplateSteps, compose, tmpl};

fn woof_woof() -> Component {
    tmpl("<form {}>{}</form>", [])
}

#[test]
fn test_parse_str() {
    let rules = HtmlRules::new();

    let expected = TemplateSteps {
        steps: Vec::from([
            Vec::from([
                Step {
                    kind: StepKind::Initial,
                    origin: 0,
                    target: 0,
                },
                Step {
                    kind: StepKind::Element,
                    origin: 0,
                    target: 1,
                },
                Step {
                    kind: StepKind::Tag,
                    origin: 1,
                    target: 5,
                },
                Step {
                    kind: StepKind::ElementSpace,
                    origin: 5,
                    target: 6,
                },
            ]),
            Vec::from([
                Step {
                    kind: StepKind::InjectionConfirmed,
                    origin: 7,
                    target: 8,
                },
                Step {
                    kind: StepKind::ElementClosed,
                    origin: 8,
                    target: 9,
                },
            ]),
            Vec::from([
                Step {
                    kind: StepKind::InjectionConfirmed,
                    origin: 10,
                    target: 11,
                },
                Step {
                    kind: StepKind::Element,
                    origin: 11,
                    target: 12,
                },
                Step {
                    kind: StepKind::TailElementSolidus,
                    origin: 12,
                    target: 13,
                },
                Step {
                    kind: StepKind::TailTag,
                    origin: 13,
                    target: 17,
                },
                Step {
                    kind: StepKind::TailElementClosed,
                    origin: 17,
                    target: 18,
                },
            ]),
        ]),
        injs: Vec::from([
            Step {
                kind: StepKind::AttrMapInjection,
                origin: 6,
                target: 7,
            },
            Step {
                kind: StepKind::DescendantInjection,
                origin: 9,
                target: 10,
            },
        ]),
    };

    let woof_woof_components = woof_woof();

    if let Component::Tmpl(tmpl, _) = woof_woof_components {
        let results = compose(&rules, &tmpl.template_str);
        assert_eq!(expected, results);
    }
}
