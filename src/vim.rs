// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

// String1: 双引号字符串.
// String2: 单引号字符串.

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Comment,
    Number,
    Plain,
    Punctuation,
    Reserve,
    Space,
    String1,
    String2,
    Word,
}

// Trait.

// Struct.

pub struct VimFormatter;

impl crate::Formatter for VimFormatter {
    fn format(text: &str) -> String {
        let (words, ts) = VimFormatter::split(text.as_bytes());
        // println!("{:?}", words);
        let text = VimFormatter::rebuild(words.as_slice(), ts.as_slice());
        // println!("\n\n{}", text);
        return text;
    }
}

impl VimFormatter {
    fn split(text: &[u8]) -> (Vec<String>, Vec<Type>) {
        // words是切割的结果, 由Plain分支做push.
        // ts是对应words的类型, 由各个分支做push.
        // buffer是缓存上一个和上上一个.
        let mut words: Vec<String> = Vec::with_capacity(0xFFF);
        let mut ts: Vec<Type> = Vec::new();
        let mut status: Type = Type::Plain;
        let mut x: usize = 0;
        let mut word: Vec<u8> = Vec::new();
        let mut buffer: (String, String) = (String::from("\n"), String::from("\n"));
        while x < text.len() {
            match status {
                Type::Plain => {
                    if 0 < word.len() {
                        let w: String = String::from_utf8(word).unwrap();
                        buffer.0 = buffer.1;
                        buffer.1 = w.clone();
                        words.push(w.clone());
                        word = Vec::new();
                    }
                    if buffer.1.starts_with("normal")
                        || buffer.1.starts_with("abbreviate")
                        || buffer.1.starts_with("autocmd")
                        || buffer.1.starts_with("echo")
                        || buffer.1.starts_with("execute")
                        || buffer.1.starts_with("set")
                        || buffer.1.starts_with("source")
                        || buffer.1.starts_with("syntax")
                        || buffer.1.starts_with("map")
                        || buffer.1.starts_with("nmap")
                        || buffer.1.starts_with("imap")
                        || buffer.1.starts_with("vmap")
                    {
                        status = Type::Reserve;
                    } else if text[x].is_ascii_alphabetic() || text[x] == b'_' || text[x] == b'$' {
                        status = Type::Word;
                    } else if text[x] == b'"' {
                        if buffer.1.find('\n').is_some() {
                            // 开头的".
                            status = Type::Comment;
                        } else {
                            status = Type::String1;
                        }
                    } else if text[x] == b'\'' {
                        status = Type::String2;
                    } else if text[x].is_ascii_punctuation() {
                        status = Type::Punctuation;
                    } else if text[x].is_ascii_digit() {
                        status = Type::Number;
                    } else if text[x].is_ascii_whitespace() {
                        status = Type::Space;
                    } else {
                        panic!("");
                    }
                    word.push(text[x]);
                }
                Type::Comment => {
                    if text[x] == b'\n' {
                        // 遇到换行之后结束.
                        ts.push(Type::Comment);
                        status = Type::Plain;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                Type::String1 => {
                    if text[x] == b'\\' {
                        // 字符串遇到转义, 一次读两个.
                        if text.len() < x + 1 {
                            panic!("");
                        }
                        word.push(text[x]);
                        word.push(text[x + 1]);
                        x += 1;
                    } else if text[x] == b'"' {
                        // 字符串结束.
                        word.push(text[x]);
                        ts.push(Type::String1);
                        status = Type::Plain;
                    } else {
                        word.push(text[x]);
                    }
                }
                Type::String2 => {
                    if text[x] == b'\'' {
                        word.push(text[x]);
                        ts.push(Type::String2);
                        status = Type::Plain;
                    } else {
                        word.push(text[x]);
                    }
                }
                Type::Reserve => {
                    if text[x] == b'\n' {
                        // 遇到换行之后结束.
                        ts.push(Type::Reserve);
                        status = Type::Plain;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                Type::Space => {
                    if !text[x].is_ascii_whitespace() {
                        ts.push(Type::Space);
                        status = Type::Plain;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                Type::Punctuation => {
                    ts.push(Type::Punctuation);
                    status = Type::Plain;
                    x -= 1;
                }
                Type::Number => match text[x] {
                    0..=9 | b'a'..=b'f' | b'A'..=b'F' | b'x' | b'o' => {
                        word.push(text[x]);
                    }
                    _ => {
                        ts.push(Type::Number);
                        status = Type::Plain;
                        x -= 1;
                    }
                },
                Type::Word => match text[x] {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b':' | b'!' => {
                        word.push(text[x]);
                    }
                    _ => {
                        ts.push(Type::Word);
                        status = Type::Plain;
                        x -= 1;
                    }
                },
            }
            x += 1;
        }
        if word.len() != 0 {
            words.push(String::from_utf8(word).unwrap());
            ts.push(status);
        }
        if words.len() == 0 {
            return (words, ts);
        }
        let mut new_words: Vec<String> = Vec::new();
        let mut new_ts: Vec<Type> = Vec::new();
        x = if words[0].starts_with("\n") || words[0].starts_with("\t") || words[0].starts_with(" ")
        {
            1
        } else {
            0
        };
        while x < words.len() {
            if let Type::Space = ts[x] {
                if words[x].contains("\n") {
                    if words[x].find("\n").unwrap() != words[x].rfind("\n").unwrap() {
                        new_words.push("\n\n".to_string());
                    } else {
                        new_words.push("\n".to_string());
                    }
                    new_ts.push(Type::Space);
                }
            } else if words[x] == "-" && x + 1 < words.len() && Type::Number == ts[x + 1] {
                new_words.push(words[x].clone() + words[x + 1].as_str());
                new_ts.push(Type::Number);
                x += 1;
            } else if (words[x] == "&" && x + 1 < words.len() && words[x + 1] == "&")
                || (words[x] == "|" && x + 1 < words.len() && words[x + 1] == "|")
                || (words[x] == "=" && x + 1 < words.len() && words[x + 1] == "~")
                || (words[x] == "!" && x + 1 < words.len() && words[x + 1] == "~")
                || (words[x] == "=" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "!" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "+" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "-" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "*" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "/" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "%" && x + 1 < words.len() && words[x + 1] == "=")
                || (words[x] == "-" && x + 1 < words.len() && words[x + 1] == ">")
            {
                new_words.push(words[x].clone() + words[x + 1].as_str());
                new_ts.push(Type::Punctuation);
                x += 1;
            } else {
                new_words.push(words[x].clone());
                new_ts.push(ts[x].clone());
            }
            x += 1;
        }
        return (new_words, new_ts);
    }

    fn rebuild(words: &[String], ts: &[Type]) -> String {
        let mut text: String = String::new();
        // is_start: 是开头, 默认否.
        // is_space: 是要留空, 默认是.
        // indent: 缩进, 由'('和')'控制改变.
        // buffer: 上一个和下一个, 窗口滚动.
        let mut is_start: bool = false;
        let mut is_space: bool = false;
        let mut indent: i64 = 0;
        let mut buffer: (String, String) = (String::new(), String::new());
        // 下面的_operation表示三种状态, (一定是, 一定否, 默认).
        // 默认行为, 如果要求留空, 则接空格.
        // 最后加上这个词.
        // closure_start,closure_end, 表示逻辑控制块.
        let mut is_space_operation: [Vec<&str>; 3] = [vec![], vec![], vec![]];
        is_space_operation[0] = vec![];
        is_space_operation[1] = vec![".", ",", "(", ")", ":", "]", "}"];
        is_space_operation[2] = vec!["{", "["];
        let mut write_operation: [Vec<&str>; 3] = [vec![], vec![], vec![]];
        write_operation[0] = vec![];
        write_operation[1] = vec![];
        write_operation[2] = vec![];
        let closure_start: Vec<&str> = vec![
            "function",
            "function!",
            "if",
            "elseif",
            "else",
            "for",
            "while",
            "try",
            "catch",
        ];
        let closure_end: Vec<&str> = vec![
            "endfunction",
            "elseif",
            "else",
            "endif",
            "endfor",
            "endwhile",
            "catch",
            "endtry",
        ];
        // 默认行为, 每一轮循环之后, 状态都归到默认.
        // is_start = false.
        // is_space = true.
        let mut is_space_operation_2: [Vec<&str>; 3] = [vec![], vec![], vec![]];
        is_space_operation_2[0] = vec![",", ")", ":", "]"];
        is_space_operation_2[1] = vec!["!", ".", "(", "[", "{"];
        is_space_operation_2[2] = vec!["}"];
        let mut x: usize = 0;
        let append_indent = |t: &mut String, i: i64| {
            for _ in 0..(i as usize) {
                t.push(' ');
            }
        };
        while x < words.len() {
            if x + 1 < words.len() {
                buffer.1 = words[x + 1].clone();
            } else {
                buffer.1 = String::new();
            }
            if words[x] == "[" {
                if buffer.0.ends_with("=") || buffer.0 == "return" {
                    is_space = true;
                } else {
                    is_space = false;
                }
            }
            if words[x] == " " {
                // 空格, 无任何操作.
            } else if let Type::Space = ts[x] {
                text.push_str(words[x].as_str());
                is_start = true;
                is_space = false;
            } else if let Type::Reserve = ts[x] {
                text.push_str(words[x].as_str());
                is_start = false;
                is_space = true;
            } else if words[x] == "\\" {
                if is_start {
                    append_indent(&mut text, indent + 4);
                }
                text.push_str(words[x].as_str());
                is_start = false;
                is_space = false;
            } else if false {
                //
            } else if false {
                //
            } else {
                if is_start {
                    if closure_end.contains(&words[x].as_str()) {
                        append_indent(&mut text, indent - 4);
                    } else {
                        append_indent(&mut text, indent);
                    }
                    is_space = false;
                }
                if !is_start && is_space_operation[0].contains(&words[x].as_str()) {
                    text.push_str(" ");
                } else if !is_start && is_space_operation[1].contains(&words[x].as_str()) {
                    //
                } else if is_space {
                    text.push_str(" ");
                }
                if write_operation[0].contains(&words[x].as_str()) {
                    text.push_str(words[x].as_str());
                } else if write_operation[1].contains(&words[x].as_str()) {
                    //
                } else {
                    text.push_str(words[x].as_str());
                }
                is_start = false;
                if is_space_operation_2[0].contains(&words[x].as_str()) {
                    is_space = true;
                } else if is_space_operation_2[1].contains(&words[x].as_str()) {
                    is_space = false;
                } else {
                    is_space = true;
                }
            }
            if closure_start.contains(&words[x].as_str()) {
                indent += 4;
            }
            if closure_end.contains(&words[x].as_str()) {
                indent -= 4;
            }
            buffer.0 = words[x].clone();
            x += 1;
        }
        return text;
    }
}

// Function.
