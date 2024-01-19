extern crate regex;
mod utils;
pub mod enums;

use std::str::FromStr;
use regex::*;
use utils::*;
use crate::enums::WordBounds;

/// This library provides a set of traits and extension methods for &str and/or String
/// to facilitate common string manipulations routines that may require multiple steps
/// with the Rust standard library + Regex.
/// Once installed you need not explicitly add regex::* to your project and
/// string types will have many new match, replace, split and extract methods.
/// Most methods imvoling regular expressions have variants ending in result returning the reuslt
/// type with an error from the Regex crate and without, that return false and skips replacements
/// if the regular is invalid. Use the main methods if you have tested your regular expression.
/// There are also variants with a case_insensitive flag and without (_ci and _cs).
/// When used on arrays or vectors of strings each regular expression will only be compiled and checked once, when you need 
/// to search within a large set of text records. 
/// Complex regular expressions, e.g. with look behind (?<=foo) or look ahead, work best after isolating a sample text snippet via simpler text-matching methods.
/// Always consider the simplest strategy for extracting text, e.g. via to_head_tail(), to_segments(), before resorting to the regex-enabled pattern-prefixed methods.

/// Core regular expression match methods
pub trait PatternMatch {
  /// Apply a regular expression match on the current string
  /// If the regex doesn't compile it will return an error
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error>;

  /// Apply a regular expression match on the current string with a boolean case_insensitive flag
  /// NB: If the regex doesn't compile it will return false
  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool;

  /// Apply a regular expression match on the current string in case-insensitive mode
  /// NB: If the regex doesn't compile it will return false
  fn pattern_match_ci(&self, pattern: &str) -> bool;

  /// Apply a regular expression match on the current string in case-sensitive mode
  /// NB: If the regex doesn't compile it will return false
  fn pattern_match_cs(&self, pattern: &str) -> bool;

}

/// Core regular expression replacement methods 
pub trait PatternReplace {
  /// Apply a regular expression match on the current string with a boolean case_insensitive flag
  /// NB: If the regex doesn't compile it will return an Error, otherwise in Ok result.
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;

  /// Replace all matches of the pattern within a longer text with a boolean case_insensitive flag
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_result(&self, pattern: &str, replacement: &str,case_insensitive: bool) -> Result<Self, Error> where Self:Sized;

  /// Replace all matches of the pattern within a longer text in case-insensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> Self where Self:Sized;

  /// Replace all matches of the pattern within a longer text in case-sensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> Self where Self:Sized;

}

/// Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats
pub trait StripCharacters {

  /// Removes all characters that any are not letters or digits, such as punctuation or symobols
  /// Letters include those used in most non-Latin alphabets
  fn strip_non_alphanum(&self) -> Self where Self:Sized;

  // Remove all characters except digits, including punctuation such as decmimal points
  fn strip_non_digits(&self) -> Self where Self:Sized;

  /// Extracts valid numeric string components from a longer string
  fn to_numeric_strings(&self) -> Vec<String>;

  /// Always interpret numeric strings with dots as thousand separators and commas as decimal separators
  fn to_numeric_strings_euro(&self) -> Vec<String>;

  fn to_numeric_strings_conditional(&self, enforce_comma_separator: bool) -> Vec<String>;

  /// Extract numeric strings and cast to numbers with conditional logic over commas and dots,
  /// The boolean flag enforces European logic where dots separate thousands and commas decimals
  /// Otherwise the correct format is deduced. Numeric strings are problematic when they only contain
  /// one comma or point. Otherwise the last separator is always considered the decimal separator if 
  /// it differs from the first separators.
  fn to_numbers_conditional<T: FromStr>(&self, enforce_comma_separator: bool) -> Vec<T>;

  /// Extracts valid integers or floats from a longer string
  fn to_numbers<T: FromStr>(&self) -> Vec<T>;

  fn to_numbers_euro<T: FromStr>(&self) -> Vec<T>;

  /// Correct numbers to conform to use dots (periods, full-stops) only as decimal separators
  /// Works only on the first number encountered and used with to_numeric_strings or to_numeric_strings_euro
  /// to correct multiple numbers in a longer string
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> Self;

  /// Extracts the first valid integer or float from a longer string if present
  fn to_first_number<T: FromStr + Copy>(&self) -> Option<T>;

  /// Extracts the first valid integer or float from a longer string
  /// if commas are used for decimals and dots for thousand separators
  fn to_first_number_euro<T: FromStr + Copy>(&self) -> Option<T>;

  /// Removes all characters no used in valid numeric sequences
  fn strip_non_numeric(&self) -> Self where Self:Sized;

}

/// Regex-free matcher methods for common use cases
pub trait SimpleMatch {
  /// Starts with a case-insensitive alphanumeric sequence
  fn starts_with_ci(&self, pattern: &str) -> bool;
  
  /// Starts with a case-insensitive alphanumeric sequence
  fn starts_with_ci_alphanum(&self, pattern: &str) -> bool;
  
  /// Ends with a case-insensitive alphanumeric sequence
  fn ends_with_ci(&self, pattern: &str) -> bool;
  
  /// Ends with a case-insensitive alphanumeric sequence
  fn ends_with_ci_alphanum(&self, pattern: &str) -> bool;

  /// Contains a case-insensitive alphanumeric sequence
  fn contains_ci(&self, pattern: &str) -> bool;
  
  /// Contains a case-insensitive alphanumeric sequence
  fn contains_ci_alphanum(&self, pattern: &str) -> bool;
}

/// Method to check if the string may be parsed to an integer or float
pub trait IsNumeric {
  /// strict check on a numeric string before using ```.parse::<T>()```
  /// use trim() or correct_numeric_string() first for looser number validation
  /// This mirrors a similar function in PHP, but is will fail with spaces or 
  /// any non-numeric characters other than a leading minus or a single decimal point
  /// For characters, is_numeric checks for decimal digit-equivalent characters
  fn is_numeric(&self) -> bool;
}

/// Implementation for &str / String
impl IsNumeric for str {

  /// Check if the string may be parsed to a number
  /// This is a now a strict regex-free check
  /// Use trim() or correct_numeric_string() first for looser number validation
  fn is_numeric(&self) -> bool {
    let num_chars = self.chars().count();
    let last_index = num_chars - 1;
    let mut num_valid = 0usize;
    let mut index = 0usize;
    let mut num_decimal_separators = 0usize;
    for c in self.chars().into_iter() {
      let is_digit = c.is_digit(10);
      let valid_char =  if is_digit {
        true
      } else {
        match c {
          '-' => index == 0,
          '.' => index < last_index && num_decimal_separators < 1,
          _ => false
        }
      };
      if c == '.' {
        num_decimal_separators += 1;
      }
      if valid_char {
        num_valid += 1;
      }
      index += 1;
    }
    num_valid == num_chars
  }
}

/// Converts arrays or vectors of strs to a vector of owned strings
pub trait ToStrings {
  fn to_strings(&self) -> Vec<String>;
}

impl<T: ToString> ToStrings for Vec<T> {
  /// Converts arrays or vectors of strs to a vector of owned strings
  fn to_strings(&self) -> Vec<String> {
      self.into_iter().map(|s| s.to_string()).collect()
  }
}

impl<T: ToString> ToStrings for [T] {
  /// Converts arrays or vectors of strs to a vector of owned strings
  fn to_strings(&self) -> Vec<String> {
      self.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }
}

/// Return the indices of all ocurrences of a string
pub trait MatchOccurrences {
  /// Return the indices only of all matches of a given string pattern (not a regular expression)
  /// Builds on match_indices in the Rust standard library
  fn find_matched_indices(&self, pat: &str) -> Vec<usize>;
}


