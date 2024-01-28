use regex::Error;
use crate::utils::build_regex;


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

/// Boolean methods to match a pattern within an array of strings
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

/// Multiple match methods for arrays or vectors of strings
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
