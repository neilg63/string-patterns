use crate::{PatternReplace, utils::{build_whole_word_pattern, build_word_pattern, build_optional_whole_word_pattern}, WordBounds, PatternMatch, PatternCapture};

// Set of traits with extension methods to match or replace one or more whole words or sets of whole words
// with various word boundary and case-sensitivity rules

/// Provides methods to match words with differnt word boundary and case-semsitivity rules 
pub trait MatchWord where Self:PatternMatch, Self:PatternCapture {

  /// Match a word with bounds options and case_insensitive flag
  fn match_word_bounds(&self, word: &str, bounds: WordBounds, case_insensitive: bool) -> bool {
    let word_pattern = bounds.to_pattern(word);
    self.pattern_match(&word_pattern, case_insensitive)
  }

   /// Case-conditional match of a whole word
  /// To match only the start or end, use the start and end methods or expand the pattern with \w* at either end
  fn match_word(&self, word: &str, case_insensitive: bool) -> bool {
    let pattern = build_whole_word_pattern(word);
    self.pattern_match(&pattern, case_insensitive)
  }

  /// Match any whole words only with a boolean case_insensitive flag
  fn match_any_words(&self, words: &[&str], case_insensitive: bool) -> bool {
    let pattern = build_optional_whole_word_pattern(words);
    self.pattern_match(&pattern, case_insensitive)
  }

  /// Case-conditional match from the start of a word boundary
  fn match_word_start(&self, word: &str, case_insensitive: bool) -> bool {
    let word_pattern = build_word_pattern(word, WordBounds::Start);
    self.pattern_match(&word_pattern, case_insensitive)
  }

  /// Case-conditional match to the end of a word boundary
  fn match_word_end(&self, word: &str, case_insensitive: bool) -> bool {
    let word_pattern = build_word_pattern(word, WordBounds::End);
    self.pattern_match(&word_pattern, case_insensitive)
  }
  
  /// Case-insensitive whole word match, for words with optional hyphens use -?, e.g. hip-?hop matches hip-hop and hiphop, but not hip-hopping
  /// To match only the start or end, use the start and end methods or expand the pattern with \w* at either end
  fn match_word_ci(&self, word: &str) -> bool {
    self.match_word(word, true)
  }

  /// Match any whole words only in case-insensitive mode
  fn match_any_words_ci(&self, words: &[&str]) -> bool {
    let pattern = build_optional_whole_word_pattern(words);
    self.pattern_match(&pattern, true)
  }

  /// Match any whole words only in case-sensitive mode
  fn match_any_words_cs(&self, words: &[&str]) -> bool {
    let pattern = build_optional_whole_word_pattern(words);
    self.pattern_match(&pattern, false)
  }

  /// Case-insensitive match from the start of a word boundary
  fn match_word_start_ci(&self, word: &str) -> bool {
    self.match_word_start(word, true)
  }

  /// Case-insensitive match to the end of a word boundary
  fn match_word_end_ci(&self, word: &str) -> bool {
    self.match_word_end(word, true)
  }
  /// Case-sensitive whole word match, for words with optional hyphens use -?, e.g. hip-?hop matches hip-hop and hiphop, but not hip-hopping
  /// To match only the start or end, use the start and end methods or expand the pattern with \w* at either end
  fn match_word_cs(&self, word: &str) -> bool {
    self.match_word(word, false)
  }

  /// Case-sensitive match from the start of a word boundary
  fn match_word_start_cs(&self, word: &str) -> bool {
    self.match_word_start(word, false)
  }

  /// Match all whole words in case-insensitive mode
  fn match_words_ci(&self, words: &[&str]) -> bool {
    self.match_words_bounds(words, WordBounds::Both, true)
  }

  /// Match all whole words in case-sensitive mode
  fn match_words_cs(&self, words: &[&str]) -> bool {
    self.match_words_bounds(words, WordBounds::Both, false)
  }

  /// Case-sensitive match to the end of a word boundary
  fn match_word_end_cs(&self, word: &str) -> bool {
    self.match_word_end(word, false)
  }

  /// Count matched words from an array of strs with boundary and case_insensitive options
  fn count_matched_words_bounds(&self, words: &[&str], bounds: WordBounds, case_insensitive: bool) -> usize {
    let mut num_matched = 0;
    for word in words {
      let pattern = bounds.to_pattern(word);
      if self.pattern_match(&pattern, case_insensitive) {
        num_matched += 1;
      }
    }
    num_matched
  }

  /// Match all words in array with boundary and case_insensitive options
  fn match_words_bounds(&self, words: &[&str], bounds: WordBounds, case_insensitive: bool) -> bool {
    words.len() == self.count_matched_words_bounds(words, bounds, case_insensitive)
  }

  /// Match all whole words only with a boolean case_insensitive flag
  fn match_words(&self, words: &[&str], case_insensitive: bool) -> bool {
    self.match_words_bounds(words, WordBounds::Both, case_insensitive)
  }

   /// Match sets of words with positivity, pattern and case_insensitive parameters in tuples
  /// e.g. to match sentences with cat(s) but not dog(s) (lower case only)
  /// let sets = [(true, "cats?", true), (false, "dogs?", false)]; 
  fn match_words_sets_conditional(&self, sets: &[(bool, &str, bool)]) -> bool {
    let num_words = sets.len();
    let mut num_matched = 0;
    for row in sets {
      let (is_positive, word, case_insensitive) = *row;
      let pattern = build_whole_word_pattern(word);
      if self.pattern_match(&pattern, case_insensitive) == is_positive {
        num_matched += 1;
      }
    }
    num_matched == num_words
  }

