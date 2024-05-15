//! This is a top-level file comment (`//!`). In `lib.rs` specifically,
//! it shows up as the root-level crate documentation.
//!
//! For documentation about the different types of Doc comments in Rust,
//! see <https://cheats.rs/#documentation>

/// This is an outer-line doc comment (triple slash, `///`).
/// This documentation gets attached to the symbol in Rustdoc,
/// and shows up in generated documentation.
///
/// Say hi to someone!
///
/// # Examples
/// This code block gets ran by Rustdoc and is considered a "doctest".
/// ```
/// use rust_template::greet;
///
/// assert_eq!(greet("Bob"), String::from("Hello Bob!"));
/// ```
pub fn greet(name: &str) -> String {
	format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
	use crate::greet;

	#[test]
	fn greet_bob() {
		assert_eq!(greet("Bob"), String::from("Hello Bob!"));
	}

	#[test]
	fn greet_the_world() {
		assert_eq!(greet("World"), String::from("Hello World!"));
	}
}
