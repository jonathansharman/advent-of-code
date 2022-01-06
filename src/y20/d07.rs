use crate::io::read_lines;
use std::collections::{HashMap, HashSet};

crate::test::test_part!(test1, part1, 248);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let contained_bags: HashMap<String, Vec<String>> = read_lines("input/2020/07.txt")
		.map(|line| {
			let tokens: Vec<_> = line.split_whitespace().collect();
			let outer = format!("{} {}", tokens[0], tokens[1]);
			let mut inner = Vec::new();
			for i in (4..tokens.len()).step_by(4) {
				if tokens[i].parse::<u8>().is_ok() {
					inner.push(format!("{} {}", tokens[i + 1], tokens[i + 2]));
				}
			}
			(outer, inner)
		})
		.collect();
	let mut bags_containing_gold = HashSet::new();
	for outer in contained_bags.keys() {
		visit(&contained_bags, &mut bags_containing_gold, outer);
	}
	bags_containing_gold.len()
}

fn visit(
	contained_bags: &HashMap<String, Vec<String>>,
	bags_containing_gold: &mut HashSet<String>,
	outer: &str,
) -> bool {
	if bags_containing_gold.contains(outer) {
		return true;
	}
	for inner in contained_bags.get(outer).unwrap().iter() {
		if inner == "shiny gold" || visit(contained_bags, bags_containing_gold, inner) {
			bags_containing_gold.insert(outer.to_owned());
			return true;
		}
	}
	false
}

pub fn part2() -> usize {
	read_lines("input/2020/07.txt").count()
}
