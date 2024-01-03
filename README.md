# String Patterns

This library makes it easier to validate and manipulate strings in Rust. It builds on Rust's standard library with help from the default regular expression library, *regex*. It has no other dependencies. It aims to make working with strings as easy in Rust as it is Javascript or Python without compromising performance.
The library provides a number of utility methods to split strings into vectors of strings or a head and tail components and to extract valid numbers from longer texts. I will add more documentation as the library progresses beyond the alpha stage. I added variant match and replace methods with _ci (case-insensitive) or _cs (case-sensitive) suffixes as shorthand for the equivalent plain methods thatv require a boolean case_insensitive flag. In case-insensitive mode the non-capturing /(?i)/ flag is prepended automatically.

##### standard Rust with the Regex library
```rust

fn is_valid_time_string(input: &str) -> bool {
  let time_format_pattern = r#"^([01]\d|2[0-3])?:[0-5]\d(:[0-5]\d)?$"#;
  let re = Regex::new(time_format_pattern);
  if let Ok(is_matched) =  re.is_match(input) {
    is_matched
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
  let regex_str = r#"(\w)o\b$"#;
  let re = Regex::new(&regex_str);
  if let Ok(repl_string) =  re.replace_all(input, "$1um") {
    repl_string
  } else {
    input.to_string()
  }
}
```

##### with the string-patterns library
```rust

fn replace_final_os(input: &str) -> String {
  input.to_string().pattern_replace_ci(r#"(\w)o\b$"#, "$1um") // case insensitive replacement
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

##### Replace text in a vector of strings
```rust
let sample_strings = ["apples", "bananas", "carrots", "dates"].to_strings(); /// cast to vector of owned strings
let pattern = r#"a([pr])"#;
let replacement = "æ$1";
let new_strings = sample_strings.pattern_replace(pattern, replacement);
/// should yield the strings "æpples", "bananas", "cærrots", "dates"
```

##### Replace multiple pattern/replacement pairs 
```rust
let source_str = "The dying Edmund decides to try to save Lear and Cordelia.".to_string();
  let pattern_replacements = [
    (r#"\bEdmund\b"#, "Edward"),
    (r#"\bCordelia\b"#, "Cecilia")
  ];
/// Should equal "The dying Edward decides to try to save Lear and Cecilia."
let target_str = source_str.pattern_replace_pairs(&pattern_replacements); 
```

##### Extract the third non-empty segment of a long path name
```rust
let path_string = "/var/www/mysite.com/web/uploads".to_string();
if let Some(domain) = path_string.to_segment("/", 2) {
  println!("The site name is: {}". domain); // "mysite.com" is an owned string
}
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

NB: Although I've used the library methods in three of my commercial projects, this project is very much in its alpha stage as I evaluate
which of the many auxiliary methods, not documented here, belong in this library. Version updates in the 0.1.x series reflect mainly corrections to this file.
