use aoc::{input, input::ParseCommaSeparated};
use regex::Regex;

aoc::test::test_part!(test1, part1, 436);
aoc::test::test_part!(test2, part2, ?);

struct Machine {
	lights: u32,
	buttons: Vec<u32>,
	joltages: Vec<u32>,
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
					.parse_comma_separated::<u32>()
					.map(|bit| 1 << bit)
					.sum()
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
				lights ^= (bits & 1) * machine.buttons[i];
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

fn fewest_presses_joltage(
	buttons: &[u32],
	joltages: Vec<u32>,
	current_presses: u32,
) -> Option<u32> {
	let Some((first, rest)) = buttons.split_first() else {
		return joltages
			.into_iter()
			.all(|joltage| joltage == 0)
			.then_some(current_presses);
	};
	// Compute the max number of useful presses of the first button without
	// immediately exceeding any joltage limits.
	let max_presses = joltages
		.iter()
		.enumerate()
		.filter_map(|(i, &joltage)| {
			(((first >> i) & 1) == 1).then_some(joltage)
		})
		.min()
		// Every button should affect at least one joltage, but if one doesn't,
		// then there's no benefit to pressing it.
		.unwrap_or(0);
	(0..=max_presses)
		.filter_map(|presses| {
			let updated_presses = current_presses + presses;
			// The problem expresses joltages as increasing with each button
			// press, but it's simpler to count down.
			let mut reduced_joltages = joltages.clone();
			for (i, reduced_joltage) in reduced_joltages.iter_mut().enumerate()
			{
				*reduced_joltage -= ((first >> i) & 1) * presses;
			}
			fewest_presses_joltage(rest, reduced_joltages, updated_presses)
		})
		.min()
}

pub fn part2() -> u32 {
	parse_machines()
		.map(|machine| {
			fewest_presses_joltage(&machine.buttons, machine.joltages, 0)
				.unwrap()
		})
		.sum()
}
