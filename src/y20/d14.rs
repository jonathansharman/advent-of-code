use std::collections::HashMap;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 17481577045893);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut mem: HashMap<usize, usize> = HashMap::new();
	let mut on_mask = 0;
	let mut off_mask = 0;
	for instruction in read_lines("input/2020/14.txt") {
		let (lhs, rhs) = instruction.split_once(" = ").unwrap();
		if lhs == "mask" {
			on_mask = 0;
			off_mask = 0;
			for (i, bit) in rhs.chars().rev().enumerate() {
				match bit {
					'1' => on_mask |= 1 << i,
					'0' => off_mask |= 1 << i,
					_ => {}
				}
			}
		} else {
			let addr = lhs[4..lhs.len() - 1].parse::<usize>().unwrap();
			let value = (rhs.parse::<usize>().unwrap() | on_mask) & !off_mask;
			mem.insert(addr, value);
		}
	}
	mem.values().sum()
}

pub fn part2() -> usize {
	0
}
