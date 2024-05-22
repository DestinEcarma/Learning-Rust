// * 8. String to Integer (atoi) - https://leetcode.com/problems/string-to-integer-atoi/description/

#![allow(dead_code, unused_mut, unused_variables)]

pub struct Solution;

impl Solution {
	pub fn my_atoi(s: String) -> i32 {
		let trimmed = s.trim_start();

		let (result, sign) = match trimmed.strip_prefix('-') {
			Some(temp) => (temp, -1),
			None => (trimmed.strip_prefix('+').unwrap_or(trimmed), 1),
		};

		result
			.chars()
			.map(|c| c.to_digit(10))
			.take_while(Option::is_some)
			.flatten()
			.fold(0, |acc, digit| {
				acc.saturating_mul(10).saturating_add(sign * digit as i32)
			})
	}
}

pub fn test() {
	assert_eq!(Solution::my_atoi("42".to_string()), 42);
	assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
	assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
	assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
	assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
}
