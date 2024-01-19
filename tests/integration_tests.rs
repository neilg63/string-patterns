use string_patterns::{*, enums::WordBounds};

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
  let path_string = "/var/www/mysite.com/web/uploads";
  // ignore leading slash
  let domain = path_string.to_segment("/",2).unwrap_or("".to_string()); 
  let expected_string = "mysite.com".to_string();
  assert_eq!(domain, expected_string);
}

#[test]
fn test_to_string_vector() {
  // ignore leading slash
  let path_string = "/var/www/mysite.com/web/uploads";
  let segments = path_string.to_segments("/"); 
  assert_eq!(segments.len(), 5);
  let fourth_element = segments.get(3).unwrap().to_owned();
  let expected_string = "web".to_owned();
  assert_eq!(fourth_element, expected_string);
}

#[test]
fn test_to_segments() {
  let path_string = "/var/www/mysite.com/web/uploads/".to_string();
  // should extract only non-empty segments
  let segments = path_string.to_segments("/"); 
  let expected_segments = ["var", "www", "mysite.com", "web", "uploads"].to_strings();
  assert_eq!(segments, expected_segments);
  // convert all parts split by a separator whether empty or not
  let parts = path_string.to_parts("/"); 
  let expected_parts = ["", "var", "www", "mysite.com", "web", "uploads", ""].to_strings();
  assert_eq!(parts, expected_parts);
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
fn test_replace_many_words() {
  let sample_text = "Twenty cats lived in the large mansion.".to_string();
  let replacement_pairs = [("cats", "dogs"), ("mansion", "hut"), ("large", "tiny")];
  let expected_text = "Twenty dogs lived in the tiny hut.".to_string();
  assert_eq!(sample_text.replace_words_ci(&replacement_pairs), expected_text);
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
fn test_is_numeric() {
  let num_str_1 = "-1227.75";
  assert!(num_str_1.is_numeric());
  
  let num_str_2 = "-1,227.75"; // will not validate with commas, unless corrected
  assert_eq!(num_str_2.is_numeric(), false);
  // &str has to be cast to an owned String first
  assert!(num_str_2.to_owned().correct_numeric_string(false).is_numeric());

  let num_str_3 = "-1.227,75"; // European-style with commas as decimal separators
  assert!(num_str_3.to_owned().correct_numeric_string(true).is_numeric());
}

#[test]
fn test_has_digits() {
  // Does this have a valid decimal digit sequence that may be extracted as a valid number
  let num_str_1 = "serial number: 93025371";
  assert!(num_str_1.has_digits());

  // Is this a valid decimal digit sequence that may be cast to an integer
  let num_str_1 = "93025371";
  assert!(num_str_1.is_digits_only());
  
  // Is this a valid hexadecimal string that may be cast to a float via from_str_radix(16)
  let num_str_2 = "1ec9F9a";
  assert!(num_str_2.is_digits_only_radix(16));
}

#[test]
fn test_match_ocurrences() {
  // As this works on literal strs/Strings only it may only match a set number of characters
  let str = "The fox jumped out of the box into the mixing bowl.";
  
  let x_indices = str.find_matched_indices("x");
  let expected_x_indices: Vec<usize> = vec![6, 28, 41];
  assert_eq!(x_indices, expected_x_indices);

  let ox_indices = str.find_matched_indices("ox");
  let expected_ox_indices: Vec<usize> = vec![5, 27];
  assert_eq!(ox_indices, expected_ox_indices);
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

#[test]
fn test_correct_floats() {
  let source_str = "Ho pagato 15,00€ per l'ingresso.".to_string();
  // with numbers that can be corrected parsed and cast to floats
  let target_str = "Ho pagato 15.00€ per l'ingresso.".to_string(); 
  // Correct a euro-style number and always interpret commas as decimal separators.
  assert_eq!(source_str.correct_numeric_string(true), target_str);

  let source_str_2 = "Pesa 1.678 grammi".to_string(); 
  let target_str_2 = "Pesa 1678 grammi".to_string(); // do not use in longer phrases with commas and dots as punctuation
  // Correct a euro-style number and always interpret commas as decimal separators.
  assert_eq!(source_str_2.correct_numeric_string(true), target_str_2);

  let sample_str = "Ho pagato 12,50€ per 1.500 grammi di sale.".to_string();
  // with numbers that can be corrected parsed and cast to floats
  let target_numbers = vec![12.5f32, 1500f32]; 
  // Correct euro-style numbers and convert to 32-bit floats
  assert_eq!(sample_str.to_numbers_euro::<f32>(), target_numbers);
}

#[test]
fn test_match_word() {
  let source_str = "Lions are unique among cats in that they live in a group or pride.";
  let target_word = "lions?"; // optional s at the end
  assert!(source_str.match_word_ci(target_word));
  // check if the above numbers parse successfully to numbers
  assert!(source_str.match_word_start_ci("uniq"));

  assert!(source_str.match_word_end_ci("nique"));

  assert!(source_str.match_word_bounds("cats", WordBounds::Both, true));

  // lion and cat must occur within 20 letters of each other
  assert!(source_str.match_words_by_proximity("lions", "cats", -20, 20, true));
}

#[test]
fn test_match_count() {
  let long_text = r#"Newborn cubs are helpless and blind and have a thick coat with dark spots that usually disappear with maturity.
      Cubs are able to follow their mothers at about three months of age and are weaned by six or seven months.
      They begin participating in kills by 11 months but probably cannot survive on their own until they are two years old. 
      Although lionesses will nurse cubs other than their own, 
      they are surprisingly inattentive mothers and often leave their cubs alone for up to 24 hours."#;
  assert_eq!(long_text.count_pattern(r#"\s\d\d\b"#, false), 2); // two-digit numerals
  assert_eq!(long_text.count_word("cubs?", true), 4); // occurrences of cub or cubs with any upper, lower or mixed case letters
  // check if the above numbers parse successfully to numbers

  let sample_text = r#"Humpty Dumpty sat on a wall,
          Humpty Dumpty had a great fall
          All the king's horses and all the king's men
          Couldn't put Humpty together again."#;
   assert_eq!(sample_text.count_word("humpty", true), 3);
  
}

#[test]
fn test_first_match() {
  let sample_text = r#"Trout belong mainly to two genera: Oncorhynchus and Salvelinus."#;
  let pattern = r#"\bonco\w+us\b?"#;
  let target_matched_str = "Oncorhynchus";
  let matched_item = sample_text.pattern_first_match(pattern, true);
  assert_eq!(matched_item.unwrap().as_str(), target_matched_str); 
}

#[test]
fn test_replace_word() {
  let sample_text = "On one of the darkest days of the year she fled on a boat";
  // This is a contrived example, only `on` should be replaced and not one
  let target_text = "upon one of the darkest days of the year she fled upon a boat";
  let word = "on";
  let replacement = "upon";
  let replacement_string = sample_text.to_string().replace_word_ci(word, replacement);
  assert_eq!(replacement_string, target_text.to_string()); 
}

#[test]
fn match_all_or_any_words() {
  let sample_text = "A species of teleost fish usually lives in only one kind of habitat at any stage of its life cycle.";
  let words = ["fish", "habitat", "trout"];
  assert_eq!(sample_text.match_words_ci(&words), false); // does not contain trout and should be false
  assert!(sample_text.match_any_words_ci(&words)); 
}

#[test]
fn test_first_match_count() {
  let sample_text = r#"Lionesses living in open savanna do most of the hunting, whereas males typically appropriate their meals from the female’s kills"#;
  
  // Method return full details of the first match for subsequent manipulation
  let matched_item = sample_text.pattern_first_match(r#"\bsavannah?"#, true);
  assert_eq!(matched_item.unwrap().start(), 25); // The first occurence should start at position 25

  // convenience method if only need the end index of the first match
  let first_end_index = sample_text.pattern_first_end_index(r#"\bsavannah?"#, true);
  assert_eq!(first_end_index.unwrap(), 32); // The first occurence should end at position 32
  
  // check if the above numbers parse successfully to numbers
  
}

#[test]
fn test_word_bounds() {
  let sample_text = r#"Lionesses living in open savanna do most of the hunting"#;
  
  // Check Word bounds is accessible externally
  let is_matched = sample_text.match_word_bounds(r#"savan"#, WordBounds::Start, true);
  assert!(is_matched); 
  
}


#[test]
fn test_split_on_pattern() {
  let sample_text = r#"fifteen,thousand;and;eighty-two"#;
  
  // Check Word bounds is accessible externally
  let result = sample_text.pattern_split(r#"[,;-]"#, false);
  let part_4_opt = result.get(3);
  let expected_part_4 = "eighty".to_string();
  assert_eq!(part_4_opt.unwrap().to_owned(), expected_part_4); 
  
  let (head, tail) = sample_text.pattern_split_pair_cs(r#"[,;-]"#);
  let expected_head = "fifteen".to_string();
  let expected_tail = "thousand;and;eighty-two".to_string();
  assert_eq!(head, expected_head); 
  assert_eq!(tail, expected_tail); 

  let sample_string = "first / second - third ; fourth";
  let pattern = r#"\s*[/;-]\s*"#;
  let (head, tail) = sample_string.pattern_split_pair_cs(pattern); 
  let expected_head = "first".to_string();
  let expected_tail = "second - third ; fourth".to_string();
  assert_eq!(head, expected_head); 
  assert_eq!(tail, expected_tail); 
}