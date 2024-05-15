/// Create a [`String`] literal
///
/// This provides a slightly shorter alternative to creating a `String`
/// from a literal.
///
/// # Examples
/// Empty strings:
/// ```
/// use string_literals::s;
///
/// let old = String::new();
/// let s1 = s!();
/// let s2 = s!("");
/// assert!(s1.is_empty());
/// assert!(s2.is_empty());
/// ```
///
/// Non-empty strings:
/// ```
/// use string_literals::s;
///
/// let old1 = String::from("Alice");
/// let old2 = "Alice".to_owned();
/// let old3 = "Alice".to_string();
/// let new  = s!("Alice");
/// assert_eq!(new, String::from("Alice"));
/// ```
#[macro_export]
macro_rules! s {
	() => {
		String::new()
	};
	("") => {
		String::new()
	};
	($s: expr) => {
		$s.to_owned()
	};
}

/// Create an [`array`] of `[String; N]` with string literals
///
/// # Examples
/// Empty arrays:
/// ```
/// use string_literals::string_arr;
///
/// let arr: [String; 0] = string_arr![];
/// assert!(arr.is_empty());
/// ```
///
/// Non-empty arrays:
/// ```
/// use string_literals::string_arr;
///
/// let old: [String; 2] = [String::from("Alice"), String::from("Bob")];
/// let new: [String; 2] = string_arr!["Alice", "Bob"];
/// assert_eq!(new.len(), 2);
/// assert_eq!(new[0], String::from("Alice"));
/// assert_eq!(new[1], String::from("Bob"));
/// ```
#[macro_export]
macro_rules! string_arr {
	() => {
		[]
	};
	($($x:expr),+ $(,)?) => {
		[$($x.to_owned()),*]
	}
}

/// Create a [`Vec`] of `Vec<String>` with string literals.
///
/// This macro also allows zero arguments. In this case however, it would be
/// shorter to call `vec![]` or `Vec::new()`.
///
/// # Examples
/// ```
/// use string_literals::string_vec;
///
/// let old: Vec<String> = vec![String::from("Alice"), String::from("Bob")];
/// let new: Vec<String> = string_vec!["Alice", "Bob"];
/// assert_eq!(new.len(), 2);
/// assert_eq!(new[0], "Alice".to_owned());
/// assert_eq!(new[1], "Bob".to_owned());
/// ```
#[macro_export]
macro_rules! string_vec {
	() => {
		Vec::new()
	};
	($($x:expr),+ $(,)?) => {
		vec![$($x.to_owned()),+]
	};
}

/// Create a [`HashMap`](std::collections::HashMap) of [`HashMap<String, String>`] with string literals
///
/// This macro also allows zero arguments (an empty hash map). In this case
/// however, it would be shorter in length to call `HashMap::new()`.
///
/// # Examples
/// Empty hash maps:
/// ```
/// use std::collections::HashMap;
/// use string_literals::string_hashmap;
///
/// let map: HashMap<String, String> = string_hashmap!{};
/// assert!(map.is_empty());
/// ```
///
/// Non-empty hash maps:
/// ```
/// use std::collections::HashMap;
/// use string_literals::string_hashmap;
///
/// let map: HashMap<String, String> = string_hashmap!{
///     "Banana" => "Good",
///     "Apple" => "Okay",
/// };
/// assert_eq!(map[&"Banana".to_owned()], "Good".to_owned());
/// assert_eq!(map[&"Apple".to_owned()], "Okay".to_owned());
/// ```
#[macro_export]
macro_rules! string_hashmap {
	() => {
		{
			let mut _map = ::std::collections::HashMap::new();
			_map
		}
	};
	($($k:expr => $v:expr),+ $(,)?) => {
		{
			let mut _map = ::std::collections::HashMap::new();
			$(_map.insert($k.to_owned(), $v.to_owned());)*
			_map
		}
	};
}
