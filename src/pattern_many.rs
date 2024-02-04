use crate::{PatternMatch, PatternReplace, WordBounds};

/// Provides methods to match with multiple patterns 
/// expressed as arrays of tuples or simple strs (for pattern_match_many_ci and pattern_match_many_cs)
pub trait PatternMatchMany where Self:PatternMatch {
  /// Matches all of the patterns in case-sensitivity flag
  /// with an array of tuples (patterns, case_insensitive)
  fn pattern_match_many(&self, patterns: &[&str], case_insensitive: bool) -> bool {
    let mut num_matched:usize = 0;
    let num_patterns = patterns.len();
    for pattern in patterns {
      if self.pattern_match(pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched == num_patterns
  }

  /// Matches all of the patterns with case-insensitive flag
  /// e.g. ```(r#"a[ck]"#, true)``` "ac" or "ak" whether upper, lower or mixed case
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_match_many_mixed(&self, pattern_sets: &[(&str, bool)]) -> bool {
    let mut num_matched:usize = 0;
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
    let mut num_matched:usize = 0;
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

/// Implement PatternMatchMany for &str/String
impl PatternMatchMany for str {
}


/// Implement PatternMatchMany for vectors of strings.
impl PatternMatchMany for [String] {  
}

/// Test multiple false or positive patterns and return vector of booleans with the results for each item
pub trait PatternMatchesMany where Self:PatternMatch {
  fn pattern_matches_conditional(&self, pattern_sets: &[(&str, bool)], bounds:WordBounds) -> Vec<bool>;

  fn pattern_word_matches_conditional(&self, pattern_sets: &[(&str, bool)]) -> Vec<bool> {
    self.pattern_matches_conditional(&pattern_sets, WordBounds::Both)
  }

  fn pattern_matches_conditional_ci(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<(&str, bool)> = patterns.into_iter().map(|s| (*s, true)).collect();
    self.pattern_matches_conditional(&pattern_sets, WordBounds::None)
  }

  fn pattern_matches_conditional_cs(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<(&str, bool)> = patterns.into_iter().map(|s| (*s, false)).collect();
    self.pattern_matches_conditional(&pattern_sets, WordBounds::None)
  }

  fn pattern_word_matches_conditional_ci(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<(&str, bool)> = patterns.into_iter().map(|s| (*s, true)).collect();
    self.pattern_matches_conditional(&pattern_sets, WordBounds::Both)
  }

  fn pattern_word_matches_conditional_cs(&self, patterns: &[&str]) -> Vec<bool> {
    let pattern_sets: Vec<(&str, bool)> = patterns.into_iter().map(|s| (*s, false)).collect();
    self.pattern_matches_conditional(&pattern_sets, WordBounds::Both)
  }
}

impl PatternMatchesMany for str {
  /// test for multiple conditions. All other trait methods are derived from this
  fn pattern_matches_conditional(&self, pattern_sets: &[(&str, bool)], bounds:WordBounds) -> Vec<bool> {
    let mut matched_items: Vec<bool> = Vec::with_capacity(pattern_sets.len());
    for pattern_set in pattern_sets {
       let (pattern, case_insensitive) = *pattern_set;
       let bounded_pat = bounds.to_pattern(pattern);
       let is_matched = self.pattern_match(&bounded_pat, case_insensitive);
       matched_items.push(is_matched);
     }
     matched_items
   }
}

/// Provides methods to replace with multiple patterns 
/// expressed as arrays of tuples
pub trait PatternReplaceMany {
  /// Replaces multiple sets of patterns with replacements in case-sensitive mode
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_replace_pairs(&self, replacement_sets: &[(&str, &str)], case_insensitive: bool) -> Self where Self: Sized;

  /// Replaces multiple sets of patterns with replacements in case-insensitive mode
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_replace_pairs_ci(&self, replacement_sets: &[(&str, &str)]) -> Self where Self: Sized {
    self.pattern_replace_pairs(replacement_sets, true)
  }

  /// Replaces multiple sets of patterns with replacements in case-insensitive mode
  /// with an array of tuples (pattern, replacement, case_insensitive)
  fn pattern_replace_pairs_cs(&self, replacement_sets: &[(&str, &str)]) -> Self where Self: Sized {
    self.pattern_replace_pairs(replacement_sets, false)
  }

  /// Replaces multiple sets of patterns with replacements in case-sensitive mode
  /// with an array of simple tuples (pattern, replacement)
  fn pattern_replace_sets(&self, replacement_sets: &[(&str, &str, bool)]) -> Self where Self: Sized;
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
  fn pattern_replace_pairs(&self, replacement_pairs: &[(&str, &str)], case_sensitive: bool) -> String {
    let mut return_string = self.clone();
    for replacement_pair in replacement_pairs {
      let (pattern, replacement) = *replacement_pair;
      if let Ok(new_string) = return_string.pattern_replace_result(pattern, replacement, case_sensitive) {
        return_string = new_string;
      }
    }
    return_string
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
  fn pattern_replace_pairs(&self, replacement_pairs: &[(&str, &str)], case_insensitive: bool) -> Vec<String> {
    let mut return_strings = self.clone();
    for replacement_pair in replacement_pairs {
      let (pattern, replacement) = *replacement_pair;
      if let Ok(new_string) = return_strings.pattern_replace_result(pattern, replacement, case_insensitive) {
        return_strings = new_string;
      }
    }
    return_strings
  }
}
