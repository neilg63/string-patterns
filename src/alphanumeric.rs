use std::str::FromStr;
use crate::{MatchOccurrences, utils::add_sanitized_numeric_string, ToSegments};

// Set of traits with extension methods to match core alphanumeric, numeric character patterns with words
// ascertain if strings contain valid numbers and extract numbers as floats or integers

/// Method to check if the string may be parsed to an integer or float
pub trait IsNumeric {
  /// strict check on a numeric string before using ```.parse::<T>()```
  /// use trim() or correct_numeric_string() first for looser number validation
  /// This mirrors a similar function in T-SQL, jQuery or the PHP standard library, which is more useful than only checking for digits.
  /// It will fail with spaces or any non-numeric characters other than a leading minus or a single decimal point
  /// For characters, is_numeric checks for decimal digit-equivalent characters
  fn is_numeric(&self) -> bool;
}

/// Implementation for &str / String
impl IsNumeric for str {

  /// Check if the string may be parsed to a number
  /// This is a now a strict regex-free check
  /// Use trim() or correct_numeric_string() first for looser number validation
  fn is_numeric(&self) -> bool {
    let num_chars = self.chars().count();
    let last_index = num_chars - 1;
    let mut num_valid: usize = 0;
    let mut index: usize = 0;
    let mut num_decimal_separators = 0usize;
    for c in self.chars().into_iter() {
      let is_digit = c.is_digit(10);
      let valid_char =  if is_digit {
        true
      } else {
        match c {
          '-' => index == 0,
          '.' => index < last_index && num_decimal_separators < 1,
          _ => false
        }
      };
      if c == '.' {
        num_decimal_separators += 1;
      }
      if valid_char {
        num_valid += 1;
      }
      index += 1;
    }
    num_valid == num_chars
  }
}


/// Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats
pub trait StripCharacters {

  /// Removes all characters that any are not letters or digits, such as punctuation or symbols
  /// Letters include those used in most non-Latin alphabets
  fn strip_non_alphanum(&self) -> String;

  // Remove all characters except digits, including punctuation such as decmimal points
  fn strip_non_digits(&self) -> String;

  /// Extracts valid numeric string components from a longer string
  fn to_numeric_strings(&self) -> Vec<String> {
    self.to_numeric_strings_conditional(false)
  }

  /// Always interpret numeric strings with dots as thousand separators and commas as decimal separators
  fn to_numeric_strings_euro(&self) -> Vec<String> {
    self.to_numeric_strings_conditional(true)
  }

  fn to_numeric_strings_conditional(&self, enforce_comma_separator: bool) -> Vec<String>;

  /// Extract numeric strings and cast to numbers with conditional logic over commas and dots,
  /// The boolean flag enforces European logic where dots separate thousands and commas decimals
  /// Otherwise the correct format is deduced. Numeric strings are problematic when they only contain
  /// one comma or point. Otherwise the last separator is always considered the decimal separator if 
  /// it differs from the first separators.
  fn to_numbers_conditional<T: FromStr>(&self, enforce_comma_separator: bool) -> Vec<T>;

  /// Extracts valid integers or floats from a longer string
  fn to_numbers<T: FromStr>(&self) -> Vec<T> {
    self.to_numbers_conditional::<T>(false)
  }

  /// Extract numeric string using European-style decimal commas
  fn to_numbers_euro<T: FromStr>(&self) -> Vec<T> {
    self.to_numbers_conditional::<T>(true)
  }

  /// Correct numbers to conform to use dots (periods, full-stops) only as decimal separators
  /// Works only on the first number encountered and used with to_numeric_strings or to_numeric_strings_euro
  /// to correct multiple numbers in a longer string
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> String;

  /// Extracts the first valid integer or float from a longer string if present
  fn to_first_number<T: FromStr + Copy>(&self) -> Option<T> {
    if let Some(number) = self.to_numbers::<T>().first() {
      Some(*number)
    } else {
      None
    }
  }

  /// Extracts the first valid integer or float from a longer string
  /// if commas are used for decimals and dots for thousand separators  
  fn to_first_number_euro<T: FromStr + Copy>(&self) -> Option<T> {
    if let Some(number) = self.to_numbers_euro::<T>().first() {
      Some(*number)
    } else {
      None
    }
  }

  /// Removes all characters no used in valid numeric sequences
  /// with single spaces between numbers
  fn strip_non_numeric(&self) -> String {
    self.to_numeric_strings().join(" ")
  }

}


impl StripCharacters for str {
    
