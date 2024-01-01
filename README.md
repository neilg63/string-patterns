# String Patterns

This library makes it easier to validate and post-process strings in Rust. It builds on the Rust standard library with help from the default regular expression library, regex. It has no other dependencies. It aims to make working with strings as easy in Rust as it is Javascript or python without compromising performance:

### standard Rust with the Regex library
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

### with the string-patterns library
```rust
fn is_valid_time_string(input: &str) -> bool {
  input.to_string().pattern_match(r#"^\d\d?:\d\d(:\d\d)?$"#)
}
```

### standard Rust with the Regex library
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

### with the string-patterns library
```rust

fn replace_final_os(input: &str) -> bool {
  input.pattern_replace(r#"(\w)o\b$"#, "$1um")
}
```


NB: Although I've used the library methods in three of my commercial projects, this library is very much an alpha release as I evaluate
which of the many auxiliary methods, not documented here, belong in this library.
