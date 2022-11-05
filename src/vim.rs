// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

static mut INDENT: i64 = 0;
static mut INDENT_TRIGGER_BEFORE: fn(_: usize) = |_: usize| {};
static mut INDENT_TRIGGER_AFTER: fn(_: usize) = |_: usize| {};
static mut IS_START: IsStart = IsStart::No;
static mut IS_START_TRIGGER_BEFORE: fn(_: usize) = |_: usize| {};
static mut IS_START_TRIGGER_AFTER: fn(_: usize) = |_: usize| {};
static mut IS_NEEDED_SPACE: IsNeededSpace = IsNeededSpace::No;
static mut IS_NEEDED_SPACE_TRIGGER_BEFORE: fn(_: usize) = |_: usize| {};
static mut IS_NEEDED_SPACE_TRIGGER_AFTER: fn(_: usize) = |_: usize| {};
static mut DONT_APPEND_FILTER: fn(_: usize) -> bool = |_: usize| false;
static mut BLOCK_BEGIN_FILTER: fn(_: usize) -> bool = |_: usize| false;
static mut BLOCK_END_FILTER: fn(_: usize) -> bool = |_: usize| false;
static mut TEXT: String = String::new();
static mut WORDS: Vec<String> = Vec::new();
static mut TYPES: Vec<WordType> = Vec::new();
static mut BUFFER_WORDS: [String; 8] = [
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
    String::new(),
];
static mut BUFFER_TYPES: [WordType; 8] = [
    WordType::Space,
    WordType::Space,
    WordType::Space,
    WordType::Space,
    WordType::Space,
    WordType::Space,
    WordType::Space,
    WordType::Space,
];

// Enum.

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum SplitState {
    #[default]
    Neutral,
    Command,
    Comment,
    Space,
    String1,
    String2,
    Punctuation,
    Word,
}

