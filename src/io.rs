use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(path: P) -> impl Iterator<Item = String>
where
	P: AsRef<Path>,
{
	let file = std::fs::File::open(path).expect("could not open input file");
	BufReader::new(file)
		.lines()
		.map(|line| line.expect("could not read line"))
}

pub fn read_integers<P>(path: P) -> impl Iterator<Item = i32>
where
	P: AsRef<Path>,
{
	read_lines(path).map(|line| line.parse::<i32>().expect("could not parse line as i32"))
}
