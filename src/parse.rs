use crate::routes;
use crate::routes::StepKind;
use crate::rulesets::RulesetImpl;
use crate::sliding_window::SlidingWindow;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Step {
    pub kind: StepKind,
    pub origin: usize,
    pub target: usize,
}

pub fn parse_str(rules: &dyn RulesetImpl, template_str: &str, intial_kind: StepKind) -> Vec<Step> {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    // this is the "state" of parsing.
    let mut tag: &str = "";
    let mut inj_kind = intial_kind;
    let mut sliding_window: Option<SlidingWindow> = None;
    let mut contentless_edge = false;

    for (index, glyph) in template_str.char_indices() {
        let mut next_step_origin = index;

        // <!--comment_edge_case-->
        if contentless_edge {
            contentless_edge = false;
            if let Err(_) = push_contentless_steps_edge(rules, &mut steps, tag, index) {
                return steps;
            };
        }

        if let Some(ref mut slider) = sliding_window {
            if !slider.slide(glyph) {
                continue;
            }

            // <-- comment case -->
            if let Some(_closing_sequence) = rules.get_close_sequence_from_contentless_tag(tag) {
                if let Err(_) = push_contentless_steps(rules, &mut steps, tag, index) {
                    return steps;
                };
            }

            if let Some(_alt_text_tag) = rules.get_close_sequence_from_alt_text_tag(tag) {
                if let Err(_) = push_alt_element_steps(rules, &mut steps, tag, index) {
                    return steps;
                };
            }

            sliding_window = None;
            continue;
        }

        // route next step
        let end_step = match steps.last_mut() {
            Some(step) => step,
            _ => return steps,
        };
        // mark progression
        end_step.target = index;

        // step kind delta
        let mut curr_kind = match end_step.kind {
            StepKind::InjectionConfirmed => routes::route(glyph, &inj_kind),
            _ => routes::route(glyph, &end_step.kind),
        };
        if curr_kind == end_step.kind {
            continue;
        }
        if is_injection_kind(&curr_kind) {
            inj_kind = end_step.kind.clone();
        }

        match end_step.kind {
            StepKind::ElementClosed => {
                // ALT ELEMENTS
                if let Some(close_seq) = rules.get_close_sequence_from_alt_text_tag(tag) {
                    let mut slider = SlidingWindow::new(close_seq);
                    slider.slide(glyph);
                    sliding_window = Some(slider);
                    curr_kind = StepKind::TextAlt;
                }
            }
            StepKind::Tag => {
                tag = get_text_from_step(template_str, &end_step);

                // COMMENTS
                if let Some(prefix) = rules.tag_prefix_of_contentless(tag) {
                    let diff = &tag[prefix.len()..];
                    tag = prefix;

                    end_step.target = end_step.origin + prefix.len();
                    next_step_origin = end_step.target;

                    if let Some(close_seq) = rules.get_close_sequence_from_contentless_tag(prefix) {
                        curr_kind = StepKind::TextAlt;

                        let mut slider = SlidingWindow::new(close_seq);
                        for glypher in diff.chars() {
                            slider.slide(glypher);
                        }

                        match slider.slide(glyph) {
                            true => contentless_edge = true,
                            _ => sliding_window = Some(slider),
                        }
                    }
                }
            }
            _ => {}
        }

        // Add CURRENT STEP
        steps.push(Step {
            kind: curr_kind,
            origin: next_step_origin,
            target: index,
        });
    }

    if let Some(step) = steps.last_mut() {
        step.target = template_str.len();
    }

    steps
}

pub fn get_text_from_step<'a>(template_str: &'a str, step: &Step) -> &'a str {
    &template_str[step.origin..step.target]
}

fn is_injection_kind(step_kind: &StepKind) -> bool {
    match step_kind {
        StepKind::AttrMapInjection => true,
        StepKind::DescendantInjection => true,
        _ => false,
    }
}

fn push_alt_element_steps(
    rules: &dyn RulesetImpl,
    steps: &mut Vec<Step>,
    tag: &str,
    index: usize,
) -> Result<(), ()> {
    let step = match steps.last_mut() {
        Some(step) => step,
        _ => return Err(()),
    };

    let closing_sequence = match rules.get_close_sequence_from_alt_text_tag(tag) {
        Some(sequence) => sequence,
        _ => return Ok(()),
    };

    step.target = index - (closing_sequence.len() - 1);
    steps.push(Step {
        kind: StepKind::TailTag,
        origin: index - (closing_sequence.len() - 1),
        target: index - (closing_sequence.len()),
    });

    Ok(())
}

fn push_contentless_steps(
    rules: &dyn RulesetImpl,
    steps: &mut Vec<Step>,
    tag: &str,
    index: usize,
) -> Result<(), ()> {
    let closing_sequence = match rules.get_close_sequence_from_contentless_tag(tag) {
        Some(sequence) => sequence,
        _ => return Ok(()),
    };

    let step = match steps.last_mut() {
        Some(step) => step,
        _ => return Err(()),
    };

    step.target = index - (closing_sequence.len() - 1);
    steps.push(Step {
        kind: StepKind::TailTag,
        origin: index - (closing_sequence.len() - 1),
        target: index,
    });
    steps.push(Step {
        kind: StepKind::TailElementClosed,
        origin: index,
        target: index,
    });

    Ok(())
}

fn push_contentless_steps_edge(
    rules: &dyn RulesetImpl,
    steps: &mut Vec<Step>,
    tag: &str,
    index: usize,
) -> Result<(), ()> {
    let closing_sequence = match rules.get_close_sequence_from_contentless_tag(tag) {
        Some(sequence) => sequence,
        _ => return Ok(()),
    };

    let step = match steps.last_mut() {
        Some(step) => step,
        _ => return Err(()),
    };

    let target = step.target;

    step.target = step.target - closing_sequence.len() + 1;
    let next_origin = step.target;
    let next_target = step.target + closing_sequence.len() - 1;

    match step.target == step.origin {
        true => {
            // <!---->
            step.kind = StepKind::TailTag;
            step.target = next_target;
        }
        _ => {
            // <!--case-->
            steps.push(Step {
                kind: StepKind::TailTag,
                origin: next_origin,
                target: next_target,
            });
        }
    }

    steps.push(Step {
        kind: StepKind::TailElementClosed,
        origin: target,
        target: index,
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use crate::components::{attr_val, list, text, tmpl, Component};
    use crate::routes::StepKind;
    use crate::rulesets::ServerRules;
    use crate::template_steps::{compose, Results};

    fn woof() -> Component {
        tmpl("<input type=submit value=\"yus -_-\">", [])
    }

    fn woof_woof() -> Component {
        let descendants = list([text("you're a boy kisser aren't you >:3"), woof()]);

        let attributes = list([attr_val("action", "/uwu"), attr_val("method", "post")]);

        tmpl("<form {}>{}</form>", [attributes, descendants])
    }

    #[test]
    fn test_parse_str() {
        let rules = ServerRules::new();

        let template = woof_woof();
        let expected = Results {
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

        if let Component::Tmpl(tmpl) = template {
            let results = compose(&rules, &tmpl.template_str);
            assert_eq!(expected, results);
        }
    }
}
