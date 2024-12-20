use crate::grid::Grid;

// TODO: I would love to have an input!() macro that looks up the input file
// corresponding to the name of the calling file, but I haven't been able to
// figure out a way to do that yet.

pub fn parse_lines<Item>(input: &str) -> impl Iterator<Item = Item> + '_
where
	Item: std::str::FromStr,
	<Item as std::str::FromStr>::Err: std::fmt::Debug,
{
	input.lines().map(|line| line.parse().unwrap())
}

pub fn parse_comma_separated_items<Item>(
	input: &str,
) -> impl Iterator<Item = Item> + '_
where
	Item: std::str::FromStr,
	<Item as std::str::FromStr>::Err: std::fmt::Debug,
{
	input.split(',').map(|s| s.parse::<Item>().unwrap())
}

pub fn parse_grid<T>(input: &str, f: impl Fn(char) -> T) -> Grid<T> {
	input
		.lines()
		.map(|line| line.chars().map(&f).collect())
		.collect()
}
