use aoc::{
	graph::Digraph,
	grid::{Grid, Point},
	io::read_lines,
	neighbors,
};

aoc::test::test_part!(test1, part1, 688);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let map = Grid::from_iter(
		read_lines("input/10.txt")
			.map(|line| line.bytes().map(|b| b - b'0').collect()),
	);
	let mut trails = Digraph::new();
	let mut trailheads = Vec::new();
	let mut peaks = Vec::new();
	for (node, height) in map.tiles() {
		match height {
			0 => trailheads.push(node),
			9 => peaks.push(node),
			_ => {}
		}
		// TODO: Port neighbors functionality to the grid module.
		for neighbor in neighbors::four(
			map.height() as usize,
			map.width() as usize,
			node.row as usize,
			node.col as usize,
		)
		.into_iter()
		.filter_map(|neighbor| {
			let neighbor = Point::from(neighbor);
			let neighbor_height = *map.get(neighbor).unwrap();
			(neighbor_height == height + 1).then_some(neighbor)
		}) {
			trails.insert_edge(node, neighbor, 1);
		}
	}
	trailheads
		.into_iter()
		.map(|trailhead| {
			peaks
				.iter()
				.filter(|&peak| {
					trails
						.shortest_distance(trailhead, |node| node == peak)
						.is_some()
				})
				.count()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
