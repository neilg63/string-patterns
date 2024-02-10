use regex::Error;
use crate::utils::build_regex;

/// Core regular expression match methods
pub trait PatternMatch {
  /// Apply a regular expression match on the current string
  /// If the regex doesn't compile it will return an error
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error>;

  /// Apply a regular expression match on the current string with a boolean case_insensitive flag
  /// NB: If the regex doesn't compile it will return false
  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool {
    if let Ok(matched) = self.pattern_match_result(pattern, case_insensitive){
      matched
    } else {
      false
    }
  }

  /// if the pattern does not match the source string or the regex fails
  fn pattern_match_ci(&self, pattern: &str) -> bool {
    self.pattern_match(pattern, true)
  }

  /// Simple case-sensitive regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  fn pattern_match_cs(&self, pattern: &str) -> bool {
    self.pattern_match(pattern, false)
  }

}

/// Implement regular expression match and replace methods for str and owned String
impl PatternMatch for str {

  ///
  /// Simple regex-compatible match method that will return an optional boolean 
  /// - Some(true) means the regex is valid and the string matches
  /// - Some(false) means the regex is valid and the string does not match
  /// - None means the regex is not valid and can this not be evaluated
  /// Only the pattern_match_result needs to be implemented
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re)  => Ok(re.is_match(self)),
      Err(error) => Err(error)
    }
  }
}

/// Boolean methods to match a pattern within an array of strings
impl PatternMatch for [&str] {
  /// The regex is only compiled when validating an array of strings
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().any(|segment| re.is_match(*segment))),
      Err(error) => Err(error)
    }
  }
}

/// Boolean methods to match a pattern within an array of strings
impl PatternMatch for [String] {
  /// The regex is only compiled when validating an array of strings
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().any(|segment| re.is_match(segment))),
      Err(error) => Err(error)
    }
  }
}

/// Pattern methods for arrays or vectors only, return vectors of booleans matching each input string
pub trait PatternMatches {

  /// Returns result with a vector of tuples with matched status and sampled string reference
  /// for an array or vector of strings with a case-insensitive flag
  /// or an error if the regex does not compile
  fn pattern_matched_pairs_result(&self, pattern: &str, case_insensitive: bool) -> Result<Vec<(bool, &str)>, Error>;


  /// Return a default vector of paired tuples if the regular expression fails in pattern_matched_pairs or pattern_matches
  /// This has to implemented separately for all other derived methods returning vectors to work correctly
  fn pattern_matched_pairs_default(&self) -> Vec<(bool, &str)>;

  /// Returns a vector of tuples with matched status and sampled string reference
  /// for an array or vector of strings with a case-insensitive flag
  /// If the regular expression fails all items will be false
  fn pattern_matched_pairs(&self, pattern: &str, case_insensitive: bool) -> Vec<(bool, &str)> {
    match self.pattern_matched_pairs_result(pattern, case_insensitive) {
      Ok(results) => results,
      Err(_error) => self.pattern_matched_pairs_default()
    }
  }

  /// Returns result with a vector of boolean matches for an array or vector of strings with case-insensitive flag
  /// or an error if the regex does not compile
  fn pattern_matches_result(&self, pattern: &str, case_insensitive: bool) -> Result<Vec<bool>, Error> {
    match self.pattern_matched_pairs_result(pattern, case_insensitive) {
      Ok(items) => Ok(items.into_iter().map(|(result, _item)| result).collect::<Vec<bool>>()),
      Err(error) => Err(error)
    }
  }

  /// Returns a filtered vector of matched string references (&str) with case-insensitive flag
  fn pattern_matches_filtered(&self, pattern: &str, case_insensitive: bool) -> Vec<&str> {
    self.pattern_matched_pairs(pattern, case_insensitive).into_iter().filter(|(is_matched, _item)| *is_matched).map(|(_is_matched, item)| item).collect()
  }

  /// Returns a filtered vector of matched string references (&str) in case-insensitive mode
  fn pattern_matches_filtered_ci(&self, pattern: &str) -> Vec<&str> {
    self.pattern_matched_pairs(pattern, true).into_iter().filter(|(is_matched, _item)| *is_matched).map(|(_is_matched, item)| item).collect()
  }

  /// Returns a filtered vector of matched string references (&str) in case-sensitive mode
  fn pattern_matches_filtered_cs(&self, pattern: &str) -> Vec<&str> {
    self.pattern_matched_pairs(pattern, false).into_iter().filter(|(is_matched, _item)| *is_matched).map(|(_is_matched, item)| item).collect()
  }


  /// Returns vector of boolean matches for an array or vector of strings with case-insensitive flag
  /// must be reimplemented from pattern_matches_result owing to trait bound constraints on unsized arrays
  fn pattern_matches(&self, pattern: &str, case_insensitive: bool) -> Vec<bool> {
    self.pattern_matched_pairs(pattern, case_insensitive).into_iter().map(|(matched, _item)| matched).collect()
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

/// Multiple match methods for arrays or vectors of &str values
impl PatternMatches for [&str] {

  /// Returns an Ok result with a vector of boolean matches for an array or vector of strings with a case-insensitive flag
  /// and an error only if the regex fails to compile.
  fn pattern_matched_pairs_result(&self, pattern: &str, case_insensitive: bool) -> Result<Vec<(bool, &str)>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().map(|segment| (re.is_match(*segment), *segment)).collect::<Vec<(bool, &str)>>()),
      Err(error) => Err(error)
    }
  }

  // Implement default vector of (bool, &str) results for [&str]
  fn pattern_matched_pairs_default(&self) -> Vec<(bool, &str)> {
    self.into_iter().map(|item| (false, *item)).collect()
  }

}

/// Multiple match methods for arrays or vectors of strings
/// Implemented separately because of irresolvable Iterator trait bounds rules in [Rust 2018](https://doc.rust-lang.org/stable/edition-guide/rust-2021/IntoIterator-for-arrays.html)
/// and because both String and &str are normalised to vectors with string references in the return types
impl PatternMatches for [String] {

  /// Returns an Ok result with a vector of boolean matches for an array or vector of strings with a case-insensitive flag
  /// and an error only if the regex fails to compile.
  fn pattern_matched_pairs_result(&self, pattern: &str, case_insensitive: bool) -> Result<Vec<(bool, &str)>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().map(|segment| (re.is_match(segment), segment.as_str())).collect::<Vec<(bool, &str)>>()),
      Err(error) => Err(error)
    }
  }

  // Implement default vector of (bool, &str) results for [String]
  fn pattern_matched_pairs_default(&self) -> Vec<(bool, &str)> {
    self.into_iter().map(|item| (false, item.as_str())).collect()
  }
  
}