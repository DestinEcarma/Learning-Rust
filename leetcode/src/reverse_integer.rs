// * 7. Reverse Integer - https://leetcode.com/problems/reverse-integer/description/

#![allow(dead_code, unused_mut, unused_variables)]

pub struct Solution;

impl Solution {
	pub fn reverse(mut x: i32) -> i32 {
		let mut result = 0_i32;
		let mut prev = 0_i32;

		while x != 0 {
			let r = x % 10;

			result = result.saturating_mul(10).saturating_add(r);

			if prev != (result - r) / 10 {
				return 0;
			}

			x /= 10;
			prev = result;
		}

		result
	}
}

pub fn test() {
	assert_eq!(Solution::reverse(123), 321);
	assert_eq!(Solution::reverse(-123), -321);
	assert_eq!(Solution::reverse(120), 21);
	assert_eq!(Solution::reverse(0), 0);
	assert_eq!(Solution::reverse(1534236469), 0);
}
