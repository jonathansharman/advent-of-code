use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 5962);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> isize {
	let mut numbers = read_lines("input/2022/20.txt")
		.enumerate()
		.map(|(i, line)| (i, line.parse::<isize>().unwrap()))
		.collect_vec();
	let m = (numbers.len() - 1) as isize;
	for i in 0..numbers.len() {
		let cur_idx = numbers.iter().position(|(j, _)| *j == i).unwrap();
		let shift = numbers[cur_idx].1;
		let target_idx = (((cur_idx as isize + shift) % m + m) % m) as usize;
		match cur_idx.cmp(&target_idx) {
			std::cmp::Ordering::Equal => (),
			std::cmp::Ordering::Less => {
				numbers.insert(target_idx + 1, numbers[cur_idx]);
				numbers.remove(cur_idx);
			}
			std::cmp::Ordering::Greater => {
				numbers.insert(target_idx, numbers[cur_idx]);
				numbers.remove(cur_idx + 1);
			}
		}
	}
	let numbers = numbers.into_iter().map(|(_, n)| n).collect_vec();
	let zero_idx = numbers.iter().position(|n| *n == 0).unwrap();
	numbers[(zero_idx + 1000) % numbers.len()]
		+ numbers[(zero_idx + 2000) % numbers.len()]
		+ numbers[(zero_idx + 3000) % numbers.len()]
}

pub fn part2() -> isize {
	let mut numbers = read_lines("input/2022/20.txt")
		.enumerate()
		.map(|(i, line)| (i, 811589153 * line.parse::<isize>().unwrap()))
		.collect_vec();
	let m = (numbers.len() - 1) as isize;
	for _ in 0..10 {
		for i in 0..numbers.len() {
			let cur_idx = numbers.iter().position(|(j, _)| *j == i).unwrap();
			let shift = numbers[cur_idx].1;
			let target_idx =
				(((cur_idx as isize + shift) % m + m) % m) as usize;
			match cur_idx.cmp(&target_idx) {
				std::cmp::Ordering::Equal => (),
				std::cmp::Ordering::Less => {
					numbers.insert(target_idx + 1, numbers[cur_idx]);
					numbers.remove(cur_idx);
				}
				std::cmp::Ordering::Greater => {
					numbers.insert(target_idx, numbers[cur_idx]);
					numbers.remove(cur_idx + 1);
				}
			}
		}
	}
	let numbers = numbers.into_iter().map(|(_, n)| n).collect_vec();
	let zero_idx = numbers.iter().position(|n| *n == 0).unwrap();
	numbers[(zero_idx + 1000) % numbers.len()]
		+ numbers[(zero_idx + 2000) % numbers.len()]
		+ numbers[(zero_idx + 3000) % numbers.len()]
}