impl MatchOccurrences for str {
    /// Return the indices only of all matches of a given regular expression
  fn find_matched_indices(&self, pat: &str) -> Vec<usize> {
    self.match_indices(pat).into_iter().map(|pair| pair.0).collect::<Vec<usize>>()
  }
}

/// Implement regular expression match and replace methods for str and owned String
impl PatternMatch for str {

  ///
  /// Simple regex-compatible match method that will return an optional boolean 
  /// - Some(true) means the regex is valid and the string matches
  /// - Some(false) means the regex is valid and the string does not match
  /// - None means the regex is not valid and can this not be evaluated
  /// 
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re)  => Ok(re.is_match(self)),
      Err(error) => Err(error)
    }
}

  ///
  /// Simple regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  /// 
  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool {
      if let Ok(re) = build_regex(pattern, case_insensitive) {
        re.is_match(self)
      } else {
        false
      }
  }

  ///
  /// Simple case-insensitive regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  fn pattern_match_ci(&self, pattern: &str) -> bool {
    self.pattern_match(pattern, true)
  }

  ///
  /// Simple case-sensitive regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  fn pattern_match_cs(&self, pattern: &str) -> bool {
    self.pattern_match(pattern, false)
  }

}

/// Core regex replacement methods for Strings
impl PatternReplace for String {
  
  /// Regex-enabled replace method that will return an OK String result if successful and an error if the regex fails
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<String, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(re.replace_all(self, replacement).to_string()),
      Err(error) => Err(error)
    }
  }

  /// Simple regex-enabled replace method that will return the same string if the regex fails
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> String {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }


  /// Simple case-insensitive regex-enabled replace method that will return the same string if the regex fails
  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> String {
    self.pattern_replace(pattern, replacement, true)
  }


  /// Simple case-sensitive regex-enabled replace method that will return the same string if the regex fails
  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> String {
    self.pattern_replace(pattern, replacement, false)
  }

}

impl StripCharacters for String {
    
  /// Remove all characters that are not letters or numerals for later string comparison. Does not use a regular expression
  fn strip_non_alphanum(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_alphanumeric()).collect::<String>()
  }

  /// Remove all characters that are not numerals for later string comparison. Does not use a regular expression
  fn strip_non_digits(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_digit(10)).collect::<String>()
  }

  /// Correct numeric strings with commas as thousand separators or as decimal separators
  /// to a regular format with punctuation only for decimal points before being parsed to an integer or float
  /// This is best used only with numeric strings as it will strip commas and dots not used as decimal separators
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> Self {
      let commas = self.find_matched_indices(",");
      let last_comma_index = commas.last().unwrap_or(&0).to_owned();
      let points = self.find_matched_indices(".");
      let last_point_index = points.last().unwrap_or(&0).to_owned();
      let num_commas = commas.len();
      if points.len() > 1 || (last_comma_index > last_point_index  && num_commas <= 1) || (enforce_comma_separator && num_commas <= 1) {
        if num_commas < 1 {
          self.replace(".", "")
        } else {
          let (main, dec_part) = self.to_start_end(",");
          [main.replace(".", ""), dec_part].join(".")
        }
      } else {
        self.replace(",", "")
      }
  }

  /// conditionally extract numeric strings from a longer string
  fn to_numeric_strings_conditional(&self, enforce_comma_separator: bool) -> Vec<String> {
    let mut prev_char = ' ';
    let mut seq_num = 0;
    let mut num_string = String::new();
    let mut output: Vec<String> = Vec::new();
    for component in self.chars() {
      let mut is_end = false;
      if component.is_digit(10) {
        num_string.push(component);
        seq_num += 1;
      } else if prev_char.is_digit(10) {
        match component {
          '.' | '․' | ',' => {
            if component == ',' {
              num_string.push(',');
            } else {
              num_string.push('.');
            }
            seq_num = 0;
          },
          _ => {
            is_end = true;
          }
        }
      } else {
        is_end = true;
      }
      if is_end {
        if seq_num > 0 {
          add_sanitized_numeric_string(&mut output, &num_string.correct_numeric_string(enforce_comma_separator));
          num_string = String::new();
        }
        seq_num = 0;
      }
      prev_char = component;
    }
    if num_string.len() > 0 {
      add_sanitized_numeric_string(&mut output, &num_string.correct_numeric_string(enforce_comma_separator));
    }
    output
  }

  fn to_numeric_strings(&self) -> Vec<String> {
    self.to_numeric_strings_conditional(false)
  }

  fn to_numeric_strings_euro(&self) -> Vec<String> {
    self.to_numeric_strings_conditional(true)
  }

  fn to_numbers_conditional<T: FromStr>(&self, enforce_comma_separator: bool) -> Vec<T> {
    self.to_numeric_strings_conditional(enforce_comma_separator).into_iter()
      .map(|s| s.parse::<T>())
      .filter(|s| s.is_ok())
      .map(|s| s.ok().unwrap())
      .collect::<Vec<T>>()
  }

  fn to_numbers<T: FromStr>(&self) -> Vec<T> {
    self.to_numbers_conditional(false)
  }

  fn to_numbers_euro<T: FromStr>(&self) -> Vec<T> {
    self.to_numbers_conditional(true)
  }

  fn to_first_number<T: FromStr + Copy>(&self) -> Option<T> {
    if let Some(number) = self.to_numbers::<T>().first() {
      Some(*number)
    } else {
      None
    }
  }

  fn to_first_number_euro<T: FromStr + Copy>(&self) -> Option<T> {
    if let Some(number) = self.to_numbers_euro::<T>().first() {
      Some(*number)
    } else {
      None
    }
  }

  fn strip_non_numeric(&self) -> String {
    self.to_numeric_strings().join(" ")
  }

}


/// Methods to validate strings with character classes
pub trait CharGroupMatch {
  /// Does the string contain any decimal digits
  fn has_digits(&self) -> bool;

  /// Does the string contain any digits any supported radix
  fn has_digits_radix(&self, radix: u8) -> bool;

  /// Does the string contain any alphanumeric characters including those from non-Latin alphabets
  fn has_alphanumeric(&self) -> bool;

  /// Does the string contain any letters including those from non-Latin alphabets, but excluding digits
  fn has_alphabetic(&self) -> bool;

  fn is_digits_only(&self) -> bool;

  /// Does the string contain any digits any supported radix
  fn is_digits_only_radix(&self, radix: u8) -> bool;

}

impl CharGroupMatch for str {

  fn has_digits(&self) -> bool {
      self.chars().any(|c| c.is_ascii_digit())
  }

  fn has_digits_radix(&self, radix: u8) -> bool {
    self.chars().any(|c| c.is_digit(radix as u32))
  }

  fn has_alphanumeric(&self) -> bool {
      self.chars().any(char::is_alphanumeric)
  }

  fn has_alphabetic(&self) -> bool {
    self.chars().any(char::is_alphabetic)
  }

  fn is_digits_only(&self) -> bool {
    self.chars().all(|c| c.is_ascii_digit())
  }

  /// Does the string contain any digits any supported radix
  fn is_digits_only_radix(&self, radix: u8) -> bool {
    self.chars().all(|c| c.is_digit(radix as u32))
  }

}

impl SimpleMatch for str {
  /// Starts with a case-insensitive sequence
  fn starts_with_ci(&self, pattern: &str) -> bool {
    self.to_lowercase().starts_with(&pattern.to_lowercase())
  }
  