// Neutral=>State2,...,
// State2=>Neutral,...,
// Neutral=>State3,...,
// State3=>Neutral,...,

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum WordType {
    Command,
    Comment,
    Punctuation,
    #[default]
    Space,
    String1,
    String2,
    Word,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum IsStart {
    #[default]
    Yes,
    No,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum IsNeededSpace {
    #[default]
    Yes,
    No,
}

// Trait.

// Struct.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VimFormatter;

impl crate::Formatter for VimFormatter {
    fn format(text: &str) -> String {
        unsafe {
            VimFormatter::split_text(("\n".to_string() + text + "\n").as_bytes());
            // VimFormatter::debug_print();
            VimFormatter::rebuild_text();
            return TEXT.clone();
        }
    }
}

impl VimFormatter {
    #[allow(unused_mut)]
    unsafe fn split_text(text: &[u8]) {
        let mut split_state: SplitState = SplitState::Neutral;
        let mut word: Vec<u8> = Vec::new();
        let mut last_type: WordType = WordType::Space;
        let mut x: usize = 0;
        while x < text.len() {
            match split_state {
                SplitState::Neutral => {
                    if 0 < word.len() {
                        WORDS.push(String::from_utf8(word.clone()).unwrap());
                        TYPES.push(last_type.clone());
                        word = Vec::new();
                    }
                    word.push(text[x]);
                    if text[x] == b'"' && (x == 0 || WORDS[WORDS.len() - 1].contains("\n")) {
                        split_state = SplitState::Comment;
                    } else if text[x].is_ascii_whitespace() {
                        split_state = SplitState::Space;
                    } else if text[x] == b'"' {
                        split_state = SplitState::String1;
                    } else if text[x] == b'\'' {
                        split_state = SplitState::String2;
                    } else if text[x].is_ascii_alphanumeric() || text[x] == b'$' || text[x] == b'_'
                    {
                        split_state = SplitState::Word;
                    } else if text[x].is_ascii_punctuation() {
                        split_state = SplitState::Punctuation;
                    } else {
                        panic!("x={:?}, text[x]='{:?}'", x, text[x] as char);
                    }
                }
                SplitState::Command => {
                    last_type = WordType::Command;
                    let mut read_continue: bool = true;
                    while read_continue && x < text.len() {
                        while x < text.len() {
                            if text[x] == b'\n' {
                                break;
                            } else {
                                word.push(text[x]);
                            }
                            x += 1;
                        }
                        read_continue = false;
                        let mut y: usize = 0;
                        while x + y < text.len() {
                            if text[x + y].is_ascii_whitespace() {
                                y += 1;
                                continue;
                            } else if text[x + y] == b'\\' {
                                // 继续读.
                                read_continue = true;
                                while text[x].is_ascii_whitespace() {
                                    word.push(text[x]);
                                    x += 1;
                                }
                                break;
                            } else {
                                // 结束读.
                                read_continue = false;
                                break;
                            }
                        }
                    }
                    if x < text.len() {
                        x -= 1;
                    }
                    split_state = SplitState::Neutral;
                }
                SplitState::Comment => {
                    last_type = WordType::Comment;
                    if text[x] == b'\n' {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Punctuation => {
                    last_type = WordType::Comment;
                    split_state = SplitState::Neutral;
                    x -= 1;
                }
                SplitState::Space => {
                    last_type = WordType::Space;
                    if !text[x].is_ascii_whitespace() {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::String1 => {
                    last_type = WordType::String1;
                    if text[x] == b'\\' {
                        word.push(text[x + 0]);
                        word.push(text[x + 1]);
                        x += 1;
                    } else if text[x] == b'"' {
                        word.push(text[x]);
                        split_state = SplitState::Neutral;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::String2 => {
                    last_type = WordType::String2;
                    if text[x] == b'\'' {
                        word.push(text[x]);
                        split_state = SplitState::Neutral;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Word => {
                    last_type = WordType::Word;
                    if !text[x].is_ascii_alphanumeric()
                        && text[x] != b'!'
                        && text[x] != b'$'
                        && text[x] != b'-'
                        && text[x] != b':'
                        && text[x] != b'_'
                    {
                        let word_2: String = String::from_utf8(word.clone()).unwrap();
                        if [
                            "abbreviate",
                            "autocmd",
                            "echo",
                            "imap",
                            "map",
                            "nmap",
                            "normal",
                            "set",
                            "source",
                            "syntax",
                            "vmap",
                        ]
                        .contains(&word_2.as_str())
                        {
                            word.push(text[x]);
                            split_state = SplitState::Command;
                        } else {
                            split_state = SplitState::Neutral;
                            x -= 1;
                        }
                    } else {
                        word.push(text[x]);
                    }
                }
            }
            x += 1;
        }
        if WORDS.len() != 0 {
            WORDS.push(String::from_utf8(word).unwrap());
            TYPES.push(last_type);
        }
        x = 0;
        let mut new_words: Vec<String> = Vec::new();
        let mut new_types: Vec<WordType> = Vec::new();
        while x < WORDS.len() {
            if TYPES[x] == WordType::Space {
                if WORDS[x].contains("\n") {
                    if WORDS[x].find("\n") == WORDS[x].rfind("\n") {
                        new_words.push("\n".to_string());
                    } else {
                        new_words.push("\n\n".to_string());
                    }
                } else {
                    new_words.push(" ".to_string());
                }
                new_types.push(WordType::Space);
            } else if false
                || x + 2 < WORDS.len()
                    && WORDS[x] == "="
                    && WORDS[x + 1] == "~"
                    && WORDS[x + 2] == "#"
                || x + 2 < WORDS.len()
                    && WORDS[x] == "="
                    && WORDS[x + 1] == "~"
                    && WORDS[x + 2] == "?"
                || x + 2 < WORDS.len()
                    && WORDS[x] == "!"
                    && WORDS[x + 1] == "~"
                    && WORDS[x + 2] == "?"
                || x + 2 < WORDS.len()
                    && WORDS[x] == "!"
                    && WORDS[x + 1] == "~"
                    && WORDS[x + 2] == "?"
            {
                new_words.push(WORDS[x].clone() + WORDS[x + 1].as_str() + WORDS[x + 2].as_str());
                new_types.push(TYPES[x].clone());
                x += 2;
            } else if false
                || x + 1 < WORDS.len() && WORDS[x] == "=" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "!" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == ">" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "<" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "+" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "-" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "*" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "/" && WORDS[x + 1] == "="
                || x + 1 < WORDS.len() && WORDS[x] == "-" && WORDS[x + 1] == ">"
                || x + 1 < WORDS.len() && WORDS[x] == "&" && WORDS[x + 1] == "&"
                || x + 1 < WORDS.len() && WORDS[x] == "|" && WORDS[x + 1] == "|"
                || x + 1 < WORDS.len() && WORDS[x] == "=" && WORDS[x + 1] == "~"
                || x + 1 < WORDS.len() && WORDS[x] == "!" && WORDS[x + 1] == "~"
            {
                new_words.push(WORDS[x].clone() + WORDS[x + 1].as_str());
                new_types.push(TYPES[x].clone());
                x += 1;
            } else {
                new_words.push(WORDS[x].clone());
                new_types.push(TYPES[x].clone());
            }
            x += 1;
        }
        WORDS = new_words;
        TYPES = new_types;
        return;
    }

    #[allow(unused_unsafe)]
    #[allow(unused_variables)]
    unsafe fn rebuild_text() {
        let mut x: usize = 0;
        INDENT_TRIGGER_BEFORE = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                //
            } else if BLOCK_END_FILTER(x) {
                INDENT -= 4;
            } else {
                //
            }
        };
        INDENT_TRIGGER_AFTER = |x: usize| {
            if BLOCK_BEGIN_FILTER(x) {
                INDENT += 4;
            } else if false {
                //
            } else {
                //
            }
        };
        IS_START_TRIGGER_BEFORE = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_START = IsStart::Yes;
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_START = IsStart::No;
            } else {
                //
            }
        };
        IS_START_TRIGGER_AFTER = |x: usize| {
            if ["\n", "\n\n"].contains(&WORDS[x].as_str()) {
                IS_START = IsStart::Yes;
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_START = IsStart::No;
            } else if [" "].contains(&WORDS[x].as_str()) {
                //
            } else {
                IS_START = IsStart::No;
            }
        };
        IS_NEEDED_SPACE_TRIGGER_BEFORE = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if [".", ",", "(", ")", "]", "\\"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else if WORDS[x] == "[" {
                if BUFFER_WORDS[3].ends_with("=")
                    || BUFFER_WORDS[3].ends_with(" ") && BUFFER_WORDS[2].ends_with("=")
                {
                    IS_NEEDED_SPACE = IsNeededSpace::Yes;
                } else {
                    IS_NEEDED_SPACE = IsNeededSpace::No;
                }
            } else {
                //
            }
        };
        IS_NEEDED_SPACE_TRIGGER_AFTER = |x: usize| {
            if [",", ")", "]"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if [".", "(", "[", "\\", "!"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else if [" "].contains(&WORDS[x].as_str()) {
                //
            } else {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            }
        };
        DONT_APPEND_FILTER = |x: usize| -> bool {
            if [" "].contains(&WORDS[x].as_str()) {
                true
            } else if [""].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        BLOCK_BEGIN_FILTER = |x: usize| -> bool {
            if [
                "if",
                "elseif",
                "else",
                "function",
                "function!",
                "for",
                "(",
                "[",
                "{",
            ]
            .contains(&WORDS[x].as_str())
            {
                true
            } else if [""].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        BLOCK_END_FILTER = |x: usize| -> bool {
            if [
                "elseif",
                "else",
                "endif",
                "endfunction",
                "endfor",
                ")",
                "]",
                "}",
            ]
            .contains(&WORDS[x].as_str())
            {
                true
            } else if [""].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        VimFormatter::buffer_roll_new(WORDS[0 + 1].clone(), TYPES[0 + 1].clone());
        VimFormatter::buffer_roll_new(WORDS[0 + 2].clone(), TYPES[0 + 2].clone());
        VimFormatter::buffer_roll_new(WORDS[0 + 3].clone(), TYPES[0 + 3].clone());
        while x < WORDS.len() {
            if x + 4 < WORDS.len() {
                VimFormatter::buffer_roll_new(WORDS[x + 4].clone(), TYPES[x + 4].clone());
            } else {
                VimFormatter::buffer_roll_new(String::new(), WordType::Space);
            }
            INDENT_TRIGGER_BEFORE(x);
            IS_START_TRIGGER_BEFORE(x);
            IS_NEEDED_SPACE_TRIGGER_BEFORE(x);
            if let IsStart::Yes = IS_START {
                VimFormatter::append_indent(INDENT);
            } else if IsNeededSpace::Yes == IS_NEEDED_SPACE && WORDS[x] != " " {
                TEXT.push(' ');
            };
            if !DONT_APPEND_FILTER(x) {
                TEXT.push_str(WORDS[x].as_str());
            }
            INDENT_TRIGGER_AFTER(x);
            IS_START_TRIGGER_AFTER(x);
            IS_NEEDED_SPACE_TRIGGER_AFTER(x);
            VimFormatter::buffer_roll_old(WORDS[x].clone(), TYPES[x].clone());
            x += 1;
        }
        TEXT = TEXT.trim().to_string();
        return;
    }

    unsafe fn append_indent(indent: i64) {
        for _ in 0..(indent as usize) {
            TEXT.push(' ');
        }
        return;
    }

    unsafe fn buffer_roll_old(word: String, tp: WordType) {
        const X: usize = 0;
        BUFFER_WORDS[X + 0] = BUFFER_WORDS[X + 1].clone();
        BUFFER_WORDS[X + 1] = BUFFER_WORDS[X + 2].clone();
        BUFFER_WORDS[X + 2] = BUFFER_WORDS[X + 3].clone();
        BUFFER_WORDS[X + 3] = word;
        BUFFER_TYPES[X + 0] = BUFFER_TYPES[X + 1].clone();
        BUFFER_TYPES[X + 1] = BUFFER_TYPES[X + 2].clone();
        BUFFER_TYPES[X + 2] = BUFFER_TYPES[X + 3].clone();
        BUFFER_TYPES[X + 3] = tp;
        return;
    }

    unsafe fn buffer_roll_new(word: String, tp: WordType) {
        const X: usize = 4;
        BUFFER_WORDS[X + 0] = BUFFER_WORDS[X + 1].clone();
        BUFFER_WORDS[X + 1] = BUFFER_WORDS[X + 2].clone();
        BUFFER_WORDS[X + 2] = BUFFER_WORDS[X + 3].clone();
        BUFFER_WORDS[X + 3] = word;
        BUFFER_TYPES[X + 0] = BUFFER_TYPES[X + 1].clone();
        BUFFER_TYPES[X + 1] = BUFFER_TYPES[X + 2].clone();
        BUFFER_TYPES[X + 2] = BUFFER_TYPES[X + 3].clone();
        BUFFER_TYPES[X + 3] = tp;
        return;
    }

    #[allow(dead_code)]
    unsafe fn debug_print() {
        println!("TYPES:\n{:?}\n", TYPES);
        println!("WORDS:\n{:?}\n", WORDS);
        return;
    }
}

// Function.
// // Use.
//
// // Enum.
//
// // String1: 双引号字符串.
// // String2: 单引号字符串.
//
// #[derive(Debug, Clone, PartialEq)]
// enum Type {
//     Comment,
//     Number,
//     Plain,
//     Punctuation,
//     Reserve,
//     Space,
//     String1,
//     String2,
//     Word,
// }
//
// // Trait.
//
// // Struct.
//
// pub struct VimFormatter;
//
// impl crate::Formatter for VimFormatter {
//     fn format(text: &str) -> String {
//         let (words, ts) = VimFormatter::split(text.as_bytes());
//         // println!("{:?}", words);
//         let text = VimFormatter::rebuild(words.as_slice(), ts.as_slice());
//         // println!("\n\n{}", text);
//         return text;
//     }
// }
//
// impl VimFormatter {
//     fn split(text: &[u8]) -> (Vec<String>, Vec<Type>) {
//         // words是切割的结果, 由Plain分支做push.
//         // ts是对应words的类型, 由各个分支做push.
//         // buffer是缓存上一个和上上一个.
//         let mut words: Vec<String> = Vec::with_capacity(0xFFF);
//         let mut ts: Vec<Type> = Vec::new();
//         let mut status: Type = Type::Plain;
//         let mut x: usize = 0;
//         let mut word: Vec<u8> = Vec::new();
//         let mut buffer: (String, String) = (String::from("\n"), String::from("\n"));
//         while x < text.len() {
//             match status {
//                 Type::Plain => {
//                     if 0 < word.len() {
//                         let w: String = String::from_utf8(word).unwrap();
//                         buffer.0 = buffer.1;
//                         buffer.1 = w.clone();
//                         words.push(w.clone());
//                         word = Vec::new();
//                     }
//                     if buffer.1.starts_with("normal")
//                         || buffer.1.starts_with("abbreviate")
//                         || buffer.1.starts_with("autocmd")
//                         || buffer.1.starts_with("echo")
//                         || buffer.1.starts_with("execute")
//                         || buffer.1.starts_with("set")
//                         || buffer.1.starts_with("source")
//                         || buffer.1.starts_with("syntax")
//                         || buffer.1.starts_with("map")
//                         || buffer.1.starts_with("nmap")
//                         || buffer.1.starts_with("imap")
//                         || buffer.1.starts_with("vmap")
//                     {
//                         status = Type::Reserve;
//                     } else if text[x].is_ascii_alphabetic() || text[x] == b'_' || text[x] == b'$' {
//                         status = Type::Word;
//                     } else if text[x] == b'"' {
//                         if buffer.1.find('\n').is_some() {
//                             // 开头的".
//                             status = Type::Comment;
//                         } else {
//                             status = Type::String1;
//                         }
//                     } else if text[x] == b'\'' {
//                         status = Type::String2;
//                     } else if text[x].is_ascii_punctuation() {
//                         status = Type::Punctuation;
//                     } else if text[x].is_ascii_digit() {
//                         status = Type::Number;
//                     } else if text[x].is_ascii_whitespace() {
//                         status = Type::Space;
//                     } else {
//                         panic!("");
//                     }
//                     word.push(text[x]);
//                 }
//                 Type::Comment => {
//                     if text[x] == b'\n' {
//                         // 遇到换行之后结束.
//                         ts.push(Type::Comment);
//                         status = Type::Plain;
//                         x -= 1;
//                     } else {
//                         word.push(text[x]);
//                     }
//                 }
//                 Type::String1 => {
//                     if text[x] == b'\\' {
//                         // 字符串遇到转义, 一次读两个.
//                         if text.len() < x + 1 {
//                             panic!("");
//                         }
//                         word.push(text[x]);
//                         word.push(text[x + 1]);
//                         x += 1;
//                     } else if text[x] == b'"' {
//                         // 字符串结束.
//                         word.push(text[x]);
//                         ts.push(Type::String1);
//                         status = Type::Plain;
//                     } else {
//                         word.push(text[x]);
//                     }
//                 }
//                 Type::String2 => {
//                     if text[x] == b'\'' {
//                         word.push(text[x]);
//                         ts.push(Type::String2);
//                         status = Type::Plain;
//                     } else {
//                         word.push(text[x]);
//                     }
//                 }
//                 Type::Reserve => {
//                     if text[x] == b'\n' {
//                         // 遇到换行之后结束.
//                         ts.push(Type::Reserve);
//                         status = Type::Plain;
//                         x -= 1;
//                     } else {
//                         word.push(text[x]);
//                     }
//                 }
//                 Type::Space => {
//                     if !text[x].is_ascii_whitespace() {
//                         ts.push(Type::Space);
//                         status = Type::Plain;
//                         x -= 1;
//                     } else {
//                         word.push(text[x]);
//                     }
//                 }
//                 Type::Punctuation => {
//                     ts.push(Type::Punctuation);
//                     status = Type::Plain;
//                     x -= 1;
//                 }
//                 Type::Number => match text[x] {
//                     0..=9 | b'a'..=b'f' | b'A'..=b'F' | b'x' | b'o' => {
//                         word.push(text[x]);
//                     }
//                     _ => {
//                         ts.push(Type::Number);
//                         status = Type::Plain;
//                         x -= 1;
//                     }
//                 },
//                 Type::Word => match text[x] {
//                     b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b':' | b'!' => {
//                         word.push(text[x]);
//                     }
//                     _ => {
//                         ts.push(Type::Word);
//                         status = Type::Plain;
//                         x -= 1;
//                     }
//                 },
//             }
//             x += 1;
//         }
//         if word.len() != 0 {
//             words.push(String::from_utf8(word).unwrap());
//             ts.push(status);
//         }
//         if words.len() == 0 {
//             return (words, ts);
//         }
//         let mut new_words: Vec<String> = Vec::new();
//         let mut new_ts: Vec<Type> = Vec::new();
//         x = if words[0].starts_with("\n") || words[0].starts_with("\t") || words[0].starts_with(" ")
//         {
//             1
//         } else {
//             0
//         };
//         while x < words.len() {
//             if let Type::Space = ts[x] {
//                 if words[x].contains("\n") {
//                     if words[x].find("\n").unwrap() != words[x].rfind("\n").unwrap() {
//                         new_words.push("\n\n".to_string());
//                     } else {
//                         new_words.push("\n".to_string());
//                     }
//                     new_ts.push(Type::Space);
//                 }
//             } else if words[x] == "-" && x + 1 < words.len() && Type::Number == ts[x + 1] {
//                 new_words.push(words[x].clone() + words[x + 1].as_str());
//                 new_ts.push(Type::Number);
//                 x += 1;
//             } else if (words[x] == "&" && x + 1 < words.len() && words[x + 1] == "&")
//                 || (words[x] == "|" && x + 1 < words.len() && words[x + 1] == "|")
//                 || (words[x] == "=" && x + 1 < words.len() && words[x + 1] == "~")
//                 || (words[x] == "!" && x + 1 < words.len() && words[x + 1] == "~")
//                 || (words[x] == "=" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "!" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "+" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "-" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "*" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "/" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "%" && x + 1 < words.len() && words[x + 1] == "=")
//                 || (words[x] == "-" && x + 1 < words.len() && words[x + 1] == ">")
//             {
//                 new_words.push(words[x].clone() + words[x + 1].as_str());
//                 new_ts.push(Type::Punctuation);
//                 x += 1;
//             } else {
//                 new_words.push(words[x].clone());
//                 new_ts.push(ts[x].clone());
//             }
//             x += 1;
//         }
//         return (new_words, new_ts);
//     }
//
//     fn rebuild(words: &[String], ts: &[Type]) -> String {
//         let mut text: String = String::new();
//         // is_start: 是开头, 默认否.
//         // is_space: 是要留空, 默认是.
//         // indent: 缩进, 由'('和')'控制改变.
//         // buffer: 上一个和下一个, 窗口滚动.
//         let mut is_start: bool = false;
//         let mut is_space: bool = false;
//         let mut indent: i64 = 0;
//         let mut buffer: (String, String) = (String::new(), String::new());
//         // 下面的_operation表示三种状态, (一定是, 一定否, 默认).
//         // 默认行为, 如果要求留空, 则接空格.
//         // 最后加上这个词.
//         // closure_start,closure_end, 表示逻辑控制块.
//         let mut is_space_operation: [Vec<&str>; 3] = [vec![], vec![], vec![]];
//         is_space_operation[0] = vec![];
//         is_space_operation[1] = vec![".", ",", "(", ")", ":", "]", "}"];
//         is_space_operation[2] = vec!["{", "["];
//         let mut write_operation: [Vec<&str>; 3] = [vec![], vec![], vec![]];
//         write_operation[0] = vec![];
//         write_operation[1] = vec![];
//         write_operation[2] = vec![];
//         let closure_start: Vec<&str> = vec![
//             "function",
//             "function!",
//             "if",
//             "elseif",
//             "else",
//             "for",
//             "while",
//             "try",
//             "catch",
//         ];
//         let closure_end: Vec<&str> = vec![
//             "endfunction",
//             "elseif",
//             "else",
//             "endif",
//             "endfor",
//             "endwhile",
//             "catch",
//             "endtry",
//         ];
//         // 默认行为, 每一轮循环之后, 状态都归到默认.
//         // is_start = false.
//         // is_space = true.
//         let mut is_space_operation_2: [Vec<&str>; 3] = [vec![], vec![], vec![]];
//         is_space_operation_2[0] = vec![",", ")", ":", "]"];
//         is_space_operation_2[1] = vec!["!", ".", "(", "[", "{"];
//         is_space_operation_2[2] = vec!["}"];
//         let mut x: usize = 0;
//         let append_indent = |t: &mut String, i: i64| {
//             for _ in 0..(i as usize) {
//                 t.push(' ');
//             }
//         };
//         while x < words.len() {
//             if x + 1 < words.len() {
//                 buffer.1 = words[x + 1].clone();
//             } else {
//                 buffer.1 = String::new();
//             }
//             if words[x] == "[" {
//                 if buffer.0.ends_with("=") || buffer.0 == "return" {
//                     is_space = true;
//                 } else {
//                     is_space = false;
//                 }
//             }
//             if words[x] == " " {
//                 // 空格, 无任何操作.
//             } else if let Type::Space = ts[x] {
//                 text.push_str(words[x].as_str());
//                 is_start = true;
//                 is_space = false;
//             } else if let Type::Reserve = ts[x] {
//                 text.push_str(words[x].as_str());
//                 is_start = false;
//                 is_space = true;
//             } else if words[x] == "\\" {
//                 if is_start {
//                     append_indent(&mut text, indent + 4);
//                 }
//                 text.push_str(words[x].as_str());
//                 is_start = false;
//                 is_space = false;
//             } else if false {
//                 //
//             } else if false {
//                 //
//             } else {
//                 if is_start {
//                     if closure_end.contains(&words[x].as_str()) {
//                         append_indent(&mut text, indent - 4);
//                     } else {
//                         append_indent(&mut text, indent);
//                     }
//                     is_space = false;
//                 }
//                 if !is_start && is_space_operation[0].contains(&words[x].as_str()) {
//                     text.push_str(" ");
//                 } else if !is_start && is_space_operation[1].contains(&words[x].as_str()) {
//                     //
//                 } else if is_space {
//                     text.push_str(" ");
//                 }
//                 if write_operation[0].contains(&words[x].as_str()) {
//                     text.push_str(words[x].as_str());
//                 } else if write_operation[1].contains(&words[x].as_str()) {
//                     //
//                 } else {
//                     text.push_str(words[x].as_str());
//                 }
//                 is_start = false;
//                 if is_space_operation_2[0].contains(&words[x].as_str()) {
//                     is_space = true;
//                 } else if is_space_operation_2[1].contains(&words[x].as_str()) {
//                     is_space = false;
//                 } else {
//                     is_space = true;
//                 }
//             }
//             if closure_start.contains(&words[x].as_str()) {
//                 indent += 4;
//             }
//             if closure_end.contains(&words[x].as_str()) {
//                 indent -= 4;
//             }
//             buffer.0 = words[x].clone();
//             x += 1;
//         }
//         return text;
//     }
// }
//
// // Function.
