// * 5. Longest Palindromic Substring - https://leetcode.com/problems/longest-palindromic-substring/description/

#![allow(dead_code, unused_mut, unused_variables)]

pub struct Solution;

impl Solution {
	#[must_use]
	pub fn longest_palindrome(s: String) -> String {
		let mut result = (0, 0);
		let mut current = (0, 0);

		let chars = s.as_bytes();
		let len = s.len();

		for i in 0..len * 2 {
			current.0 = i / 2;
			current.1 = current.0 + (i % 2 != 0) as usize;

			while current.0 <= current.1 && current.0 < len && current.1 < len {
				if chars[current.0] == chars[current.1] {
					if (current.1 - current.0) > (result.1 - result.0) {
						result = (current.0, current.1);
					}
				} else {
					break;
				}

				current.0 -= 1;
				current.1 += 1;
			}
		}

		s[result.0..=result.1].to_string()
	}
}

pub fn test() {
	assert_eq!(Solution::longest_palindrome("babad".to_string()), "aba");
	assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
}
