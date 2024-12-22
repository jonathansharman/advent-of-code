use std::collections::{HashMap, HashSet};

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 2461);
aoc::test::test_part!(
	test2,
	part2,
	"ltbj,nrfmm,pvhcsn,jxbnb,chpdjkf,jtqt,zzkq,jqnhd".to_string()
);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Ingredient(String);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Allergen(String);

struct Food {
	ingredients: HashSet<Ingredient>,
	allergens: HashSet<Allergen>,
}

fn read_foods() -> Vec<Food> {
	input!()
		.lines()
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

fn ingredient_allergens(foods: &[Food]) -> HashMap<Ingredient, Allergen> {
	let mut possible_ingredients: HashMap<Allergen, Vec<HashSet<Ingredient>>> =
		HashMap::new();
	for food in foods {
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
	let mut ingredient_allergens = HashMap::new();
	while let Some((determined_allergen, determined_ingredient)) = queue.pop() {
		ingredient_allergens
			.insert(determined_ingredient.clone(), determined_allergen.clone());
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
	ingredient_allergens
}

pub fn part1() -> usize {
	let foods = read_foods();
	let ingredient_allergens = ingredient_allergens(&foods);
	foods
		.into_iter()
		.flat_map(|food| {
			food.ingredients.into_iter().filter(|ingredient| {
				!ingredient_allergens.contains_key(ingredient)
			})
		})
		.count()
}

pub fn part2() -> String {
	let foods = read_foods();
	ingredient_allergens(&foods)
		.into_iter()
		.sorted_by(|(_, a), (_, b)| a.0.cmp(&b.0))
		.map(|(ingredient, _)| ingredient.0)
		.join(",")
}
