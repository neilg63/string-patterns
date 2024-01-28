use crate::{PatternMatch, PatternReplace};

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
