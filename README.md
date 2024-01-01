# String Patterns

This library makes it easier to validate and post-process strings in Rust. It builds on the Rust standard library with help from the default regular expression library, regex. It has no other dependencies. It aims to make working with strings as easy in Rust as it is Javascript or Python without compromising performance.
The library provides a number of utility methods to split strings into vectors of strings or a head and tail components and to extract valid numbers from longer texts. I will add more documentation as the library progresses beyond the alpha stage.

##### standard Rust with the Regex library
```rust

fn is_valid_time_string(input: &str) -> bool {
  let regex_str = r#"^\d\d?:\d\d(:\d\d)?$"#;
  let re = Regex::new(&regex_str);
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
  input.to_string().pattern_match(r#"^\d\d?:\d\d(:\d\d)?$"#)
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
  input.to_string().pattern_replace(r#"(\w)o\b$"#, "$1um")
}
```
##### extract the third part of a long path name
```rust
let path_string = "/var/www/mysite.com/web/uploads".to_string();
let domain = path_string.segment(2); // Some("mysite.com".to_string())
```

##### extract the first decimal value as an f64 from a longer string
```rust
let input_string = "Price £12.50 each".to_string();
let price_gbp = input_string.to_first_number(); // 12.5 as f64
```


NB: Although I've used the library methods in three of my commercial projects, this library is very much an alpha release as I evaluate
which of the many auxiliary methods, not documented here, belong in this library.
