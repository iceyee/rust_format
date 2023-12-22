// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

// Trait.

// Struct.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonFormatter;

impl crate::Formatter for JsonFormatter {
    fn format(text: &str) -> String {
        // use serde_json::Value;
        // use std::collections::HashMap;

        let json: serde_json::Value = serde_json::from_str(text).expect("json.rs 489");
        let output: String = serde_json::to_string_pretty(&json).expect("json.rs 609");
        return output;
    }
}

// Function.
