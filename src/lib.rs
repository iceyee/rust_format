// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

pub mod html;
pub mod java;
pub mod js;
pub mod shell;
pub mod template;
pub mod vim;
pub mod xml;

// Enum.

// Trait.

pub trait Formatter {
    fn format(text: &str) -> String;
}

// Struct.

// Function.