  /// Remove all characters that are not letters or numerals for later string comparison. Does not use a regular expression
  /// Will remove all spaces separating words
  fn strip_non_alphanum(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_alphanumeric()).collect::<String>()
  }

  /// Remove all characters that are not numerals for later string comparison. Does not use a regular expression
  /// Will remove all spaces separating numbers
  /// Use strip_non_numeric to extract a string with valid numbers only separated by spaces
  fn strip_non_digits(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_digit(10)).collect::<String>()
  }

  /// Correct numeric strings with commas as thousand separators or as decimal separators
  /// to a regular format with punctuation only for decimal points before being parsed to an integer or float
  /// This is best used only with numeric strings as it will strip commas and dots not used as decimal separators
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> String {
      let commas = self.find_matched_indices(",");
      let last_comma_index = commas.last().unwrap_or(&0).to_owned();
      let points = self.find_matched_indices(".");
      let last_point_index = points.last().unwrap_or(&0).to_owned();
      let num_commas = commas.len();
      if points.len() > 1 || (last_comma_index > last_point_index  && num_commas <= 1) || (enforce_comma_separator && num_commas <= 1) {
        if num_commas < 1 {
          self.replace(".", "")
        } else {
          let (main, dec_part) = self.to_start_end(",");
          [main.replace(".", ""), dec_part].join(".")
        }
      } else {
        self.replace(",", "")
      }
  }

  /// conditionally extract numeric strings from a longer string
  fn to_numeric_strings_conditional(&self, enforce_comma_separator: bool) -> Vec<String> {
    let mut prev_char = ' ';
    let mut seq_num = 0;
    let mut num_string = String::new();
    let mut output: Vec<String> = Vec::new();
    for component in self.chars() {
      let mut is_end = false;
      if component.is_digit(10) {
        if prev_char == '-' {
          num_string.push(prev_char);  
        }
        num_string.push(component);
        seq_num += 1;
      } else if prev_char.is_digit(10) {
        match component {
          '.' | '․' | ',' => {
            if component == ',' {
              num_string.push(',');
            } else {
              num_string.push('.');
            }
            seq_num = 0;
          },
          _ => {
            is_end = true;
          }
        }
      } else {
        is_end = true;
      }
      if is_end {
        if seq_num > 0 {
          add_sanitized_numeric_string(&mut output, &num_string.correct_numeric_string(enforce_comma_separator));
          num_string = String::new();
        }
        seq_num = 0;
      }
      prev_char = component;
    }
    if num_string.len() > 0 {
      add_sanitized_numeric_string(&mut output, &num_string.correct_numeric_string(enforce_comma_separator));
    }
    output
  }

  fn to_numbers_conditional<T: FromStr>(&self, enforce_comma_separator: bool) -> Vec<T> {
    self.to_numeric_strings_conditional(enforce_comma_separator).into_iter()
      .map(|s| s.parse::<T>())
      .filter(|s| s.is_ok())
      .map(|s| s.ok().unwrap())
      .collect()
  }

}


/// Methods to validate strings with character classes
pub trait CharGroupMatch {
  /// Does the string contain any decimal digits
  fn has_digits(&self) -> bool;

  /// Does the string contain any digits any supported radix
  fn has_digits_radix(&self, radix: u8) -> bool;

  /// Does the string contain any alphanumeric characters including those from non-Latin alphabets
  fn has_alphanumeric(&self) -> bool;

  /// Does the string contain any letters including those from non-Latin alphabets, but excluding digits
  fn has_alphabetic(&self) -> bool;

  fn is_digits_only(&self) -> bool;

  /// Does the string contain any digits any supported radix
  fn is_digits_only_radix(&self, radix: u8) -> bool;

}

impl CharGroupMatch for str {

  fn has_digits(&self) -> bool {
      self.chars().any(|c| c.is_ascii_digit())
  }

  fn has_digits_radix(&self, radix: u8) -> bool {
    self.chars().any(|c| c.is_digit(radix as u32))
  }

  fn has_alphanumeric(&self) -> bool {
      self.chars().any(char::is_alphanumeric)
  }

  fn has_alphabetic(&self) -> bool {
    self.chars().any(char::is_alphabetic)
  }

  fn is_digits_only(&self) -> bool {
    self.chars().all(|c| c.is_ascii_digit())
  }

  /// Does the string contain any digits any supported radix
  fn is_digits_only_radix(&self, radix: u8) -> bool {
    self.chars().all(|c| c.is_digit(radix as u32))
  }

}
