[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/string-patterns)
[![crates.io](https://img.shields.io/crates/v/string-patterns.svg)](https://crates.io/crates/string-patterns)
[![docs.rs](https://docs.rs/string-patterns/badge.svg)](https://docs.rs/string-patterns)

# String Patterns

This library makes it easier to process strings in Rust. It builds on Rust's standard library with help from its default regular expression crate, *[regex](https://crates.io/crates/regex)*. It has no other dependencies. It aims to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax. Simpler string matching methods such as starts_with, contains or ends_with will always perform better, especially when processing large data sets. To this end, the crate provides methods such as *starts_with_ci* and *starts_with_ci_alphanum* for basic string validation without regular expressions as well as extension methods to split strings into vectors of strings or a *head* and *tail* components.

### Method overview
- All pattern-prefixed methods use regular expressions via the Regex crate
- All other extension methods use standard library functions only to match, remove or extract character sequences.
- Methods ending in _result return a Result with a regex::Error if the regular expression fails
- Many methods without *_ci* or *_cs* suffixes require a boolean *case_insensitive* parameter
- Methods ending in *_cs* are case-sensitive
- Methods ending in *_ci* are case-insensitive
- Methods containing *_word(s)_* match whole or partial words depending on boundary rules
- Methods containing *_match_many_* require all patterns within an array to match
- Methods containing *_match_any_* return true if any of the patterns within an array match
- Methods containing *split* return either a vector or tuple pair.
- Methods containing *_part(s)* always include leading or trailing separators and may return empty elements in vectors
- Methods containing *segment(s)* ignore leading, trailing, repeated consecutive separators and thus exclude empty elements
- In tuples returned from *segment(s)* and *part(s)* methods, *head* means the segment before the first split and tail the remainder, while *start* means the whole string before the last split and *end* only the last part of the last matched separator.

Version 0.2.0 introduced additional methods to capture and count matched strings with offsets and version 0.2.5 added methods to match, replace, capture and count words without intrusive word boundary anchors.

The is_numeric() method, in the *IsNumeric* trait, applies a strict regex-free check on compatibility with the *parse()* method. It differs from char::is_numeric which checks for digit-like characters only and does not match minus or decimal points. Parallel *has_digits* and *has_digits_only* methods are implemented for the *CharGroupMatch* trait to test only for unsigned integers. An example below shows you how to combine *pattern_split* and *to_first_number* to capture numbers within longer texts as floats.

In case-insensitive mode the non-capturing **/(?i)/** flag is prepended automatically, but omitted if you add another non-capturing group at the start of your regular expression. In every other way, the pattern-prefixed methods behave like *re.is_match*, *re.replace_all*, *re.find* and *re.capture_iter* methods in the Regex crate. String-patterns unleashes most of the core functionality of the Regex crate, on which it depends, to cover most common use cases in text processing and to act as a building block for specific validators (e.g. email validation) and text transformers. 

Most of the *match* methods will work on *&str* and *String*, while the replacement methods are only implemented for *owned strings*. Likewise, match methods are implemented for arrays and vectors of strings, while replacement methods are only implemented for vectors of *owned strings*. The traits may be implemented for structs or tuples with a string field. Version 2.10 added PatternSplit with results as String vectors or tuples.

##### Regular expression match in standard Rust with the Regex library
```rust

fn is_valid_time_string(input: &str) -> bool {
  let time_format_pattern = r#"^([01]\d|2[0-3])?:[0-5]\d(:[0-5]\d)?$"#;
  if let Ok(re) = Regex::new(time_format_pattern) {
    re.is_match(input)
  } else {
    false
  }
}
```

##### More concise syntax with the string-patterns library
```rust
fn is_valid_time_string(input: &str) -> bool {
  input.pattern_match_cs(r#"^([01]\d|2[0-3])?:[0-5]\d(:[0-5]\d)?$"#)
}
```

##### Sample replacement in standard Rust with the Regex library
```rust

fn replace_final_os(input: &str) -> String {
  let regex_str = r#"(\w)o\b"#;
  if let Ok(re) = Regex::new(regex_str) {
    re.replace_all(input, "${1}um").to_string()
  } else {
    input.to_string()
  }
}
```

##### More concise syntax with the string-patterns library
```rust

fn replace_final_os(input: &str) -> String {
  // case-insensitive replacement,
  // NB: the regex syntax and capture rules are enforced by the Regex library
  input.to_string().pattern_replace_ci(r#"(\w)o\b"#, "${1}um") 
}
```

##### Simple case-insensitive match on string value
```rust
let str_1 = "Dog food";
if str_1.starts_with_ci("dog") {
  println!("{} is dog-related", str_1);
}
```

##### Simple case-insensitive match on the alphanumeric characters only in a longer string
```rust
// This method is handy for validating text values from external data sources with
// inconsistent naming conventions, e.g. first-name, first_name, firstName or "first name"
let str_1 = "Do you spell hip-hop with a hyphen?";
if str_1.contains_ci_alphanum("hiphop") {
  println!("{} is hip-hop-related", str_1);
}
```

##### Extract the first match from a string
```rust
let str_1 = "The park has many lions, spotted hyenas, leopards, rhinoceroses, hippopotamuses, giraffes, cheetahs and baboons";
if let Some(matched_item) = str_1.pattern_first_match(r#"\bspotted\s+\w+\b"#, true) {
  println!("`{}` occurs between positions {} and {}", matched_item.as_str(), matched_item.start(), matched_item.end());
}
```

##### Count matches of a pattern
```rust
let sample_text = r#"Humpty Dumpty sat on a wall,
          Humpty Dumpty had a great fall
          All the king's horses and all the king's men
          Couldn't put Humpty together again."#;
  let sample_word = "humpty";
  // count the number of whole words in case-insensitive mode
  let num_occurrences = sample_text.count_word("humpty", true);
  println!("{} occurs {} times in the above text", sample_word, num_occurrences );
```

##### Replace text in a vector of strings
```rust
let sample_strings = ["apples", "bananas", "carrots", "dates"].to_strings(); /// cast to vector of owned strings
let pattern = r#"a([pr])"#;
let replacement = "æ$1";
// With arrays or vectors the regex need only be compiled once
// case-insensitive replacement
let new_strings = sample_strings.pattern_replace_ci(pattern, replacement); 
/// should yield the strings "æpples", "bananas", "cærrots", "dates"
/// only replacing 'a' with 'æ' before 'p' or 'r'
```

##### Replace multiple pattern/replacement pairs 
```rust
let source_str = "Colourful fishing boats adorned the island's harbours.".to_string();
  let pattern_replacements = [
    ("colour", "color"),
    ("harbour", "harbor"),
  ];
/// Should read "Colorful fishing boats adorned the island's harbors"
let target_str = source_str.pattern_replace_pairs_cs(&pattern_replacements); 
// NB: Prior to version 0.2.19  this was pattern_replace_pairs()
// which now requires a second parameter
```

##### Replace multiple word pairs in case-sensitive mode
```rust
/// This should have the same result as above but with cleaner and less error-prone syntax
let source_str = "The dying King Edmund decides to try to save Lear and Cordelia.";
  let pattern_replacements = [
    ("Edmund", "Edward"),
    ("Lear", "Larry"),
    ("Cordelia", "Cecilia")
  ];
/// Should read "The dying King Edward decides to try to save Larry and Cecilia."
let target_str = source_str.to_string().replace_words_cs(&pattern_replacements); 
```

##### Match any words in case-insensitive mode
```rust
let source_str = "Two cheetahs ran across the field";
let cat_like_words = [
  "lions?","tigers?", "pumas?",
  "panthers?", "jaguars?", "leopards?",
  "lynx(es)?", "cheetahs?"
];
if source_str.match_any_words_ci(&cat_like_words) {
  println!("`{}` is related to cats", source_str);
}
```

##### Extract the third non-empty segment of a long path name
```rust
let path_string = "/var/www/mysite.com/web/uploads".to_string();
if let Some(domain) = path_string.to_segment("/", 2) {
  println!("The domain folder name is: {}", domain); // "mysite.com" is an owned string
}
```

##### Extract the *head and tail* or *start and end* from a longer string 
```rust
let test_string = "long-list-of-technical-words".to_string();
let (head, tail) = test_string.to_head_tail("-");
println!("Head: {}, tail: {}", head, tail); // Head: long, tail: list-of-technical-words

let (start, end) = test_string.to_start_end("-");
println!("Start: {}, end: {}", start, end); // Start: long-list-of-technical, end: words
```

##### Extract the first decimal value as an f64 from a longer string
```rust
const GBP_TO_EURO: f64 = 0.835;

let sample_str = "Price £12.50 each".to_string();
if let Some(price_gbp) = sample_str.to_first_number::<f64>() {
    let price_eur = price_gbp / GBP_TO_EURO;
    println!("The price in euros is {:.2}", price_eur);
}
```

##### Extract numeric sequences from phrases and convert them to a vector of floats
```rust
// extract European-style numbers with commas as decimal separators and points as thousand separators
let sample_str = "2.500 grammi di farina costa 9,90€ al supermercato.".to_string();
  let numbers: Vec<f32> = sample_str.to_numbers_euro();
  // If two valid numbers are matched assume the first is the weight
  if numbers.len() > 1 {
    let weight_grams = numbers[0];
    let price_euros = numbers[1];
    let price_per_kg = price_euros / (weight_grams / 1000f32);
    // the price in kg should be 3.96
    println!("Flour costs €{:.2} per kilo", price_per_kg);
  }
```

##### Extract three float values from a longer string
```rust

let input_str = "-78.29826, 34.15 160.9";
// the pattern expects valid decimal numbers separated by commas and/or one or more spaces
let split_pattern = r#"(\s*,\s*|\s+)"#;

let numbers: Vec<f64> = input_str.pattern_split_cs(split_pattern)
    .into_iter().map(|s| s.to_first_number::<f64>())
    .filter(|nr| nr.is_some())
    .map(|s| s.unwrap()).collect();
// yields a vector of three f64 numbers [-78.29826, 34.15, 160.9];
```

##### Test the proximity of two words
```rust
let source_str = "Lions are unique among cats in that they live in a group or pride.";
// Do the words 'lion(s)' and 'cat(s)' occur within 20 characters of each other?
if source_str.match_words_by_proximity("lions?", "cats?", -20, 20, true) {
  println!("This sentence mentions lions in the context of cats");
}
```


##### Split a string on a pattern
```rust
let sample_string = "books, records and videotapes";
let pattern = r#"\s*(,|and)\s"#;
 // case-insensitive split
let items = sample_string.pattern_split_ci(pattern);
// should yield a vector of strings: "books", "records", "videotapes"
```

##### Split a string into head / tail pair (case-sensitively)
```rust
let sample_string = "first / second - third ; fourth";
let pattern = r#"\s*[/;-]\s*"#;
// case-sensitive split
let (head, tail) = sample_string.pattern_split_pair_cs(pattern); 
// should yield => head: "first" and tail: "second - third ; fourth"
```

### Traits

- **CharGroupMatch**:	Has methods to validate strings with character classes, has_digits, has_alphanumeric, has_alphabetic
- **IsNumeric**	Provides a method to check if the string may be parsed to an integer or float
- **StripCharacters**:	Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats without regular expressions
- **SimpleMatch**:	Regex-free matcher methods for common validation rules, e.g. starts_with_ci_alphanum checks if the first letters or numerals in a sample string in case-insensitive mode without regular expressions.
- **MatchOccurrences**:	Returns the indices of all ocurrences of an exact string
- **PatternMatch**	Core regular expression match methods, wrappers for re.is_match with case-insensitive (_ci) and case-sensitive (_cs) variants
- **PatternMatchMany**:	Provides methods to match with multiple patterns expressed as arrays of tuples or simple strs
- **PatternMatchesMany**: As above but returns a vector of booleans with the results for each pattern with variant method for whole word matches. New to 0.2.20
- **PatternMatches**:	Pattern methods for arrays or vectors only, returns vectors of booleans matching each input string
- **PatternReplace**:	Core regular expression replacement methods
- **PatternReplaceMany**:	Provides methods to replace with multiple patterns expressed as arrays of tuples
- **PatternSplit**:	Methods to split strings to vectors or head/tail tuples of strings
- **MatchWord**: Has convenience methods to match words with various word boundary rules. New to 0.2.5
- **ReplaceWord**: Provides methods to replace one or more words with clean syntax. New to 0.2.5
- **PatternCapture**: Returns captures or vectors of each match, whether overlapping or not, and counts of matching patterns or words. New to version 0.2.0
- **ToSegments**:	Methods to split a longer string on a separator and return a vector of strings, a tuple of two strings or single optional string segment Note some methods may return empty segments in the case of leading, trailing or repeated separators.
- **ToStrings**:	Converts arrays or vectors of strs to a vector of owned strings

### Enums
- **WordBounds**:	Has options for *Start*, *End* and *Both* with a method to render regular expression subpatterns with the correct word boundaries

### Dev Notes
This crate is still in its alpha stage, but has already been used in 3 API projects. Since version 0.2.14 the code base has been organised into separate files for each set of traits with related implementations. 

#### Recent Version Notes
Version 0.2.17 makes the *build_regex(pattern: &str, case_insensitive: bool)* available to implementors. This is a wrapper *for Regex::new(re: &str)*, but has a convenient case_insensitive parameter and avoids having to explicity import the *regex* crate*. 
In version 0.2.19 default implementations have been added for many variant methods in PatternMatch, PatternReplace, PatternMatchMany and PatternReplaceMany. The last two traits depend on *PatternMatch* and *PatternReplace* respectively. For *PatternMatch* only the base method *pattern_match_result* needs to be implemented and for *PatternReplace* only *pattern_replace_result* and *pattern_replace* need custom implementations, the latter only because the fallback value may have different trait and lifetimes constraints for arrays and vectors. Version 0.2.20 adds *PatternMatchesMany*, which returns a vector of matched patterns, expressed as arrays.
Some updates only reflect minor corrections to these notes and comments in other files.