  /// Starts with a case-insensitive alphanumeric sequence
  fn starts_with_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum().starts_with(&pattern.to_lowercase())
  }
  
  /// Ends with a case-insensitive sequence
  fn ends_with_ci(&self, pattern: &str) -> bool {
    self.to_lowercase().ends_with(&pattern.to_lowercase())
  }
  
  /// Ends with a case-insensitive alphanumeric sequence
  fn ends_with_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum().ends_with(&pattern.to_lowercase())
  }

  /// Contains a case-insensitive sequence
  fn contains_ci(&self, pattern: &str) -> bool {
    self.to_lowercase().contains(&pattern.to_lowercase())
  }
  
  /// Contains a case-insensitive alphanumeric sequence
  fn contains_ci_alphanum(&self, pattern: &str) -> bool {
    self.to_lowercase().strip_non_alphanum().contains(&pattern.to_lowercase())
  }
}


/// Pattern methods for arrays or vectors only, return vectors of booleans matching each input string
pub trait PatternMatches {
  /// Returns result with a vector of boolean matches for an array or vector of strings with case-insensitive flag
  /// or an error if the regex does not compile
  fn pattern_matches_result(&self, pattern: &str, case_insensitive: bool) -> Result<Vec<bool>, Error>;

  /// Returns vector of boolean matches for an array or vector of strings with case-insensitive flag
  fn pattern_matches(&self, pattern: &str, case_insensitive: bool) -> Vec<bool>;

  /// Returns vector of boolean matches for an array or vector of strings in case-insensitive mode
  fn pattern_matches_ci(&self, pattern: &str) -> Vec<bool>;

  /// Returns vector of boolean matches for an array or vector of strings in case-sensitive mode
  fn pattern_matches_cs(&self, pattern: &str) -> Vec<bool>;
}

impl PatternMatch for [String] {
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().any(|segment| re.is_match(segment))),
      Err(error) => Err(error)
    }
  }
  /// Simple regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool {
    self.pattern_match_result(pattern, case_insensitive).unwrap_or(false)
  }

  /// Case-insensitive regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  fn pattern_match_ci(&self, pattern: &str) -> bool {
    self.pattern_match(pattern, true)
  }

  /// Case-sensitive regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  fn pattern_match_cs(&self, pattern: &str) -> bool {
    self.pattern_match(pattern, false)
  }

}

impl PatternMatches for [String] {

  /// Returns an Ok result with a vector of boolean matches for an array or vector of strings with a case-insensitive flag
  /// and an error only if the regex fails to compile.
  fn pattern_matches_result(&self, pattern: &str, case_insensitive: bool) -> Result<Vec<bool>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().map(|segment| re.is_match(segment)).collect::<Vec<bool>>()),
      Err(error) => Err(error)
    }
  }

  /// Returns vector of boolean matches for an array or vector of strings with case-insensitive flag
  fn pattern_matches(&self, pattern: &str, case_insensitive: bool) -> Vec<bool> {
    match self.pattern_matches_result(pattern, case_insensitive) {
      Ok(results) => results,
      Err(_error) => self.into_iter().map(|_segment| false).collect::<Vec<bool>>()
    }
  }

  /// Returns vector of boolean matches for an array or vector of strings in case-insensitive mode
  fn pattern_matches_ci(&self, pattern: &str) -> Vec<bool> {
    self.pattern_matches(pattern, true)
  }

  /// Returns vector of boolean matches for an array or vector of strings in case-sensitive mode
  fn pattern_matches_cs(&self, pattern: &str) -> Vec<bool> {
    self.pattern_matches(pattern, false)
  }

}

impl PatternReplace for Vec<String> {
  ///
  /// Optional regex-enabledd replace method that will return None if the regex fails
  /// 
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<Vec<String>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => {
        let replacements = self.into_iter().map(|segment| re.replace_all(segment, replacement).to_string()).collect::<Vec<String>>();
        Ok(replacements)
      },
      Err(error) => Err(error)
    }
  }

  ///
  /// Simple regex-enabledd replace method that will return the same string if the regex fails
  /// 
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Vec<String> {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }


  /// Simple case-insensitive regex-enabled replace method that will return the same string if the regex fails
  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> Vec<String> {
    self.pattern_replace(pattern, replacement, true)
  }

  /// Simple case-sensitive regex-enabled replace method that will return the same string if the regex fails
  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> Vec<String> {
    self.pattern_replace(pattern, replacement, false)
  }

}

/// Provides methods to match with multiple patterns 
/// expressed as arrays of tuples or simple strs (for pattern_match_many_ci and pattern_match_many_cs)
pub trait PatternMatchMany {
  /// Matches all of the patterns in case-sensitivity flag
  /// with an array of tuples (patterns, case_insensitive)
  fn pattern_match_many(&self, patterns: &[&str], case_insensitive: bool) -> bool;

  /// Matches all of the patterns in case-insensitive mode
  /// with an array of str patterns
  fn pattern_match_many_ci(&self, patterns: &[&str]) -> bool;

   /// Matches all of the patterns in case-sensitive mode
  /// with an array of str patterns
  fn pattern_match_many_cs(&self, patterns: &[&str]) -> bool;

  /// Matches all of the patterns with case-insensitive flag
  /// e.g. ```(r#"a[ck]"#, true)``` matches "ac" or "ak" whether upper, lower or mixed case
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_match_many_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool;
  
  /// string matches all conditional patterns which may be positive / negative and case insensitive or not
  fn pattern_match_many_conditional(&self, pattern_sets: &[(bool, &str, bool)]) -> bool;
  
  /// Matches one or more of the patterns in case-sensitivity flag
  /// with an array of tuples (patterns, case_insensitive)
  fn pattern_match_any(&self, patterns: &[&str], case_insensitive: bool) -> bool;

  /// Matches one or more of the patterns in case-insensitive mode
  /// with an array of str patterns
  fn pattern_match_any_ci(&self, patterns: &[&str]) -> bool;

   /// Matches one or more of the patterns in case-sensitive mode
  /// with an array of str patterns
  fn pattern_match_any_cs(&self, patterns: &[&str]) -> bool;

  /// Matches one or more of the patterns with case-insensitive boolean flag
  fn pattern_match_any_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool;
  
  /// string matches one or more conditional patterns which may be positive / negative and case insensitive or not
  fn pattern_match_any_conditional(&self, pattern_sets: &[(bool, &str, bool)]) -> bool;
}

/// Provides methods to replace with multiple patterns 
/// expressed as arrays of tuples
pub trait PatternReplaceMany {
  /// Replaces multiple sets of patterns with replacements in case-sensitive mode
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_replace_pairs(&self, replacement_sets: &[(&str, &str)]) -> Self where Self: Sized;

  /// Replaces multiple sets of patterns with replacements in case-insensitive mode
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_replace_pairs_ci(&self, replacement_sets: &[(&str, &str)]) -> Self where Self: Sized;

  /// Replaces multiple sets of patterns with replacements in case-sensitive mode
  /// with an array of simple tuples (pattern, replacement)
  fn pattern_replace_sets(&self, replacement_sets: &[(&str, &str, bool)]) -> Self where Self: Sized;
}

impl PatternMatchMany for str {

