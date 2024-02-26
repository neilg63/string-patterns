[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/string-patterns)
[![crates.io](https://img.shields.io/crates/v/string-patterns.svg)](https://crates.io/crates/string-patterns)
[![docs.rs](https://docs.rs/string-patterns/badge.svg)](https://docs.rs/string-patterns)

# String Patterns

This library makes it easier to work with regular expressions in Rust. It builds on the standard regular expression crate, *[regex](https://crates.io/crates/regex)*. It has no other dependencies, but supplements *[simple-string-patterns](https://crates.io/crates/simple-string-patterns)*, which provides an assortment of regex-free extension methods to match, split and filter strings by character types or ranges, relying only on the standard library.

Together, these crates aim to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax. Simpler string matching methods such as starts_with, contains or ends_with will always perform better, especially when processing large data sets. 

### Method overview
- All pattern-prefixed methods use regular expressions via the Regex crate
- All other extension methods use standard library functions only to match, remove or extract character sequences.
- Methods ending in _result return a Result with a regex::Error if the regular expression fails
- Many methods without *_ci* or *_cs* suffixes require a boolean *case_insensitive* parameter
- Methods ending in *_cs* are case-sensitive
- Methods ending in *_ci* are case-insensitive
- Methods containing *_word(s)* match whole or partial words depending on boundary rules
- Methods containing *_match_all* require all patterns within an array to match.
- Methods containing *_match_any* return true if any of the patterns within an array match
- Methods ending in *_captures* return iterable Regex capture objects.
- Methods ending in *_matches* *_matches_vec* or *_matches_ouet* return vectors of Regex match objects with start and end offsets.
- Methods with *_matches_filtered* return filtered vectors of matched strings slices
- Methods containing *_split* return either a vector or tuple pair.


Version 0.3.0 only includes the core text-processing extensions that rely on regular expressions. Other methods bundled with earlier versions have migrated to the [simple-string-patterns](https://crates.io/crates/simple-string-patterns) crate. These crates supplement each other, but may be independently installed if you only need some of their features.

### Removed methods
Only one *regex* method, **match_words_by_proximity*,  has been removed. However, it will reappear in the future *string-patterns-extras* crate. 

#### Case Sensitivity
In case-insensitive mode the non-capturing **/(?i)/** flag is prepended automatically, but omitted if you add another non-capturing group at the start of your regular expression. In every other way, the pattern-prefixed methods behave like *re.is_match*, *re.replace_all*, *re.find* and *re.capture_iter* methods in the Regex crate. String-patterns unleashes most of the core functionality of the Regex crate, on which it depends, to cover most common use cases in text processing and to act as a building block for specific validators (e.g. email validation) and text transformers. 

Most *match* methods will work on *&str* and *String*, while replacement methods are only implemented for *owned strings*. Likewise, match methods are implemented for arrays and vectors of strings or *string slices*, while replacement methods are only implemented for vectors of *owned strings*. The traits may be implemented for structs or tuples with a string field. 

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

##### Simple case-insensitive match
```rust
let str_1 = "Dog food";
if str_1.starts_with_ci("dog") {
  println!("{} is dog-related", str_1);
}
```

##### Simple case-insensitive match on the alphanumeric characters only in a longer text
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

##### Match within an array of strings
```rust
let sample_strs = [
  "pictures_Italy-1997",
  "photos-portugal-2001",
  "imagini-italia_2002",
  "images-france-2003",
];
let test_pattern = r#"[^a-z]ital(y|ia)"#; // matches 'italy' or 'italia'
// The regular expression will only be compiled once
if sample_strs.pattern_match_ci(test_pattern) {
  println!("Some of these folders are related to Italy");
}

// Filter the above array
let filtered_strs = sample_strs.pattern_matches_filtered_ci(test_pattern);
// should yield ["pictures_Italy-1997","imagini-italia_2002"]
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

##### Split a string on a pattern
```rust
let sample_string = "books, records and videotapes";
let pattern = r#"\s*(,|and)\s"#;
 // case-insensitive split
let items = sample_string.pattern_split_ci(pattern);
// should yield a vector of strings: vec!["books", "records", "videotapes"]
```

##### Split a string into head / tail pair (case-sensitively)
```rust
let sample_string = "first / second - third ; fourth";
let pattern = r#"\s*[/;-]\s*"#;
// case-sensitive split
let (head, tail) = sample_string.pattern_split_pair_cs(pattern); 
// should yield => head: "first" and tail: "second - third ; fourth"
```

##### Fetch a vector of pattern match objects with start and end indices as well as the captured substrings.
```rust
let sample_string = "All the world's a stage, and all the men and women merely players.";
// Words ending in 'men' with 0 to 4 preceding characters. the sequence in parentheses is an inner capture.
let pattern = r#"\b\w{0,4}(men)\b"#;
let outer_matches = sample_string.pattern_matches_outer(pattern,true);
// should yield a vector with the outer matches only, but with with start and end offsets
if let Some(second_match) = outer_matches.get(1) {
  println!("the second match '{}'' starts at {} and ends at {}", second_match.as_str(), second_match.start(), second_match.end());
  // should print the matched word 'woman' and its start and end indices
}

let all_captures = sample_string.pattern_captures(pattern, true);
/// Yields an iterable regex::Captures object with all nested captured groups
```

### Sample implementation of PatternMatch for a custom struct
```rust
use string_patterns::PatternMatch;

// Simple struct with a core text field
#[derive(Debug, Clone)]
pub struct Message {
  text: String,
  timestamp: i64,
  from: String,
  to: String,
}

impl PatternMatch for Message {
  // All other pattern_match variants with a single regular expression are implemented automatically
  fn pattern_match_result(&self, pattern: &str, case_sensitive: bool) -> Result<bool, Error> {
    self.text.pattern_match_result(pattern, case_sensitive)
  }
}
```

##### Extract three float values from a longer string
This example requires the *simple-string-patterns* crate.
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

### Traits

- **PatternMatch**	Core regular expression match methods, wrappers for re.is_match with case-insensitive (_ci) and case-sensitive (_cs) variants
- **PatternMatchMany**:	Provides methods to match with multiple patterns expressed as arrays of tuples or simple strs
- **PatternMatchesMany**: As above but returns a vector of booleans with the results for each pattern with variant method for whole word matches.
- **PatternMatches**:	Pattern methods for arrays or vectors only, returns vectors of pairs of boolean outcomes and string slices, vectors of booleans matching each input string or filtered vectors of matched string slices
- **PatternReplace**:	Core regular expression replacement methods
- **PatternReplaceMany**:	Provides methods to replace with multiple patterns expressed as arrays of tuples
- **PatternSplit**:	Methods to split strings to vectors or head/tail tuples of strings
- **MatchWord**: Has convenience methods to match words with various word boundary rules.
- **ReplaceWord**: Provides methods to replace one or more words with clean syntax.
- **PatternCapture**: Returns captures or vectors of each match, whether overlapping or not, and counts of matching patterns or words.

### Enums
- **WordBounds**:	Has options for *Start*, *End* and *Both* with a method to render regular expression subpatterns with the correct word boundaries
  Options:
  - None: No bounds
  - Start: From word start
  - End: To word end
  - Both: Whole word, but spaces or other punctuation may occur within the pattern to match one or more words

### Dev Notes
As of version 0.3.0, this crate is feature complete, although still in a beta stage. All new features will be in a future *string-patterns-extras* crate that builds on this library and *simple-string-patterns*.
Notes for the 0.2.* series can be found in the [GitHub repo](https://github.com/neilg63/string-patterns) in the v0-2 branch. If you upgrade from a pre-0.3.0 version, you may need to install  *simple-string-patterns* as well.

NB: Some updates reflect editorial changes only.