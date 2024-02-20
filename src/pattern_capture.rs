use regex::{Captures, Match, Regex};

use crate::{utils::{build_regex, build_whole_word_pattern}, SimpleEnclode};

/// Set of methods to capture groups or match objects derived from Regex::captures.
pub trait PatternCapture<'a> {

  /// Yields an option with Regex::Captures as returned from re.captures, Accepts a boolean case_insensitive flag
  fn pattern_captures(&self, pattern: &str, case_insensitive: bool) -> Option<Captures>;

  /// Yields a vector of Match objects with two modes, outer will whole groups only, otherwise uniqe matched groups and subgroups
  /// Use either pattern_matches_vec or pattern_matches_outer
  fn pattern_matches_as_vec(&'a self, pattern: &str, case_insensitive: bool, outer: bool) -> Vec<Match>;

  /// Yields a vector of Match objects with start and end index + the captured string. Accepts a boolean case_insensitive flag
  /// Unlike pattern_captures, this method will only return unique matches including subgroups
  fn pattern_matches_vec(&'a self, pattern: &str, case_insensitive: bool) -> Vec<Match<'a>> {
    self.pattern_matches_as_vec(pattern, case_insensitive, false)
  }

   /// Yields a vector of Match objects with start and end index + the captured string. Accepts a boolean case_insensitive flag
  /// Unlike pattern_captures, this method will only outer matches for whole pattern
  fn pattern_matches_outer(&'a self, pattern: &str, case_insensitive: bool) -> Vec<Match<'a>> {
    self.pattern_matches_as_vec(pattern, case_insensitive, true)
  }

 /// Yields an option with first match object if available with a boolean case_insensitive flag
  /// As this uses re.find it will be fast than the matching last_match method
  fn pattern_first_match(&'a self, pattern: &str, case_insensitive: bool) -> Option<Match<'a>>;

   /// Match the first capture within parentheses
  fn pattern_first_inner_match(&'a self, pattern: &str, case_insensitive: bool) -> Option<Match<'a>> {
    let matched_segments = self.pattern_matches_vec(pattern, case_insensitive);
    let match_opt = if pattern.has_parentheses() && matched_segments.len() > 1 {
      matched_segments.get(1)
    } else {
      matched_segments.get(0)
    };
    if let Some(matched_item) = match_opt {
      Some(matched_item.to_owned())
    } else {
      None
    }
  }

  /// Yields an option with last match object if available with a boolean case_insensitive flag
  fn pattern_last_match(&'a self, pattern: &str, case_insensitive: bool) -> Option<Match> {
    let matched_segments = self.pattern_matches_vec(pattern, case_insensitive);
    if let Some(last) = matched_segments.last() {
      Some(*last)
    } else {
      None
    }
  }

  /// returns an option with a pair of match objects
  /// If there is only one match the match objects will have the same indices
  fn pattern_first_last_matches(&'a self, pattern: &str, case_insensitive: bool) -> Option<(Match, Match)> {
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
  fn pattern_first_index(&'a self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.start())
    } else {
      None
    }
  }

  /// Yields an option with an unsigned integer for the index of the end of the first match
  /// with a boolean case_insensitive flag
  fn pattern_first_end_index(&'a self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.end())
    } else {
      None
    }
  }

  /// Yields an option with an unsigned integer for the index of the start of the last match
  /// with a boolean case_insensitive flag
  fn pattern_last_start_index(&'a self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.start())
    } else {
      None
    }
  }

  // Yields an option with an unsigned integer for the index of the end of the last match
  /// with a boolean case_insensitive flag
  fn pattern_last_index(&'a self, pattern: &str, case_insensitive: bool) -> Option<usize> {
    if let Some(first) = self.pattern_first_match(pattern, case_insensitive) {
      Some(first.end())
    } else {
      None
    }
  }

  // Counts the number of matches with a boolean case_insensitive flag
  fn count_pattern(&'a self, pattern: &'a str, case_insensitive: bool) -> usize {
    self.pattern_matches_vec(pattern, case_insensitive).len()
  }

  // Counts the number of matches with a boolean case_insensitive flag
  fn count_word(&'a self, word: &'a str, case_insensitive: bool) -> usize {
    let pattern = build_whole_word_pattern(word);
    self.pattern_matches_vec(&pattern, case_insensitive).len()
  }
}

pub fn find_matches_within_haystack<'a>(haystack: &'a str, pattern: &str, case_insensitive: bool, outer: bool) -> (Vec<Match<'a>>, Option<Box<Regex>>) {
  let mut matched_items: Vec<Match<'a>> = Vec::new();
  if let Ok(re) = build_regex(pattern, case_insensitive) {
    let mut item_keys: Vec<(&str, usize, usize)> = Vec::new();
    for inner_captures in re.captures_iter(haystack) {
      for capture_opt in inner_captures.iter() {
        if let Some(matched_item) = capture_opt {
          let item_str = matched_item.as_str();
          
          let item_key = (item_str, matched_item.start(), matched_item.end());
          let is_matched = if outer { 
            true
          } else {
            item_keys.contains(&item_key) == false
          };
          if is_matched {
            matched_items.push(matched_item.to_owned());
            if !outer {
              item_keys.push(item_key);
            }
          }
          // if only capturing the first group of outer matches, break the inner loop here and move onto the next outer group
          if outer {
            break;
          }
        }
      }
    }
    (matched_items, Some(Box::new(re)))
  } else {
    (matched_items, None)
  }
}

impl<'a> PatternCapture<'a> for str {

  // Yields an option with Regex::Captures as returned from re.captures, Accepts a boolean case_insensitive flag
  fn pattern_captures(&self, pattern: &str, case_insensitive: bool) -> Option<Captures> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      re.captures(self)
    } else {
      None
    }
  }

  /// Yields a vector of Match objects with start and end index + the captured string. Accepts a boolean case_insensitive flag
  /// Unlike pattern_captures, this method will only return unique matches
  fn pattern_matches_as_vec(&'a self, pattern: &str, case_insensitive: bool, outer: bool) -> Vec<Match<'a>> {
    let (matched_items, _rgx) = find_matches_within_haystack(self, pattern, case_insensitive, outer);
    matched_items
  }

  /// Yields an option with first match object if available with a boolean case_insensitive flag
  /// As this uses re.find it will be fast than the matching last_match method
  fn pattern_first_match(&'a self, pattern: &str, case_insensitive: bool) -> Option<Match<'a>> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
      re.find(self)
    } else {
      None
    }
  }
  

}


