
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
  NotContains(&'a str, bool)
}

impl<'a> StringBounds<'a> {

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
      Self::StartsWith(_, val) => val,
      Self::EndsWith(_, val) => val,
      Self::Contains(_, val) => val,
      Self::NotStartsWith(_, val) => val,
      Self::NotEndsWith(_, val) => val,
      Self::NotContains(_, val) => val,
    }.to_owned()
  }

  pub fn pattern(&self) -> &'a str {
    match self {
      Self::StartsWith(txt, _) => txt,
      Self::EndsWith(txt, _) => txt,
      Self::Contains(txt, _) => txt,
      Self::NotStartsWith(txt, _) => txt,
      Self::NotEndsWith(txt, _) => txt,
      Self::NotContains(txt, _) => txt,
    }.to_owned()
  }

}