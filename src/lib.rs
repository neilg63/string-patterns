extern crate regex;

mod utils;
pub mod enums;
pub mod alphanumeric;
pub mod segments;
pub mod simple_match;
pub mod pattern_match;
pub mod pattern_replace;
pub mod pattern_many;
pub mod pattern_split;
pub mod pattern_capture;
pub mod words;

pub use crate::enums::WordBounds;
pub use crate::alphanumeric::*;
pub use crate::segments::*;
pub use crate::simple_match::*;
pub use crate::pattern_match::*;
pub use crate::pattern_replace::*;
pub use crate::pattern_many::*;
pub use crate::pattern_split::*;
pub use crate::pattern_capture::*;
pub use crate::words::*;

// This library provides a set of traits and extension methods for &str and/or String
// to facilitate common string manipulations routines that may require multiple steps
// with the Rust standard library + Regex.
// Once installed you need not explicitly add regex::* to your project and
// string types will have many new match, replace, split and extract methods.
// Most methods imvoling regular expressions have variants ending in result returning the reuslt
// type with an error from the Regex crate and without, that return false and skips replacements
// if the regular is invalid. Use the main methods if you have tested your regular expression.
// There are also variants with a case_insensitive flag and without (_ci and _cs).
// When used on arrays or vectors of strings each regular expression will only be compiled and checked once, when you need 
// to search within a large set of text records. 
// Complex regular expressions, e.g. with look behind (?<=foo) or look ahead, work best after isolating a sample text snippet via simpler text-matching methods.
// Always consider the simplest strategy for extracting text, e.g. via to_head_tail(), to_segments(), before resorting to the regex-enabled pattern-prefixed methods.
