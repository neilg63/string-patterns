[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/string-patterns)
[![crates.io](https://img.shields.io/crates/v/string-patterns.svg)](https://crates.io/crates/string-patterns)
[![docs.rs](https://docs.rs/string-patterns/badge.svg)](https://docs.rs/string-patterns)

# String Patterns

This library makes it easier to validate and manipulate strings in Rust. It builds on Rust's standard library with help from the default regular expression crate, *regex*. It has no other dependencies. It aims to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax and without unduly compromising performance if used sparingly alongside simpler string matching functions such as starts_with, contains or ends_with. To this end, the crate provides methods such as *starts_with_ci* and *starts_with_ci_alphanum* for basic string validation without regular expressions. 

The library provides a number of utility methods to split strings into vectors of strings or a head and tail components and to extract valid numbers from longer texts. Version 0.2.0 has extra methods to capture and count matched strings with offsets to facilitate advanced text processing and version 0.2.5 introduces new methods to match and replace words without intrusive word boundary anchors. The is_numeric() method in the IsNumeric trait now applies a strict regex-free check on compatibility with the parse() method and should not be confused char::is_numeric which checks for digit-like characters only and will not match minus or decimal points. Version 2.13 corrects am issue in to_first_number() that led negative numbers to be interpreted as positives. I added a new example below to show how *pattern_split* and to *first_number* this may be used together to process number-like strings.

Variant *match* and *replace* methods with _ci (case-insensitive) or _cs (case-sensitive) suffixes are shorthand for the equivalent plain methods that require a boolean *case_insensitive* parameter. In case-insensitive mode the non-capturing /(?i)/ flag is prepended automatically. This will not be prepended if you add another non-capturing group at the start of your regex. In every other way, the pattern-prefixed methods behave in the same way as *re.is_match*, *re.replace_all*, *re.find* and *re.capture_iter* methods in the Regex library. String-patterns unleashes most of the core functionality of the Regex crate, on which it depends, to cover most common use cases in text processing and to act as a building block for specific validators (e.g. email validation) and text transformers. 

Most of the *match* methods will work on *&str* and *String*, while the replacement methods are only implemented for *owned strings*. Likewise, match methods are implemented for arrays and vectors of strings, while replacement methods are only implemented for vectors of *owned strings*. The traits may be implemented for structs or tuples with a string field. Version 2.10 adds PatternSplit with results as String vectors.

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
  println!("{} occurs {} times in the above text", sample_word, sample_text.count_word("humpty", true) )
```

##### Replace text in a vector of strings
```rust
let sample_strings = ["apples", "bananas", "carrots", "dates"].to_strings(); /// cast to vector of owned strings
let pattern = r#"a([pr])"#;
let replacement = "æ$1";
// With arrays or vectors the regex need only be compiled once
let new_strings = sample_strings.pattern_replace_ci(pattern, replacement); // case-insensitive replacement
/// should yield the strings "æpples", "bananas", "cærrots", "dates" only replacing 'a' with 'æ' before 'p' or 'r'
```

##### Replace multiple pattern/replacement pairs 
```rust
let source_str = "Colourful fishing boats adorned the island's harbours.".to_string();
  let pattern_replacements = [
    ("colour", "color"),
    ("harbour", "harbor"),
  ];
/// Should read "Colorful fishing boats adorned the island's harbors"
let target_str = source_str.pattern_replace_pairs(&pattern_replacements); 
```

##### Replace multiple word pairs in case-sensitive mode
```rust
/// This should have the same result as above but with cleaner and less error-prone syntax
let source_str = "The dying King Edmund decides to try to save Lear and Cordelia.".to_string();
  let pattern_replacements = [
    ("Edmund", "Edward"),
    ("Lear", "Larry"),
    ("Cordelia", "Cecilia")
  ];
/// Should read "The dying King Edward decides to try to save Larry and Cecilia."
let target_str = source_str.replace_words_cs(&pattern_replacements); 
```

##### Match any words in case-insensitive mode
```rust
let source_str = "Two cheetahs ran across the field".to_string();
let cat_like_words = ["lions?","tigers?", "pumas?", "panthers?", "jaguars?", "leopards?", "lynx(es)?", "cheetahs?"];
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
- **MatchOccurrences**:	Return the indices of all ocurrences of an exact string
- **PatternMatch**	Core regular expression match methods, wrappers for re.is_match with case-insensitive (_ci) and case-sensitive (_cs) variants
- **PatternMatchMany**:	Provides methods to match with multiple patterns expressed as arrays of tuples or simple strs
- **PatternMatches**:	Pattern methods for arrays or vectors only, return vectors of booleans matching each input string
- **PatternReplace**:	Core regular expression replacement methods
- **PatternReplaceMany**:	Provides methods to replace with multiple patterns expressed as arrays of tuples
- **PatternSplit**:	Methods to split strings to vectors or head/tail tuples of strings
- **MatchWord**: Has convenience methods to match words with various word boundary rules. New to 0.2.0
- **ReplaceWord**: Provides methods to replace one or more words with clean syntax. New to 0.2.5
- **PatternCapture**: Returns captures or vectors of each match, whether overlapping or not, and counts of matching patterns or words. New to version 0.2.0
- **ToSegments**:	Methods to split a longer string on a separator and return a vector of strings, a tuple of two strings or single optional string segment Note some methods may return empty segments in the case of leading, trailing or repeated separators.
- **ToStrings**:	Converts arrays or vectors of strs to a vector of owned strings

### Enums
- **WordBounds**:	Has options for *Start*, *End* and *Both* with a method to render regular expression subpatterns with the correct word boundaries

NB: Although I've used the library methods in three of my commercial projects, this crate is very much in its alpha stage as I evaluate
which of the many auxiliary methods, not documented here, belong in this library. Many minor version updates in the 0.1.x 0.2.x series reflect mainly corrections to this file.
