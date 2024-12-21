use std::collections::{HashMap, HashSet};

aoc::test::test_part!(test1, part1, 248);
aoc::test::test_part!(test2, part2, 57281);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	let contained_bags = get_contained_bags();
	let mut bags_containing_gold = HashSet::new();
	for outer in contained_bags.keys() {
		contains_gold(&contained_bags, &mut bags_containing_gold, outer);
	}
	bags_containing_gold.len()
}

pub fn part2() -> usize {
	contained_bags_count(&get_contained_bags(), "shiny gold")
}

fn contains_gold(
	contained_bags: &HashMap<String, Vec<(usize, String)>>,
	bags_containing_gold: &mut HashSet<String>,
	outer: &str,
) -> bool {
	if bags_containing_gold.contains(outer) {
		return true;
	}
	for inner in contained_bags.get(outer).unwrap().iter() {
		if inner.1 == "shiny gold"
			|| contains_gold(contained_bags, bags_containing_gold, &inner.1)
		{
			bags_containing_gold.insert(outer.to_owned());
			return true;
		}
	}
	false
}

fn get_contained_bags() -> HashMap<String, Vec<(usize, String)>> {
	INPUT
		.lines()
		.map(|line| {
			let tokens: Vec<_> = line.split_whitespace().collect();
			let outer = format!("{} {}", tokens[0], tokens[1]);
			let mut inner = Vec::new();
			for i in (4..tokens.len()).step_by(4) {
				if let Ok(count) = tokens[i].parse::<usize>() {
					inner.push((
						count,
						format!("{} {}", tokens[i + 1], tokens[i + 2]),
					));
				}
			}
			(outer, inner)
		})
		.collect()
}

fn contained_bags_count(
	contained_bags: &HashMap<String, Vec<(usize, String)>>,
	outer: &str,
) -> usize {
	contained_bags
		.get(outer)
		.unwrap()
		.iter()
		.map(|(count, bag)| {
			count * (1 + contained_bags_count(contained_bags, bag))
		})
		.sum()
}
