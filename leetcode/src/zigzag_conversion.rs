// * 6. Zigzag Conversion - https://leetcode.com/problems/zigzag-conversion/description/

#![allow(dead_code, unused_mut, unused_variables)]

pub struct Solution;

impl Solution {
	pub fn convert(s: String, num_rows: i32) -> String {
		let mut result: Vec<_> = (0..num_rows)
			.chain((1..num_rows - 1).rev())
			.cycle()
			.zip(s.chars())
			.collect();

		result.sort_by_key(|&(row, _)| row);
		result.into_iter().map(|(_, c)| c).collect()
	}
}

#[rustfmt::skip]
pub fn test() {
	assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
	assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
}
