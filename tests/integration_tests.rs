use string_patterns::{enums::StringBounds, *};

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

 let groups = [("/", 1), ("-", 2)];
 let file_path = "pictures/holiday-france-1983/originals";
 let current_year: i32 = 2024;  
 let invalid_age: i32 = 0;
 let expected_age: i32 = 41;
 let matched_year = if let Some(year_string) = file_path.to_inner_segment(&groups) {
  // only parse age if matched, standard parse() is fine, but to_first_number() will strip any characters before or after the first number.
  year_string.parse::<i32>().unwrap_or(invalid_age)
 } else {
  invalid_age
 }; // should yield 1983
 assert_eq!(current_year - matched_year, expected_age);
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
  let source_str = "comma,separated,string";
  let start = "comma,separated".to_string();
  let end = "string".to_string();
  assert_eq!(source_str.to_start_end(","), (start, end) );
  let source_str = "one-item".to_string();
  let empty_end = "".to_string();
  assert_eq!(source_str.to_start_end(","), (source_str, empty_end) );
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
  let sample_strings = ["sheepwolves", "wild lions", "safari parks"].to_strings();
  let converted_strings = sample_strings.pattern_replace_pairs_ci(&replacement_pairs);
  let expected_strings = ["sheepdogs", "wild cats", "gardens"].to_strings();
  for i in 0..converted_strings.len() {
    assert_eq!(converted_strings.get(i).unwrap().to_owned(), expected_strings.get(i).unwrap().to_owned());
  }
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
  let owned_phrases = phrases.to_strings();
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
fn test_is_numeric() {
  let num_str_1 = "-1227.75";
  assert!(num_str_1.is_numeric());
  
  let num_str_2 = "-1,227.75"; // will not validate with commas, unless corrected
  assert_eq!(num_str_2.is_numeric(), false);
  // &str has to be cast to an owned String first
  assert!(num_str_2.correct_numeric_string(false).is_numeric());

  let num_str_3 = "-1.227,75"; // European-style with commas as decimal separators
  assert!(num_str_3.correct_numeric_string(true).is_numeric());

  let num_str_4 = "$19.99 each"; // Should fail, as this will not parse directly to a float
  assert!(!num_str_4.is_numeric());
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
  let source_str = "I spent £9999.99 on 2 motorbikes at the age of 72.";
  let target_str = "9999.99 2 72".to_string();
  assert_eq!(source_str.strip_non_numeric(), target_str);

  
  let target_str = "Ispent999999on2motorbikesattheageof72".to_string();
  assert_eq!(source_str.strip_non_alphanum(), target_str);
  // check if ythe above numbers parse successfully to numbers
  assert_eq!(source_str.to_numbers::<f64>(), vec![9999.99f64, 2f64, 72f64]);

  assert_eq!(source_str.to_first_number::<f32>().unwrap_or(0f32), 9999.99f32);

  let input_text = "I'd like 2.5lb of flour please";

  assert_eq!(input_text.to_first_number::<f32>().unwrap_or(0f32), 2.5f32);
  
  // Standard European price format. This is not ambiguous because both a dot and comma are both present
  let input_text = "Il conto è del 1.999,50€. Come vuole pagare?";
  assert_eq!(input_text.to_first_number::<f32>().unwrap_or(0f32), 1999.5f32);

  // Rounded amount in the European format. The absence of a secondary separator makes this
  // value ambigiuous
  let input_text = "Il furgone pesa 1.500kg";
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

#[test]
fn test_pattern_split_to_numbers() {
  let input_str = "-78.29826, 34.15 160.9";

  let numbers = input_str.pattern_split_cs(r#"(\s*,\s*|\s+)"#)
    .into_iter().map(|s| s.to_first_number::<f64>())
    .filter(|nr| nr.is_some())
    .map(|s| s.unwrap()).collect::<Vec<f64>>();

  let first_number = numbers.get(0).unwrap_or(&0f64).to_owned();
  let second_number = numbers.get(1).unwrap_or(&0f64).to_owned();
  let third_number = numbers.get(2).unwrap_or(&0f64).to_owned();
  assert!(first_number < -78f64 && first_number > -79f64);
  assert_eq!(second_number, 34.15f64);
  assert_eq!(third_number, 160.9f64);
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
fn test_matched_conditional() {
  let conditions = [
    StringBounds::StartsWithCi("jan", true),
    StringBounds::EndsWithCi("images", true),
    StringBounds::ContainsCi("2023", true),
  ];

  let folder_1 = "Jan_2023_IMAGES";

  let folder_2 = "january_2024_Images";

  assert_eq!(folder_1.matched_conditional(&conditions), vec![true, true, true]);

  assert!(folder_1.match_all_conditional(&conditions));

  assert_eq!(folder_2.matched_conditional(&conditions), vec![true, true, false]);

  let test_strs = ["image", "cat", "garden"];

  let folder_3 = "cat-IMAGES_Garden";
  let folder_4 = "images-of-cats-and-dogs-in-the-park";

  assert!(folder_3.contains_all_conditional_ci(&test_strs));
  // the second folder should not match all conditions
  assert_eq!(folder_4.contains_all_conditional_ci(&test_strs), false);

  let file_names = [
    "edited-img-Nepal-Feb-2003.psd",
    "image-Thailand-Mar-2003.jpg",
    "photo_Nepal_Jan-2005.jpg",
    "image-India-Mar-2003.jpg",
    "pic_nepal_Dec-2004.png"
  ];

  let mixed_conditions = [
    StringBounds::ContainsCi("nepal", true),
    StringBounds::EndsWithCi(".psd", false),
  ];

  let file_name_a = file_names[0];
  let file_name_b = file_names[2];

  assert!(file_name_a.match_all_conditional(&mixed_conditions) == false);

  assert!(file_name_b.match_all_conditional(&mixed_conditions));
  
  let nepal_jpg_files: Vec<&str> = file_names.filter_all_conditional(&mixed_conditions);

  assert_eq!(nepal_jpg_files.len(), 2);

  assert_eq!(nepal_jpg_files[0], file_name_b);

  let file_names_vector = file_names.to_strings();

  let nepal_jpg_files_vector: Vec<&str> = file_names_vector.filter_all_conditional(&mixed_conditions);
  
  assert_eq!(nepal_jpg_files_vector.len(), 2);

}