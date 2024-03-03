use crate::{build_regex, utils::build_whole_word_pattern};



pub trait PatternFilter<'a, T> where T:Sized {
  /// Filter an array of strs by the pattern
  fn pattern_filter(&'a self, pattern: &str, case_insensitive: bool) -> Vec<T>;

  /// Filters strings in case-insensitive mode
  fn pattern_filter_ci(&'a self, pattern: &str) -> Vec<T> {
    self.pattern_filter(pattern, true)
  }

  /// Filters strings in case-sensitive mode
  fn pattern_filter_cs(&'a self, pattern: &str) -> Vec<T> {
    self.pattern_filter(pattern, false)
  }

  /// Filters strings by whole word regex patterns with case-insensitive flag
  fn pattern_filter_word(&'a self, pattern: &str, case_insensitive: bool) -> Vec<T> {
    let word_pattern = build_whole_word_pattern(pattern);
    self.pattern_filter(&word_pattern, case_insensitive)
  }

  /// Filters strings by whole word regex patterns in case-insensitive mode
  fn pattern_filter_word_ci(&'a self, pattern: &str) -> Vec<T> {
    self.pattern_filter_word(pattern, true)
  }

  /// Filters strings by whole word regex patterns in case-sensitive mode
  fn pattern_filter_word_cs(&'a self, pattern: &str) -> Vec<T> {
    self.pattern_filter_word(pattern, false)
  }
}

impl<'a> PatternFilter<'a, String> for [String] {
  /// Filter an array of strs by the pattern
  fn pattern_filter(&'a self, pattern: &str, case_insensitive: bool) -> Vec<String> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      self.into_iter().filter(|s| re.is_match(s)).map(|s| s.to_owned()).collect::<Vec<String>>()
    } else {
      self.to_owned()
    }
  }
}

impl<'a> PatternFilter<'a, &'a str> for [&str] {
  /// Filter an array of strs by the pattern
  fn pattern_filter(&'a self, pattern: &str, case_insensitive: bool) -> Vec<&str> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      self.into_iter().filter(|s| re.is_match(s)).map(|s| s.to_owned()).collect::<Vec<&str>>()
    } else {
      self.into_iter().map(|s| s.to_owned()).collect::<Vec<&str>>()
    }
  }
}