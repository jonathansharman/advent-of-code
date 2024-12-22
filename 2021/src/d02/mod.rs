use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 2187380);
aoc::test::test_part!(test2, part2, 2086357770);

pub fn part1() -> i64 {
	let (depth, latitude) =
		input!()
			.lines()
			.map(|line| {
				let (direction, distance_string) = line
					.split(' ')
					.collect_tuple()
					.expect("expected exactly two elements");
				let distance = distance_string
					.parse::<i64>()
					.expect("could not parse distance");
				(direction.to_owned(), distance)
			})
			.fold((0, 0), |(depth, latitude), (direction, distance)| {
				match direction.as_str() {
					"forward" => (depth, latitude + distance),
					"down" => (depth + distance, latitude),
					"up" => (depth - distance, latitude),
					_ => panic!("unexpected direction: {direction}"),
				}
			});
	depth * latitude
}

pub fn part2() -> i64 {
	let (depth, latitude, _) = input!()
		.lines()
		.map(|line| {
			let (direction, distance_string) = line
				.split(' ')
				.collect_tuple()
				.expect("expected exactly two elements");
			let distance = distance_string
				.parse::<i64>()
				.expect("could not parse distance");
			(direction.to_owned(), distance)
		})
		.fold(
			(0, 0, 0),
			|(depth, latitude, aim), (direction, distance)| match direction
				.as_str()
			{
				"forward" => (depth + aim * distance, latitude + distance, aim),
				"down" => (depth, latitude, aim + distance),
				"up" => (depth, latitude, aim - distance),
				_ => panic!("unexpected direction: {direction}"),
			},
		);
	depth * latitude
}
