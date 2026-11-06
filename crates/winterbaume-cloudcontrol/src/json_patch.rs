//! RFC 6902 JSON Patch over `serde_json::Value`.
//!
//! Cloud Control applies an `UpdateResource` patch document to the current
//! read model, so patches may address nested properties such as
//! `/ResponseHeadersPolicyConfig/Comment` or array elements such as
//! `/Tags/0/Value`.

use serde_json::Value;

/// Apply every operation of `document` to `target`, stopping at the first
/// failure. `target` may be partly modified when an error is returned, so
/// callers patch a copy.
pub(crate) fn apply(target: &mut Value, document: &str) -> Result<(), String> {
    let operations: Vec<Value> =
        serde_json::from_str(document).map_err(|e| format!("Invalid patch document: {e}"))?;
    for operation in &operations {
        apply_operation(target, operation)?;
    }
    Ok(())
}

fn apply_operation(target: &mut Value, operation: &Value) -> Result<(), String> {
    let op = operation
        .get("op")
        .and_then(Value::as_str)
        .ok_or("Patch operation is missing \"op\"")?;
    let path = operation
        .get("path")
        .and_then(Value::as_str)
        .ok_or("Patch operation is missing \"path\"")?;
    let path = parse_pointer(path)?;
    let value = || {
        operation
            .get("value")
            .cloned()
            .ok_or_else(|| format!("Patch operation \"{op}\" is missing \"value\""))
    };
    let from = || {
        operation
            .get("from")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("Patch operation \"{op}\" is missing \"from\""))
            .and_then(parse_pointer)
    };
    match op {
        "add" => add(target, &path, value()?),
        "remove" => remove(target, &path).map(drop),
        "replace" => {
            let slot = get_mut(target, &path)?;
            *slot = value()?;
            Ok(())
        }
        "move" => {
            let from = from()?;
            if path.len() > from.len() && path[..from.len()] == from[..] {
                return Err("Cannot move a value into one of its children".to_string());
            }
            let moved = remove(target, &from)?;
            add(target, &path, moved)
        }
        "copy" => {
            let copied = get_mut(target, &from()?)?.clone();
            add(target, &path, copied)
        }
        "test" => {
            if *get_mut(target, &path)? == value()? {
                Ok(())
            } else {
                Err(format!("Patch test failed at {}", format_pointer(&path)))
            }
        }
        other => Err(format!("Unsupported patch operation \"{other}\"")),
    }
}

fn parse_pointer(pointer: &str) -> Result<Vec<String>, String> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    let rest = pointer
        .strip_prefix('/')
        .ok_or_else(|| format!("Invalid JSON pointer \"{pointer}\""))?;
    Ok(rest
        .split('/')
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect())
}

fn format_pointer(tokens: &[String]) -> String {
    tokens
        .iter()
        .map(|t| format!("/{}", t.replace('~', "~0").replace('/', "~1")))
        .collect()
}

fn array_index(token: &str, len: usize, allow_end: bool) -> Result<usize, String> {
    if token == "-" && allow_end {
        return Ok(len);
    }
    let index: usize = token
        .parse()
        .map_err(|_| format!("Invalid array index \"{token}\""))?;
    if index > len || (index == len && !allow_end) {
        return Err(format!("Array index {index} is out of bounds"));
    }
    Ok(index)
}

fn get_mut<'a>(target: &'a mut Value, path: &[String]) -> Result<&'a mut Value, String> {
    let mut current = target;
    for token in path {
        current = match current {
            Value::Object(map) => map.get_mut(token),
            Value::Array(items) => {
                let index = array_index(token, items.len(), false)?;
                items.get_mut(index)
            }
            _ => None,
        }
        .ok_or_else(|| format!("Path {} does not exist", format_pointer(path)))?;
    }
    Ok(current)
}

fn add(target: &mut Value, path: &[String], value: Value) -> Result<(), String> {
    let Some((last, parent)) = path.split_last() else {
        *target = value;
        return Ok(());
    };
    match get_mut(target, parent)? {
        Value::Object(map) => {
            map.insert(last.clone(), value);
            Ok(())
        }
        Value::Array(items) => {
            let index = array_index(last, items.len(), true)?;
            items.insert(index, value);
            Ok(())
        }
        _ => Err(format!(
            "Parent of {} is not an object or array",
            format_pointer(path)
        )),
    }
}

fn remove(target: &mut Value, path: &[String]) -> Result<Value, String> {
    let (last, parent) = path
        .split_last()
        .ok_or("Cannot remove the whole resource model")?;
    let missing = || format!("Path {} does not exist", format_pointer(path));
    match get_mut(target, parent)? {
        Value::Object(map) => map.remove(last).ok_or_else(missing),
        Value::Array(items) => {
            let index = array_index(last, items.len(), false)?;
            Ok(items.remove(index))
        }
        _ => Err(missing()),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::apply;

    #[test]
    fn nested_and_array_operations() {
        let mut model = json!({"A": {"B": 1}, "Tags": [{"Key": "k", "Value": "v"}]});
        apply(
            &mut model,
            r#"[
                {"op": "replace", "path": "/A/B", "value": 2},
                {"op": "add", "path": "/Tags/-", "value": {"Key": "x", "Value": "y"}},
                {"op": "replace", "path": "/Tags/0/Value", "value": "w"},
                {"op": "copy", "from": "/A", "path": "/C"},
                {"op": "move", "from": "/C/B", "path": "/D"},
                {"op": "test", "path": "/D", "value": 2},
                {"op": "remove", "path": "/A"}
            ]"#,
        )
        .unwrap();
        assert_eq!(
            model,
            json!({
                "C": {}, "D": 2,
                "Tags": [{"Key": "k", "Value": "w"}, {"Key": "x", "Value": "y"}]
            })
        );
    }

    #[test]
    fn rejects_missing_paths() {
        let mut model = json!({"A": {}});
        assert!(
            apply(
                &mut model,
                r#"[{"op": "replace", "path": "/A/B", "value": 1}]"#
            )
            .is_err()
        );
        assert!(apply(&mut model, r#"[{"op": "remove", "path": "/B"}]"#).is_err());
        assert!(apply(&mut model, r#"[{"op": "add", "path": "/X/Y", "value": 1}]"#).is_err());
        assert!(apply(&mut model, r#"[{"op": "test", "path": "/A", "value": 1}]"#).is_err());
    }
}
