use serde_json::{from_str, Map, Value};

use super::log;

pub struct JsonFormat;

impl log::Format for JsonFormat {
    fn parse_log(&self, s: &str) -> Option<Vec<log::KeyVal>> {
        let value: Map<String, Value> = match from_str(s) {
            Ok(val) => val,
            Err(_) => return None,
        };

        let mut out = Vec::with_capacity(value.len());
        for (key, val) in value {
            let val = match val {
                Value::String(v) => v,
                _ => val.to_string(),
            };
            out.push(log::KeyVal { key, val });
        }

        Some(out)
    }
}
