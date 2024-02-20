use crate::{enums::StringBounds, utils::{pairs_to_string_bounds, strs_to_string_bounds}, StripCharacters};

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

/// Implementation for &str/String 
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


/// Test multiple patterns and return vector of booleans with the results for each item
pub trait SimpleMatchesMany where Self:SimpleMatch {

  /// test for multiple conditions. All other trait methods are derived from this
  fn matched_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<bool>;

  /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
  fn contains_conditional(&self, pattern_sets: &[(&str, bool)]) -> Vec<bool> {
    let pattern_sets: Vec<StringBounds> = pairs_to_string_bounds(pattern_sets, 2);
    self.matched_conditional(&pattern_sets)
   }

  /// Test for presecnce of simple patterns in case-insensitive mode
  fn contains_conditional_ci(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, true, 2);
    self.matched_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-sensitive mode
  fn contains_conditional_cs(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, false, 2);
    self.matched_conditional(&pattern_sets)
  }
  
}

impl SimpleMatchesMany for str {

  // test for multiple conditions. All other trait methods are derived from this
  fn matched_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<bool> {
    let mut matched_items: Vec<bool> = Vec::with_capacity(pattern_sets.len());
    for item in pattern_sets {
      let ci = item.case_insensitive();
      // cast the sample string to lowercase for case-insenitive matches
      let base = if ci {
        self.to_lowercase()
      } else {
        self.to_owned()
      };
      // cast the simple pattern to lowercase for case-insenitive matches
      let pattern = if ci {
        item.pattern().to_lowercase()
      } else {
        item.pattern().to_owned()
      };
      // check if outcome of starts_with, ends_with or contains test matches the positivity value
      let is_matched = if item.starts_with() {
        base.starts_with(&pattern)
      } else if item.ends_with() {
        base.ends_with(&pattern)
      } else {
        base.contains(&pattern)
      } == item.is_positive();
       matched_items.push(is_matched);
     }
     matched_items
   }
}

/// Test multiple patterns and return boolean
pub trait SimpleMatchAll where Self:SimpleMatchesMany {

  /// test for multiple conditions. All other trait methods are derived from this
  fn match_all_conditional(&self, pattern_sets: &[StringBounds]) -> bool;

  /// test for multiple conditions with simple tuple pairs of pattern + case-insenitive flag
  fn contains_all_conditional(&self, pattern_sets: &[(&str, bool)]) -> bool {
    let pattern_sets: Vec<StringBounds> = pairs_to_string_bounds(pattern_sets, 2);
    self.match_all_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-insensitive mode
  fn contains_all_conditional_ci(&self, patterns: &[&str]) -> bool {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, true, 2);
    self.match_all_conditional(&pattern_sets)
  }

  /// Test for presecnce of simple patterns in case-sensitive mode
  fn contains_all_conditional_cs(&self, patterns: &[&str]) -> bool {
    let pattern_sets: Vec<StringBounds> = strs_to_string_bounds(patterns, false, 2);
    self.match_all_conditional(&pattern_sets)
  }
  
}

impl SimpleMatchAll for str {

  // test for multiple conditions. All other 'many' trait methods are derived from this
  fn match_all_conditional(&self, pattern_sets: &[StringBounds]) -> bool {
    self.matched_conditional(pattern_sets).into_iter().all(|matched| matched)
  }

}


/// Test multiple patterns and return a filtered vector of string slices
pub trait SimpleFilterAll {

  /// test for multiple conditions. All other trait methods are derived from this
  fn filter_all_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<&str>;
  
}

/// Filter strings by one or more StringBounds rules
impl SimpleFilterAll for [&str] {

  // filter string slices by multiple conditions
  fn filter_all_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<&str> {
    self.into_iter().map(|s| s.to_owned()).filter(|s| s.match_all_conditional(pattern_sets)).collect::<Vec<&str>>()
  }

}


impl SimpleFilterAll for [String] {
  // filter strings by multiple conditions
  fn filter_all_conditional(&self, pattern_sets: &[StringBounds]) -> Vec<&str> {
    self.into_iter().filter(|s| s.match_all_conditional(pattern_sets)).map(|s| s.as_str()).collect::<Vec<&str>>()
  }

}
