use regex::{Regex, Error};
use crate::enums::{StringBounds, WordBounds};

/// Build a regular expression with an optional case-insenistive non-capturing group
/// If the source pattern starts with a non-capturing group, this will be ignored irrespective of the case_insenistive flag
pub fn build_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, Error> {
  let mut parts: Vec<&str> = vec![];
  // do not case-insensitive flag if a similar flag is already in the regular expression
  if case_insensitive && pattern.starts_with("(?") == false {
    parts.push("(?i)");
  }
  parts.push(pattern);
  let regex_str = parts. concat();
  Regex::new(&regex_str)
}

// Miscellaneous utility functions that do not belong to structs
/// corrects a numeric string after it has been extracted by removing trailing dots or commas
pub(crate) fn add_sanitized_numeric_string(output: &mut Vec<String>, num_string: &str) {
  output.push(num_string.trim_end_matches(".").trim_end_matches(",").to_string());
}

// internal utility methods

/// build regex pattern with word boundaries and WordBounds options
pub(crate) fn build_word_pattern(word: &str, bounds: WordBounds) -> String {
  bounds.to_pattern(word)
}

/// build regex pattern with whole word matches only. Does allow multiple word matches
/// if wildcards allowing spaces or punctuation are in the regex patterm
pub(crate) fn build_whole_word_pattern(word: &str) -> String {
  build_word_pattern(word, WordBounds::Both)
}

/// constructs an optional match group for whole words from an array of strs
/// e.g. &["cat?", "dog"] will match strings where cat and/or dog appear as whole words.
/// should be used with build_regex above or pattern_match / pattern_replace
pub(crate) fn build_optional_whole_word_pattern(words: &[&str]) -> String {
  let word_pattern = ["(", &words.join("|"), ")"].concat();
  build_word_pattern(&word_pattern, WordBounds::Both)
}

/*
* Convert an str array to vector of tuple pairs with the second element having the same boolean value
* as used in many multple match methods where the boolean element indicates case-sensitivity
*/
pub(crate) fn strs_to_str_bool_pairs<'a>(strs: &'a [&str], bool_val: bool) -> Vec<(&'a str, bool)> {
  strs.into_iter().map(|s| (*s, bool_val)).collect()
}

/*
* Convert an array of strs to a vector of SimpleBounds with start/end/contains and case-sensity rules
* as used in matched_conditional
* Only used internally with interger mode
* 0 = Start, 1 = End, 2+ = Contains
*/
pub(crate) fn strs_to_string_bounds<'a>(strs: &'a [&str], case_sensitive: bool, mode: u8) -> Vec<StringBounds<'a>> {
  strs.into_iter().map(|txt| StringBounds::new(mode, *txt, case_sensitive)).collect()
}

/*
* Convert an array of str/boolean tuples to a vector of SimpleBounds with start/end/contains
* as used in matched_conditional
* Only used internally with interger mode
* 0 = Start, 1 = End, 2+ = Contains
*/
pub(crate) fn pairs_to_string_bounds<'a>(pairs: &'a [(&str, bool)], mode: u8) -> Vec<StringBounds<'a>> {
  pairs.into_iter().map(|(txt, ci)| StringBounds::new(mode, *txt, *ci)).collect()
}