  /// Match sets of words with positivity and pattern tuple in case-insensitive mode
  /// e.g. to match sentences with cat(s) but not dog(s) (lower case only)
  /// let sets = [(true, "cats?"), (false, "dogs?")];
  fn match_words_sets_conditional_ci(&self, tuples: &[(bool, &str)]) -> bool {
    let num_words = tuples.len();
    let mut num_matched = 0;
    for row in tuples {
      let (is_positive, word) = *row;
      let pattern = build_whole_word_pattern(word);
      if self.pattern_match(&pattern, true) == is_positive {
        num_matched += 1;
      }
    }
    num_matched == num_words
  }

  /// Check if whole word patterns occur in close proximity as defined by their min and max values
  /// If the second word may occur before the first the min value should negative
  /// The distance between the end of the first and start of the second word is measured
  fn match_words_by_proximity(&self, first: &str, second: &str, min: i16, max: i16, case_insensitive: bool) -> bool {
    let word_pattern_1 = build_whole_word_pattern(first);
    let word_pattern_2 = build_whole_word_pattern(second);
    if let Some((first_first,first_last)) = self.pattern_first_last_matches(&word_pattern_1, case_insensitive) {
      if let Some((second_first, second_last)) = self.pattern_first_last_matches(&word_pattern_2, case_insensitive) {
        let diff_i64 = second_last.start() as i64 - first_first.end() as i64;
        // although indices are usize and convert to i64 for negative values, only consider differences in i16 range (-32768 to 32767)
        // which suffices for text proximity matches
        if diff_i64 >= i16::MIN as i64 && diff_i64 <= i16::MAX as i64 {
          let diff = diff_i64 as i16;
          return diff >= min && diff <= max;
        } else if min < 0 {
          // reverse match logic if negative min offsets are allowed
          let diff_2_i64 = first_last.start() as i64 - second_first.end() as i64;
          if diff_2_i64 >= i16::MIN as i64 && diff_2_i64 <= i16::MAX as i64 {
            let diff_2 = diff_i64 as i16;
            return diff_2 >= min && diff_2 <= max;
          }
        }
      }
    }
    false
  }

}

/// Automatic implementation for str/String as both implement PatternMatch and PatternCapture in this crate
impl MatchWord for str {
}

/// Methods for whole or partial word replacements
pub trait ReplaceWord where Self:PatternReplace {

  /// Replace words with boundary and case_insensitive options
  fn replace_word_bounds(&self, word: &str, replacement: &str, bounds: WordBounds, case_insensitive: bool) -> Self where Self:Sized;

  /// Replace whole words with case_insensitive options
  fn replace_word(&self, word: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;

  /// Replace whole words with in case-insensitive mode
  fn replace_word_ci(&self, word: &str, replacement: &str) -> Self where Self:Sized {
    let pattern = build_whole_word_pattern(word);
    self.pattern_replace(&pattern, replacement, true)
  }

  /// Replace whole words with in case-sensitive mode
  fn replace_word_cs(&self, word: &str, replacement: &str) -> Self where Self:Sized {
    let pattern = build_whole_word_pattern(word);
    self.pattern_replace(&pattern, replacement, false)
  }

  /// Replace one or pairs of whole words with a boolean case_insensitive flag
  fn replace_words(&self, pairs: &[(&str, &str)], case_insensitive: bool) -> Self where Self:Sized;

  /// Replace one or pairs of whole words in case-insensitive mode
  fn replace_words_ci(&self, pairs: &[(&str, &str)]) -> Self where Self:Sized {
    self.replace_words(pairs, true)
  }

  /// Replace one or pairs of whole words in case-sensitive mode
  fn replace_words_cs(&self, pairs: &[(&str, &str)]) -> Self where Self:Sized {
    self.replace_words(pairs, false)
  }

  /// Replace one or sets of whole words with case_insensitive flags as the last tuple element
  fn replace_word_sets(&self, pairs: &[(&str, &str, bool)]) -> Self where Self:Sized;

}


/// Methods for whole or partial word replacements
impl ReplaceWord for String {

  /// Replace words with boundary and case_insensitive options
  fn replace_word_bounds(&self, word: &str, replacement: &str, bounds: WordBounds, case_insensitive: bool) -> String {
    let pattern = build_word_pattern(word, bounds);
    self.pattern_replace(&pattern, replacement, case_insensitive)
  }

  /// Replace whole words with case_insensitive options
  fn replace_word(&self, word: &str, replacement: &str, case_insensitive: bool) -> String {
    let pattern = build_whole_word_pattern(word);
    self.pattern_replace(&pattern, replacement, case_insensitive)
  }
  
  /// Replace one or pairs of whole words with a boolean case_insensitive flag
  fn replace_words(&self, pairs: &[(&str, &str)], case_insensitive: bool) -> String {
    let mut output = self.clone();
    for pair in pairs {
      let (word, replacement) = *pair;
      let pattern = build_whole_word_pattern(word);
      output = output.pattern_replace(&pattern, replacement, case_insensitive);
    }
    output
  }

  /// Replace one or sets of whole words with case_insensitive flags as the last tuple element
  fn replace_word_sets(&self, tuples: &[(&str, &str, bool)]) -> String {
    let mut output = self.clone();
    for row in tuples {
      let (word, replacement, case_insensitive) = *row;
      let pattern = build_whole_word_pattern(word);
      output = output.pattern_replace(&pattern, replacement, case_insensitive);
    }
    output
  }

}
