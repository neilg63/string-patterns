extern crate regex;
mod utils;

use std::str::FromStr;
use regex::*;
use utils::*;

pub trait PatternMatch {
  /// Apply a regular expression match on the current String-like object
  /// If the regex doesn't compile it will return an error
  /// 
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error>;

  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool;

}

pub trait PatternReplace {
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Self where Self:Sized;

  fn pattern_replace_result(&self, pattern: &str, replacement: &str,case_insensitive: bool) -> Result<Self, Error> where Self:Sized;

}

pub trait StripCharacters {

  /// Removes all characters that any are not letters or digits, such as punctuation or symobols
  /// Letters include those used in most non-Latin alphabets
  fn strip_non_chars(&self) -> Self where Self:Sized;

  // Remove all characters except digits, including punctuation such as decmimal points
  fn strip_non_digits(&self) -> Self where Self:Sized;

  /// Extracts valid numeric string components from a longer string
  fn to_numeric_strings(&self) -> Vec<String>;

  /// Always interpret numeric strings with dots as thousand separators and commas as decimal separators
  fn to_numeric_strings_euro(&self) -> Vec<String>;

  fn to_numeric_strings_conditional(&self, enforce_comma_separator: bool) -> Vec<String>;

  /// Extract numeric strings and cast to numbers with conditional logic over commas and dots,
  /// The boolean flag enforces European logic where dots separate thousands and commas decimals
  /// Otherwise the correct format is deduced. Numeric strings are problematic when they only contain
  /// one comma or point. Otherwise the last separator is always considered the decimal separator if 
  /// it differs from the first separators.
  fn to_numbers_conditional<T: FromStr>(&self, enforce_comma_separator: bool) -> Vec<T>;

  /// Extracts valid integers or floats from a longer string
  fn to_numbers<T: FromStr>(&self) -> Vec<T>;

  fn to_numbers_euro<T: FromStr>(&self) -> Vec<T>;

  /// Correct number
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> Self;

  /// Extracts the first valid integer or float from a longer string if present
  fn to_first_number<T: FromStr + Copy>(&self) -> Option<T>;

  /// Extracts the first valid integer or float from a longer string
  /// if commas are used for decimals and dots for thousand separators
  fn to_first_number_euro<T: FromStr + Copy>(&self) -> Option<T>;

  /// Removes all characters no used in valid numeric sequences
  fn strip_non_numeric(&self) -> Self where Self:Sized;

}

/// Methods to validate strings with character classes
pub trait CharGroupMatch {
  /// Does the string contain any digits
  fn has_digits(&self) -> bool;

  /// Does the string contain any alphanumeric characters including those from non-Latin alphabets
  fn has_alphanumeric(&self) -> bool;

  /// Does the string contain any letters including those from non-Latin alphabets, but excluding digits
  fn has_alphabetic(&self) -> bool;
}

/// Method to check if the string may be parsed to an integer or float
pub trait IsNumeric {
  fn is_numeric(&self) -> bool;
}

impl IsNumeric for String {

  /// Check if the string may be parsed to a number
  fn is_numeric(&self) -> bool {
      self.pattern_match(r#"^-?(\d+,)*\d+((\.)\d+)?"#, false)
  }
}

pub trait ToSegments {

  /// Extract a vector of non-empty strings from a string-like object with a given separator
  /// excluding leading, trailing or double separators
  fn to_segments(&self, separator: &str) -> Vec<Self> where Self:Sized;

  /// Extract a vector of strings from a string-like object with a given separator
  fn to_parts(&self, separator: &str) -> Vec<Self> where Self:Sized;

