use regex::Error;
use crate::utils::build_regex;

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
  fn pattern_split(&self, pattern: &str, case_sensitive: bool) -> Vec<String> {
    match self.pattern_split_result(pattern, case_sensitive) {
      Ok(parts) => parts,
      Err(_error) => vec![],
    }
  }

  /// Splits a string on a regular expression in case-isensitive mode. 
  /// Returns  a vector of strings, empty if the regular expression fails.
  fn pattern_split_ci(&self, pattern: &str) -> Vec<String> {
    self.pattern_split(pattern, true)
  }

  /// Splits a string on a regular expression in case-sensitive mode. 
  /// Returns  a vector of strings, empty if the regular expression fails.
  fn pattern_split_cs(&self, pattern: &str) -> Vec<String> {
    self.pattern_split(pattern, false)
  }

  /// Splits a string on a regular expression with boolean case_insensitive flag. 
  /// Returns a tuple with head and tail. The tail will be en empty string if not matched
  fn pattern_split_pair(&self, pattern: &str, case_sensitive: bool) -> (String, String) {
    match self.pattern_split_pair_result(pattern, case_sensitive) {
      Ok(parts) => parts,
      Err(_error) => ("".to_owned(), "".to_owned()),
    }
  }

  /// Split a string on a regular expression in case-isensitive mode. 
  /// Returns a tuple with head and tail. The tail will be en empty string if not matched
  fn pattern_split_pair_ci(&self, pattern: &str) -> (String, String) {
    self.pattern_split_pair(pattern, true)
  }

  /// Split a string on a regular expression in case-sensitive mode. 
  /// Returns a tuple with head and tail. The tail will be en empty string if not matched
  fn pattern_split_pair_cs(&self, pattern: &str) -> (String, String) {
    self.pattern_split_pair(pattern, false)
  }

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

}
