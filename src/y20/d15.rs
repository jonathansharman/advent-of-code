use std::collections::HashMap;

use crate::io::read_comma_separated_integers;

crate::test::test_part!(test1, part1, 403);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut called_numbers = read_comma_separated_integers("input/2020/15.txt")
		.collect::<Vec<usize>>();
	let mut last_called_turns = called_numbers[..called_numbers.len() - 1]
		.iter()
		.enumerate()
		.map(|(i, n)| (*n, i))
		.collect::<HashMap<_, _>>();
	loop {
		let turn = called_numbers.len();
		let last_called = *called_numbers.last().unwrap();
		if turn == 2020 {
			return last_called;
		}
		let call = last_called_turns
			.get(&last_called)
			.map(|last_called_turn| turn - 1 - last_called_turn)
			.unwrap_or_default();
		last_called_turns.insert(last_called, turn - 1);
		called_numbers.push(call);
	}
}

pub fn part2() -> usize {
	0
}
