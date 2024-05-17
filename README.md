# string-literals

[![License](https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue?style=flat-square)](#license)
[![nightly docs](https://img.shields.io/github/deployments/neoncitylights/string-literals/github-pages?style=flat-square&label=nightly%20docs)](https://neoncitylights.github.io/string-literals/string_literals/index.html)
[![docs.rs](https://img.shields.io/docsrs/string-literals/latest?style=flat-square&label=docs.rs)](https://docs.rs/string-literals/)
[![CI](https://img.shields.io/github/actions/workflow/status/neoncitylights/string-literals/.github/workflows/main.yml?style=flat-square)](https://github.com/neoncitylights/string-literals/actions/workflows/main.yml)
[![Codecov](https://img.shields.io/codecov/c/github/neoncitylights/string-literals?style=flat-square&logo=codecov&logoColor=%23fff)](https://codecov.io/gh/neoncitylights/string-literals)

A very tiny crate with Rust macros to more easily create String types.

When creating string literals in Rust, the given type is `&str`. To create an owned `String` type,
you either need to:

- pass in the literal to `String::from()`,
- call `.to_owned()`, or `.to_string()` on the `&str` literal

This crate aims to make this slightly more ergonomic; see examples below in the [Usage](#Usage) section.

## Install

```shell
cargo add string-literals
```

## Usage

### Strings

```rust
use string_literals::s;

let old = "Hello, world!".to_owned();
let new = s!("Hello, world!");
```

### Arrays, vectors

```rust
use string_literals::{string_arr, string_vec};

let old_arr: [String; 3] = ["Banana".to_owned(), "Apple".to_owned(), "Orange".to_owned()];
let new_arr: [String; 3] = string_arr!["Banana", "Apple", "Orange"];

let old_vec = vec!["Banana".to_owned(), "Apple".to_owned(), "Orange".to_owned()];
let new_vec = string_vec!["Banana", "Apple", "Orange"];
```

### Hash maps

```rust
use std::collections::HashMap;
use string_literals::string_hashmap;

let mut old1: HashMap<String, String> = HashMap::new();
old1.insert("Banana".to_owned(), "Good".to_owned());
old1.insert("Apple".to_owned(), "Okay".to_owned());

let old2: HashMap<String, String> = HashMap::from([
    ("Banana".to_owned(), "Good".to_owned()),
    ("Apple".to_owned(), "Okay".to_owned()),
]);

let new: HashMap<String, String> = string_hashmap! {
    "Banana" => "Good",
    "Apple" => "Okay",
};
```

## License

Licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
