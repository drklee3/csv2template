use heck::*;
use std::collections::HashMap;
use tera::{Error, Result, Value};

pub fn convert_case(val: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let case_type = args
        .get("type")
        .ok_or_else(|| Error::msg("Missing type"))?
        .as_str()
        .ok_or_else(|| Error::msg("Type isn't a string"))?
        .to_lowercase();

    let val_str = val
        .as_str()
        .ok_or_else(|| Error::msg("Value isn't a string"))?;

    let converted_str = match case_type.as_str() {
        "camel" | "camelcase" => val_str.to_camel_case(),
        "snake" | "snake_case" => val_str.to_snake_case(),
        "kebab" | "kebab-case" => val_str.to_kebab_case(),
        "shouty snake" | "shouty_snake" | "shouty_snake_case" => val_str.to_shouty_snake_case(),
        "mixed" | "mixedcase" => val_str.to_mixed_case(),
        "title" | "title case" => val_str.to_title_case(),
        _ => return Err(Error::msg("Invalid case type")),
    };

    Ok(Value::String(converted_str))
}