  /// Extract only the head before the first occurrence of a separator
  fn to_head(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract only the last segment
  fn to_end(&self, separator: &str) -> Self  where Self:Sized;

  /// Extract a string-like segment identified by its index from the components of a string with a given separator
  /// e.g. String::from("10/11/2024") .to_segment(1) yields "11"
  fn to_segment(&self, separator: &str, index: i32) -> Option<Self>  where Self:Sized;

  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<Self>  where Self:Sized;

  /// extract the remainder after the head
  fn to_tail(&self, separator: &str) -> Self where Self:Sized;

  /// extract the first and last parts after the first occurrence of the separator
  fn to_head_tail(&self, separator: &str) -> (Self, Self)  where Self:Sized;

  /// extract the first and last parts after the last occurrence of the separator
  fn to_start_end(&self, separator: &str) -> (Self, Self)  where Self:Sized;

}

/// Converts arrays or vectors of strs to a vector of owned strings
pub trait ToStrings {
  fn to_strings(&self) -> Vec<String>;
}

impl<T: ToString> ToStrings for Vec<T> {
  /// Converts arrays or vectors of strs to a vector of owned strings
  fn to_strings(&self) -> Vec<String> {
      self.into_iter().map(|s| s.to_string()).collect()
  }
}

impl<T: ToString> ToStrings for [T] {
  fn to_strings(&self) -> Vec<String> {
      self.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }
}

/// Return the indices of all ocurrences of a character
pub trait MatchOccurrences {
  fn find_matched_indices(&self, pat: &str) -> Vec<usize>;
}



impl MatchOccurrences for String {
  fn find_matched_indices(&self, pat: &str) -> Vec<usize> {
    self.match_indices(pat).into_iter().map(|pair| pair.0).collect::<Vec<usize>>()
  }
}


impl PatternMatch for String {

  ///
  /// Simple regex-compatible match method that will return an optional boolean 
  /// - Some(true) means the regex is valid and the string matches
  /// - Some(false) means the regex is valid and the string does not match
  /// - None means the regex is not valid and can this not be evaluated
  /// 
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re)  => Ok(re.is_match(self)),
      Err(error) => Err(error)
    }
}

  ///
  /// Simpple regex-compatible match method that will return false 
  /// if the pattern does not match the source string or the regex fails
  /// 
  fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool {
      if let Ok(re) = build_regex(pattern, case_insensitive) {
        re.is_match(self)
      } else {
        false
      }
  }
}


impl PatternReplace for String {
  ///
  /// Optional regex-enabledd replace method that will return None if the regex fails
  /// 
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<String, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(re.replace_all(self, replacement).to_string()),
      Err(error) => Err(error)
    }
  }

  ///
  /// Simple regex-enabledd replace method that will return the same string if the regex fails
  /// 
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> String {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

}

impl StripCharacters for String {
    

  fn strip_non_chars(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_alphanumeric()).collect::<String>()
  }

  fn strip_non_digits(&self) -> String {
    self.chars().into_iter().filter(|c| c.is_digit(10)).collect::<String>()
  }

  /// Correct numeric strings with commas as thousand separators or as decimal separators
  /// to a regular format with punctuation only for decimal points before being parsed to an integer or float
  fn correct_numeric_string(&self, enforce_comma_separator: bool) -> Self {
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
        self.to_owned()
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

  fn to_numeric_strings(&self) -> Vec<String> {
    self.to_numeric_strings_conditional(false)
  }

  fn to_numeric_strings_euro(&self) -> Vec<String> {
    self.to_numeric_strings_conditional(true)
  }

  fn to_numbers_conditional<T: FromStr>(&self, enforce_comma_separator: bool) -> Vec<T> {
    self.to_numeric_strings_conditional(enforce_comma_separator).into_iter()
      .map(|s| s.parse::<T>())
      .filter(|s| s.is_ok())
      .map(|s| s.ok().unwrap())
      .collect::<Vec<T>>()
  }

  fn to_numbers<T: FromStr>(&self) -> Vec<T> {
    self.to_numbers_conditional(false)
  }

  fn to_numbers_euro<T: FromStr>(&self) -> Vec<T> {
    self.to_numbers_conditional(true)
  }

  fn to_first_number<T: FromStr + Copy>(&self) -> Option<T> {
    if let Some(number) = self.to_numbers::<T>().first() {
      Some(*number)
    } else {
      None
    }
  }

  fn to_first_number_euro<T: FromStr + Copy>(&self) -> Option<T> {
    if let Some(number) = self.to_numbers_euro::<T>().first() {
      Some(*number)
    } else {
      None
    }
  }

  fn strip_non_numeric(&self) -> String {
    self.to_numeric_strings().join(" ")
  }

}

