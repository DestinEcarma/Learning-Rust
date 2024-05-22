mod longest_palindrome_substring;
mod reverse_integer;
mod string_to_integer;
mod zigzag_conversion;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	let method = args.get(1).map(String::as_str);

	match method {
		Some("longest_palindrome_substring") => longest_palindrome_substring::test(),
		Some("reverse_integer") => reverse_integer::test(),
		Some("string_to_integer") => string_to_integer::test(),
		Some("zigzag_conversion") => zigzag_conversion::test(),
		_ => println!("Usage: leetcode <problem>"),
	}
}
