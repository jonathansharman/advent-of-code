use std::collections::{HashMap, HashSet};

use aoc::{input, input::ParseCommaSeparated};
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

aoc::test::test_part!(test1, part1, 436);
aoc::test::test_part!(test2, part2, ?);

struct Machine {
	lights: u32,
	buttons: Vec<HashSet<usize>>,
	joltages: Vec<Joltage>,
}

#[derive(Clone, Debug)]
struct Joltage {
	level: u32,
	buttons: HashSet<usize>,
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
		let buttons: Vec<HashSet<usize>> = button_regex
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
			.enumerate()
			.map(|(i, level)| {
				let buttons = buttons
					.iter()
					.enumerate()
					.filter_map(|(j, button_joltages)| {
						button_joltages.contains(&i).then_some(j)
					})
					.collect();
				Joltage { level, buttons }
			})
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

fn fewest_presses_joltage(
	buttons: HashMap<usize, HashSet<usize>>,
	joltages: HashMap<usize, Joltage>,
	current_presses: u32,
) -> Option<u32> {
	// Choose the joltage to eliminate based on which has the fewest affecting
	// buttons, to reduce the search space of possible buttons to press. Break
	// ties arbitrarily using the joltage indices.
	let Some((_, joltage)) = joltages.iter().min_by(|(i1, j1), (i2, j2)| {
		j1.buttons.len().cmp(&j2.buttons.len()).then(i1.cmp(i2))
	}) else {
		return Some(current_presses);
	};

	// Choose an arbitrary button that affects this joltage.
	let button_idx = joltage.buttons.iter().next().unwrap();
	let button_joltages = &buttons[button_idx];

	// Try pressing the button some number of times up to the remaining joltage
	// level, L. As an optimization, if only one button affects this joltage, we
	// only need to try exactly L presses since pressing it fewer times
	// certainly can't produce a valid solution.
	let min_presses = if joltage.buttons.len() == 1 {
		joltage.level
	} else {
		0
	};
	(min_presses..=joltage.level)
		.filter_map(|presses| {
			let next_presses = current_presses + presses;

			// This button will not be pressed again.
			let mut next_buttons = buttons.clone();
			next_buttons.remove(button_idx);

			// Update each joltage to account for the button presses.
			let mut next_joltages = joltages.clone();
			for joltage_idx in button_joltages {
				// If a joltage for this button is missing, then that
				// joltage has already reached zero, and this button can't
				// be pressed.
				let joltage = next_joltages.get_mut(joltage_idx)?;
				// Remove the pressed button from the joltages' button sets.
				joltage.buttons.remove(button_idx);
				// A joltage may not be reduced below zero.
				joltage.level = joltage.level.checked_sub(presses)?;
				// Remove a joltage when its level reaches zero.
				if joltage.level == 0 {
					next_joltages.remove(joltage_idx);
					for button_joltages in next_buttons.values_mut() {
						button_joltages.remove(joltage_idx);
					}
				} else if joltage.buttons.is_empty() {
					// This joltage level wasn't satisfied but also no
					// longer has any buttons, so there's no solution on
					// this path.
					return None;
				}
			}

			fewest_presses_joltage(next_buttons, next_joltages, next_presses)
		})
		.min()
}

pub fn part2() -> u32 {
	parse_machines()
		.par_bridge()
		.map(|machine| {
			fewest_presses_joltage(
				machine.buttons.into_iter().enumerate().collect(),
				machine.joltages.into_iter().enumerate().collect(),
				0,
			)
			.unwrap()
		})
		.sum()
}
