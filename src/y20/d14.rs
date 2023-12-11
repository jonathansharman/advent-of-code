use std::collections::HashMap;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 17481577045893);
crate::test::test_part!(test2, part2, 4160009892257);

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
			let addr = lhs[4..lhs.len() - 1].parse().unwrap();
			let value = (rhs.parse::<usize>().unwrap() | on_mask) & !off_mask;
			mem.insert(addr, value);
		}
	}
	mem.values().sum()
}

pub fn part2() -> usize {
	let mut mem: HashMap<usize, usize> = HashMap::new();
	let mut on_mask = 0;
	let mut float_bits = Vec::new();
	for instruction in read_lines("input/2020/14.txt") {
		let (lhs, rhs) = instruction.split_once(" = ").unwrap();
		if lhs == "mask" {
			on_mask = 0;
			float_bits.clear();
			for (i, bit) in rhs.chars().rev().enumerate() {
				match bit {
					'1' => on_mask |= 1 << i,
					'X' => float_bits.push(i),
					_ => {}
				}
			}
		} else {
			let mut addrs =
				vec![lhs[4..lhs.len() - 1].parse::<usize>().unwrap() | on_mask];
			for float_bit in float_bits.iter() {
				addrs = addrs
					.into_iter()
					.flat_map(|addr| {
						[addr | (1 << float_bit), addr & !(1 << float_bit)]
					})
					.collect();
			}
			let value = rhs.parse().unwrap();
			for addr in addrs {
				mem.insert(addr, value);
			}
		}
	}
	mem.values().sum()
}