  /// Matches all of the patterns in case-sensitivity flag
  /// with an array of tuples (patterns, case_insensitive)
  fn pattern_match_many(&self, patterns: &[&str], case_insensitive: bool) -> bool {
    let mut num_matched = 0usize;
    let num_patterns = patterns.len();
    for pattern in patterns {
      if self.pattern_match(pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// Matches all of the patterns in case-insensitive mode
  /// with an array of str patterns
  fn pattern_match_many_ci(&self, patterns: &[&str]) -> bool {
    self.pattern_match_many(patterns, true)
  }

  /// Matches all of the patterns in case-sensitive mode
  /// with an array of str patterns
  fn pattern_match_many_cs(&self, patterns: &[&str]) -> bool {
    self.pattern_match_many(patterns, false)
  }

  /// Matches all of the patterns with case-insensitive flag
  /// e.g. ```(r#"a[ck]"#, true)``` "ac" or "ak" whether upper, lower or mixed case
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_match_many_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool {
    let mut num_matched = 0usize;
    let num_patterns = pattern_sets.len();
    for pair in pattern_sets {
      let (pattern, case_insensitive) = *pair;
      if self.pattern_match(pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// Matches all of the patterns with positivity condition and case-insensitive flag
  /// e.g. ```(false, "a[ck]", true)``` does not contain "ac" or "ak" whether upper, lower or mixed case
  /// with an array of tuples (positive, pattern, case_insensitive)
  fn pattern_match_many_conditional(&self, pattern_sets: &[(bool, &str, bool)]) -> bool {
    let mut num_matched = 0usize;
    let num_patterns = pattern_sets.len();
    for pattern_set in pattern_sets {
      let (is_positive, pattern, case_insensitive) = *pattern_set;
      let is_matched = self.pattern_match(pattern, case_insensitive);
      if is_matched == is_positive {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// Matches one or more of the patterns in case-sensitivity flag
  /// with an array of tuples (patterns, case_insensitive)
  fn pattern_match_any(&self, patterns: &[&str], case_insensitive: bool) -> bool {
    for pattern in patterns {
      if self.pattern_match(pattern, case_insensitive) {
         return true;
      }
    }
    false
  }

  /// Matches one or more of the patterns in case-insensitive mode
  /// with an array of str patterns
  fn pattern_match_any_ci(&self, patterns: &[&str]) -> bool {
    self.pattern_match_any(patterns, true)
  }

  /// Matches one or more of the patterns in case-sensitive mode
  /// with an array of str patterns
  fn pattern_match_any_cs(&self, patterns: &[&str]) -> bool {
    self.pattern_match_any(patterns, false)
  }

  /// Matches one or more of the patterns with case-insensitive flag
  /// e.g. ```(r#"a[ck]"#, true)``` matches "ac" or "ak" whether upper, lower or mixed case
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_match_any_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool {
    for pair in pattern_sets {
      let (pattern, case_insensitive) = *pair;
      if self.pattern_match(pattern, case_insensitive) {
         return true;
      }
    }
    false
  }

  /// Matches one or more of the patterns with positivity condition and case-insensitive flag
  fn pattern_match_any_conditional(&self, pattern_sets: &[(bool, &str, bool)]) -> bool {
   for pattern_set in pattern_sets {
      let (is_positive, pattern, case_insensitive) = *pattern_set;
      let is_matched = self.pattern_match(pattern, case_insensitive);
      if is_matched == is_positive {
         return true;
      }
    }
    false
  }

}

impl PatternReplaceMany for String {
  /// Replaces multiple sets of patterns with replacements and boolean case sensitivity 
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_replace_sets(&self, replacement_sets: &[(&str, &str, bool)]) -> String {
    let mut return_string = self.clone();
    for replacement_set in replacement_sets {
      let (pattern, replacement, case_insensitive) = *replacement_set;
      if let Ok(new_string) = return_string.pattern_replace_result(pattern, replacement, case_insensitive) {
        return_string = new_string;
      }
    }
    return_string
  }

  /// Replaces multiple sets of patterns with replacements in case-sensitive mode
  /// with an array of simple tuples (pattern, replacement)
  fn pattern_replace_pairs(&self, replacement_pairs: &[(&str, &str)]) -> String {
    let mut return_string = self.clone();
    for replacement_pair in replacement_pairs {
      let (pattern, replacement) = *replacement_pair;
      if let Ok(new_string) = return_string.pattern_replace_result(pattern, replacement, false) {
        return_string = new_string;
      }
    }
    return_string
  }

  /// Replaces multiple sets of patterns with replacements in case-insensitive mode
  /// with an array of simple tuples (pattern, replacement)
  fn pattern_replace_pairs_ci(&self, replacement_pairs: &[(&str, &str)]) -> String {
    let mut return_string = self.clone();
    for replacement_pair in replacement_pairs {
      let (pattern, replacement) = *replacement_pair;
      if let Ok(new_string) = return_string.pattern_replace_result(pattern, replacement, true) {
        return_string = new_string;
      }
    }
    return_string
  }
}

/// Implement PatternMatchMany for vectors of strings.
impl PatternMatchMany for [String] {

  /// Match all from an array of strs with a boolean case_insensitive parameter
  fn pattern_match_many(&self, patterns: &[&str], case_insensitive: bool) -> bool {
    let mut num_matched = 0usize;
    let num_patterns = patterns.len();
    for pattern in patterns {
      if self.pattern_match(pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// Match all from an array of strs with a in case-insensitive mode
  fn pattern_match_many_ci(&self, patterns: &[&str]) -> bool {
    self.pattern_match_many(patterns, true)
  }

  /// Match all from an array of strs with a in case-sensitive mode
  fn pattern_match_many_cs(&self, patterns: &[&str]) -> bool {
    self.pattern_match_many(patterns, false)
  }

  /// Match all from an array of tuples of strs and boolean case_insensitive flags
  fn pattern_match_many_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool {
    let mut num_matched = 0usize;
    let num_patterns = pattern_sets.len();
    for pair in pattern_sets {
      let (pattern, case_insensitive) = *pair;
      if self.pattern_match(pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// Match all from an array of tuples of boolean positive, strs and boolean case_insensitive flags
  /// Will check of each pattern is or is not matched with mixed case-sensitity rules
  fn pattern_match_many_conditional(&self, pattern_sets: &[(bool, &str, bool)]) -> bool {
    let mut num_matched = 0usize;
    let num_patterns = pattern_sets.len();
    for pattern_set in pattern_sets {
      let (is_positive, pattern, case_insensitive) = *pattern_set;
      let is_matched = self.pattern_match(pattern, case_insensitive);
      if is_matched == is_positive {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// with an array of tuples (patterns, case_insensitive)
  /// Matches one or more of the patterns in case-sensitivity flag
  fn pattern_match_any(&self, patterns: &[&str], case_insensitive: bool) -> bool {
    for pattern in patterns {
      if self.pattern_match(pattern, case_insensitive) {
         return true;
      }
    }
    false
  }

  /// Matches one or more of the patterns in case-insensitive mode
  /// with an array of str patterns
  fn pattern_match_any_ci(&self, patterns: &[&str]) -> bool {
    self.pattern_match_any(patterns, true)
  }

  /// Matches one or more of the patterns in case-sensitive mode
  /// with an array of str patterns
  fn pattern_match_any_cs(&self, patterns: &[&str]) -> bool {
    self.pattern_match_any(patterns, false)
  }

  /// Matches one or more of the patterns with case-insensitive flag
  /// e.g. ```(r#"a[ck]"#, true)``` matches "ac" or "ak" whether upper, lower or mixed case
  /// with an array of tuples (pattepattern_match_many_cirn, replacement, case_insensitive)
  fn pattern_match_any_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool {
    for pair in pattern_sets {
      let (pattern, case_insensitive) = *pair;
      if self.pattern_match(pattern, case_insensitive) {
         return true;
      }
    }
    false
  }

  /// Matches one or more of the patterns with positivity condition and case-insensitive flag
  fn pattern_match_any_conditional(&self, pattern_sets: &[(bool, &str, bool)]) -> bool {
    for pattern_set in pattern_sets {
      let (is_positive, pattern, case_insensitive) = *pattern_set;
      let is_matched = self.pattern_match(pattern, case_insensitive);
      if is_matched == is_positive {
         return true;
      }
    }
    false
  }
}

/// ReplaceMany implementation for vectors of owned strings
impl PatternReplaceMany for Vec<String> {

  /// Replace all matched patterns with mixed case-sensitivity flags, expressed as tuples of (pattern: &str, replacement: &str, case_insensitive: bool)
  fn pattern_replace_sets(&self, replacement_sets: &[(&str, &str, bool)]) -> Vec<String> {
    let mut return_strings = self.clone();
    for replacement_set in replacement_sets {
      let (pattern, replacement, case_insensitive) = *replacement_set;
      if let Ok(new_strings) = return_strings.pattern_replace_result(pattern, replacement, case_insensitive) {
        return_strings = new_strings;
      }
    }
    return_strings
  }

  /// Replace all matched patterns in case-sensitive mode (unless defined in the pattern via (?i)),
  /// expressed as tuples of (pattern: &str, replacement: &str)
  fn pattern_replace_pairs(&self, replacement_pairs: &[(&str, &str)]) -> Vec<String> {
    let mut return_strings = self.clone();
    for replacement_pair in replacement_pairs {
      let (pattern, replacement) = *replacement_pair;
      if let Ok(new_string) = return_strings.pattern_replace_result(pattern, replacement, false) {
        return_strings = new_string;
      }
    }
    return_strings
  }

  /// Replace all matched patterns in case-insensitive,
  /// expressed as tuples of (pattern: &str, replacement: &str)
  fn pattern_replace_pairs_ci(&self, replacement_pairs: &[(&str, &str)]) -> Vec<String> {
    let mut return_strings = self.clone();
    for replacement_pair in replacement_pairs {
      let (pattern, replacement) = *replacement_pair;
      if let Ok(new_string) = return_strings.pattern_replace_result(pattern, replacement, true) {
        return_strings = new_string;
      }
    }
    return_strings
  }
}


/// Provides methods to split a &str/string on a regular expression
pub trait PatternSplit {
    /// Splits a string on a regular expression with boolean case_insensitive flag. 
    /// Returns result with vector of the parts between matches.
    fn pattern_split_result(&self, pattern: &str, case_sensitive: bool) -> Result<Vec<String>, Error>;

    /// Splits a string on a regular expression with boolean case_insensitive flag. 
    /// Returns result with a tuple with head and tail or an error.
    fn pattern_split_pair_result(&self, pattern: &str, case_sensitive: bool) -> Result<(String, String), Error>;

    /// Splits a string on a regular expression with boolean case_insensitive flag. 
    /// Returns  a vector of strings, empty if the regular expression fails.
    fn pattern_split(&self, pattern: &str, case_sensitive: bool) -> Vec<String>;

    /// Splits a string on a regular expression in case-sensitive mode. 
    /// Returns  a vector of strings, empty if the regular expression fails.
    fn pattern_split_cs(&self, pattern: &str) -> Vec<String>;

    /// Splits a string on a regular expression in case-isensitive mode. 
    /// Returns  a vector of strings, empty if the regular expression fails.
    fn pattern_split_ci(&self, pattern: &str) -> Vec<String>;

    /// Splits a string on a regular expression with boolean case_insensitive flag. 
    /// Returns a tuple with head and tail. The tail will be en empty string if not matched
    fn pattern_split_pair(&self, pattern: &str, case_sensitive: bool) -> (String, String);

    /// Split a string on a regular expression in case-sensitive mode. 
    /// Returns a tuple with head and tail. The tail will be en empty string if not matched
    fn pattern_split_pair_cs(&self, pattern: &str) -> (String, String);

    /// Split a string on a regular expression in case-isensitive mode. 
    /// Returns a tuple with head and tail. The tail will be en empty string if not matched
    fn pattern_split_pair_ci(&self, pattern: &str) -> (String, String);
}

/// Implemented for &str and available to String too
impl PatternSplit for str {

  /// Split a string on a regular expression into a result with a vector of strings
  fn pattern_split_result(&self, pattern: &str, case_sensitive: bool) -> Result<Vec<String>, Error> {
    match build_regex(pattern, case_sensitive) {
      Ok(regex) => Ok(regex.split(self).into_iter().map(|s| s.to_string()).collect::<Vec<String>>()),
      Err(error) => Err(error),
    }
  }

  /// Split a string on a regular expression into a result with a tuple of head / tail strings
  fn pattern_split_pair_result(&self, pattern: &str, case_sensitive: bool) -> Result<(String, String), Error> {
    match build_regex(pattern, case_sensitive) {
      Ok(regex) => {
        let parts = regex.splitn(self, 2).collect::<Vec<&str>>();
        let head = parts.get(0).unwrap_or(&"").to_owned().to_string();
        let tail = parts.get(1).unwrap_or(&"").to_owned().to_string();
        Ok((head, tail))
      },
      Err(error) => Err(error),
    }
  }

  fn pattern_split(&self, pattern: &str, case_sensitive: bool) -> Vec<String> {
    match self.pattern_split_result(pattern, case_sensitive) {
      Ok(parts) => parts,
      Err(_error) => vec![],
    }
  }

  fn pattern_split_pair(&self, pattern: &str, case_sensitive: bool) -> (String, String) {
    match self.pattern_split_pair_result(pattern, case_sensitive) {
      Ok(parts) => parts,
      Err(_error) => ("".to_owned(), "".to_owned()),
    }
  }

  fn pattern_split_cs(&self, pattern: &str) -> Vec<String> {
    self.pattern_split(pattern, false)
  }

  fn pattern_split_ci(&self, pattern: &str) -> Vec<String> {
    self.pattern_split(pattern, true)
  }

  fn pattern_split_pair_cs(&self, pattern: &str) -> (String, String) {
    self.pattern_split_pair(pattern, false)
  }

  fn pattern_split_pair_ci(&self, pattern: &str) -> (String, String) {
    self.pattern_split_pair(pattern, true)
  }

}

/// Methods to split a longer strong on a separator and return a vector of strings,
/// a tuple of two strings or single optional string segment
/// Note some methods may return empty segments in the case of leading, trailing or repeated separators
/// See notes below
pub trait ToSegments {

  /// Extract a vector of non-empty strings from a string-like object with a given separator
  /// excluding leading, trailing or double separators
  fn to_segments(&self, separator: &str) -> Vec<String>;

  /// Extract a vector of strings from a string-like object with a given separator
  fn to_parts(&self, separator: &str) -> Vec<String>;

  /// Extract only the head before the first occurrence of a separator
  fn to_head(&self, separator: &str) -> String;

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> String;

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> String;

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> String;

  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> String;

  /// Extract only the last segment
  fn to_end(&self, separator: &str) -> String;

  /// Extract a string-like segment identified by its index from the components of a string with a given separator
  /// e.g. String::from("10/11/2024") .to_segment(1) yields "11"
  fn to_segment(&self, separator: &str, index: i32) -> Option<String>;

  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String>;

  /// extract the remainder after the head
  fn to_tail(&self, separator: &str) -> String;

  /// extract the first and last parts after the first occurrence of the separator
  fn to_head_tail(&self, separator: &str) -> (String, String);

  /// extract the first and last parts after the last occurrence of the separator
  fn to_start_end(&self, separator: &str) -> (String, String);

}

/// Implement string segment split and capture method for String
impl ToSegments for str {

  /// Splits a string on the exact separator, whether initial, final or repeated.
  /// May yield empty segments
  fn to_parts(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }

  /// Splits a string on a separator, but only returns an array of non-empty strings
  /// skipping leading, trailing or repeated separators that may otherwise yield empty strings
  fn to_segments(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).filter(|s| s.len() > 0).collect::<Vec<String>>()
  }

  fn to_head(&self, separator: &str) -> String {
    if let Some((head, _tail)) = self.split_once(separator) {
      head.to_string()
    } else {
      self.to_owned()
    }
  }

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.ends_with(separator) && self.len() > separator_len {
      let end_index = self.len() - separator_len;
      self[0..end_index].to_string().to_end(separator)
    } else {
      self.to_end(separator)
    }
  }

  /// extract the last segment whether empty or not
  fn to_end(&self, separator: &str) -> String {
    let parts = self.to_parts(separator);
    if parts.len() > 0 {
      parts.last().unwrap_or(&self.to_string()).to_owned()
    } else {
      self.to_owned()
    }
  }

  fn to_tail(&self, separator: &str) -> String {
    let parts = self.to_parts(separator);
    let num_parts = parts.len();
    if num_parts > 0 {
      parts[1..num_parts].join(separator)
    } else {
      self.to_owned()
    }
  }

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.starts_with(separator) && self.len() > separator_len {
      self[separator_len..self.len()].to_string().to_head(separator)
    } else {
      self.to_head(separator)
    }
  }

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.starts_with(separator) && self.len() > separator_len {
      self[separator_len..].to_string().to_tail(separator)
    } else {
      self.to_tail(separator)
    }
  }
  
  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.ends_with(separator) && self.len() > separator_len {
      let end_index = self.len() - separator_len;
      self[0..end_index].to_string().to_tail(separator)
    } else {
      self.to_tail(separator)
    }
  }

  /// Extract an indexed segment yielded by splitting a string. 
  /// A negative index parameter will start from the end 
  fn to_segment(&self, separator: &str, index: i32) -> Option<String> {
    let parts = self.to_segments(separator);
    let num_parts = parts.len();
    let target_index = if index >= 0 { index as usize } else { (num_parts as i32 + index) as usize };
    if target_index < num_parts {
      if let Some(segment) = parts.get(target_index) {
        Some(segment.to_owned())
      } else {
        None
      }
    } else {
      None
    }
  }

  /// extract inner segment via a set of tuples with separators and indices.
  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String> {
    if groups.len() > 0 {
      let mut matched: Option<String> = None;
      let mut current_string = self.to_string();
      for group in groups {
        if current_string.len() > 0 {
          let (separator, index) = group;
          matched = current_string.to_segment(*separator, *index);
          current_string = matched.clone().unwrap_or("".to_string());
        }
      }
      matched
    } else {
      None
    }
  }

  /// 
  /// Extract a tuple of the head and remainder, like split_once but returns Strings
  fn to_head_tail(&self, separator: &str) -> (String, String) {
    if let Some((head, tail)) = self.split_once(separator) {
      (head.to_string(), tail.to_string())
    } else {
      ("".to_owned(), self.to_owned())
    }
  }

  /// 
  /// Extract a tuple of the tail and remainder, like split_once in reverse and returning strings
  fn to_start_end(&self, separator: &str) -> (String, String) {
    let parts = self.to_parts(separator);
    let last_part = "".to_string();
    let num_parts = parts.len();
    if num_parts > 0 {
      let end_index = num_parts - 1;
      let start = parts[0..end_index].join(separator);
      let end = self.to_end(separator);
      (start, end)
    } else {
      (self.to_owned(), last_part)
    }
  }

}


/// Set of methods to capture groups or match objects derived from Regex::captures.
pub trait PatternCapture {

  /// Yields an option with Regex::Captures as returned from re.captures, Accepts a boolean case_insensitive flag
  fn pattern_captures(&self, pattern: &str, case_insensitive: bool) -> Option<Captures>;

  /// Yields a vector of Match objects with start and end index + the captured string. Accepts a boolean case_insensitive flag
  fn pattern_matches_vec(&self, pattern: &str, case_insensitive: bool) -> Vec<Match>;

  /// Yields an option with first match object if available with a boolean case_insensitive flag
  fn pattern_first_match(&self, pattern: &str, case_insensitive: bool) -> Option<Match>;

  /// Yields an option with last match object if available with a boolean case_insensitive flag
  fn pattern_last_match(&self, pattern: &str, case_insensitive: bool) -> Option<Match>;

  /// returns an option with a pair of match objects
  /// If there is only one match the match objects will have the same indices
  fn pattern_first_last_matches(&self, pattern: &str, case_insensitive: bool) -> Option<(Match, Match)>;

  /// Yields an option with an unsigned integer for the index of the start of the first match
  /// with a boolean case_insensitive flag
  fn pattern_first_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize>;

  /// Yields an option with an unsigned integer for the index of the end of the first match
  /// with a boolean case_insensitive flag
  fn pattern_first_end_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize>;

  /// Yields an option with an unsigned integer for the index of the start of the last match
  /// with a boolean case_insensitive flag
  fn pattern_last_start_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize>;

  /// Yields an option with an unsigned integer for the index of the end of the last match
  /// with a boolean case_insensitive flag
  fn pattern_last_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize>;

  // Counts the number of matches with a boolean case_insensitive flag
  fn count_pattern(&self, pattern: &str, case_insensitive: bool) -> usize;

  // Counts the number of whole words with a boolean case_insensitive flag
  fn count_word(&self, word: &str, case_insensitive: bool) -> usize;
}

impl PatternCapture for str {

  // Yields an option with Regex::Captures as returned from re.captures, Accepts a boolean case_insensitive flag
  fn pattern_captures(&self, pattern: &str, case_insensitive: bool) -> Option<Captures> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      re.captures(self)
    } else {
      None
    }
  }

  /// Yields a vector of Match objects with start and end index + the captured string. Accepts a boolean case_insensitive flag
  fn pattern_matches_vec(&self, pattern: &str, case_insensitive: bool) -> Vec<Match> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      let mut matched_items: Vec<Match> = Vec::new();
      for capture in re.captures_iter(self)  {
        for matched_opt in capture.iter() {
          if let Some(matched_item) = matched_opt {
            matched_items.push(matched_item);
          }
        }
      }
      matched_items
    } else {
      vec![]
    }
  }

  /// Yields an option with first match object if available with a boolean case_insensitive flag
  /// As this uses re.find it will be fast than the matching last_match method
  fn pattern_first_match(&self, pattern: &str, case_insensitive: bool) -> Option<Match> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      re.find(self)
    } else {
      None
    }
  }

  /// Yields an option with last match object if available with a boolean case_insensitive flag
  fn pattern_last_match(&self, pattern: &str, case_insensitive: bool) -> Option<Match> {
    let matched_segments = self.pattern_matches_vec(pattern, case_insensitive);
    if let Some(last) = matched_segments.last() {
      Some(*last)
    } else {
      None
    }
  }

  /// returns an option with a pair of match objects
  /// If there is only one match the match objects will have the same indices
  fn pattern_first_last_matches(&self, pattern: &str, case_insensitive: bool) -> Option<(Match, Match)> {
    let matched_segments = self.pattern_matches_vec(pattern, case_insensitive);
    if let Some(first) = matched_segments.get(0) {
      if let Some(last) = matched_segments.last() {
        return Some((*first, *last));
      }
    }
    None
  }

  /// Yields an option with an unsigned integer for the index of the start of the last match
  /// with a boolean case_insensitive flag
  fn pattern_first_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.start())
    } else {
      None
    }
  }

  /// Yields an option with an unsigned integer for the index of the end of the first match
  /// with a boolean case_insensitive flag
  fn pattern_first_end_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.end())
    } else {
      None
    }
  }

  /// Yields an option with an unsigned integer for the index of the start of the last match
  /// with a boolean case_insensitive flag
  fn pattern_last_start_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.start())
    } else {
      None
    }
  }

  // Yields an option with an unsigned integer for the index of the end of the last match
  /// with a boolean case_insensitive flag
  fn pattern_last_index(&self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.end())
    } else {
      None
    }
  }

  // Counts the number of matches with a boolean case_insensitive flag
  fn count_pattern(&self, pattern: &str, case_insensitive: bool) -> usize {
    self.pattern_matches_vec(pattern, case_insensitive).len()
  }

  // Counts the number of matches with a boolean case_insensitive flag
  fn count_word(&self, word: &str, case_insensitive: bool) -> usize {
    let pattern = build_whole_word_pattern(word);
    self.pattern_matches_vec(&pattern, case_insensitive).len()
  }

}

