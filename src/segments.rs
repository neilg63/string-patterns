/// Methods to split a longer strong on a separator and return a vector of strings,
/// a tuple of two strings or single optional string segment
/// Note some methods may return empty segments in the case of leading, trailing or repeated separators
/// See notes below
pub trait ToSegments {

  /// Extract a vector of non-empty strings from a string-like object with a given separator
  /// excluding leading, trailing or double separators
  fn to_segments(&self, separator: &str) -> Vec<String>;

  /// Extract a vector of strings from a string-like object with a given separator
  fn to_parts(&self, separator: &str) -> Vec<String>;

  /// Extract only the head before the first occurrence of a separator
  fn to_head(&self, separator: &str) -> String;

  /// Extract only the first segment before the first occurrence of a non-initial separator
  fn to_first(&self, separator: &str) -> String;

  /// Extract only the remainder after the first occurrence of a non-initial separator
  fn to_remainder_end(&self, separator: &str) -> String;

  /// Extract only the last segment after the last occurrence of a non-final separator
  fn to_last(&self, separator: &str) -> String;

  /// Extract only the beginning before the last segment following the last occurrence of a non-final separator
  fn to_remainder_start(&self, separator: &str) -> String;

  /// Extract only the last segment
  fn to_end(&self, separator: &str) -> String;

  /// Extract a string-like segment identified by its index from the components of a string with a given separator
  /// e.g. String::from("10/11/2024") .to_segment(1) yields "11"
  fn to_segment(&self, separator: &str, index: i32) -> Option<String>;

  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String>;

  /// extract the remainder after the head
  fn to_tail(&self, separator: &str) -> String;

  /// extract the first and last parts after the first occurrence of the separator
  fn to_head_tail(&self, separator: &str) -> (String, String);

  /// extract the first and last parts after the last occurrence of the separator
  fn to_start_end(&self, separator: &str) -> (String, String);

}

/// Implement string segment split and capture method for String
impl ToSegments for str {

  /// Splits a string on the exact separator, whether initial, final or repeated.
  /// May yield empty segments
  fn to_parts(&self, separator: &str) -> Vec<String> {
    let splitter = self.split(separator);
    splitter.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
  }

  /// Splits a string on a separator, but only returns an array of non-empty strings
  /// skipping leading, trailing or repeated separators that may otherwise yield empty strings
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

  /// extract the last segment whether empty or not
  fn to_end(&self, separator: &str) -> String {
    let parts = self.to_parts(separator);
    if let Some(end) = parts.last() {
      end.to_owned()
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

  /// extract an inner segment via a set of tuples with separators and indices.
  /// e.g. [("/", 1), ("-", 2)] applied to "pictures/holiday-france-1983/originals" 
  /// would match "1983" as an optional string
  fn to_inner_segment(&self, groups: &[(&str, i32)]) -> Option<String> {
    if groups.len() > 0 {
      let mut matched: Option<String> = None;
      let mut current_string = self.to_string();
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
    let num_parts = parts.len();
    if num_parts > 1 {
      let end_index = num_parts - 1;
      let start = parts[0..end_index].join(separator);
      let end = self.to_end(separator);
      (start, end)
    } else {
      (self.to_owned(), "".to_string())
    }
  }

}
