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
    Space,
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
    Space,
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
pub struct Template;

impl crate::Formatter for Template {
    fn format(text: &str) -> String {
        unsafe {
            Template::split_text(("\n".to_string() + text + "\n").as_bytes());
            Template::rebuild_text();
            // Template::debug_print();
            return TEXT.clone();
        }
    }
}

impl Template {
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
                }
                SplitState::Space => last_type = WordType::Space,
                SplitState::Word => last_type = WordType::Word,
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
            } else {
                //
            }
        };
        INDENT_TRIGGER_AFTER = |x: usize| {
            if BLOCK_BEGIN_FILTER(x) {
                INDENT += 4;
            } else if BLOCK_END_FILTER(x) {
                INDENT -= 4;
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
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else {
                //
            }
        };
        IS_NEEDED_SPACE_TRIGGER_AFTER = |x: usize| {
            if [""].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
            } else if [""].contains(&WORDS[x].as_str()) {
                IS_NEEDED_SPACE = IsNeededSpace::No;
            } else {
                IS_NEEDED_SPACE = IsNeededSpace::Yes;
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
        Template::buffer_roll_new(WORDS[0 + 1].clone(), TYPES[0 + 1].clone());
        Template::buffer_roll_new(WORDS[0 + 2].clone(), TYPES[0 + 2].clone());
        Template::buffer_roll_new(WORDS[0 + 3].clone(), TYPES[0 + 3].clone());
        while x < WORDS.len() {
            if x + 4 < WORDS.len() {
                Template::buffer_roll_new(WORDS[x + 4].clone(), TYPES[x + 4].clone());
            } else {
                Template::buffer_roll_new(String::new(), WordType::Space);
            }
            INDENT_TRIGGER_BEFORE(x);
            IS_START_TRIGGER_BEFORE(x);
            IS_NEEDED_SPACE_TRIGGER_BEFORE(x);
            if IsStart::Yes == IS_START {
                Template::append_indent(INDENT);
            } else if IsNeededSpace::Yes == IS_NEEDED_SPACE {
                TEXT.push(' ');
            };
            if !DONT_APPEND_FILTER(x) {
                TEXT.push_str(WORDS[x].as_str());
            }
            INDENT_TRIGGER_AFTER(x);
            IS_START_TRIGGER_AFTER(x);
            IS_NEEDED_SPACE_TRIGGER_AFTER(x);
            Template::buffer_roll_old(WORDS[x].clone(), TYPES[x].clone());
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
