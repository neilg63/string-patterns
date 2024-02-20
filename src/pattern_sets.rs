use crate::{find_matches_within_haystack, utils::to_optional_end_pattern, PatternCapture, PatternMatch, PatternReplace, SimpleEnclode};
use regex::{Error, Match, Regex};

pub const MAIN_REGEX_IS_EMPTY_ERROR_TEXT: &'static str = "Core regex is empty";

#[derive(Debug, Clone)]
pub struct MatchSet<'a> {
  case_insensitive: bool,
  behind: Option<(&'a str, bool)>,
  main: &'a str,
  ahead: Option<(&'a str, bool)>,
}

impl<'a> MatchSet<'a> {
  pub fn new(main_pattern: &'a str, case_insensitive: bool) -> Self {
    MatchSet {
      case_insensitive,
      behind: None,
      main: main_pattern,
      ahead: None,
    }
  }

  pub fn new_ci(main_pattern: &'a str) -> Self {
    MatchSet {
      case_insensitive: true,
      behind: None,
      main: main_pattern,
      ahead: None,
    }
  }

  pub fn new_cs(main_pattern: &'a str) -> Self {
    MatchSet {
      case_insensitive: false,
      behind: None,
      main: main_pattern,
      ahead: None,
    }
  }

  pub fn empty() -> Self {
    MatchSet {
      case_insensitive: false,
      behind: None,
      main: "",
      ahead: None,
    }
  }

  pub fn case_insensitive(&mut self) -> Self {
    self.case_insensitive = true;
    self.clone()
  }

  pub fn case_sensitive(&mut self) -> Self {
    self.case_insensitive = false;
    self.clone()
  }

  pub fn look_behind(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.behind = Some((pattern, is_positive));
    self.clone()
  }

  pub fn look_ahead(&mut self, pattern: &'a str, is_positive: bool) -> Self {
    self.ahead = Some((pattern, is_positive));
    self.clone()
  }


  pub fn has_look_around(&self) -> bool {
    self.behind.is_some() || self.ahead.is_some()
  }

  pub fn has_core(&self) -> bool {
    self.main.len() > 0
  }

  pub fn matches_result(&self, sample: &str) -> Result<bool, Error> {
    if self.has_look_around() {
      if self.has_core() {
        let matched_items = sample.pattern_matches_outer(&self.main, self.case_insensitive);
        let mut num_look_arounds: u8 = 0;
        let mut num_matches: u8 = 0;
        if matched_items.len() > 0 {
          if let Some((behind_pattern, is_pos)) = self.behind {
            if let Some(inner_match) = sample.pattern_first_inner_match(behind_pattern, self.case_insensitive) {
              let is_matched = matched_items.iter().any(|m| inner_match.end() < m.start());
              num_look_arounds += 1;
              if is_matched == is_pos {
                num_matches += 1;
              }
            }
          }
          if let Some((ahead_pattern, is_pos)) = self.ahead {
            if let Some(inner_match) = sample.pattern_last_match(ahead_pattern, self.case_insensitive) {
              let is_matched = matched_items.iter().any(|m| inner_match.start() > m.end());
              num_look_arounds += 1;
              if is_matched == is_pos {
                num_matches += 1;
              }
            }
          }
          Ok(num_matches == num_look_arounds)
        } else {
          Ok(false)
        }
      } else {
        Err(Error::Syntax(MAIN_REGEX_IS_EMPTY_ERROR_TEXT.to_owned()))
      }
    } else if self.has_core() {
      sample.pattern_match_result(self.main, self.case_insensitive)
    } else {
      Err(Error::Syntax(MAIN_REGEX_IS_EMPTY_ERROR_TEXT.to_owned()))
    }
  }

  pub fn matched_vec(&self, sample: &'a str) -> Vec<Match<'a>> {
    let (matched_items, _main_rgx_opt) = self.matched_vec_and_main(sample);
    matched_items
  }

  fn matched_vec_and_main(&self, sample: &'a str) -> (Vec<Match<'a>>, Option<Box<Regex>>) {
    let mut matches: Vec<Match> = Vec::new();
    let mut main_rgx_option = None;
    if self.has_look_around() {
      if self.has_core() {
        //let matched_items = sample.pattern_matches_outer(self.main, self.case_insensitive);
        let (matched_items, main_rgx_opt) = find_matches_within_haystack(sample, &self.main, self.case_insensitive, true);
        main_rgx_option = main_rgx_opt;
        if matched_items.len() > 0 {
          for m_item in matched_items {
            let mut num_look_arounds: u8 = 0;
            let mut num_matches: u8 = 0;
            if let Some((behind_pattern, is_pos)) = self.behind {
              if let Some(inner_match) = sample.pattern_first_match(behind_pattern, self.case_insensitive) {
                let end_pattern = to_optional_end_pattern(m_item.as_str());
                let inner_str = inner_match.as_str();
                let inner_len = inner_str.len();
                let behind_str = inner_str.to_owned().pattern_replace_cs(&end_pattern, "");
                let len_diff = inner_len - behind_str.len();
                let target_end = if len_diff < inner_match.end() { inner_match.end() - len_diff } else { inner_len };
                println!("{} -- {:?} end {}", inner_match.as_str(), end_pattern, target_end);
                let is_matched = inner_match.end() == m_item.start();
                num_look_arounds += 1;
                if is_matched == is_pos {
                  num_matches += 1;
                }
              }
            }
            if let Some((ahead_pattern, is_pos)) = self.ahead {
              if let Some(inner_match) = sample.pattern_last_match(ahead_pattern, self.case_insensitive) {
                let is_matched = inner_match.start() == m_item.end();
                num_look_arounds += 1;
                if is_matched == is_pos {
                  num_matches += 1;
                }
              }
            }
            if num_matches == num_look_arounds {
              matches.push(m_item);
            }
          }
        }
      }
    } else if self.has_core() {
      matches = sample.pattern_matches_outer(self.main, self.case_insensitive);
    }
    (matches, main_rgx_option)
  }

  pub fn replace(&self, sample: &'a str, replacement: &'a str) -> String {
    let (matches, main_rgx_opt) = self.matched_vec_and_main(sample);
    
    let mut index: usize = 0;
    let len = sample.len();
    let num_matches = matches.len();
    let last_index = if num_matches > 0 { num_matches - 1 } else { 0 };
    let mut prev_end:usize = 0;
    if let Some(main_rgx) = main_rgx_opt {
      if num_matches > 0 {
        let mut parts: Vec<String> = Vec::with_capacity(matches.len());
        for m_item in matches {
          if index == 0 && m_item.start() > 0 {
            let sub = &sample[0..m_item.start()];
            parts.push(sub.to_string());
          } else if prev_end > 0 {
            let sub = &sample[prev_end..m_item.start()];
            parts.push(sub.to_string());
          }
          let substr = sample[m_item.start()..m_item.end()].to_string();
          // let repl = substr.pattern_replace(self.main, replacement, self.case_insensitive);
          // do not recompile the main regex
          let repl = main_rgx.replace_all(&substr, replacement).to_string();
          parts.push(repl);
          if index == last_index && m_item.end() < len {
            let sub = &sample[m_item.end()..len];
            parts.push(sub.to_string() );
          }
          prev_end = m_item.end();
          index += 1;
        }
        parts.concat()
      } else {
        sample.to_owned()
      }
    } else {
      sample.to_owned()
    }
  }

}

pub trait PatternSet<'a> {
  
  fn pattern_set_match_result(&'a self, ms: &'a MatchSet) -> Result<bool, Error>;

  fn pattern_set_match(&'a self, ms: &'a MatchSet) -> bool {
    if let Ok(is_matched) = self.pattern_set_match_result(ms) {
      is_matched
    } else {
      false
    }
  }
  
  fn pattern_set_matches(&'a self, ms: &'a MatchSet) -> Vec<Match<'a>>;

  fn pattern_set_replace(&'a self, ms: &'a MatchSet, replacement: &'a str) -> String;
}

impl<'a> PatternSet<'a> for str {

  fn pattern_set_match_result(&'a self, ms: &'a MatchSet) -> Result<bool, Error> {
    ms.matches_result(self)
  }

  fn pattern_set_matches(&'a self, ms: &'a MatchSet) -> Vec<Match<'a>> {
    ms.matched_vec(self)
  }

  fn pattern_set_replace(&'a self, ms: &'a MatchSet, replacement: &'a str) -> String {
    ms.replace(self, replacement)
  }
}