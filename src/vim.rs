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
            VimFormatter::rebuild_text();
            // VimFormatter::debug_print();
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
                                x += 1;
                            }
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
                || x + 2 < WORDS.len()
                    && WORDS[x] == "="
                    && WORDS[x + 1] == "="
                    && WORDS[x + 2] == "#"
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
            } else if ["\n", "\n\n", ".", ",", "(", ")", "]", "\\"].contains(&WORDS[x].as_str()) {
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
        APPEND_TRIGGER_BEFORE = |x: usize| {
            if ["\\"].contains(&WORDS[x].as_str()) {
                APPEND = 4;
            } else if [""].contains(&WORDS[x].as_str()) {
                //
            } else {
                APPEND = 0;
            }
        };
        APPEND_TRIGGER_AFTER = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                //
            } else if [""].contains(&WORDS[x].as_str()) {
                //
            } else {
                //
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
                "while",
                "try",
                "catch",
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
                "endwhile",
                "catch",
                "endtry",
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
        if x + 3 < WORDS.len() {
            VimFormatter::buffer_roll_new(WORDS[x + 3].clone(), TYPES[x + 3].clone());
        } else {
            VimFormatter::buffer_roll_new(String::new(), WordType::Space);
        }
        while x < WORDS.len() {
            if x + 4 < WORDS.len() {
                VimFormatter::buffer_roll_new(WORDS[x + 4].clone(), TYPES[x + 4].clone());
            } else {
                VimFormatter::buffer_roll_new(String::new(), WordType::Space);
            }
            INDENT_TRIGGER_BEFORE(x);
            IS_START_TRIGGER_BEFORE(x);
            IS_NEEDED_SPACE_TRIGGER_BEFORE(x);
            APPEND_TRIGGER_BEFORE(x);
            if let IsStart::Yes = IS_START {
                VimFormatter::append_indent(INDENT + APPEND);
            } else if IsNeededSpace::Yes == IS_NEEDED_SPACE && WORDS[x] != " " {
                TEXT.push(' ');
            };
            if !DONT_APPEND_FILTER(x) {
                TEXT.push_str(WORDS[x].as_str());
            }
            INDENT_TRIGGER_AFTER(x);
            IS_START_TRIGGER_AFTER(x);
            IS_NEEDED_SPACE_TRIGGER_AFTER(x);
            APPEND_TRIGGER_AFTER(x);
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
