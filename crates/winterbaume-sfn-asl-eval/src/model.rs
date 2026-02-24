use std::collections::HashMap;

/// The type of an ASL state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateType {
    Task,
    Pass,
    Choice,
    Wait,
    Succeed,
    Fail,
    Parallel,
    Map,
}

impl StateType {
    /// Returns `true` for terminal state types that never need `Next` or `End`.
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Succeed | Self::Fail)
    }
}

/// Error returned when parsing an unknown state type string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownStateType;

impl std::fmt::Display for UnknownStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown ASL state type")
    }
}

impl std::str::FromStr for StateType {
    type Err = UnknownStateType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Task" => Ok(Self::Task),
            "Pass" => Ok(Self::Pass),
            "Choice" => Ok(Self::Choice),
            "Wait" => Ok(Self::Wait),
            "Succeed" => Ok(Self::Succeed),
            "Fail" => Ok(Self::Fail),
            "Parallel" => Ok(Self::Parallel),
            "Map" => Ok(Self::Map),
            _ => Err(UnknownStateType),
        }
    }
}

/// A parsed ASL state.
#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    pub state_type: Option<StateType>,
    pub resource: Option<String>,
    pub next: Option<String>,
    pub end: bool,
    /// Target state names referenced by `Choices[*].Next` and `Default`.
    pub choice_targets: Vec<String>,
    pub default: Option<String>,
    pub has_choices: bool,
    pub has_item_processor: bool,
    /// JSON Pointer to this state (e.g. `/States/MyState`).
    pub pointer: String,
}

/// A parsed ASL state machine (top-level or nested inside Parallel/Map).
#[derive(Debug, Clone)]
pub struct StateMachine {
    pub start_at: Option<String>,
    pub states: HashMap<String, State>,
    /// JSON Pointer to this state machine object.
    pub pointer: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_type_from_str_valid() {
        assert_eq!("Task".parse::<StateType>(), Ok(StateType::Task));
        assert_eq!("Pass".parse::<StateType>(), Ok(StateType::Pass));
        assert_eq!("Choice".parse::<StateType>(), Ok(StateType::Choice));
        assert_eq!("Wait".parse::<StateType>(), Ok(StateType::Wait));
        assert_eq!("Succeed".parse::<StateType>(), Ok(StateType::Succeed));
        assert_eq!("Fail".parse::<StateType>(), Ok(StateType::Fail));
        assert_eq!("Parallel".parse::<StateType>(), Ok(StateType::Parallel));
        assert_eq!("Map".parse::<StateType>(), Ok(StateType::Map));
    }

    #[test]
    fn state_type_from_str_invalid() {
        assert!("Unknown".parse::<StateType>().is_err());
        assert!("task".parse::<StateType>().is_err());
        assert!("".parse::<StateType>().is_err());
    }

    #[test]
    fn terminal_states() {
        assert!(StateType::Succeed.is_terminal());
        assert!(StateType::Fail.is_terminal());
        assert!(!StateType::Task.is_terminal());
        assert!(!StateType::Choice.is_terminal());
    }
}
