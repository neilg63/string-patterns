[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/string-patterns)
[![crates.io](https://img.shields.io/crates/v/string-patterns.svg)](https://crates.io/crates/string-patterns)
[![docs.rs](https://docs.rs/string-patterns/badge.svg)](https://docs.rs/string-patterns)

# String Patterns

This library makes it easier to validate and manipulate strings in Rust. It builds on Rust's standard library with help from the default regular expression library, *regex*. It has no other dependencies. It aims to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax and without unduly compromising performance if used sparingly alongside simpler string matching functions such as starts_with, contains or ends_with. To this end, I added methods such as *starts_with_ci* and *starts_with_ci_alphanum* for basic string validation without regular expressions. 

The library provides a number of utility methods to split strings into vectors of strings or a head and tail components and to extract valid numbers from longer texts. Version 0.2.0 has extra methods for capture and match objects to facilitate to advanced text processing;

I added variant match and replace methods with _ci (case-insensitive) or _cs (case-sensitive) suffixes as shorthand for the equivalent plain methods that require a boolean *case_insensitive* parameter. In case-insensitive mode the non-capturing /(?i)/ flag is prepended automatically. This will not be prepended if you add another non-capturing group at the start of your regex. In every other way, the pattern-prefixed methods act as wrappers for the equivalent *re.is_match*, *re.replace_all*, *re.find* and *re.capture_iter* methods in the Regex library and should cover most common uses cases for regular expressions.

Most of the *match* methods will work on *&str* and *String*, while the replacement methods are only implemented for *owned strings*. Likewise, match methods are implemented for arrays and vectors of strings, while replacement methods are only implemented for vectors of *owned strings*;

I will add more documentation as the library progresses beyond the alpha stage. 

##### standard Rust with the Regex library
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

##### with the string-patterns library
```rust
fn is_valid_time_string(input: &str) -> bool {
  input.pattern_match_cs(r#"^([01]\d|2[0-3])?:[0-5]\d(:[0-5]\d)?$"#)
}
```

##### standard Rust with the Regex library
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

##### with the string-patterns library
```rust

fn replace_final_os(input: &str) -> String {
  input.to_string().pattern_replace_ci(r#"(\w)o\b"#, "${1}um") // case-insensitive replacement
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
let str_1 = "Do you spell hip-hop with a hyphen?";
if str_1.contains_ci_alphanum("hiphop") {
  println!("{} is hip-hop-related", str_1);
}
```

##### Extract the first match from a string
```rust
let str_1 = "The park has many lions, spotted hyenas, leopards, rhinoceroses, hippopotamuses, giraffes, cheetahs and baboons";
if let Some(matched_item) = str_1.pattern_first_match(r#"\bspotted\s+\w+\b"#) {
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
  println!("{} occurs {} in the above text", sample_word, sample_text.count_word("humpty", true) )
```

##### Replace text in a vector of strings
```rust
let sample_strings = ["apples", "bananas", "carrots", "dates"].to_strings(); /// cast to vector of owned strings
let pattern = r#"a([pr])"#;
let replacement = "æ$1";
// With arrays or vectors the regex need only be compiled once
let new_strings = sample_strings.pattern_replace_ci(pattern, replacement); // case-insensitive replacement
/// should yield the strings "æpples", "bananas", "cærrots", "dates"
```

##### Replace multiple pattern/replacement pairs 
```rust
let source_str = "The dying King Edmund decides to try to save Lear and Cordelia.".to_string();
  let pattern_replacements = [
    (r#"\bEdmund\b"#, "Edward"),
    (r#"\bLear\b"#, "Larry"),
    (r#"\bCordelia\b"#, "Cecilia")
  ];
/// Should equal "The dying King Edward decides to try to save Larry and Cecilia."
let target_str = source_str.pattern_replace_pairs(&pattern_replacements); 
```

##### Extract the third non-empty segment of a long path name
```rust
let path_string = "/var/www/mysite.com/web/uploads".to_string();
if let Some(domain) = path_string.to_segment("/", 2) {
  println!("The site name is: {}", domain); // "mysite.com" is an owned string
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
    println!("The price is euros is {:.2}", price_eur);
}
```

##### Test the proximity of two words
```rust
let source_str = "Lions are unique among cats in that they live in a group or pride.";
// Do the words 'lion(s)' and 'cat(s)' occur within 20 characters of each other?
if source_str.match_words_by_proximity("lions?", "cats?", -20, 20, true) {
  println!("This sentence mentions lions in the context of cats");
}
```

### Traits

- **CharGroupMatch**:	Methods to validate strings with character classes
- **IsNumeric**	Method to check if the string may be parsed to an integer or float
- **MatchOccurrences**:	Return the indices of all ocurrences of a character
- **PatternMatch**	Core regular expression match methods
- **PatternMatchMany**:	Provides methods to match with multiple patterns expressed as arrays of tuples or simple strs
- **PatternMatches**:	Pattern methods for arrays or vectors only, return vectors of booleans matching each input string
- **PatternReplace**:	Core regular expression replacement methods
- **PatternReplaceMany**:	Provides methods to replace with multiple patterns expressed as arrays of tuples
- **MatchWord**: Has convenience methods to match words with various word boundary rules. New to 0.2.0
- **PatternCapture**: Returns captures or vectors of each match, whether overlapping or not and to return counts of matching patterns or words. New to version 0.2.0
- **SimpleMatch**:	Regex-free matcher methods for common use cases
- **StripCharacters**:	Set of methods to strip unwanted characters by type or extract vectors of numeric strings, integers or floats
- **ToSegments**:	Methods to split a longer strong on a separator and return a vector of strings, a tuple of two strings or single optional string segment Note some methods may return empty segments in the case of leading, trailing or repeated separators See notes below
- **ToStrings**:	Converts arrays or vectors of strs to a vector of owned strings

NB: Although I've used the library methods in three of my commercial projects, this crate is very much in its alpha stage as I evaluate
which of the many auxiliary methods, not documented here, belong in this library. Version updates in the 0.1.x series reflect mainly corrections to this file.
