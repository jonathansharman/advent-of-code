use aoc::{input, input::ParseCommaSeparated};
use regex::Regex;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

struct Machine {
	lights: u32,
	buttons: Vec<u32>,
	_joltages: Vec<u32>,
}

fn parse_machines() -> impl Iterator<Item = Machine> {
	let lights_regex = Regex::new(r"\[(.+)\]").unwrap();
	let button_regex = Regex::new(r"\(([0-9,]+)\)").unwrap();
	let joltages_regex = Regex::new(r"\{(.+)\}").unwrap();

	input!().lines().map(move |line| {
		let lights = lights_regex.captures(line).unwrap()[1]
			.chars()
			.enumerate()
			.map(|(i, c)| if c == '#' { 1 << i } else { 0 })
			.sum();
		let buttons = button_regex
			.captures(line)
			.unwrap()
			.iter()
			.skip(1)
			.map(|button| {
				button
					.unwrap()
					.as_str()
					.parse_comma_separated::<u32>()
					.enumerate()
					.map(|(i, bit)| bit << i)
					.sum()
			})
			.collect();
		let joltages = (&joltages_regex.captures(line).unwrap()[1])
			.parse_comma_separated()
			.collect();
		Machine {
			lights,
			buttons,
			_joltages: joltages,
		}
	})
}

fn fewest_presses(machine: Machine) -> u32 {
	(0..2u32.pow(machine.buttons.len() as u32))
		.filter_map(|mut button_mask| {
			let mut lights = 0;
			let mut i = 0;
			while button_mask > 0 {
				if (button_mask & 1) == 1 {
					lights ^= machine.buttons[i];
				}
				i += 1;
				button_mask >>= 1;
			}
			(lights == machine.lights).then(|| button_mask.count_ones())
		})
		.min()
		.unwrap()
}

pub fn part1() -> u32 {
	parse_machines().map(fewest_presses).sum()
}

pub fn part2() -> usize {
	0
}
