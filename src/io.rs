use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::grid::Grid;

pub fn read_lines<P>(path: P) -> impl Iterator<Item = String>
where
	P: AsRef<Path>,
{
	let file = std::fs::File::open(path).unwrap();
	BufReader::new(file).lines().map(Result::unwrap)
}

pub fn parse_lines<P, Item>(path: P) -> impl Iterator<Item = Item>
where
	P: AsRef<Path>,
	Item: std::str::FromStr,
	<Item as std::str::FromStr>::Err: std::fmt::Debug,
{
	read_lines(path).map(|line| line.parse().unwrap())
}

pub fn read_comma_separated_integers<P, Item>(
	path: P,
) -> impl Iterator<Item = Item>
where
	P: AsRef<Path>,
	Item: std::str::FromStr,
	<Item as std::str::FromStr>::Err: std::fmt::Debug,
{
	let file = std::fs::File::open(path).unwrap();
	BufReader::new(file).split(b',').map(|s| {
		std::str::from_utf8(&s.unwrap())
			.unwrap()
			.parse::<Item>()
			.unwrap()
	})
}

pub fn read_grid<P, T>(path: P, f: impl Fn(char) -> T) -> Grid<T>
where
	P: AsRef<Path>,
{
	let file = std::fs::File::open(path).unwrap();
	BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().chars().map(&f).collect())
		.collect()
}
