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

pub struct ShellFormatter;

impl crate::Formatter for ShellFormatter {
    fn format(text: &str) -> String {
        let words = ShellFormatter::split(text.as_bytes());
        // println!("{:?}", words);
        let text = ShellFormatter::rebuild(words.as_slice());
        // println!("\n\n{}", text);
        return text;
    }
}

impl ShellFormatter {
    // 切割后的结果有, 空白(' ','\n','\n\n'), 单个标点(包括&&和||), 和其它非空白.
    fn split(text: &[u8]) -> Vec<String> {
        // String1: 双引号字符串.
        // String2: 单引号字符串.
        enum Status {
            Comment,
            Plain,
            String1,
            String2,
        }
        let mut words: Vec<String> = Vec::with_capacity(0xFFF);
        let mut status: Status = Status::Plain;
        let mut x: usize = 0;
        let mut word: Vec<u8> = Vec::new();
        while x < text.len() {
            match status {
                Status::Comment => {
                    if text[x] == b'\n' {
                        // 注释遇到换行之后结束.
                        words.push(String::from_utf8(word).unwrap());
                        word = Vec::new();
                        status = Status::Plain;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                Status::Plain => {
                    if text[x] == b'"' {
                        // 字符串开头.
                        if 0 < word.len() {
                            words.push(String::from_utf8(word).unwrap());
                            word = Vec::new();
                        }
                        word.push(b'"');
                        status = Status::String1;
                    } else if text[x] == b'\'' {
                        // 字符串开头.
                        if 0 < word.len() {
                            words.push(String::from_utf8(word).unwrap());
                            word = Vec::new();
                        }
                        word.push(b'\'');
                        status = Status::String2;
                    } else if text[x] == b'#' {
                        // 注释开头.
                        if 0 < word.len() {
                            words.push(String::from_utf8(word).unwrap());
                            word = Vec::new();
                        }
                        word.push(b'#');
                        status = Status::Comment;
                    } else if text[x].is_ascii_punctuation() {
                        // 标点.
                        if 0 < word.len() {
                            words.push(String::from_utf8(word).unwrap());
                            word = Vec::new();
                        }
                        word.push(text[x]);
                        words.push(String::from_utf8(word).unwrap());
                        word = Vec::new();
                    } else if text[x].is_ascii_whitespace() {
                        // 空白.
                        if 0 < word.len() {
                            words.push(String::from_utf8(word).unwrap());
                            word = Vec::new();
                        }
                        while x < text.len() && text[x].is_ascii_whitespace() {
                            word.push(text[x]);
                            x += 1;
                        }
                        x -= 1;
                        words.push(String::from_utf8(word).unwrap());
                        word = Vec::new();
                    } else {
                        // 其它的.
                        word.push(text[x]);
                    }
                }
                Status::String1 => {
                    if text[x] == b'\\' {
                        // 字符串遇到转义, 一次读两个.
                        if text.len() < x + 1 {
                            panic!("");
                        }
                        word.push(b'\\');
                        word.push(text[x + 1]);
                        x += 1;
                    } else if text[x] == b'"' {
                        // 字符串结束.
                        word.push(b'"');
                        words.push(String::from_utf8(word).unwrap());
                        word = Vec::new();
                        status = Status::Plain;
                    } else {
                        word.push(text[x]);
                    }
                }
                Status::String2 => {
                    if text[x] == b'\'' {
                        word.push(b'\'');
                        words.push(String::from_utf8(word).unwrap());
                        word = Vec::new();
                        status = Status::Plain;
                    } else {
                        word.push(text[x]);
                    }
                }
            }
            x += 1;
        }
        if words.len() == 0 {
            return words;
        }
        let mut new_words: Vec<String> = Vec::new();
        x = if words[0].starts_with("\n") || words[0].starts_with("\t") || words[0].starts_with(" ")
        {
            1
        } else {
            0
        };
        while x < words.len() {
            if words[x].starts_with("\n") || words[x].starts_with("\t") || words[x].starts_with(" ")
            {
                if words[x].contains("\n") {
                    if words[x].find("\n").unwrap() != words[x].rfind("\n").unwrap() {
                        new_words.push("\n\n".to_string());
                    } else {
                        new_words.push("\n".to_string());
                    }
                } else {
                    new_words.push(" ".to_string());
                }
            } else if words[x] == "&" && x + 1 < words.len() && words[x + 1] == "&" {
                new_words.push("&&".to_string());
                x += 1;
            } else if words[x] == "|" && x + 1 < words.len() && words[x + 1] == "|" {
                new_words.push("||".to_string());
                x += 1;
            } else {
                new_words.push(words[x].clone());
            }
            x += 1;
        }
        return new_words;
    }

    fn rebuild(words: &[String]) -> String {
        let mut text: String = String::new();
        // is_start: 是开头, 默认否.
        // is_append: 是接上一行, 默认否.
        // is_space: 是要留空, 默认否.
        // indent: 缩进, 由'('和')'控制改变.
        // pipe_indent: 管道缩进, 由';'和'\n'控制改变.
        // buffer: 预读上上一个, 上一个和下一个, 窗口滚动.
        // status_done: is_start, is_append, is_space, 这三个状态是否已经调整好.
        let mut is_start: bool = false;
        // let mut is_append: bool = false;
        let mut is_append: i64 = 0;
        let mut is_space: bool = false;
        let mut indent: i64 = 0;
        let mut pipe_indent: i64 = 0;
        let mut pipe_indent_stack: Vec<i64> = Vec::new();
        let mut buffer: (String, String, String) = (String::new(), String::new(), String::new());
        // 默认行为, 是开头就接上, 缩进indent+pipe_indent+is_append.
        // 否则如果要求留空, 则接空格.
        // 最后加上这个词.
        // 如果要拒绝默认行为, 可以在下面列表设置.
        // closure_start,closure_end, 表示逻辑控制块.
        let is_start_refuse: Vec<&str> = vec![" "];
        let is_append_refuse: Vec<&str> = vec![" ", "&&", "||"];
        let is_space_refuese: Vec<&str> = vec![" ", ";", "|", ")", "}"];
        let write_refuse: Vec<&str> = vec![" ", "|"];
        let closure_start: Vec<&str> = vec!["{", "(", "[", "do", "then", "else"];
        let closure_end: Vec<&str> = vec!["}", ")", "]", "done", "elif", "else", "fi"];
        // 默认行为, 每一轮循环之后, 状态都归到默认.
        // is_start = false.
        // is_append = 0.
        // is_space = false|true, 看后面.
        // 如果要拒绝默认行为, 可以在下面列表设置.
        let is_start_refuse_2: Vec<&str> = vec![" ", "\n", "\n\n"];
        let is_append_refuse_2: Vec<&str> = vec![" ", "\n"];
        let is_space_refuese_2: Vec<&str> = vec![" ", ";", "|", "(", "}", ")", "}"];
        let mut x: usize = 0;
        let append_indent = |t: &mut String, i: i64| {
            for _ in 0..(i as usize) {
                t.push(' ');
            }
        };
        while x < words.len() {
            if x + 1 < words.len() {
                buffer.2 = words[x + 1].clone();
            } else {
                buffer.2 = String::new();
            }
            if is_start {
                if !is_start_refuse.contains(&words[x].as_str()) {
                    let mut i: i64 = indent + pipe_indent;
                    if !is_append_refuse.contains(&words[x].as_str()) {
                        i += is_append;
                    }
                    if closure_end.contains(&words[x].as_str()) {
                        i -= 4;
                    }
                    append_indent(&mut text, i);
                }
            } else if is_space {
                if !is_space_refuese.contains(&words[x].as_str()) {
                    text.push_str(" ");
                }
            }
            if !write_refuse.contains(&words[x].as_str()) {
                text.push_str(words[x].as_str());
            }
            //
            //
            if words[x] == "\n" {
                is_start = true;
                if buffer.1 == "\\" {
                    is_append = 4;
                } else {
                    is_append = 0;
                    pipe_indent = 0;
                }
            } else if words[x] == "\n\n" {
                is_start = true;
                is_append = 0;
                pipe_indent = 0;
            } else if words[x] == ";" {
                is_space = true;
                pipe_indent = 0;
            } else if words[x] == "|" {
                text.push_str(" |");
                is_space = true;
                pipe_indent += 4;
            } else if words[x] == "(" {
                is_space = false;
            } else if words[x] == "{" {
                is_space = false;
            } else if words[x] == ")" {
                is_space = true;
            } else if words[x] == "}" {
                is_space = true;
            }
            //
            //
            if !is_start_refuse_2.contains(&words[x].as_str()) {
                is_start = false;
            }
            if !is_append_refuse_2.contains(&words[x].as_str()) {
                is_append = 0;
            }
            if !is_space_refuese_2.contains(&words[x].as_str()) {
                if buffer.2 == " " {
                    is_space = true;
                } else {
                    is_space = false;
                }
            }
            if closure_start.contains(&words[x].as_str()) {
                indent += 4;
                indent += pipe_indent;
                pipe_indent_stack.push(pipe_indent);
                pipe_indent = 0;
            }
            if closure_end.contains(&words[x].as_str()) {
                pipe_indent = pipe_indent_stack.pop().unwrap();
                indent -= pipe_indent;
                indent -= 4;
            }
            if words[x] != " " {
                buffer.0 = buffer.1.clone();
                buffer.1 = words[x].clone();
            }
            x += 1;
        }
        return text;
    }

    // fn rebuild(words: &[String]) -> String {
    //     let mut text: String = String::new();
    //     // is_start: 是开头, 默认否.
    //     // is_append: 是接上一行, 默认否.
    //     // is_space: 是要留空, 默认否.
    //     // indent: 缩进, 由'('和')'控制改变.
    //     // pipe_indent: 管道缩进, 由';'和'\n'控制改变.
    //     // buffer: 预读上上一个, 上一个和下一个, 窗口滚动.
    //     // status_done: is_start, is_append, is_space, 这三个状态是否已经调整好.
    //     let mut is_start: bool = false;
    //     // let mut is_append: bool = false;
    //     let mut is_append: i64 = 0;
    //     let mut is_space: bool = false;
    //     let mut indent: i64 = 0;
    //     let mut pipe_indent: i64 = 0;
    //     let mut pipe_indent_stack: Vec<i64> = Vec::new();
    //     let mut buffer: (String, String, String) = (String::new(), String::new(), String::new());
    //     let mut x: usize = 0;
    //     let append_indent = |t: &mut String, i: i64| {
    //         for _ in 0..(i as usize) {
    //             t.push(' ');
    //         }
    //     };
    //     let mut status_done;
    //     while x < words.len() {
    //         if x + 1 < words.len() {
    //             buffer.2 = words[x + 1].clone();
    //         } else {
    //             buffer.2 = String::new();
    //         }
    //         if words[x] == " " {
    //             // 空格, 什么也不做.
    //             status_done = true;
    //         } else if words[x] == "\n" || words[x] == "\n\n" {
    //             if buffer.1 != "\\" {
    //                 pipe_indent = 0;
    //             }
    //             is_start = true;
    //             if buffer.1 == "\\" && words[x] != "\n\n" {
    //                 is_append = 4;
    //             } else {
    //                 is_append = 0;
    //             }
    //             is_space = false;
    //             status_done = true;
    //             text.push_str(words[x].as_str());
    //         } else if words[x] == "&&" || words[x] == "||" {
    //             if is_start {
    //                 append_indent(&mut text, indent + pipe_indent);
    //                 text.push_str(words[x].as_str());
    //             } else {
    //                 text.push_str(" ");
    //                 text.push_str(words[x].as_str());
    //             }
    //             is_start = false;
    //             is_append = 0;
    //             is_space = true;
    //             status_done = true;
    //         } else if words[x] == "(" || words[x] == "{" {
    //             if is_start {
    //                 append_indent(&mut text, indent + pipe_indent + is_append);
    //             } else if is_space {
    //                 text.push_str(" ");
    //             }
    //             text.push_str(words[x].as_str());
    //             indent += 4;
    //             indent += pipe_indent;
    //             pipe_indent_stack.push(pipe_indent);
    //             pipe_indent = 0;
    //             status_done = false;
    //         } else if words[x] == ")" || words[x] == "}" {
    //             if is_start {
    //                 append_indent(&mut text, indent + pipe_indent + is_append - 4);
    //             }
    //             text.push_str(words[x].as_str());
    //             pipe_indent = pipe_indent_stack.pop().unwrap();
    //             indent -= 4;
    //             indent -= pipe_indent;
    //             is_start = false;
    //             is_append = 0;
    //             is_space = true;
    //             status_done = true;
    //         } else if words[x] == "|" {
    //             if is_start {
    //                 append_indent(&mut text, indent + pipe_indent);
    //             } else {
    //                 text.push_str(" ");
    //             }
    //             text.push_str(words[x].as_str());
    //             pipe_indent += 4;
    //             is_start = false;
    //             is_append = 0;
    //             is_space = true;
    //             status_done = true;
    //         } else if words[x] == ";" {
    //             if is_start {
    //                 append_indent(&mut text, indent + pipe_indent);
    //             }
    //             text.push_str(words[x].as_str());
    //             pipe_indent = 0;
    //             is_start = false;
    //             is_append = 0;
    //             is_space = true;
    //             status_done = true;
    //         } else {
    //             if is_start {
    //                 append_indent(&mut text, indent + pipe_indent + is_append);
    //             } else if is_space {
    //                 text.push_str(" ");
    //             }
    //             text.push_str(words[x].as_str());
    //             is_start = false;
    //             is_append = 0;
    //             if buffer.2 == " " {
    //                 is_space = true;
    //             } else {
    //                 is_space = false;
    //             }
    //             status_done = true;
    //             if words[x] == "do" {
    //                 indent += 4;
    //             } else if words[x] == "done" {
    //                 indent -= 4;
    //             }
    //         }
    //         if !status_done {
    //             is_start = false;
    //             is_append = 0;
    //             is_space = false;
    //         }
    //         if words[x] != " " {
    //             buffer.0 = buffer.1.clone();
    //             buffer.1 = words[x].clone();
    //         }
    //         x += 1;
    //     }
    //     return text;
    // }
}

// Function.
