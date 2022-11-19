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
static mut APPEND: i64 = 0;
static mut APPEND_TRIGGER_BEFORE: fn(_: usize) = |_: usize| {};
static mut APPEND_TRIGGER_AFTER: fn(_: usize) = |_: usize| {};

// Enum.

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
enum SplitState {
    #[default]
    Neutral,
    Anotation,
    Comment1,
    Comment2,
    Import,
    Punctuation,
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
    Anotation,
    Comment,
    Import,
    Punctuation,
    #[default]
    Space,
    StringA,
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
pub struct JavaFormatter;

impl crate::Formatter for JavaFormatter {
    fn format(text: &str) -> String {
        unsafe {
            JavaFormatter::split_text(("\n".to_string() + text + "\n").as_bytes());
            JavaFormatter::rebuild_text();
            JavaFormatter::debug_print();
            return TEXT.clone();
        }
    }
}

impl JavaFormatter {
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
                    if false {
                    } else if text[x] == b'@' {
                        split_state = SplitState::Anotation;
                    } else if text[x] == b'/' && x + 1 < text.len() && text[x + 1] == b'/' {
                        split_state = SplitState::Comment1;
                    } else if text[x] == b'/' && x + 1 < text.len() && text[x + 1] == b'*' {
                        word.push(text[x + 1]);
                        split_state = SplitState::Comment2;
                        x += 1;
                    } else if x + 5 < text.len()
                        && text[x + 0] == b'i'
                        && text[x + 1] == b'm'
                        && text[x + 2] == b'p'
                        && text[x + 3] == b'o'
                        && text[x + 4] == b'r'
                        && text[x + 5] == b't'
                    {
                        split_state = SplitState::Import;
                    } else if x + 6 < text.len()
                        && text[x + 0] == b'p'
                        && text[x + 1] == b'a'
                        && text[x + 2] == b'c'
                        && text[x + 3] == b'k'
                        && text[x + 4] == b'a'
                        && text[x + 5] == b'g'
                        && text[x + 6] == b'e'
                    {
                        split_state = SplitState::Import;
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
                        panic!("");
                    }
                }
                SplitState::Anotation => {
                    last_type = WordType::Anotation;
                    let mut deep: i64 = 0;
                    let mut y: usize = 0;
                    let mut is_string1: bool = false;
                    let mut is_string2: bool = false;
                    loop {
                        if text[x + y].is_ascii_whitespace()
                            && deep == 0
                            && !is_string1
                            && !is_string2
                        {
                            break;
                        } else if text[x + y] == b'\\' && (is_string1 || is_string2) {
                            word.push(text[x + y + 0]);
                            word.push(text[x + y + 1]);
                            y += 1;
                        } else if text[x + y] == b'"' {
                            word.push(text[x + y]);
                            is_string1 = !is_string1;
                        } else if text[x + y] == b'\'' {
                            word.push(text[x + y]);
                            is_string2 = !is_string1;
                        } else if is_string1 || is_string2 {
                            word.push(text[x + y]);
                        } else if text[x + y] == b'(' || text[x + y] == b'{' {
                            word.push(text[x + y]);
                            deep += 1;
                        } else if text[x + y] == b')' || text[x + y] == b'}' {
                            word.push(text[x + y]);
                            deep -= 1;
                        } else {
                            word.push(text[x + y]);
                        }
                        y += 1;
                    }
                    split_state = SplitState::Neutral;
                    x += y;
                    x -= 1;
                }
                SplitState::Comment1 => {
                    last_type = WordType::Comment;
                    if text[x] == b'\n' {
                        split_state = SplitState::Neutral;
                        x -= 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Comment2 => {
                    last_type = WordType::Comment;
                    if x + 1 < text.len() && text[x + 0] == b'*' && text[x + 1] == b'/' {
                        word.push(text[x + 0]);
                        word.push(text[x + 1]);
                        split_state = SplitState::Neutral;
                        x += 1;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Import => {
                    last_type = WordType::Import;
                    if text[x] == b';' {
                        split_state = SplitState::Neutral;
                    }
                    word.push(text[x]);
                }
                SplitState::Punctuation => {
                    last_type = WordType::Punctuation;
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
                    last_type = WordType::StringA;
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
                    last_type = WordType::StringA;
                    if text[x] == b'\\' {
                        word.push(text[x + 0]);
                        word.push(text[x + 1]);
                        x += 1;
                    } else if text[x] == b'\'' {
                        word.push(text[x]);
                        split_state = SplitState::Neutral;
                        x += 0;
                    } else {
                        word.push(text[x]);
                    }
                }
                SplitState::Word => {
                    last_type = WordType::Word;
                    if !text[x].is_ascii_alphanumeric() && text[x] != b'$' && text[x] != b'_' {
                        split_state = SplitState::Neutral;
                        x -= 1;
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
                    new_types.push(WordType::Space);
                } else {
                    // new_words.push(" ".to_string());
                    // new_types.push(WordType::Space);
                }
            } else if x + 2 < WORDS.len()
                && (WORDS[x + 0] == ">" && WORDS[x + 1] == ">" && WORDS[x + 2] == "="
                    || WORDS[x + 0] == "<" && WORDS[x + 1] == "<" && WORDS[x + 2] == "="
                    || WORDS[x + 0] == "." && WORDS[x + 1] == "." && WORDS[x + 2] == ".")
            {
                new_words.push(WORDS[x + 0].clone() + &WORDS[x + 1] + &WORDS[x + 2]);
                new_types.push(WordType::Punctuation);
                x += 2;
            } else if x + 1 < WORDS.len()
                && (WORDS[x + 0] == "&" && WORDS[x + 1] == "&"
                    || WORDS[x + 0] == "|" && WORDS[x + 1] == "|"
                    || WORDS[x + 0] == "+" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "-" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "*" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "/" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "=" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "!" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "^" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "|" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == ">" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "<" && WORDS[x + 1] == "="
                    || WORDS[x + 0] == "+" && WORDS[x + 1] == "+"
                    || WORDS[x + 0] == "-" && WORDS[x + 1] == "-"
                    || WORDS[x + 0] == "-" && WORDS[x + 1] == ">"
                    || WORDS[x + 0] == "<" && WORDS[x + 1] == ">"
                    || WORDS[x + 0] == "<" && WORDS[x + 1] == "<"
                    || WORDS[x + 0] == ">" && WORDS[x + 1] == ">")
            {
                new_words.push(WORDS[x + 0].clone() + &WORDS[x + 1]);
                new_types.push(WordType::Punctuation);
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
            } else if [""].contains(&WORDS[x].as_str()) {
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
            } else if BLOCK_END_FILTER(x) {
                // INDENT -= 4;
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
            } else {
                IS_START = IsStart::No;
            }
        };
        IS_NEEDED_SPACE_TRIGGER_BEFORE = |x: usize| {
            if ["{"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if [".", ")", "[", "]", "}", ";", "...", "++", "--", "<>", ","]
                .contains(&WORDS[x].as_str())
            {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else if WORDS[x] == "(" && BUFFER_TYPES[3] == WordType::Word {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else {
                //
            }
        };
        IS_NEEDED_SPACE_TRIGGER_AFTER = |x: usize| {
            if [")", "]", "}", ";", "...", "++", "--", "<>", ","].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if ["\n", "\n\n", ".", "(", "[", "{", "!"].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            }
        };
        APPEND_TRIGGER_BEFORE = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                //
            } else if [""].contains(&WORDS[x].as_str()) {
                //
            } else {
                //
            }
        };
        APPEND_TRIGGER_AFTER = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                //
            } else if ["\n", "\n\n"].contains(&WORDS[x].as_str()) {
                //
            } else if false
                || WORDS[x] == ";"
                || WORDS[x] == ":"
                || WORDS[x] == "{"
                || WORDS[x] == "}"
                || TYPES[x] == WordType::Anotation
                || TYPES[x] == WordType::Comment
                || TYPES[x] == WordType::Import
            {
                APPEND = 0;
            } else {
                APPEND = 4;
            }
        };
        DONT_APPEND_FILTER = |x: usize| -> bool {
            if [""].contains(&WORDS[x].as_str()) {
                true
            } else if [""].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        BLOCK_BEGIN_FILTER = |x: usize| -> bool {
            if ["(", "{"].contains(&WORDS[x].as_str()) {
                true
            } else if [")", "}"].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        BLOCK_END_FILTER = |x: usize| -> bool {
            if [")", "}"].contains(&WORDS[x].as_str()) {
                true
            } else if ["(", "{"].contains(&WORDS[x].as_str()) {
                false
            } else {
                false
            }
        };
        JavaFormatter::buffer_roll_new(WORDS[0 + 1].clone(), TYPES[0 + 1].clone());
        JavaFormatter::buffer_roll_new(WORDS[0 + 2].clone(), TYPES[0 + 2].clone());
        JavaFormatter::buffer_roll_new(WORDS[0 + 3].clone(), TYPES[0 + 3].clone());
        while x < WORDS.len() {
            if x + 4 < WORDS.len() {
                JavaFormatter::buffer_roll_new(WORDS[x + 4].clone(), TYPES[x + 4].clone());
            } else {
                JavaFormatter::buffer_roll_new(String::new(), WordType::Space);
            }
            INDENT_TRIGGER_BEFORE(x);
            IS_START_TRIGGER_BEFORE(x);
            IS_NEEDED_SPACE_TRIGGER_BEFORE(x);
            APPEND_TRIGGER_BEFORE(x);
            if IsStart::Yes == IS_START {
                JavaFormatter::append_indent(INDENT + APPEND);
            } else if IsNeededSpace::Yes == IS_NEEDED_SPACE {
                TEXT.push(' ');
            };
            if !DONT_APPEND_FILTER(x) {
                TEXT.push_str(WORDS[x].as_str());
            }
            INDENT_TRIGGER_AFTER(x);
            IS_START_TRIGGER_AFTER(x);
            IS_NEEDED_SPACE_TRIGGER_AFTER(x);
            APPEND_TRIGGER_AFTER(x);
            JavaFormatter::buffer_roll_old(WORDS[x].clone(), TYPES[x].clone());
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
