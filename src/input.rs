use crate::grid::Grid;

/// The text of `input.txt`, as a `&'static str`.
#[macro_export]
macro_rules! input {
	() => {
		include_str!("input.txt")
	};
}

pub use input;

pub trait ParseLines {
	fn parse_lines<Item>(&self) -> impl Iterator<Item = Item> + '_
	where
		Item: std::str::FromStr,
		<Item as std::str::FromStr>::Err: std::fmt::Debug;
}

impl ParseLines for &'static str {
	fn parse_lines<Item>(&self) -> impl Iterator<Item = Item> + '_
	where
		Item: std::str::FromStr,
		<Item as std::str::FromStr>::Err: std::fmt::Debug,
	{
		self.lines().map(|line| line.parse().unwrap())
	}
}

pub trait ParseCommaSeparated {
	fn parse_comma_separated<Item>(&self) -> impl Iterator<Item = Item> + '_
	where
		Item: std::str::FromStr,
		<Item as std::str::FromStr>::Err: std::fmt::Debug;
}

impl ParseCommaSeparated for &str {
	fn parse_comma_separated<Item>(&self) -> impl Iterator<Item = Item> + '_
	where
		Item: std::str::FromStr,
		<Item as std::str::FromStr>::Err: std::fmt::Debug,
	{
		self.split(',').map(|s| s.parse::<Item>().unwrap())
	}
}

pub trait ParseGrid {
	fn parse_grid<T>(&self, f: impl Fn(char) -> T) -> Grid<T>;
}

impl ParseGrid for &'static str {
	fn parse_grid<T>(&self, f: impl Fn(char) -> T) -> Grid<T> {
		self.lines().map(|line| line.chars().map(&f)).collect()
	}
}
