use regex::*;
use crate::enums::WordBounds;

/// Build a regular expression with an optional case-insenistive non-capturing group
/// If the source pattern starts with a non-capturing group, this will be ignored irrespective of the case_insenistive flag
pub(crate) fn build_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, Error> {
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