impl CharGroupMatch for String {

  fn has_digits(&self) -> bool {
      self.chars().any(|c| char::is_digit(c, 10))
  }

  fn has_alphanumeric(&self) -> bool {
      self.chars().any(char::is_alphanumeric)
  }

  fn has_alphabetic(&self) -> bool {
    self.chars().any(char::is_alphabetic)
  }
}

impl PatternMatch for [String] {
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => Ok(self.into_iter().any(|segment| re.is_match(segment))),
      Err(error) => Err(error)
    }
  }

    ///
    /// Simpple regex-compatible match method that will return false 
    /// if the pattern does not match the source string or the regex fails
    /// 
    fn pattern_match(&self, pattern: &str, case_insensitive: bool) -> bool {
      self.pattern_match_result(pattern, case_insensitive).unwrap_or(false)
    }
}

impl PatternReplace for Vec<String> {
  ///
  /// Optional regex-enabledd replace method that will return None if the regex fails
  /// 
  fn pattern_replace_result(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Result<Vec<String>, Error> {
    match build_regex(pattern, case_insensitive) {
      Ok(re) => {
        let replacements = self.into_iter().map(|segment| re.replace_all(segment, replacement).to_string()).collect::<Vec<String>>();
        Ok(replacements)
      },
      Err(error) => Err(error)
    }
  }

  ///
  /// Simple regex-enabledd replace method that will return the same string if the regex fails
  /// 
  fn pattern_replace(&self, pattern: &str, replacement: &str, case_insensitive: bool) -> Vec<String> {
    self.pattern_replace_result(pattern, replacement, case_insensitive).unwrap_or(self.to_owned())
  }

}



impl ToSegments for String {
  fn to_parts(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }

  fn to_segments(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).filter(|s| s.len() > 0).collect::<Vec<String>>()
  }

