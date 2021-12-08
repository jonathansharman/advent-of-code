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
	#[structopt(short, long, default_value = "8")]
	day: u32,
	#[structopt(short, long, default_value = "2")]
	part: u32,
}

fn main() {
	let opt = Opt::from_args();

	let solution_map: HashMap<(u32, u32), &dyn Solution> = [
		&y2021::Day1 as &dyn Solution,
		&y2021::Day2,
		&y2021::Day3,
		&y2021::Day4,
		&y2021::Day5,
		&y2021::Day6,
		&y2021::Day7,
		&y2021::Day8,
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