/// Provides methods to match words with differnt word boundary and case-semsitivity rules 
pub trait MatchWord {

  /// Match a word with bounds options and case_insensitive flag
  fn match_word_bounds(&self, word: &str, bounds: WordBounds, case_insensitive: bool) -> bool;

  /// Match a whole word with a case_insensitive flag
  fn match_word(&self, word: &str, case_insensitive: bool) -> bool;

  /// Match any whole words only with a boolean case_insensitive flag
  fn match_any_words(&self, words: &[&str], case_insensitive: bool) -> bool;

  /// Match from the start of a word with a case_insensitive flag
  fn match_word_start(&self, word: &str, case_insensitive: bool) -> bool;

  /// Match to the end of a word with a case_insensitive flag
  fn match_word_end(&self, word: &str, case_insensitive: bool) -> bool;

  /// Match a whole word in case-insensitive mode
  fn match_word_ci(&self, word: &str) -> bool;

  /// Match any whole words only in case-insensitive mode
  fn match_any_words_ci(&self, words: &[&str]) -> bool;

  /// Match from the start of a word in case-insensitive mode
  fn match_word_start_ci(&self, word: &str) -> bool;

  /// Match to the end of a word in case-insensitive mode
  fn match_word_end_ci(&self, word: &str) -> bool;

