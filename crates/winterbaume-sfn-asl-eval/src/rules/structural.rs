use crate::Diagnostic;
use crate::model::{StateMachine, StateType};

/// Run Phase 1 structural validation rules against a parsed state machine.
pub fn check_structural(sm: &StateMachine, diagnostics: &mut Vec<Diagnostic>) {
    // 1. NO_SUCH_STATE — StartAt references nonexistent state
    if let Some(ref start) = sm.start_at {
        if !sm.states.contains_key(start) {
            diagnostics.push(Diagnostic {
                code: "NO_SUCH_STATE",
                message: format!(
                    "StartAt value \"{start}\" references a state that does not exist"
                ),
                location: Some(format!("{}/StartAt", sm.pointer)),
            });
        }
    }

    for (name, state) in &sm.states {
        let st = &state.state_type;

        // 2. MISSING_RESOURCE_URI — Task state without Resource
        if st.as_ref() == Some(&StateType::Task) && state.resource.is_none() {
            diagnostics.push(Diagnostic {
                code: "MISSING_RESOURCE_URI",
                message: format!("Task state \"{name}\" is missing required field \"Resource\""),
                location: Some(state.pointer.clone()),
            });
        }

        // 3. MISSING_ITERATOR — Map state without Iterator or ItemProcessor
        if st.as_ref() == Some(&StateType::Map) && !state.has_item_processor {
            diagnostics.push(Diagnostic {
                code: "MISSING_ITERATOR",
                message: format!("Map state \"{name}\" is missing required field \"Iterator\" or \"ItemProcessor\""),
                location: Some(state.pointer.clone()),
            });
        }

        // 4. MISSING_CHOICES — Choice state with empty or absent Choices
        if st.as_ref() == Some(&StateType::Choice) && !state.has_choices {
            diagnostics.push(Diagnostic {
                code: "MISSING_CHOICES",
                message: format!("Choice state \"{name}\" must have a non-empty \"Choices\" array"),
                location: Some(state.pointer.clone()),
            });
        }

        // 5. MISSING_END_OR_NEXT — non-terminal, non-Choice state with neither End nor Next
        if let Some(t) = st {
            if !t.is_terminal() && *t != StateType::Choice && !state.end && state.next.is_none() {
                diagnostics.push(Diagnostic {
                    code: "MISSING_END_OR_NEXT",
                    message: format!(
                        "State \"{name}\" must have either \"End\": true or a \"Next\" field"
                    ),
                    location: Some(state.pointer.clone()),
                });
            }
        }

        // 5b. CONFLICTING_END_AND_NEXT — state has both End: true and Next
        if state.end && state.next.is_some() {
            diagnostics.push(Diagnostic {
                code: "CONFLICTING_END_AND_NEXT",
                message: format!(
                    "State \"{name}\" must not have both \"End\": true and a \"Next\" field"
                ),
                location: Some(state.pointer.clone()),
            });
        }

        // 6. NO_SUCH_STATE — Next references nonexistent state
        if let Some(ref next) = state.next {
            if !sm.states.contains_key(next) {
                diagnostics.push(Diagnostic {
                    code: "NO_SUCH_STATE",
                    message: format!(
                        "State \"{name}\" Next value \"{next}\" references a state that does not exist"
                    ),
                    location: Some(format!("{}/Next", state.pointer)),
                });
            }
        }

        // 7. NO_SUCH_STATE — Default references nonexistent state
        if let Some(ref default) = state.default {
            if !sm.states.contains_key(default) {
                diagnostics.push(Diagnostic {
                    code: "NO_SUCH_STATE",
                    message: format!(
                        "Choice state \"{name}\" Default value \"{default}\" references a state that does not exist"
                    ),
                    location: Some(format!("{}/Default", state.pointer)),
                });
            }
        }

        // 8. NO_SUCH_STATE — Choice targets reference nonexistent states
        for (i, target) in state.choice_targets.iter().enumerate() {
            if !sm.states.contains_key(target) {
                diagnostics.push(Diagnostic {
                    code: "NO_SUCH_STATE",
                    message: format!(
                        "Choice state \"{name}\" Choices[{i}].Next value \"{target}\" references a state that does not exist"
                    ),
                    location: Some(format!("{}/Choices/{i}/Next", state.pointer)),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::model::State;

    fn make_sm(start_at: Option<&str>, states: Vec<State>) -> StateMachine {
        let mut map = HashMap::new();
        for s in states {
            map.insert(s.name.clone(), s);
        }
        StateMachine {
            start_at: start_at.map(String::from),
            states: map,
            pointer: String::new(),
        }
    }

    fn pass_state(name: &str, end: bool, next: Option<&str>) -> State {
        State {
            name: name.to_string(),
            state_type: Some(StateType::Pass),
            resource: None,
            next: next.map(String::from),
            end,
            choice_targets: Vec::new(),
            default: None,
            has_choices: false,
            has_item_processor: false,
            pointer: format!("/States/{name}"),
        }
    }

    #[test]
    fn valid_minimal_no_diagnostics() {
        let sm = make_sm(Some("A"), vec![pass_state("A", true, None)]);
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.is_empty());
    }

    #[test]
    fn no_such_state_start_at() {
        let sm = make_sm(Some("Missing"), vec![pass_state("A", true, None)]);
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.iter().any(|d| d.code == "NO_SUCH_STATE"));
    }

    #[test]
    fn no_such_state_next() {
        let sm = make_sm(Some("A"), vec![pass_state("A", false, Some("Missing"))]);
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.iter().any(|d| d.code == "NO_SUCH_STATE"));
    }

    #[test]
    fn missing_resource_uri() {
        let sm = make_sm(
            Some("T"),
            vec![State {
                name: "T".into(),
                state_type: Some(StateType::Task),
                resource: None,
                next: None,
                end: true,
                choice_targets: Vec::new(),
                default: None,
                has_choices: false,
                has_item_processor: false,
                pointer: "/States/T".into(),
            }],
        );
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.iter().any(|d| d.code == "MISSING_RESOURCE_URI"));
    }

    #[test]
    fn missing_iterator() {
        let sm = make_sm(
            Some("M"),
            vec![State {
                name: "M".into(),
                state_type: Some(StateType::Map),
                resource: None,
                next: None,
                end: true,
                choice_targets: Vec::new(),
                default: None,
                has_choices: false,
                has_item_processor: false,
                pointer: "/States/M".into(),
            }],
        );
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.iter().any(|d| d.code == "MISSING_ITERATOR"));
    }

    #[test]
    fn missing_choices() {
        let sm = make_sm(
            Some("C"),
            vec![State {
                name: "C".into(),
                state_type: Some(StateType::Choice),
                resource: None,
                next: None,
                end: false,
                choice_targets: Vec::new(),
                default: None,
                has_choices: false,
                has_item_processor: false,
                pointer: "/States/C".into(),
            }],
        );
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.iter().any(|d| d.code == "MISSING_CHOICES"));
    }

    #[test]
    fn missing_end_or_next() {
        let sm = make_sm(
            Some("A"),
            vec![State {
                name: "A".into(),
                state_type: Some(StateType::Pass),
                resource: None,
                next: None,
                end: false,
                choice_targets: Vec::new(),
                default: None,
                has_choices: false,
                has_item_processor: false,
                pointer: "/States/A".into(),
            }],
        );
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(diags.iter().any(|d| d.code == "MISSING_END_OR_NEXT"));
    }

    #[test]
    fn no_such_state_default() {
        let sm = make_sm(
            Some("C"),
            vec![State {
                name: "C".into(),
                state_type: Some(StateType::Choice),
                resource: None,
                next: None,
                end: false,
                choice_targets: vec!["C".into()],
                default: Some("Missing".into()),
                has_choices: true,
                has_item_processor: false,
                pointer: "/States/C".into(),
            }],
        );
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == "NO_SUCH_STATE" && d.message.contains("Default"))
        );
    }

    #[test]
    fn no_such_state_choice_target() {
        let sm = make_sm(
            Some("C"),
            vec![State {
                name: "C".into(),
                state_type: Some(StateType::Choice),
                resource: None,
                next: None,
                end: false,
                choice_targets: vec!["Ghost".into()],
                default: None,
                has_choices: true,
                has_item_processor: false,
                pointer: "/States/C".into(),
            }],
        );
        let mut diags = Vec::new();
        check_structural(&sm, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == "NO_SUCH_STATE" && d.message.contains("Choices[0]"))
        );
    }
}
