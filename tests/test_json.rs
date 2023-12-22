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

// Function.

const TEXT: &str = r#"
{"name": "John Doe","age": 30,"city": "New York"



}

"#;

#[test]
fn test_json() {
    use rust_format::Formatter;
    println!(
        "原文:\n{}\n\n==================================================",
        TEXT
    );
    println!(
        "格式化之后:\n{}\n\n==================================================",
        rust_format::json::JsonFormatter::format(TEXT)
    );
    return;
}
