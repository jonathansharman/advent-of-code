use std::collections::HashSet;

use aoc::{input, input::ParseCommaSeparated};
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use z3::{Optimize, ast::Int};

aoc::test::test_part!(test1, part1, 436);
aoc::test::test_part!(test2, part2, 14999);

struct Machine {
	lights: u32,
	buttons: Vec<HashSet<usize>>,
	joltages: Vec<u64>,
}

fn parse_machines() -> impl Iterator<Item = Machine> {
	let lights_regex = Regex::new(r"\[(.+)\]").unwrap();
	let button_regex = Regex::new(r"\(([0-9]+(,[0-9]+)*)\)").unwrap();
	let joltages_regex = Regex::new(r"\{(.+)\}").unwrap();

	input!().lines().map(move |line| {
		let lights = lights_regex.captures(line).unwrap()[1]
			.chars()
			.enumerate()
			.map(|(i, c)| if c == '#' { 1 << i } else { 0 })
			.sum();
		let buttons = button_regex
			.captures_iter(line)
			.map(|captures| {
				captures
					.get(1)
					.unwrap()
					.as_str()
					.parse_comma_separated()
					.collect()
			})
			.collect();
		let joltages = (&joltages_regex.captures(line).unwrap()[1])
			.parse_comma_separated()
			.collect();
		Machine {
			lights,
			buttons,
			joltages,
		}
	})
}

fn fewest_presses_lights(machine: Machine) -> u32 {
	(0..2u32.pow(machine.buttons.len() as u32))
		.filter_map(|button_mask| {
			let mut lights = 0;
			let mut i = 0;
			let mut bits = button_mask;
			while bits > 0 {
				if (bits & 1) == 1 {
					for light in &machine.buttons[i] {
						lights ^= 1 << light;
					}
				}
				i += 1;
				bits >>= 1;
			}
			(lights == machine.lights).then(|| button_mask.count_ones())
		})
		.min()
		.unwrap()
}

pub fn part1() -> u32 {
	parse_machines().map(fewest_presses_lights).sum()
}

fn fewest_presses_joltage(machine: Machine) -> u64 {
	let optimizer = Optimize::new();

	let buttons: Vec<Int> = (0..machine.buttons.len())
		.map(|i| Int::new_const(format!("presses{i}")))
		.collect();
	// Buttons cannot be pressed a negative number of times.
	for button in &buttons {
		optimizer.assert(&button.ge(0));
	}

	// Total presses = sum of each button's presses.
	let presses = Int::new_const("presses");
	optimizer.assert(&presses.eq(buttons.iter().sum::<Int>()));

	let joltages: Vec<Int> =
		machine.joltages.into_iter().map(Int::from_u64).collect();
	// Each joltage is equal to the total presses of its associated buttons.
	for (joltage_idx, joltage) in joltages.iter().enumerate() {
		let sum = machine
			.buttons
			.iter()
			.enumerate()
			.filter_map(|(button_idx, joltage_set)| {
				joltage_set
					.contains(&joltage_idx)
					.then_some(&buttons[button_idx])
			})
			.sum::<Int>();
		optimizer.assert(&joltage.eq(sum));
	}

	// Minimize and evaluate total button presses.
	optimizer.minimize(&presses);
	optimizer.check(&[]);
	let model = optimizer.get_model().unwrap();
	model.eval(&presses, true).unwrap().as_u64().unwrap()
}

pub fn part2() -> u64 {
	parse_machines()
		.par_bridge()
		.map(fewest_presses_joltage)
		.sum()
}