  /// Match a whole word in case-sensitive mode
  fn match_word_cs(&self, word: &str) -> bool;

  /// Match any whole words only in case-sensitive mode
  fn match_any_words_cs(&self, words: &[&str]) -> bool;

  /// Match from the start of a word in case-sensitive mode
  fn match_word_start_cs(&self, word: &str) -> bool;

  /// Match to the end of a word in case-sensitive mode
  fn match_word_end_cs(&self, word: &str) -> bool;

  /// Match a word pair by min and max proximity, where a negative min value implies before the end of the first word
  fn match_words_by_proximity(&self, first: &str, second: &str, min: i16, max: i16, case_insensitive: bool) -> bool;

  /// Count matched words from array with boundary and case_insensitive options
  fn count_matched_words_bounds(&self, words: &[&str], bounds: WordBounds, case_insensitive: bool) -> usize;

  /// Match all words in array with boundary and case_insensitive options
  fn match_words_bounds(&self, words: &[&str], bounds: WordBounds, case_insensitive: bool) -> bool;

  /// Match all whole words only with a boolean case_insensitive flag
  fn match_words(&self, words: &[&str], case_insensitive: bool) -> bool;

  /// Match all whole words in case-insensitive mode
  fn match_words_ci(&self, words: &[&str]) -> bool;

