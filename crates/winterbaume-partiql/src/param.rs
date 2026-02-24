use serde_json::Value;

use crate::error::PartiqlError;

/// Positional parameter binder for `?` placeholders in PartiQL statements.
pub struct ParamBinder {
    params: Vec<Value>,
    next_index: usize,
}

impl ParamBinder {
    pub fn new(params: Vec<Value>) -> Self {
        Self {
            params,
            next_index: 0,
        }
    }

    /// Consume the next parameter. Returns error if exhausted.
    pub fn next(&mut self) -> Result<Value, PartiqlError> {
        if self.next_index >= self.params.len() {
            return Err(PartiqlError::ParameterCount {
                expected: self.next_index + 1,
                got: self.params.len(),
            });
        }
        let val = self.params[self.next_index].clone();
        self.next_index += 1;
        Ok(val)
    }

    /// Verify all parameters were consumed.
    pub fn verify_exhausted(&self) -> Result<(), PartiqlError> {
        if self.next_index != self.params.len() {
            return Err(PartiqlError::ParameterCount {
                expected: self.next_index,
                got: self.params.len(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_binder_consumes_in_order() {
        let mut binder = ParamBinder::new(vec![json!({"S": "a"}), json!({"N": "42"})]);
        assert_eq!(binder.next().unwrap(), json!({"S": "a"}));
        assert_eq!(binder.next().unwrap(), json!({"N": "42"}));
        binder.verify_exhausted().unwrap();
    }

    #[test]
    fn test_binder_exhausted_error() {
        let mut binder = ParamBinder::new(vec![json!({"S": "a"})]);
        binder.next().unwrap();
        assert!(binder.next().is_err());
    }

    #[test]
    fn test_binder_unconsumed_error() {
        let binder = ParamBinder::new(vec![json!({"S": "a"}), json!({"N": "42"})]);
        assert!(binder.verify_exhausted().is_err());
    }

    #[test]
    fn test_binder_empty() {
        let binder = ParamBinder::new(vec![]);
        binder.verify_exhausted().unwrap();
    }
}
