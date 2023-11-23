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

fn main() {
    use rust_format::Formatter;
    use std::env::Args;
    let mut args: Args = std::env::args();
    args.next();
    let first: Option<String> = args.next();
    let second: Option<String> = args.next();
    if second.is_none() {
        panic!("参数错误.");
    }
    let tp: String = first.unwrap();
    let file: String = second.unwrap();
    let text: String = String::from_utf8(std::fs::read(&file).unwrap()).unwrap();
    let text: String = match tp.as_str() {
        "--html" => rust_format::html::HtmlFormatter::format(&text),
        "--java" => rust_format::java::JavaFormatter::format(&text),
        "--javascript" => rust_format::js::JavascriptFormatter::format(&text),
        "--shell" => rust_format::shell::ShellFormatter::format(&text),
        "--vim" => rust_format::vim::VimFormatter::format(&text),
        "--xml" => rust_format::xml::XmlFormatter::format(&text),
        _ => {
            panic!("不支持的格式'{}'", tp);
        }
    };
    std::fs::write(&file, text.as_bytes()).unwrap();
    return;
}