  /// Match all whole words in case-sensitive mode
  fn match_words_cs(&self, words: &[&str]) -> bool;

  /// Match sets of words with positivity, pattern and case_insensitive parameters in tuples
  /// e.g. to match sentences with cat(s) but not dog(s) (lower case only)
  /// let sets = [(true, "cats?", true), (false, "dogs?", false)]; 
  fn match_words_sets_conditional(&self, sets: &[(bool, &str, bool)]) -> bool;

  /// Match sets of words with positivity and pattern tuple in case-insensitive mode
  /// e.g. to match sentences with cat(s) but not dog(s) (lower case only)
  /// let sets = [(true, "cats?"), (false, "dogs?")];
  fn match_words_sets_conditional_ci(&self, tuples: &[(bool, &str)]) -> bool;

}

/// Implement MatchWord methods for str and String
impl MatchWord for str {

  /// Match a word with bounds options and case_insensitive flag
  fn match_word_bounds(&self, word: &str, bounds: WordBounds, case_insensitive: bool) -> bool {
    let word_pattern = bounds.to_pattern(word);
    self.pattern_match(&word_pattern, case_insensitive)
  }

  /// Case-conditional match of a whole word
  /// To match only the start or end, use the start and end methods or expand the pattern with \w* at either end
  fn match_word(&self, word: &str, case_insensitive: bool) -> bool {
    let pattern = build_whole_word_pattern(word);
    self.pattern_match(&pattern, case_insensitive)
  }

  /// Case-conditional match from the start of a word boundary
  fn match_word_start(&self, word: &str, case_insensitive: bool) -> bool {
    let word_pattern = build_word_pattern(word, WordBounds::Start);
    self.pattern_match(&word_pattern, case_insensitive)
  }

  /// Case-conditional match to the end of a word boundary
  fn match_word_end(&self, word: &str, case_insensitive: bool) -> bool {
    let word_pattern = build_word_pattern(word, WordBounds::End);
    self.pattern_match(&word_pattern, case_insensitive)
  }

  /// Case-insensitive whole word match, for words with optional hyphens use -?, e.g. hip-?hop matches hip-hop and hiphop, but not hip-hopping
  /// To match only the start or end, use the start and end methods or expand the pattern with \w* at either end
  fn match_word_ci(&self, word: &str) -> bool {
    self.match_word(word, true)
  }

  /// Case-insensitive match from the start of a word boundary
  fn match_word_start_ci(&self, word: &str) -> bool {
    self.match_word_start(word, true)
  }

  /// Case-insensitive match to the end of a word boundary
  fn match_word_end_ci(&self, word: &str) -> bool {
    self.match_word_end(word, true)
  }

  /// Case-sensitive whole word match, for words with optional hyphens use -?, e.g. hip-?hop matches hip-hop and hiphop, but not hip-hopping
  /// To match only the start or end, use the start and end methods or expand the pattern with \w* at either end
  fn match_word_cs(&self, word: &str) -> bool {
    self.match_word(word, false)
  }

  /// Case-sensitive match from the start of a word boundary
  fn match_word_start_cs(&self, word: &str) -> bool {
    self.match_word_start(word, false)
  }

  /// Case-sensitive match to the end of a word boundary
  fn match_word_end_cs(&self, word: &str) -> bool {
    self.match_word_end(word, false)
  }

  /// Check if whole word patterns occur in close proximity as defined by their min and max values
  /// If the second word may occur before the first the min value should negative
  /// The distance between the end of the first and start of the second word is measured
  fn match_words_by_proximity(&self, first: &str, second: &str, min: i16, max: i16, case_insensitive: bool) -> bool {
    let word_pattern_1 = build_whole_word_pattern(first);
    let word_pattern_2 = build_whole_word_pattern(second);
    if let Some((first_first,first_last)) = self.pattern_first_last_matches(&word_pattern_1, case_insensitive) {
      if let Some((second_first, second_last)) = self.pattern_first_last_matches(&word_pattern_2, case_insensitive) {
        let diff_i64 = second_last.start() as i64 - first_first.end() as i64;
        // although indices are usize and convert to i64 for negative values, only consider differences in i16 range (-32768 to 32767)
        // which suffices for text proximity matches
        if diff_i64 >= i16::MIN as i64 && diff_i64 <= i16::MAX as i64 {
          let diff = diff_i64 as i16;
          return diff >= min && diff <= max;
        } else if min < 0 {
          // reverse match logic if negative min offsets are allowed
          let diff_2_i64 = first_last.start() as i64 - second_first.end() as i64;
          if diff_2_i64 >= i16::MIN as i64 && diff_2_i64 <= i16::MAX as i64 {
            let diff_2 = diff_i64 as i16;
            return diff_2 >= min && diff_2 <= max;
          }
        }
      }
    }
    false
  }

