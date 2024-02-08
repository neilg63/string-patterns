
/// Defines the start, end and both bounds of a word
pub enum WordBounds {
  None,
  Start,
  End,
  Both,
}

impl WordBounds {
  /// Convert word bounds 
  pub fn to_pattern(&self, word: &str) -> String {
    match self {
      WordBounds::Start => [r#"\b"#, word].concat(),
      WordBounds::End => [word, r#"\b"#].concat(),
      WordBounds::Both => [r#"\b"#, word, r#"\b"#].concat(),
      _ => word.to_owned(),
    }
  }
}

/// Defines the matching bounds of simple string matches with case-insensitive/sensitive variants
/// and accepting the string pattern and positivity flag as arguments
#[derive(Debug, Clone)]
pub enum StringBounds<'a> {
  StartsWithCi(&'a str, bool),
  EndsWithCi(&'a str, bool),
  ContainsCi(&'a str, bool),
  StartsWithCs(&'a str, bool),
  EndsWithCs(&'a str, bool),
  ContainsCs(&'a str, bool),
}

impl<'a> StringBounds<'a> {

  // Only used internally in utils
  // 0: starts with, 1 ends with, 2 (default) contains
  pub fn new(mode: u8, txt: &'a str, is_positive: bool, case_insensitive: bool) -> StringBounds<'a> {
    match mode {
      0 => if case_insensitive {
        Self::StartsWithCi(txt, is_positive)
      } else {
        Self::StartsWithCs(txt, is_positive)
      },
      1 => if case_insensitive {
        Self::EndsWithCi(txt, is_positive)
      } else {
        Self::EndsWithCs(txt, is_positive)
      },
      _ => if case_insensitive {
        Self::ContainsCi(txt, is_positive)
      } else {
        Self::ContainsCs(txt, is_positive)
      },
    }
  }

  pub fn case_insensitive(&self) -> bool {
    match self {
      Self::StartsWithCi(..) | Self::EndsWithCi(..) | Self::ContainsCi(..) => true,
      _ => false, 
    }
  }

  pub fn pattern(&self) -> &'a str {
    match self {
      Self::StartsWithCi(txt, _) | Self::EndsWithCi(txt, _) | Self::ContainsCi(txt, _) |
      Self::StartsWithCs(txt, _) | Self::EndsWithCs(txt, _) | Self::ContainsCs(txt, _ ) => txt,
    }.to_owned()
  }

  pub fn is_positive(&self) -> bool {
    match self {
      Self::StartsWithCi(_, is_pos) | Self::EndsWithCi(_, is_pos) | Self::ContainsCi(_, is_pos) |
      Self::StartsWithCs(_, is_pos) | Self::EndsWithCs(_, is_pos) | Self::ContainsCs(_, is_pos) => is_pos,
    }.to_owned()
  }

  pub fn starts_with(&self) -> bool {
    match self {
      Self::StartsWithCi(..) | Self::StartsWithCs(..) => true,
      _ => false
    }
  }

  pub fn ends_with(&self) -> bool {
    match self {
      Self::EndsWithCi(..) | Self::EndsWithCs(..) => true,
      _ => false
    }
  }

}