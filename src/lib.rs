extern crate regex;
mod utils;

use std::str::FromStr;
use regex::*;
use utils::*;

pub trait PatternMatch {
  /// Apply a regular expression match on the current String-like object
  /// If the regex doesn't compile it will return an error
  /// 
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error>;

  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool;

  fn pattern_match_ci(&self, pattern: &str) -> bool;

  fn pattern_match_cs(&self, pattern: &str) -> bool;

}

/// Core regular expression replacement methods 
pub trait PatternReplace {
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;

  fn pattern_replace_result(&self, pattern: &str, replacement: &str,case_insensitive: bool) -> Result<Self, Error> where Self:Sized;

  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> Self where Self:Sized;

  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> Self where Self:Sized;

}

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

  /// Correct number
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> Self;

  /// Extracts the first valid integer or float from a longer string if present
  fn to_first_number<T: FromStr + Copy>(&self) -> Option<T>;

  /// Extracts the first valid integer or float from a longer string
  /// if commas are used for decimals and dots for thousand separators
  fn to_first_number_euro<T: FromStr + Copy>(&self) -> Option<T>;

  /// Removes all characters no used in valid numeric sequences
  fn strip_non_numeric(&self) -> Self where Self:Sized;

}

/// Methods to validate strings with character classes
pub trait CharGroupMatch {
  /// Does the string contain any digits
  fn has_digits(&self) -> bool;

  /// Does the string contain any alphanumeric characters including those from non-Latin alphabets
  fn has_alphanumeric(&self) -> bool;

  /// Does the string contain any letters including those from non-Latin alphabets, but excluding digits
  fn has_alphabetic(&self) -> bool;
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
  fn is_numeric(&self) -> bool;
}

impl IsNumeric for String {

  /// Check if the string may be parsed to a number
  fn is_numeric(&self) -> bool {
      self.pattern_match(r#"^-?(\d+,)*\d+((\.)\d+)?"#, false)
  }
}

pub trait ToSegments {

  /// Extract a vector of non-empty strings from a string-like object with a given separator
  /// excluding leading, trailing or double separators
  fn to_segments(&self, separator: &str) -> Vec<Self> where Self:Sized;

  /// Extract a vector of strings from a string-like object with a given separator
  fn to_parts(&self, separator: &str) -> Vec<Self> where Self:Sized;

  /// Extract only the head before the first occurrence of a separator
  fn to_head(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the last segment
  fn to_end(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract a string-like segment identified by its index from the components of a string with a given separator
  /// e.g. String::from("10/11/2024") .to_segment(1) yields "11"
  fn to_segment(&self, separator: &str, index: i32) -> Option<Self>  where Self:Sized;

  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<Self>  where Self:Sized;

  /// extract the remainder after the head
  fn to_tail(&self, separator: &str) -> Self where Self:Sized;

  /// extract the first and last parts after the first occurrence of the separator
  fn to_head_tail(&self, separator: &str) -> (Self, Self)  where Self:Sized;

  /// extract the first and last parts after the last occurrence of the separator
  fn to_start_end(&self, separator: &str) -> (Self, Self)  where Self:Sized;

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
  fn to_strings(&self) -> Vec<String> {
      self.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }
}

/// Return the indices of all ocurrences of a character
pub trait MatchOccurrences {
  fn find_matched_indices(&self, pat: &str) -> Vec<usize>;
}


impl MatchOccurrences for String {
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


impl PatternReplace for String {
  ///
  /// Optional regex-enabledd replace method that will return None if the regex fails
  /// 
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<String, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(re.replace_all(self, replacement).to_string()),
      Err(error) => Err(error)
    }
  }

  ///
  /// Simple regex-enabledd replace method that will return the same string if the regex fails
  /// 
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> String {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

  ///
  /// Simple case-insensitive regex-enabledd replace method that will return the same string if the regex fails
  /// 
  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> String {
    self.pattern_replace(pattern, replacement, true)
  }

  ///
  /// Simple case-sensitive regex-enabledd replace method that will return the same string if the regex fails
  /// 
  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> String {
    self.pattern_replace(pattern, replacement, false)
  }

}

impl StripCharacters for String {
    

  fn strip_non_alphanum(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_alphanumeric()).collect::<String>()
  }

  fn strip_non_digits(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_digit(10)).collect::<String>()
  }

  /// Correct numeric strings with commas as thousand separators or as decimal separators
  /// to a regular format with punctuation only for decimal points before being parsed to an integer or float
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
        self.to_owned()
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

impl CharGroupMatch for str {

  fn has_digits(&self) -> bool {
      self.chars().any(|c| char::is_digit(c, 10))
  }

  fn has_alphanumeric(&self) -> bool {
      self.chars().any(char::is_alphanumeric)
  }

  fn has_alphabetic(&self) -> bool {
    self.chars().any(char::is_alphabetic)
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
  /// e.g. (r#"a[ck]"#, true) => matches "ac" or "ak" whether upper, lower or mixed case
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
  /// e.g. (r#"a[ck]"#, true) => matches "ac" or "ak" whether upper, lower or mixed case
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
  /// e.g. (false, "a[ck]", true) => does not contain "ac" or "ak" whether upper, lower or mixed case
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
  /// e.g. (r#"a[ck]"#, true) => matches "ac" or "ak" whether upper, lower or mixed case
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

  fn pattern_match_many_ci(&self, patterns: &[&str]) -> bool {
    self.pattern_match_many(patterns, true)
  }

  fn pattern_match_many_cs(&self, patterns: &[&str]) -> bool {
    self.pattern_match_many(patterns, false)
  }

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
  /// e.g. (r#"a[ck]"#, true) => matches "ac" or "ak" whether upper, lower or mixed case
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

impl PatternReplaceMany for Vec<String> {
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

impl ToSegments for String {

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
      parts.last().unwrap_or(self).to_owned()
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
      let mut current_string = self.clone();
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
