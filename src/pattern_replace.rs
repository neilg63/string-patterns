use regex::Error;
use crate::utils::build_regex;
use std::borrow::ToOwned;

/// Core regular expression replacement methods 
pub trait PatternReplace where Self:Sized {
  
  /// Define a custom error type associated with the trait
  type Error: std::error::Error;

  /// Associated type for owned version of Self
  type Owned: ToOwned<Owned = Self>;

  /// Replace all matches of the pattern within a longer text with a boolean case_insensitive flag
  /// NB: If the regex doesn't compile it will return an Error, otherwise in Ok result.
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_result(&self, pattern: &str, replacement: &str,case_insensitive: bool) -> Result<Self::Owned, Self::Error> where Self:Sized;

  /// Apply a regular expression match on the current string with a boolean case_insensitive flag
  /// If the regex fails, nothing will be replaced
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> <Self as PatternReplace>::Owned where Self:Sized + Clone {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or_else(|_| self.clone().to_owned())
  }

  /// Replace all matches of the pattern within a longer text in case-insensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_ci(&self, pattern: &str, replacement: &str) -> Self::Owned where Self:Sized + Clone {
    self.pattern_replace(pattern, replacement, true)
  }

  /// Replace all matches of the pattern within a longer text in case-sensitive mode
  /// If the regex fails, nothing will be replaced
  fn pattern_replace_cs(&self, pattern: &str, replacement: &str) -> Self::Owned where Self:Sized + Clone {
    self.pattern_replace(pattern, replacement, false)
  }

}

/// Core regex replacement methods for Strings
impl PatternReplace for String {

  type Owned = String;

  type Error = Error;
  
  /// Regex-enabled replace method that will return an OK String result if successful and an error if the regex fails
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<String, Self::Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(re.replace_all(self, replacement).to_string()),
      Err(error) => Err(error)
    }
  }

  /// Simple regex-enabled replace method that will return the same string if the regex fails
 /*  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> String {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  } */

}

/// Implemented separately of arrays / vectors of strings to ensure the regex is only compiled once
impl PatternReplace for Vec<String> {
  
  type Owned = Vec<String>;

  type Error = Error;
  ///
  /// Optional regex-enabled replace method that will return None if the regex fails
  /// 
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<Vec<String>, Self::Error> {
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

  /// Simple regex-enabled replace method that will return the same string if the regex fails
  /* fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Vec<String> {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  } */

}

