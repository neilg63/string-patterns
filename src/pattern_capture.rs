use regex::{Match,Captures};

use crate::utils::{build_regex, build_whole_word_pattern};

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


