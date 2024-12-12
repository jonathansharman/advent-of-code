use std::collections::{HashMap, HashSet};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 2461);
aoc::test::test_part!(test2, part2, ?);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Ingredient(String);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Allergen(String);

struct Food {
	ingredients: HashSet<Ingredient>,
	allergens: HashSet<Allergen>,
}

fn read_foods() -> Vec<Food> {
	read_lines("input/21.txt")
		.map(|line| {
			let (left, right) =
				line[..line.len() - 1].split_once(" (contains ").unwrap();
			Food {
				ingredients: left
					.split_whitespace()
					.map(|s| Ingredient(s.to_owned()))
					.collect(),
				allergens: right
					.split(", ")
					.map(|s| Allergen(s.to_owned()))
					.collect(),
			}
		})
		.collect()
}

pub fn part1() -> usize {
	let foods = read_foods();

	let mut possible_ingredients: HashMap<Allergen, Vec<HashSet<Ingredient>>> =
		HashMap::new();
	for food in &foods {
		for allergen in &food.allergens {
			possible_ingredients
				.entry(allergen.clone())
				.or_default()
				.push(food.ingredients.clone());
		}
	}
	let mut possible_ingredients: HashMap<Allergen, HashSet<Ingredient>> =
		possible_ingredients
			.into_iter()
			.map(|(allergen, ingredient_sets)| {
				(
					allergen,
					ingredient_sets
						.into_iter()
						.reduce(|left, right| {
							left.intersection(&right).cloned().collect()
						})
						.unwrap(),
				)
			})
			.collect();

	let mut queue: Vec<(Allergen, Ingredient)> = possible_ingredients
		.iter()
		.filter(|(_, ingredients)| ingredients.len() == 1)
		.map(|(allergen, ingredients)| {
			(allergen.clone(), ingredients.iter().next().unwrap().clone())
		})
		.collect();
	let mut allergenic_ingredients = HashSet::new();
	while let Some((determined_allergen, determined_ingredient)) = queue.pop() {
		allergenic_ingredients.insert(determined_ingredient.clone());
		for (allergen, ingredients) in possible_ingredients.iter_mut() {
			if *allergen != determined_allergen
				&& ingredients.remove(&determined_ingredient)
				&& ingredients.len() == 1
			{
				queue.push((
					allergen.clone(),
					ingredients.iter().next().unwrap().clone(),
				));
			}
		}
	}

	foods
		.into_iter()
		.flat_map(|food| {
			food.ingredients.into_iter().filter(|ingredient| {
				!allergenic_ingredients.contains(ingredient)
			})
		})
		.count()
}

pub fn part2() -> usize {
	0
}