  /// Count matched words from an array of strs with boundary and case_insensitive options
  fn count_matched_words_bounds(&self, words: &[&str], bounds: WordBounds, case_insensitive: bool) -> usize {
    let mut num_matched = 0;
    for word in words {
      let pattern = bounds.to_pattern(word);
      if self.pattern_match(&pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched
  }

  /// Match all words in array with boundary and case_insensitive options
  fn match_words_bounds(&self, words: &[&str], bounds: WordBounds, case_insensitive: bool) -> bool {
    words.len() == self.count_matched_words_bounds(words, bounds, case_insensitive)
  }

  /// Match all whole words only with a boolean case_insensitive flag
  fn match_words(&self, words: &[&str], case_insensitive: bool) -> bool {
    self.match_words_bounds(words, WordBounds::Both, case_insensitive)
  }

  /// Match any whole words only with a boolean case_insensitive flag
  fn match_any_words(&self, words: &[&str], case_insensitive: bool) -> bool {
    let pattern = build_optional_whole_word_pattern(words);
    self.pattern_match(&pattern, case_insensitive)
  }

  /// Match all whole words in case-insensitive mode
  fn match_words_ci(&self, words: &[&str]) -> bool {
    self.match_words_bounds(words, WordBounds::Both, true)
  }

  /// Match any whole words only in case-insensitive mode
  fn match_any_words_ci(&self, words: &[&str]) -> bool {
    let pattern = build_optional_whole_word_pattern(words);
    self.pattern_match(&pattern, true)
  }

  /// Match all whole words in case-sensitive mode
  fn match_words_cs(&self, words: &[&str]) -> bool {
    self.match_words_bounds(words, WordBounds::Both, false)
  }

  /// Match any whole words only in case-sensitive mode
  fn match_any_words_cs(&self, words: &[&str]) -> bool {
    let pattern = build_optional_whole_word_pattern(words);
    self.pattern_match(&pattern, false)
  }

  /// Match sets of words with positivity, pattern and case_insensitive parameters in tuples
  /// e.g. to match sentences with cat(s) but not dog(s) (lower case only)
  /// let sets = [(true, "cats?", true), (false, "dogs?", false)]; 
  fn match_words_sets_conditional(&self, sets: &[(bool, &str, bool)]) -> bool {
    let num_words = sets.len();
    let mut num_matched = 0;
    for row in sets {
      let (is_positive, word, case_insensitive) = *row;
      let pattern = build_whole_word_pattern(word);
      if self.pattern_match(&pattern, case_insensitive) == is_positive {
        num_matched += 1;
      }
    }
    num_matched == num_words
  }

  /// Match sets of words with positivity and pattern tuple in case-insensitive mode
  /// e.g. to match sentences with cat(s) but not dog(s) (lower case only)
  /// let sets = [(true, "cats?"), (false, "dogs?")];
  fn match_words_sets_conditional_ci(&self, tuples: &[(bool, &str)]) -> bool {
    let num_words = tuples.len();
    let mut num_matched = 0;
    for row in tuples {
      let (is_positive, word) = *row;
      let pattern = build_whole_word_pattern(word);
      if self.pattern_match(&pattern, true) == is_positive {
        num_matched += 1;
      }
    }
    num_matched == num_words
  }

}

/// Methods for whole or partial word replacements
pub trait ReplaceWord {

  /// Replace words with boundary and case_insensitive options
  fn replace_word_bounds(&self, word: &str, replacement: &str, bounds: WordBounds, case_insensitive: bool) -> Self where Self:Sized;

  /// Replace whole words with case_insensitive options
  fn replace_word(&self, word: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;

  /// Replace whole words with in case-insensitive mode
  fn replace_word_ci(&self, word: &str, replacement: &str) -> Self where Self:Sized;

  /// Replace whole words with in case-sensitive mode
  fn replace_word_cs(&self, word: &str, replacement: &str) -> Self where Self:Sized;

  /// Replace one or pairs of whole words with a boolean case_insensitive flag
  fn replace_words(&self, pairs: &[(&str, &str)], case_insensitive: bool) -> Self where Self:Sized;

  /// Replace one or pairs of whole words in case-insensitive mode
  fn replace_words_ci(&self, pairs: &[(&str, &str)]) -> Self where Self:Sized;

  /// Replace one or pairs of whole words in case-sensitive mode
  fn replace_words_cs(&self, pairs: &[(&str, &str)]) -> Self where Self:Sized;

  /// Replace one or sets of whole words with case_insensitive flags as the last tuple element
  fn replace_word_sets(&self, pairs: &[(&str, &str, bool)]) -> Self where Self:Sized;

}


/// Methods for whole or partial word replacements
impl ReplaceWord for String {

  /// Replace words with boundary and case_insensitive options
  fn replace_word_bounds(&self, word: &str, replacement: &str, bounds: WordBounds, case_insensitive: bool) -> String {
    let pattern = build_word_pattern(word, bounds);
    self.pattern_replace(&pattern, replacement, case_insensitive)
  }

  /// Replace whole words with case_insensitive options
  fn replace_word(&self, word: &str, replacement: &str, case_insensitive: bool) -> String {
    let pattern = build_whole_word_pattern(word);
    self.pattern_replace(&pattern, replacement, case_insensitive)
  }
  /// Replace whole words with in case-insensitive mode
  fn replace_word_ci(&self, word: &str, replacement: &str) -> String {
    let pattern = build_whole_word_pattern(word);
    self.pattern_replace(&pattern, replacement, true)
  }

  /// Replace whole words with in case-sensitive mode
  fn replace_word_cs(&self, word: &str, replacement: &str) -> String {
    let pattern = build_whole_word_pattern(word);
    self.pattern_replace(&pattern, replacement, false)
  }

  /// Replace one or pairs of whole words with a boolean case_insensitive flag
  fn replace_words(&self, pairs: &[(&str, &str)], case_insensitive: bool) -> String {
    let mut output = self.clone();
    for pair in pairs {
      let (word, replacement) = *pair;
      let pattern = build_whole_word_pattern(word);
      output = output.pattern_replace(&pattern, replacement, case_insensitive);
    }
    output
  }

  /// Replace one or pairs of whole words in case-insensitive mode
  fn replace_words_ci(&self, pairs: &[(&str, &str)]) -> String {
    self.replace_words(pairs, true)
  }

  /// Replace one or pairs of whole words in case-sensitive mode
  fn replace_words_cs(&self, pairs: &[(&str, &str)]) -> String {
    self.replace_words(pairs, false)
  }

  /// Replace one or sets of whole words with case_insensitive flags as the last tuple element
  fn replace_word_sets(&self, tuples: &[(&str, &str, bool)]) -> String {
    let mut output = self.clone();
    for row in tuples {
      let (word, replacement, case_insensitive) = *row;
      let pattern = build_whole_word_pattern(word);
      output = output.pattern_replace(&pattern, replacement, case_insensitive);
    }
    output
  }

}