  fn to_head(&self, separator: &str) -> String {
    if let Some((head, _tail)) = self.split_once(separator) {
      head.to_string()
    } else {
      self.to_owned()
    }
  }

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.ends_with(separator) && self.len() > separator_len {
      let end_index = self.len() - separator_len;
      self[0..end_index].to_string().to_end(separator)
    } else {
      self.to_end(separator)
    }
  }

  fn to_end(&self, separator: &str) -> String {
    let parts = self.to_parts(separator);
    if parts.len() > 0 {
      parts.last().unwrap_or(self).to_owned()
    } else {
      self.to_owned()
    }
  }

  fn to_tail(&self, separator: &str) -> String {
    let parts = self.to_parts(separator);
    let num_parts = parts.len();
    if num_parts > 0 {
      parts[1..num_parts].join(separator)
    } else {
      self.to_owned()
    }
  }

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.starts_with(separator) && self.len() > separator_len {
      self[separator_len..self.len()].to_string().to_head(separator)
    } else {
      self.to_head(separator)
    }
  }

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.starts_with(separator) && self.len() > separator_len {
      self[separator_len..].to_string().to_tail(separator)
    } else {
      self.to_tail(separator)
    }
  }
  
  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> String {
    let separator_len = separator.len();
    if self.ends_with(separator) && self.len() > separator_len {
      let end_index = self.len() - separator_len;
      self[0..end_index].to_string().to_tail(separator)
    } else {
      self.to_tail(separator)
    }
  }

  /// Extract an indexed segment yielded by splitting a string. 
  /// A negative index parameter will start from the end 
  fn to_segment(&self, separator: &str, index: i32) -> Option<String> {
    let parts = self.to_segments(separator);
    let num_parts = parts.len();
    let target_index = if index >= 0 { index as usize } else { (num_parts as i32 + index) as usize };
    if target_index < num_parts {
      if let Some(segment) = parts.get(target_index) {
        Some(segment.to_owned())
      } else {
        None
      }
    } else {
      None
    }
  }

  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String> {
    if groups.len() > 0 {
      let mut matched: Option<String> = None;
      let mut current_string = self.clone();
      for group in groups {
        if current_string.len() > 0 {
          let (separator, index) = group;
          matched = current_string.to_segment(*separator, *index);
          current_string = matched.clone().unwrap_or("".to_string());
        }
      }
      matched
    } else {
      None
    }
  }

  /// 
  /// Extract a tuple of the head and remainder, like split_once but returns Strings
  fn to_head_tail(&self, separator: &str) -> (String, String) {
    if let Some((head, tail)) = self.split_once(separator) {
      (head.to_string(), tail.to_string())
    } else {
      ("".to_owned(), self.to_owned())
    }
  }

  /// 
  /// Extract a tuple of the tail and remainder, like split_once in reverse and returning strings
  fn to_start_end(&self, separator: &str) -> (String, String) {
    let parts = self.to_parts(separator);
    let last_part = "".to_string();
    let num_parts = parts.len();
    if num_parts > 0 {
      let end_index = num_parts - 1;
      let start = parts[0..end_index].join(separator);
      let end = self.to_end(separator);
      (start, end)
    } else {
      (self.to_owned(), last_part)
    }
  }

}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_match_simple() {
    let source_str = "All living beings carry genes with harmful mutations.".to_string();
    let pattern = r"\bgenes?\b";
    assert!(source_str.pattern_match(pattern, true));
  }

  #[test]
  fn test_match_with_and_without_error() {
    let source_str = "All living beings carry genes with harmful mutations.".to_string();
    let pattern = r"\bgene(s?\b"; // bad regular expression
    assert!(source_str.pattern_match_result(pattern, true).is_err());
    let pattern = r"\bgene(s|z)?\b"; // good regular expression
    assert!(source_str.pattern_match_result(pattern, true).is_ok());
  }

  #[test]
  fn test_simple_replacement() {
    let source_str = "It measured 10cm long and 15cm wide".to_string();
    let pattern = r#"(\d+)\s*(cm)\b"#; 
    let replacement = "$1 centimetres";
    let target_str = "It measured 10 centimetres long and 15 centimetres wide".to_string();
    assert_eq!(source_str.pattern_replace(pattern, replacement,true), target_str);
  }

  #[test]
  fn test_match_in_string_array() {
    let source_strs: Vec<String>  = [
      "fisherman",
      "obbudsman", 
      "handyman"
    ].to_strings();
    let pattern = r#"\bhand\w"#; 
    assert!(source_strs.pattern_match(pattern, true));
    // should not match any of the above patterns
    let pattern2 = r#"\bpost\w"#; 
    assert_eq!(source_strs.pattern_match(pattern2, true), false);
  }

  #[test]
  fn test_vector_replacement() {
    let source_strs: Vec<String>  = vec![
      "fisherman",
      "obbudsman", 
      "handyman"
    ].to_strings();
    let pattern = r#"man\b"#; 
    let replacement = "woman";
    let target_strs: Vec<String>  = vec![
      "fisherwoman",
      "obbudswoman", 
      "handywoman"
    ].to_strings();
    assert_eq!(source_strs.pattern_replace(pattern, replacement,true),target_strs );
  }


  #[test]
  fn test_strip_non_chars() {
    let source_str = "Cañon, Zürich, Москва".to_string();
    let target_str = "CañonZürichМосква".to_string();
    assert_eq!(source_str.strip_non_chars(),target_str );
  }


  #[test]
  fn test_to_tail() {
    let source_str = "long/path/with-a-long-title/details".to_string();
    let target_str = "long".to_string();
    assert_eq!(source_str.to_inner_segment(&[("/", 2), ("-", 2)]), Some(target_str) );
  }

  #[test]
  fn test_to_inner_segment() {
    let source_str = "long/path/with-a-long-title/details".to_string();
    let target_str = "long".to_string();
    assert_eq!(source_str.to_inner_segment(&[("/", 2), ("-", 2)]), Some(target_str) );
    let source_str2 = "complex/pattern/with-many-nested|embedded-words".to_string();
    let target_str2 = "embedded".to_string();
    let pairs = [("/", 2), ("-", 2), ("|", 1)];
    assert_eq!(source_str2.to_inner_segment(&pairs), Some(target_str2) );
  }

  #[test]
  fn test_to_first() {
    let source_str = "/path/with/a/leading/slash".to_string();
    let target_str = "path".to_string();
    assert_eq!(source_str.to_first("/"), target_str );
    let source_str2 = "path/without/a/leading/slash".to_string();
    assert_eq!(source_str2.to_first("/"), target_str );
  }

  #[test]
  fn test_to_last() {
    let source_str = "/path/with/a/trailing/slash/".to_string();
    let target_str = "slash".to_string();
    assert_eq!(source_str.to_last("/"), target_str );
    let source_str2 = "/path/without/a/trailing/slash".to_string();
    assert_eq!(source_str2.to_last("/"), target_str );
  }

  #[test]
  fn test_to_head_tail() {
    let source_str = "comma,separated,string".to_string();
    let start = "comma".to_string();
    let end = "separated,string".to_string();
    assert_eq!(source_str.to_head_tail(","), (start, end) );
  }

  #[test]
  fn test_to_start_end() {
    let source_str = "comma,separated,string".to_string();
    let start = "comma,separated".to_string();
    let end = "string".to_string();
    assert_eq!(source_str.to_start_end(","), (start, end) );
  }

  #[test]
  fn test_array_str_to_vec_string() {
    let source_strs = [
      "one",
      "two",
      "three"
    ].to_strings();
    let target_vec = [
      "one",
      "two",
      "three"
    ].to_strings();
    assert_eq!(source_strs, target_vec );
  }

  #[test]
  fn test_char_group_matches() {
    let str1 = "I spent £12.50 on wine".to_string();

    assert!(str1.has_alphabetic());

    assert!(str1.has_digits());
    let str2 = "I bought a bottle of champagne for twenty pounds".to_string();
    // Deoes not contain digits
    assert!(str2.has_digits() == false);

    let str3 = "{-; _)(:-)}".to_string();
    // Deoes not contain letters os numbers
    assert!(str3.has_alphanumeric() == false);
    
  }

  #[test]
  fn test_strip_non_numeric() {
    let source_str = "I spent £9999.99 on 2 motorbikes at the age of 72.".to_string();
    let target_str = "9999.99 2 72".to_string();
    assert_eq!(source_str.strip_non_numeric(), target_str);
    // check if ythe above numbers parse successfully to numbers
    assert_eq!(source_str.to_numbers::<f64>(), vec![9999.99f64, 2f64, 72f64]);

    assert_eq!(source_str.to_first_number::<f32>().unwrap_or(0f32), 9999.99f32);

    let input_text = "I'd like 2.5lb of flour please".to_string();

    assert_eq!(input_text.to_first_number::<f32>().unwrap_or(0f32), 2.5f32);
    
    // Standard European price format. This is not ambiguous because both a dot and comma are both present
    let input_text = "Il conto è del 1.999,50€. Come vuole pagare?".to_string();
    assert_eq!(input_text.to_first_number::<f32>().unwrap_or(0f32), 1999.5f32);

    // Rounded amount in the European format. The absence of a secondary separator makes this
    // value ambigiuous
    let input_text = "Il furgone pesa 1.500kg".to_string();
    assert_eq!(input_text.to_first_number_euro::<u32>().unwrap_or(0), 1500);
  }

}