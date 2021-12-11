mod io;
mod solution;
mod y2021;

use solution::Solution;

use structopt::StructOpt;

use std::collections::HashMap;

#[derive(StructOpt)]
#[structopt(name = "advent-of-code")]
struct Opt {
	#[structopt(short, long, default_value = "2021")]
	year: u32,
	#[structopt(short, long, default_value = "11")]
	day: u32,
	#[structopt(short, long, default_value = "2")]
	part: u32,
}

fn main() {
	let opt = Opt::from_args();

	let solution_map: HashMap<(u32, u32), &dyn Solution> = [
		&y2021::Day01 as &dyn Solution,
		&y2021::Day02,
		&y2021::Day03,
		&y2021::Day04,
		&y2021::Day05,
		&y2021::Day06,
		&y2021::Day07,
		&y2021::Day08,
		&y2021::Day09,
		&y2021::Day10,
		&y2021::Day11,
	]
	.iter()
	.fold(HashMap::new(), |mut acc, &solution| {
		acc.insert((solution.year(), solution.day()), solution);
		acc
	});

	if let Some(solution) = solution_map.get(&(opt.year, opt.day)) {
		if opt.part == 1 {
			return println!("{}-{}-1: {}", opt.year, opt.day, solution.part1());
		} else if opt.part == 2 {
			return println!("{}-{}-2: {}", opt.year, opt.day, solution.part2());
		}
	}
	println!(
		"No solution for {}, day {}, part {}",
		opt.year, opt.day, opt.part
	);
}
