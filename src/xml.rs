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
static mut IS_LABEL_START_OPEN: bool = false;
static mut IS_LABEL_START_OPEN_TRIGGER_BEFORE: fn(_: usize) = |_: usize| {};
static mut IS_LABEL_START_OPEN_TRIGGER_AFTER: fn(_: usize) = |_: usize| {};

// Enum.

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum SplitState {
    #[default]
    Neutral,
    Comment,
    Content,
    Declare,
    LabelStart,
    LabelEnd,
    Slash,
    Space,
    String1,
    String2,
    Word,
}

// Neutral=>State2,...,
// State2=>Neutral,...,
// Neutral=>State3,...,
// State3=>Neutral,...,

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum WordType {
    #[default]
    Comment,
    Content,
    Declare,
    LabelStartOpen,
    LabelStartClose,
    LabelEnd,
    Slash,
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
pub struct XmlFormatter;

impl crate::Formatter for XmlFormatter {
    fn format(text: &str) -> String {
        unsafe {
            XmlFormatter::split_text(("\n".to_string() + text).as_bytes());
            XmlFormatter::debug_print();
            XmlFormatter::rebuild_text();
            return TEXT.clone();
        }
    }
}

impl XmlFormatter {
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
                    if x + 3 < text.len()
                        && text[x + 0] == b'<'
                        && text[x + 1] == b'!'
                        && text[x + 2] == b'-'
                        && text[x + 3] == b'-'
                    {
                        word.push(text[x + 1]);
                        word.push(text[x + 2]);
                        word.push(text[x + 3]);
                        split_state = SplitState::Comment;
                        x += 3;
                    } else if text[x] == b'<' {
                        if text.len() <= x + 1 {
                            panic!("");
                        } else if text[x + 1] == b'!' {
                            word.push(text[x + 1]);
                            split_state = SplitState::Declare;
                            x += 1;
                        } else if text[x + 1] == b'?' || text[x + 1] == b'/' {
                            word.push(text[x + 1]);
                            split_state = SplitState::LabelStart;
                            x += 1;
                        } else {
                            split_state = SplitState::LabelStart;
                        }
                    } else if text[x] == b'/' || text[x] == b'?' {
                        split_state = SplitState::Slash;
                    } else if text[x].is_ascii_whitespace() {
                        split_state = SplitState::Space;
                    } else if text[x] == b'"' {
                        split_state = SplitState::String1;
                    } else if text[x] == b'\'' {
                        split_state = SplitState::String2;
                    } else if text[x] == b'>' {
                        split_state = SplitState::LabelEnd;
                    } else {
                        split_state = SplitState::Word;
                    }
                }
                SplitState::Comment => {
                    last_type = WordType::Comment;
                    if x + 2 < text.len()
                        && text[x + 0] == b'-'
                        && text[x + 1] == b'-'
                        && text[x + 2] == b'>'
                    {
                        word.push(text[x + 0]);
                        word.push(text[x + 1]);
                        word.push(text[x + 2]);
                        WORDS.push(String::from_utf8(word.clone()).unwrap());
                        TYPES.push(last_type.clone());
                        word = Vec::new();
                        split_state = SplitState::Content;
                        x += 2;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Content => {
                    if 0 < word.len() && last_type != WordType::Content {
                        WORDS.push(String::from_utf8(word.clone()).unwrap());
                        TYPES.push(last_type.clone());
                        word = Vec::new();
                    }
                    last_type = WordType::Content;
                    if text[x] == b'<' {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Declare => {
                    last_type = WordType::Declare;
                    let mut counter: usize = 0;
                    loop {
                        if text[x] == b'>' && counter == 0 {
                            break;
                        } else if text[x] == b'[' {
                            counter += 1;
                        } else if text[x] == b']' {
                            counter -= 1;
                        } else {
                        }
                        word.push(text[x]);
                        x += 1;
                    }
                    split_state = SplitState::Neutral;
                    x -= 1;
                }
                SplitState::LabelStart => {
                    last_type = WordType::LabelStartOpen;
                    if text[x].is_ascii_whitespace()
                        || text[x] == b'<'
                        || text[x] == b'>'
                        || text[x] == b'/'
                        || text[x] == b'?'
                        || text[x] == b'"'
                        || text[x] == b'\''
                    {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::LabelEnd => {
                    last_type = WordType::LabelEnd;
                    split_state = SplitState::Content;
                    x -= 1;
                }
                SplitState::Slash => {
                    last_type = WordType::Slash;
                    split_state = SplitState::Neutral;
                    x -= 1;
                }
                SplitState::Space => {
                    last_type = WordType::Space;
                    if text[x].is_ascii_whitespace() {
                        word.push(text[x]);
                    } else {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    }
                }
                SplitState::String1 => {
                    last_type = WordType::String1;
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
                        split_state = SplitState::Neutral;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::String2 => {
                    last_type = WordType::String2;
                    if text[x] == b'\'' {
                        // 字符串结束.
                        word.push(text[x]);
                        split_state = SplitState::Neutral;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Word => {
                    last_type = WordType::Word;
                    if text[x].is_ascii_whitespace()
                        || text[x] == b'<'
                        || text[x] == b'>'
                        || text[x] == b'/'
                        || text[x] == b'?'
                        || text[x] == b'"'
                        || text[x] == b'\''
                    {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
            }
            x += 1;
        }
        if split_state != SplitState::Content {
            panic!("split_state: {:?}", split_state);
        }
        if WORDS.len() != 0 {
            WORDS.push(String::from_utf8(word).unwrap());
            TYPES.push(last_type);
        }
        x = 0;
        let mut new_words: Vec<String> = Vec::new();
        let mut new_types: Vec<WordType> = Vec::new();
        while x < WORDS.len() {
            if false {
            } else if TYPES[x] == WordType::Space {
                if WORDS[x].contains("\n") {
                    new_words.push("\n".to_string());
                } else {
                    new_words.push(" ".to_string());
                }
                new_types.push(WordType::Space);
            } else if TYPES[x] == WordType::Content {
                let content: String = WORDS[x].trim().to_string();
                if 0 < content.len() {
                    new_words.push(content);
                    new_types.push(WordType::Content);
                }
            } else if TYPES[x] == WordType::LabelStartOpen {
                if WORDS[x].starts_with("</") {
                    new_words.push(WORDS[x].clone());
                    new_types.push(WordType::LabelStartClose);
                } else {
                    new_words.push(WORDS[x].clone());
                    new_types.push(TYPES[x].clone());
                }
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
            } else if [""].contains(&WORDS[x].as_str()) {
                //
            } else if TYPES[x] == WordType::LabelStartClose {
                INDENT -= 4;
            } else if TYPES[x] == WordType::Slash {
                INDENT -= 4;
            } else {
                //
            }
        };
        INDENT_TRIGGER_AFTER = |x: usize| {
            if BLOCK_BEGIN_FILTER(x) {
                INDENT += 4;
            } else if BLOCK_END_FILTER(x) {
                INDENT -= 4;
            } else if TYPES[x] == WordType::LabelStartOpen {
                INDENT += 4;
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
            if [""].contains(&WORDS[x].as_str()) {
                IS_START = IsStart::Yes;
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_START = IsStart::No;
            } else {
                IS_START = IsStart::No;
            }
        };
        IS_NEEDED_SPACE_TRIGGER_BEFORE = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if [" ", ">", "/", "?"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else {
                //
            }
        };
        IS_NEEDED_SPACE_TRIGGER_AFTER = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if [" ", ">", "/", "?"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else {
                if x + 1 < WORDS.len() && WORDS[x + 1] == " " {
                    IS_NEEDED_SPACE = IsNeededSpace::Yes;
                } else {
                    IS_NEEDED_SPACE = IsNeededSpace::No;
                }
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
            if [""].contains(&WORDS[x].as_str()) {
                true
            } else if [""].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        BLOCK_END_FILTER = |x: usize| -> bool {
            if [""].contains(&WORDS[x].as_str()) {
                true
            } else if [""].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        IS_LABEL_START_OPEN_TRIGGER_BEFORE = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_LABEL_START_OPEN = true;
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_LABEL_START_OPEN = false;
            } else {
                //
            }
        };
        IS_LABEL_START_OPEN_TRIGGER_AFTER = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_LABEL_START_OPEN = true;
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_LABEL_START_OPEN = false;
            } else if TYPES[x] != WordType::Content && WORDS[x].starts_with("<") {
                if WORDS[x].starts_with("</") {
                    IS_LABEL_START_OPEN = false;
                } else {
                    IS_LABEL_START_OPEN = true;
                }
            } else if TYPES[x] == WordType::Slash {
                IS_LABEL_START_OPEN = false;
            } else {
                //
            }
        };
        XmlFormatter::buffer_roll_new(WORDS[0 + 1].clone(), TYPES[0 + 1].clone());
        XmlFormatter::buffer_roll_new(WORDS[0 + 2].clone(), TYPES[0 + 2].clone());
        XmlFormatter::buffer_roll_new(WORDS[0 + 3].clone(), TYPES[0 + 3].clone());
        while x < WORDS.len() {
            if x + 4 < WORDS.len() {
                XmlFormatter::buffer_roll_new(WORDS[x + 4].clone(), TYPES[x + 4].clone());
            } else {
                XmlFormatter::buffer_roll_new(String::new(), WordType::Space);
            }
            INDENT_TRIGGER_BEFORE(x);
            IS_START_TRIGGER_BEFORE(x);
            IS_NEEDED_SPACE_TRIGGER_BEFORE(x);
            IS_LABEL_START_OPEN_TRIGGER_BEFORE(x);
            if TYPES[x] == WordType::Comment
                || TYPES[x] == WordType::Declare
                || TYPES[x] == WordType::LabelStartOpen
                || (TYPES[x] == WordType::LabelStartClose && !IS_LABEL_START_OPEN)
            {
                TEXT.push('\n');
                XmlFormatter::append_indent(INDENT);
            }
            if let IsStart::Yes = IS_START {
                XmlFormatter::append_indent(INDENT);
            } else if let IsNeededSpace::Yes = IS_NEEDED_SPACE {
                TEXT.push(' ');
            };
            if !DONT_APPEND_FILTER(x) {
                TEXT.push_str(WORDS[x].as_str());
            }
            INDENT_TRIGGER_AFTER(x);
            IS_START_TRIGGER_AFTER(x);
            IS_NEEDED_SPACE_TRIGGER_AFTER(x);
            IS_LABEL_START_OPEN_TRIGGER_AFTER(x);
            XmlFormatter::buffer_roll_old(WORDS[x].clone(), TYPES[x].clone());
            x += 1;
        }
        TEXT = TEXT.trim().to_string();
        return;
    }

    unsafe fn append_indent(indent: i64) {
        for _ in 0..(indent) {
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
// #[derive(Debug, Clone, PartialEq)]
// enum Status {
//     InLabel,
//     Comment,
//     Content,
//     Space,
//     String1,
//     String2,
//     Word,
// }
//
// #[derive(Debug, Clone, PartialEq)]
// enum Type {
//     Comment,
//     Content,
//     LabelStartOpen,
//     LabelStartClose,
//     LabelStartDeclare,
//     LabelEnd,
//     Slash,
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
// pub struct XmlFormatter;
//
// impl crate::Formatter for XmlFormatter {
//     fn format(text: &str) -> String {
//         let (words, ts) = XmlFormatter::split(text.as_bytes());
//         // println!("{:?}", words);
//         // println!("{:?}", ts);
//         let text = XmlFormatter::rebuild(words.as_slice(), ts.as_slice());
//         // println!("\n\n{}", text);
//         return text;
//     }
// }
//
// impl XmlFormatter {
//     fn rebuild(words: &[String], ts: &[Type]) -> String {
//         let mut text: String = String::new();
//         let mut indent: i64 = 0;
//         let mut buffer: (String, String, String) = (String::new(), String::new(), String::new());
//         let mut buffer2: (Type, Type, Type) = (Type::Content, Type::Content, Type::Content);
//         // buffer3: 上一个标签和现在这个标签, 是标签头还是标签尾.
//         let mut buffer3: (bool, bool) = (false, false);
//         let mut is_space: bool = false;
//         let append_indent = |t: &mut String, i: i64| {
//             for _ in 0..(i as usize) {
//                 t.push(' ');
//             }
//         };
//         let mut x = 0;
//         while x < words.len() {
//             if x + 1 < words.len() {
//                 buffer.2 = words[x + 1].clone();
//                 buffer2.2 = ts[x + 1].clone();
//             } else {
//                 buffer.2 = String::new();
//                 buffer2.2 = Type::Content;
//             }
//             match ts[x] {
//                 Type::Comment => {
//                     text.push_str("\n");
//                     append_indent(&mut text, indent);
//                     text.push_str(words[x].as_str());
//                     buffer3.0 = buffer3.1;
//                     buffer3.1 = false;
//                 }
//                 Type::Content => {
//                     text.push_str(words[x].as_str());
//                 }
//                 Type::LabelStartOpen => {
//                     text.push_str("\n");
//                     append_indent(&mut text, indent);
//                     text.push_str(words[x].as_str());
//                     is_space = false;
//                     indent += 4;
//                     buffer3.1 = true;
//                 }
//                 Type::LabelStartClose => {
//                     indent -= 4;
//                     if buffer2.1 == Type::Content || buffer3.0 {
//                     } else {
//                         text.push_str("\n");
//                         append_indent(&mut text, indent);
//                     }
//                     text.push_str(words[x].as_str());
//                     is_space = false;
//                     buffer3.1 = false;
//                 }
//                 Type::LabelStartDeclare => {
//                     text.push_str("\n");
//                     append_indent(&mut text, indent);
//                     text.push_str(words[x].as_str());
//                     is_space = false;
//                     buffer3.1 = false;
//                 }
//                 Type::LabelEnd => {
//                     text.push_str(words[x].as_str());
//                     is_space = false;
//                     buffer3.0 = buffer3.1;
//                 }
//                 Type::Slash => {
//                     text.push_str(words[x].as_str());
//                     is_space = false;
//                     indent -= 4;
//                     buffer3.1 = false;
//                 }
//                 Type::Space => {
//                     if words[x] == "\n" {
//                         text.push_str("\n");
//                         is_space = false;
//                     }
//                 }
//                 Type::Word => {
//                     if is_space {
//                         text.push_str(" ");
//                     }
//                     text.push_str(words[x].as_str());
//                     is_space = true;
//                 }
//                 Type::String1 | Type::String2 => {
//                     panic!("");
//                 }
//             }
//             buffer.0 = buffer.1;
//             buffer.1 = words[x].clone();
//             buffer2.0 = buffer2.1;
//             buffer2.1 = ts[x].clone();
//             x += 1;
//         }
//         return text.trim().to_string();
//     }
// }
//
// // Function.
