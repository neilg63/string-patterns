use regex::Error;
use crate::utils::build_regex;
use std::borrow::ToOwned;

/// Core regular expression replacement methods 
pub trait PatternReplace where Self:Sized {

  /// Replace all matches of the pattern within a longer text with a boolean case_insensitive flag
  /// NB: If the regex doesn't compile it will return an Error, otherwise in Ok result.
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_result(&self, pattern: &str, replacement: &str,case_insensitive: bool) -> Result<Self, Error> where Self:Sized;

  /// Replace only the first match of the pattern within a longer text with a boolean case_insensitive flag
  /// NB: If the regex doesn't compile it will return an Error, otherwise in Ok result.
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_first_result(&self, pattern: &str, replacement: &str,case_insensitive: bool) -> Result<Self, Error> where Self:Sized;

  /// Replace all matches of the pattern within a longer text with a boolean case_insensitive flag
  /// Returns a copy of the same data type. If the regex fails, nothing will be replaced.
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;


  /// Replace only the first match of the pattern within a longer text with a boolean case_insensitive flag
  /// Returns a copy of the same data type. If the regex fails, nothing will be replaced.
  fn pattern_replace_first(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;


  /// Replace all matches of the pattern within a longer text in case-insensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> Self where Self:Sized {
    self.pattern_replace(pattern, replacement, true)
  }

  /// Replace all matches of the pattern within a longer text in case-sensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> Self where Self:Sized {
    self.pattern_replace(pattern, replacement, false)
  }

  /// Replace the first match only of the pattern within a longer text in case-insensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_first_ci(&self, pattern: &str, replacement: &str) -> Self where Self:Sized {
    self.pattern_replace_first(pattern, replacement, true)
  }

  /// Replace the first match only of the pattern within a longer text in case-sensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_first_cs(&self, pattern: &str, replacement: &str) -> Self where Self:Sized {
    self.pattern_replace_first(pattern, replacement, false)
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

  /// Regex-enabled replace method that will return an OK String result if successful and an error if the regex fails
  fn pattern_replace_first_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<String, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(re.replace(self, replacement).to_string()),
      Err(error) => Err(error)
    }
  }

  /// Simple regex-enabled replace-all method that will return the same string if the regex fails
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> String {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

  /// Regex-enabled single replace method that will return an OK String result if successful and an error if the regex fails
  fn pattern_replace_first(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> String {
    self.pattern_replace_first_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

}

/// Implemented separately of arrays / vectors of strings to ensure the regex is only compiled once
impl PatternReplace for Vec<String> {
  ///
  /// Optional regex-enabled replace method that will return None if the regex fails
  /// 
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<Vec<String>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => {
        let replacements = self.into_iter()
            .map(|segment| re.replace_all(segment, replacement).to_string())
            .collect::<Vec<String>>();
        Ok(replacements)
      },
      Err(error) => Err(error)
    }
  }

  ///
  /// Optional regex-enabled replace method that will return None if the regex fails
  /// 
  fn pattern_replace_first_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<Vec<String>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => {
        let replacements = self.into_iter()
            .map(|segment| re.replace(segment, replacement).to_string())
            .collect::<Vec<String>>();
        Ok(replacements)
      },
      Err(error) => Err(error)
    }
  }

  /// Simple regex-enabled replace-all method that will return the same string if the regex fails
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Vec<String> {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

  /// Simple regex-enabled replace-first method that will return the same string if the regex fails
  fn pattern_replace_first(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Vec<String> {
    self.pattern_replace_first_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

}
