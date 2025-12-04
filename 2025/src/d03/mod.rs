use std::collections::HashMap;

use aoc::input;

aoc::test::test_part!(test1, part1, 17408);
aoc::test::test_part!(test2, part2, 172740584266849);

#[derive(PartialEq, Eq, Hash)]
struct State<'a> {
	bank: &'a [u8],
	batteries_left: usize,
}

fn max_jolts<'a>(
	cache: &mut HashMap<State<'a>, usize>,
	state: State<'a>,
) -> usize {
	// Check cache.
	if let Some(&max) = cache.get(&state) {
		return max;
	}

	if state.batteries_left == 0 {
		// Base case: not allowed to add any more batteries.
		return 0;
	}

	let Some((first, rest)) = state.bank.split_first() else {
		// Base case: no additional batteries to choose from.
		return 0;
	};

	// Convert first battery character to integer.
	let first = (first - b'0') as usize;

	// Recursively compute the max for the rest of the bank after the first,
	// using all remaining batteries.
	let max1 = max_jolts(
		cache,
		State {
			bank: rest,
			batteries_left: state.batteries_left,
		},
	);

	// Compute the max for the rest when spending one battery on the first.
	let max2 = max_jolts(
		cache,
		State {
			bank: rest,
			batteries_left: state.batteries_left - 1,
		},
	);

	// Combine the smaller subresult with the first battery, which needs to be
	// shifted based on the number of digits in the subresult.
	let max2 = max2
		.checked_ilog10()
		.map_or(first, |log| first * 10usize.pow(log + 1) + max2);

	// Compute and cache the answer as the better of the two choices.
	let max = max1.max(max2);
	cache.insert(state, max);

	max
}

fn solve(max_batteries: usize) -> usize {
	input!()
		.lines()
		.map(|bank| {
			max_jolts(
				&mut HashMap::new(),
				State {
					bank: bank.as_bytes(),
					batteries_left: max_batteries,
				},
			)
		})
		.sum()
}

pub fn part1() -> usize {
	solve(2)
}

pub fn part2() -> usize {
	solve(12)
}
