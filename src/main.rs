mod io;
mod y2021;

use std::collections::HashMap;

fn main() {
	let solution_map: HashMap<(i32, i32, i32), fn() -> i64> = [
		((2021, 1, 1), y2021::d1::part1 as fn() -> i64),
		((2021, 1, 2), y2021::d1::part2),
		((2021, 2, 1), y2021::d2::part1),
		((2021, 2, 2), y2021::d2::part2),
		((2021, 3, 1), y2021::d3::part1),
		((2021, 3, 2), y2021::d3::part2),
		((2021, 4, 1), y2021::d4::part1),
		((2021, 4, 2), y2021::d4::part2),
		((2021, 5, 1), y2021::d5::part1),
		((2021, 5, 2), y2021::d5::part2),
		((2021, 6, 1), y2021::d6::part1),
		((2021, 6, 2), y2021::d6::part2),
		((2021, 7, 1), y2021::d7::part1),
		((2021, 7, 2), y2021::d7::part2),
	]
	.iter()
	.cloned()
	.collect();

	let (year, day, part) = (2021, 7, 1);
	println!(
		"{}-{}-{}: {}",
		year,
		day,
		part,
		solution_map[&(year, day, part)]()
	);
}
