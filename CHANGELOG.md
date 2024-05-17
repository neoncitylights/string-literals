# Changelog

## 1.0.1 (2024-05-17)

This minor patch provides updates to the `string_hashmap!` macro.

- meta: add ["as-is" maintenance badge](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section) to `Cargo.toml`
- docs: minor documentation example update for `string_hashmap!` macro
- perf: `string_hashmap!` macro passed with 0 arguments will have a smaller expansion
- perf: `string_hashmap!` macro passed with 1 or more arguments will call `HashMap::from()`. This allows for reserving memory upfront. Previously, the macro would expand to a `HashMap::new()` with multiple `insert()` calls after.

## 1.0.0 (2024-05-15)

- Initial release of the `string-literals` library
