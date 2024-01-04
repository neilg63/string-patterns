use regex::*;

/// Build a regular expression with an optional case-insenistive non-capturing group
/// If the source pattern starts with a non-capturing group, this will be ignored irrespective of the case_insenistive flag
pub(crate) fn build_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, Error> {
  let mut parts: Vec<&str> = vec![];
  // do not case-insensitive flag if a similar flag is already in the regular expression
  if case_insensitive && pattern.starts_with("(?") == false {
    parts.push("(?i)");
  }
  parts.push(pattern);
  let regex_str = parts. concat();
  Regex::new(&regex_str)
}

/// Miscellaneous utility functions that do not belong to structs
pub(crate) fn add_sanitized_numeric_string(output: &mut Vec<String>, num_string: &str) {
  output.push(num_string.trim_end_matches(".").trim_end_matches(",").to_string());
}