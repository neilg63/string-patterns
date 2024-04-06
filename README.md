[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/string-patterns)
[![crates.io](https://img.shields.io/crates/v/string-patterns.svg)](https://crates.io/crates/string-patterns)
[![docs.rs](https://docs.rs/string-patterns/badge.svg)](https://docs.rs/string-patterns)

# String Patterns

This library makes it easier to work with regular expressions in Rust. It builds on the standard regular expression crate, *[regex](https://crates.io/crates/regex)*. It has no other dependencies, but supplements *[simple-string-patterns](https://crates.io/crates/simple-string-patterns)*, which provides an assortment of regex-free extension methods to match, split and filter strings by character types or ranges, relying only on the standard library.

Together, these crates aim to make working with strings as easy in Rust as it is Javascript or Python with cleaner syntax. Simpler string matching methods such as starts_with, contains or ends_with will always perform better, especially when processing large data sets. 

The core *PatternMatch* and *PatternReplace* traits are implemented for arrays or vectors of strings to avoid compiling a regular expression in a loop. You may need to reimplement these for vectors of custom structs as shown in the example below. Simply calling **my_string.pattern_match("complex_regex")** in a loop is an anti-pattern leading to expensive recompilation of the same regular expression. The same principle applies to replacement methods, implemented only for *```String```* and *```Vec<String>```*.

Version 0.3.8 introduces variant *_replace_first* methods to replace only the left-most match in a sample string, implementing *re.replace* rather than *re.replace_all*. This is faster when you only need to replace one matched pattern per string. 

### Method overview
| Position | Component(s) | Meaning |
| --------- | -------- | ------- |
| end | _result | Return a *Result* with a regex::Error if the regular expression fails |
| end | - | Many match and replace methods without *_ci* or *_cs* suffixes require a boolean *case_insensitive* parameter |
| end |  _cs | Case-sensitive |
| end |  _ci | Case-insensitive |
| end, mid | _replace | Replace all matches with the sample string |
| end, mid | _replace_first | Replace only the first left-most occurrence |
| mid, end | _word(s) | Match whole or partial words depending on boundary rules |
| mid, end | _match_all | Require all patterns within an array to match |
| mid, end | *_match_any* | Return true if any of the patterns within an array match |
| end | _captures | return iterable Regex capture objects |
| mid, end | _matches | Return vectors of boolean results with arrays of regex patterns as the first argument |
| end | _matches_vec | Return vectors of *Regex::Match* objects with start and end offsets. |
| end | _matches_outer | Return vectors of outer (or whole-pattern) *Match* objects with start and end offsets. |
| end |_matches_filtered | return filtered vectors of matched strings slices |
| end, mid | _split | Return either a vector or tuple pair. |
| end, mid | _filter, _filter_word | Filter arrays or vectors of strings or str references by the a regex pattern |

Version 0.3.4 adds a *PatternFilter* with methods that filter arrays or vectors of strings or strs by a regex pattern with variants for whole word and case-insensitive matches. This mirrors the functionality in *filter_all_conditional* in *simple-string-patterns*, but with a single regular expression rather than a set of rules.

Since version 0.3.0, the crate only includes the core text-processing extensions that rely on regular expressions. Other methods bundled with earlier versions have migrated to the [simple-string-patterns](https://crates.io/crates/simple-string-patterns) crate. These crates supplement each other, but may be independently installed if you only need some of their features.

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

##### Replace the first match only
```rust
  let sample_path = "/User/me/Documents/Accounts/docs/2023/earnings-report.pdf".to_string();
  // should match any segment between / characters starting with 'doc' and ending in 's'
  let pattern = r#"/doc[^/]*s/"#;
  let replacement = r#"/files/"#;
  // Only replace the first segment matching the above pattern case-insensitively
  let new_path_1 = sample_path.pattern_replace_first_ci(pattern, replacement);
  // should yield = "/User/me/files/Accounts/docs/2023/earnings-report.pdf"
  // replace all matches. Will replace /docs/ as well as /Documents/
  let new_path_2 = sample_path.pattern_replace_ci(pattern, replacement);
  // should yield = "/User/me/files/Accounts/files/2023/earnings-report.pdf"

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

##### Filter an array or vector of strings by a regex pattern
```rust
let source_strs = [
  "Ristorante-Venezia-2019.jpg",
  "Mercado_venecia_2000.jpg",
  "Mercado_venezuela_2011.jpg",
  "Venice_Oct_2012.png",
  "2venice2003.jpg",
  "venetian_blinds.jpg",
];

/// filter by file names referencing Venice in various languages, but not Venezuela or venetian blinds
let pattern = "ven(ezia|ecia|edig|ice|ise)[^a-z]*";

let filtered_strs = source_strs.pattern_filter_ci(pattern); 
// should yield ["Ristorante-Venezia-2019.jpg", "Mercado_venecia_2000.jpg", "Venice_Oct_2012.png", "2venice2003.jpg"]

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



### Sample implementations of PatternMatch and PatternFilter for a custom struct
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
  fn pattern_match_result(&self, pattern: &str, case_insensitive: bool) -> Result<bool, Error> {
    self.text.pattern_match_result(pattern, case_insensitive)
  }
}

/// The regular expression is compiled only once. If the regex fails, all items are returned
impl<'a> PatternFilter<'a, Message> for [Message] {
  fn pattern_filter(&'a self, pattern: &str, case_insensitive: bool) -> Vec<Message> {
    if let Ok(re) = build_regex(pattern, case_insensitive) {
    self.into_iter().filter(|m| re.is_match(&m.text)).map(|m| m.to_owned()).collect::<Vec<Message>>()
    } else {
      self.to_owned()
    }
  }
}
```

### Traits

| Name | Description |
| ---- | ----------- | 
| PatternMatch | Core regular expression match methods, wrappers for re.is_match with case-insensitive (_ci) and case-sensitive (_cs) variants |
| PatternMatchMany|	Provides methods to match with multiple patterns expressed as arrays of tuples or simple strs |
| PatternMatchesMany | As above but returns a vector of booleans with the results for each pattern with variant method for whole word matches. |
| PatternMatches | Pattern methods for arrays or vectors only, returns vectors of pairs of boolean outcomes and string slices, vectors of booleans matching each input string or filtered vectors of matched string slices |
| PatternReplace | Core regular expression replacement methods |
| PatternFilter | Methods to filter arrays or vectors of strings by a single regex pattern |
| PatternReplaceMany |	Provides methods to replace with multiple patterns expressed as arrays of tuples |
| PatternSplit |	Methods to split strings to vectors or head/tail tuples of strings |
| MatchWord | Has convenience methods to match words with various word boundary rules. |
| ReplaceWord | Provides methods to replace one or more words with clean syntax. |
| PatternCapture | Returns captures or vectors of each match, whether overlapping or not, and counts of matching patterns or words. |

### Enums
- **WordBounds**:	Has options for *Start*, *End* and *Both* with a method to render regular expression subpatterns with the correct word boundaries
  Options:
  - None: No bounds
  - Start: From word start
  - End: To word end
  - Both: Whole word, but spaces or other punctuation may occur within the pattern to match one or more words

### Dev Notes
Version 0.3.8 adds variant *pattern_replace_first_result* and *pattern_replace_first* methods. These are implemented for String and Vec<String>, but need to be reimplemented for custom structs or collection types. Only the _ci and _cs variants have default implementations.

As of version 0.3.8 the crate re-exports Regex::Captures and Regex::Match to help with custom implementations.

As of version 0.3.6, the crate re-exports regex::Regex and regex::Error to help with custom implementations

As of version 0.3.0, this crate is nearly feature complete, although still in a beta stage. All new features will be in a future *string-patterns-extras* crate that builds on this library and *simple-string-patterns*. 0.3.5 has no new features, only a more notes and a few more methods have default implementations.

Notes for the 0.2.* series can be found in the [GitHub repo](https://github.com/neilg63/string-patterns) in the v0.2.* branch. If you upgrade from a pre-0.3.0 version, you may need to install  *simple-string-patterns* as well.

### Removed methods
Only one *regex* method, **match_words_by_proximity*,  has been removed. However, it will reappear in the future *string-patterns-extras* crate. 

NB: Some updates reflect editorial changes only.