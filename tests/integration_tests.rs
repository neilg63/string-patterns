use string_patterns::*;

#[cfg(test)]

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
fn test_case_insensitive_replacement() {
  let source_str = "I bought two apples in Africa".to_string();
  let pattern = r#"\ba"#; // only word-initial a
  let replacement = "ɐ";
  let target_str = "I bought two ɐpples in ɐfrica".to_string();
  assert_eq!(source_str.pattern_replace_ci(pattern, replacement), target_str);
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
  assert_eq!(source_str.strip_non_alphanum(),target_str );
}

#[test]
fn test_segment_match() {
  let path_string = "/var/www/mysite.com/web/uploads".to_string();
  let domain = path_string.to_segment("/",2).unwrap_or("".to_string()); 
  let expected_string = "mysite.com".to_string();
  assert_eq!(domain, expected_string);
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
  // Does not contain letters os numbers
  assert!(str3.has_alphanumeric() == false);
  
}

#[test]
fn test_match_many() {
  let str1 = "The 1950s proved to be a regeneration for Armstrong as both a musician and a public figure.";

  let match_patterns = [r#"\bmusician\b"#, r#"\bpublic\b"#];
  assert!(str1.pattern_match_many_ci(&match_patterns));

  let match_patterns_2 = [r#"\bmusician\b"#, r#"\bjazz\b"#];
  // Does not contains all of the above
  
  assert_eq!(str1.pattern_match_many_ci(&match_patterns_2), false);

  // Contains at least one of the above
  assert!(str1.pattern_match_any_ci(&match_patterns_2));
  
}

#[test]
fn test_simple_pattern_matches() {
  let str1 = "Picture_of my cat-2018.PNG";

  let pattern_1 = "pictureof";
  assert!(str1.starts_with_ci_alphanum(pattern_1));

  let pattern_2 = "mycat";
  assert!(str1.contains_ci_alphanum(pattern_2));

  // Ends with .png with upper, lower or mixed case letters
  assert!(str1.ends_with_ci(".png"));
  
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
