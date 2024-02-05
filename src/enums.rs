
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

#[derive(Debug, Clone)]
pub enum StringBounds<'a> {
  StartsWith(&'a str, bool),
  EndsWith(&'a str, bool),
  Contains(&'a str, bool),
  NotStartsWith(&'a str, bool),
  NotEndsWith(&'a str, bool),
  NotContains(&'a str, bool),
  StartsWithCi(&'a str),
  EndsWithCi(&'a str),
  ContainsCi(&'a str),
  NotStartsWithCi(&'a str),
  NotEndsWithCi(&'a str),
  NotContainsCi(&'a str),
  StartsWithCs(&'a str),
  EndsWithCs(&'a str),
  ContainsCs(&'a str),
  NotStartsWithCs(&'a str),
  NotEndsWithCs(&'a str),
  NotContainsCs(&'a str)
}

impl<'a> StringBounds<'a> {

  // Only case-neutral modes are supported via this constructor
  pub fn new(mode: u8, txt: &'a str, case_insensitive: bool) -> StringBounds<'a> {
    match mode {
      0 => StringBounds::StartsWith(txt, case_insensitive),
      1 => StringBounds::EndsWith(txt, case_insensitive),
      5 => StringBounds::Contains(txt, case_insensitive),
      3 => StringBounds::NotStartsWith(txt, case_insensitive),
      4 => StringBounds::NotEndsWith(txt, case_insensitive),
      _ => StringBounds::Contains(txt, case_insensitive),
    }
  }

  pub fn case_insensitive(&self) -> bool {
    match self {
      Self::StartsWith(_, val) => *val,
      Self::EndsWith(_, val) => *val,
      Self::Contains(_, val) => *val,
      Self::NotStartsWith(_, val) => *val,
      Self::NotEndsWith(_, val) => *val,
      Self::NotContains(_, val) => *val,
      Self::StartsWithCi(..) | Self::EndsWithCi(..) | Self::ContainsCi(..) | Self::NotStartsWithCi(..) | Self::NotEndsWithCi(..) | Self::NotContainsCi(..) => true,
      _ => false, 
    }
  }

  pub fn pattern(&self) -> &'a str {
    match self {
      Self::StartsWith(txt, _) => txt,
      Self::EndsWith(txt, _) => txt,
      Self::Contains(txt, _) => txt,
      Self::NotStartsWith(txt, _) => txt,
      Self::NotEndsWith(txt, _) => txt,
      Self::NotContains(txt, _) => txt,
      Self::StartsWithCi(txt) | Self::EndsWithCi(txt) | Self::ContainsCi(txt) | Self::NotStartsWithCi(txt) | Self::NotEndsWithCi(txt) | Self::NotContainsCi(txt) => txt,
      Self::StartsWithCs(txt) | Self::EndsWithCs(txt) | Self::ContainsCs(txt) | Self::NotStartsWithCs(txt) | Self::NotEndsWithCs(txt) | Self::NotContainsCs(txt) => txt,
    }.to_owned()
  }

  pub fn is_positive(&self) -> bool {
    match self {
      Self::StartsWith(..) | Self::EndsWith(..) | Self::Contains(..) | Self::StartsWithCi(..)  | Self::EndsWithCi(..) | Self::ContainsCi(..) | Self::StartsWithCs(..)  | Self::EndsWithCs(..) | Self::ContainsCs(..) => true,
      _ => false, 
    }
  }

  pub fn starts_with(&self) -> bool {
    match self {
      Self::StartsWith(..) | Self::StartsWithCi(..) | Self::StartsWithCs(..) | Self::NotStartsWith(..) | Self::NotStartsWithCi(..) | Self::NotStartsWithCs(..) => true,
      _ => false
    }
  }

  pub fn ends_with(&self) -> bool {
    match self {
      Self::EndsWith(..) | Self::EndsWithCi(..) | Self::EndsWithCs(..) | Self::NotEndsWith(..) | Self::NotEndsWithCi(..) | Self::NotEndsWithCs(..) => true,
      _ => false
    }
  }

}