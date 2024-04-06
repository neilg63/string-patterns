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
fn test_simple_first_replacement() {
  let source_str = "The cat sat on the mat eating a rat".to_string();
  let pattern = r#"at"#; 
  let replacement = "æt";
  let target_str = "The cæt sat on the mat eating a rat".to_string();
  assert_eq!(source_str.pattern_replace_first_ci(pattern, replacement), target_str);
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
  let source_strs = [
    "fisherman",
    "obbudsman", 
    "handyman"
  ];
  let pattern = r#"\bhand\w"#; 
  assert!(source_strs.pattern_match(pattern, true));
  // should not match any of the above patterns
  let pattern2 = r#"\bpost\w"#; 
  assert_eq!(source_strs.pattern_match(pattern2, true), false);
}

#[test]
fn test_vector_replacement() {
  let source_strs: Vec<String>  = [
    "fisherman",
    "obbudsman", 
    "handyman"
  ].into_iter().map(|s| s.to_string()).collect();
  let pattern = r#"man\b"#; 
  let replacement = "woman";
  let target_strs: Vec<String>  = vec![
    "fisherwoman",
    "obbudswoman", 
    "handywoman"
  ].into_iter().map(|s| s.to_string()).collect();
  assert_eq!(source_strs.pattern_replace(pattern, replacement,true),target_strs );
}

#[test]
fn test_match_all() {
  let str1 = "The 1950s proved to be a regeneration for Armstrong as both a musician and a public figure.";

  let match_patterns = [r#"\bmusician\b"#, r#"\bpublic\b"#];
  assert!(str1.pattern_match_all_ci(&match_patterns));

  let match_patterns_2 = [r#"\bmusician\b"#, r#"\bjazz\b"#];
  // Does not contains all of the above
  
  assert_eq!(str1.pattern_match_all_ci(&match_patterns_2), false);

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
fn test_replace_many() {
  let sample_text = "I have five lions and twelve wolves in my safari park.".to_string();
  let replacement_pairs = [("lions", "cats"), ("wolves", "dogs"), ("safari park", "garden")];
  let expected_text = "I have five cats and twelve dogs in my garden.".to_string();
  assert_eq!(sample_text.pattern_replace_pairs_ci(&replacement_pairs), expected_text);
  // now test multiple replacements on vectors of strings
  let sample_strings = ["sheepwolves", "wild lions", "safari parks"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
  let converted_strings = sample_strings.pattern_replace_pairs_ci(&replacement_pairs);
  let expected_strings = ["sheepdogs", "wild cats", "gardens"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
  for i in 0..converted_strings.len() {
    assert_eq!(converted_strings.get(i).unwrap().to_owned(), expected_strings.get(i).unwrap().to_owned());
  }
}


#[test]
fn test_pattern_matches_in_arrays() {
  let phrases = [
    "The eyesight of a dog is not as keen as its sense of smell",
    "The fourth critical stage in a puppy’s development is between 12 and 16 weeks",
    "The heat cycle of the female lasts from 18 to 21 days",
    "Most bitches whelp normally."
  ];
  let good_regex = "puppy";
  let expected_matches = vec![false, true, false, false];
  assert_eq!(phrases.pattern_matches_ci(good_regex), expected_matches);
  let owned_phrases: Vec<String> = phrases.into_iter().map(|s| s.to_string()).collect();
  assert_eq!(owned_phrases.pattern_matches_ci(good_regex), expected_matches);
  
  let bad_regex = r#"(eye"#;
  assert!(phrases.pattern_matches_result(bad_regex, true).is_err());
  // show return a vector of false results with the same length as the original array or vector
  assert_eq!(phrases.pattern_matches_ci(bad_regex).len(), phrases.len());
  // works on a vector of &str values too
  let letter_pattern = "[ao]g";
  let sample_str_vec = vec!["cat", "BAG", "dog", "frog", "leg", "twig", "brag"];
  let expected_matches = vec![false, true, true, true, false, false, true];
  assert_eq!(sample_str_vec.pattern_matches_ci(letter_pattern), expected_matches);
  let expected_matched_items = vec!["BAG", "dog", "frog", "brag"];
  assert_eq!(sample_str_vec.pattern_matches_filtered_ci(letter_pattern), expected_matched_items);
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

#[test]
fn test_build_regex() {
  // test if build_regex compiles
  let pattern_1 = r#"\bb[aeiou]llsh[aeiou]t\b"#;
  let regex_1 = build_regex(pattern_1, true);
  assert!(regex_1.is_ok());

  // test if build_regex handles errors correctly
  let pattern_2 = r#"\bb[aeiou]llsh[aeiout\b"#;
  let regex_2 = build_regex(pattern_2, true);
  assert!(regex_2.is_err());
}

#[test]
fn test_pattern_capture() {
  let sample_str = "We sat on the sofa together like couch potatoes.";
  let pattern = r#"\b(couch|sofa)\b"#;
  let captures = sample_str.pattern_matches_vec(pattern, true);
  assert_eq!(captures.len(), 2);

  let first_match = captures.get(0).unwrap(); 
  let second_match = captures.get(1).unwrap(); 
  assert_eq!(first_match.start(), 14);
  assert_eq!(first_match.end(), 18);
  assert_eq!(first_match.as_str(), "sofa");
  assert_eq!(second_match.start(), 33);
  assert_eq!(second_match.end(), 38);
  assert_eq!(second_match.as_str(), "couch");

  let captures = sample_str.pattern_captures(pattern, true);
  assert_eq!(captures.unwrap().len(), 2);
  
}


#[test]
fn test_pattern_count_words() {
  let sample_str = "He kept saying the same words over and over again.";
  
  assert_eq!(sample_str.count_word("over", true), 2);
}

#[test]
fn test_pattern_match_all_conditional() {
  
  // contains "android" and "linux", but does not conatain iphone: Must be Android
  let pattern_sets_android = [
    (true, "android", true),
    (true, "linux", true),
    (false, "iphone", true),
  ];

  // contains "iphone", but does not conatain linux
  let pattern_sets_apple = [
    (true, "iphone", true),
    (false, "linux", true),
  ];

  let sample_1 = "Mozilla/5.0 (Linux; Android 13; SM-S908U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Mobile Safari/537.36";

  let sample_2 = "Mozilla/5.0 (iPhone14,6; U; CPU iPhone OS 15_4 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/19E241 Safari/602.1";
  
  assert!(sample_1.pattern_match_all_conditional(&pattern_sets_android));

  assert!(sample_2.pattern_match_all_conditional(&pattern_sets_apple));

  // Alternatively test the boolean results of arrays of case-insensitive whole words 
  let words = [
    "android",
    "linux",
    "iphone",
    "mac",
  ];
  // test if the user agent string matches an Android phone
  assert_eq!(sample_1.pattern_word_matches_conditional_ci(&words), vec![true, true, false, false]);

  // test if the user agent string matches an Apple iPhone
  assert_eq!(sample_2.pattern_word_matches_conditional_ci(&words), vec![false, false, true, true]);

}

#[test]
fn test_pattern_filter() {


  // Alternatively test the boolean results of arrays of case-insensitive whole words 
  let phrases = [
    "Blackberries",
    "White board",
    "Grey scale",
    "Blackbirds",
  ];
  let filtered_phrases = vec![
    "Blackberries",
    "Blackbirds",
  ];
  let pattern = "black";
  // test if the user agent string matches an Android phone
  assert_eq!(phrases.pattern_filter_ci(pattern), filtered_phrases);

  

}

