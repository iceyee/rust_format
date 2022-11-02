// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

#[derive(Debug, Clone, PartialEq)]
enum Status {
    InLabel,
    Comment,
    Content,
    Space,
    String1,
    String2,
    Word,
}

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Comment,
    Content,
    LabelStartOpen,
    LabelStartClose,
    LabelStartDeclare,
    LabelEnd,
    Slash,
    Space,
    String1,
    String2,
    Word,
}

// Trait.

// Struct.

pub struct HtmlFormatter;

impl crate::Formatter for HtmlFormatter {
    fn format(text: &str) -> String {
        let (words, ts) = HtmlFormatter::split(text.as_bytes());
        // println!("{:?}", words);
        // println!("{:?}", ts);
        let text = HtmlFormatter::rebuild(words.as_slice(), ts.as_slice());
        // println!("\n\n{}", text);
        return text;
    }
}

impl HtmlFormatter {
    fn split(text: &[u8]) -> (Vec<String>, Vec<Type>) {
        let mut words: Vec<String> = Vec::with_capacity(0xFFF);
        let mut ts: Vec<Type> = Vec::with_capacity(0xFFF);
        let mut status: Status = Status::Content;
        let mut x: usize = 0;
        let mut word: Vec<u8> = Vec::new();
        let mut last_type: Type = Type::Content;
        while x < text.len() {
            match status {
                Status::InLabel => {
                    if 0 < word.len() {
                        words.push(String::from_utf8(word).unwrap());
                        ts.push(last_type.clone());
                        word = Vec::new();
                    }
                    if x + 3 < text.len()
                        && text[x + 0] == b'<'
                        && text[x + 1] == b'!'
                        && text[x + 2] == b'-'
                        && text[x + 2] == b'-'
                    {
                        word.push(text[x + 0]);
                        word.push(text[x + 1]);
                        word.push(text[x + 2]);
                        status = Status::Comment;
                        x += 2;
                    } else if text[x] == b'<' {
                        if x + 1 < text.len() && text[x + 1].is_ascii_alphabetic() {
                            words.push("<".to_string());
                            ts.push(Type::LabelStartOpen);
                            status = Status::Word;
                            x += 0;
                        } else if x + 2 < text.len() && text[x + 2].is_ascii_whitespace() {
                            panic!("");
                        } else if text[x + 1] == b'?' || text[x + 1] == b'!' {
                            word.push(text[x + 0]);
                            word.push(text[x + 1]);
                            words.push(String::from_utf8(word).unwrap());
                            ts.push(Type::LabelStartDeclare);
                            word = Vec::new();
                            status = Status::Word;
                            x += 1;
                        } else if text[x + 1] == b'/' {
                            words.push("</".to_string());
                            ts.push(Type::LabelStartClose);
                            status = Status::Word;
                            x += 1;
                        } else {
                            panic!("");
                        }
                    } else if text[x] == b'/' {
                        words.push("/".to_string());
                        ts.push(Type::Slash);
                        status = Status::InLabel;
                        x += 0;
                    } else if text[x].is_ascii_whitespace() {
                        status = Status::Space;
                        x -= 1;
                    } else if text[x] == b'"' {
                        word.push(text[x]);
                        status = Status::String1;
                        x += 0;
                    } else if text[x] == b'\'' {
                        word.push(text[x]);
                        status = Status::String2;
                        x += 0;
                    } else if text[x] == b'>' {
                        words.push(">".to_string());
                        ts.push(Type::LabelEnd);
                        status = Status::Content;
                        x += 0;
                    } else if text[x].is_ascii_alphabetic() {
                        status = Status::Word;
                        x -= 1;
                    } else if text[x] == b'?' {
                        words.push("?".to_string());
                        ts.push(Type::LabelEnd);
                        status = Status::InLabel;
                        x += 0;
                    } else {
                        panic!("{:?}", text[x] as char);
                    }
                }
                Status::Content => {
                    last_type = Type::Content;
                    if text[x] == b'<' {
                        status = Status::InLabel;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                Status::Comment => {
                    last_type = Type::Comment;
                    if x + 2 < text.len()
                        && text[x + 0] == b'-'
                        && text[x + 1] == b'-'
                        && text[x + 2] == b'>'
                    {
                        word.push(text[x]);
                        word.push(text[x + 1]);
                        word.push(text[x + 2]);
                        status = Status::Content;
                        x += 2;
                        words.push(String::from_utf8(word).unwrap());
                        ts.push(last_type.clone());
                        word = Vec::new();
                    } else {
                        word.push(text[x]);
                    }
                }
                Status::Space => {
                    last_type = Type::Space;
                    if text[x].is_ascii_whitespace() {
                        word.push(text[x]);
                    } else {
                        status = Status::InLabel;
                        x -= 1;
                    }
                }
                Status::String1 => {
                    last_type = Type::String1;
                    if text[x] == b'\\' {
                        // 字符串遇到转义, 一次读两个.
                        if text.len() < x + 1 {
                            panic!("");
                        }
                        word.push(text[x + 0]);
                        word.push(text[x + 1]);
                        x += 1;
                    } else if text[x] == b'"' {
                        // 字符串结束.
                        word.push(text[x]);
                        status = Status::InLabel;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                Status::String2 => {
                    last_type = Type::String2;
                    if text[x] == b'\'' {
                        // 字符串结束.
                        word.push(text[x]);
                        status = Status::InLabel;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                Status::Word => {
                    last_type = Type::Word;
                    match text[x] {
                        b'a'..=b'z'
                        | b'A'..=b'Z'
                        | b'0'..=b'9'
                        | b'_'
                        | b'-'
                        | b'.'
                        | b':'
                        | b'=' => {
                            word.push(text[x]);
                        }
                        _ => {
                            status = Status::InLabel;
                            x -= 1;
                        }
                    }
                }
            }
            x += 1;
        }
        if status != Status::Content {
            panic!("{:?}", status);
        }
        if words.len() == 0 {
            panic!("");
        }
        let mut new_words: Vec<String> = Vec::new();
        let mut new_ts: Vec<Type> = Vec::new();
        x = 0;
        while x < words.len() {
            if false {
            } else if ts[x] == Type::Space {
                if words[x].contains("\n") {
                    new_words.push("\n".to_string());
                } else {
                    new_words.push(" ".to_string());
                }
                new_ts.push(ts[x].clone());
            } else if ts[x] == Type::Content {
                let content: String = words[x].trim().to_string();
                if 0 < content.len() {
                    new_words.push(content);
                    new_ts.push(Type::Content);
                }
            } else if ts[x] == Type::Word
                && x + 1 < words.len()
                && (ts[x + 1] == Type::String1 || ts[x + 1] == Type::String2)
            {
                new_words.push(words[x + 0].clone() + words[x + 1].as_str());
                new_ts.push(Type::Word);
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
        let mut indent: i64 = 0;
        let mut buffer: (String, String, String) = (String::new(), String::new(), String::new());
        let mut buffer2: (Type, Type, Type) = (Type::Content, Type::Content, Type::Content);
        // buffer3: 上一个标签和现在这个标签, 是标签头还是标签尾.
        let mut buffer3: (bool, bool) = (false, false);
        let mut is_space: bool = false;
        let append_indent = |t: &mut String, i: i64| {
            for _ in 0..(i as usize) {
                t.push(' ');
            }
        };
        let mut x = 0;
        while x < words.len() {
            if x + 1 < words.len() {
                buffer.2 = words[x + 1].clone();
                buffer2.2 = ts[x + 1].clone();
            } else {
                buffer.2 = String::new();
                buffer2.2 = Type::Content;
            }
            match ts[x] {
                Type::Comment => {
                    text.push_str("\n");
                    append_indent(&mut text, indent);
                    text.push_str(words[x].as_str());
                    buffer3.0 = buffer3.1;
                    buffer3.1 = false;
                }
                Type::Content => {
                    text.push_str(words[x].as_str());
                }
                Type::LabelStartOpen => {
                    text.push_str("\n");
                    append_indent(&mut text, indent);
                    text.push_str(words[x].as_str());
                    is_space = false;
                    indent += 2;
                    buffer3.1 = true;
                }
                Type::LabelStartClose => {
                    indent -= 2;
                    if buffer2.1 == Type::Content || buffer3.0 {
                    } else {
                        text.push_str("\n");
                        append_indent(&mut text, indent);
                    }
                    text.push_str(words[x].as_str());
                    is_space = false;
                    buffer3.1 = false;
                }
                Type::LabelStartDeclare => {
                    text.push_str("\n");
                    append_indent(&mut text, indent);
                    text.push_str(words[x].as_str());
                    is_space = false;
                    buffer3.1 = false;
                }
                Type::LabelEnd => {
                    text.push_str(words[x].as_str());
                    is_space = false;
                    buffer3.0 = buffer3.1;
                }
                Type::Slash => {
                    text.push_str(words[x].as_str());
                    is_space = false;
                    indent -= 2;
                    buffer3.1 = false;
                }
                Type::Space => {
                    if words[x] == "\n" {
                        text.push_str("\n");
                        is_space = false;
                    }
                }
                Type::Word => {
                    if is_space {
                        text.push_str(" ");
                    }
                    text.push_str(words[x].as_str());
                    is_space = true;
                }
                Type::String1 | Type::String2 => {
                    panic!("");
                }
            }
            buffer.0 = buffer.1;
            buffer.1 = words[x].clone();
            buffer2.0 = buffer2.1;
            buffer2.1 = ts[x].clone();
            x += 1;
        }
        return text.trim().to_string();
    }
}
