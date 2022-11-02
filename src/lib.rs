// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

pub mod shell;
pub mod vim;
pub mod xml;
pub mod html;

// Enum.

// Trait.

pub trait Formatter {
    fn format(text: &str) -> String;
}

// Struct.

// Function.
