use std::collections::HashMap;

use aoc::{graph::Graph, grid::Grid, io::read_lines};

aoc::test::test_part!(test1, part1, 1437300);
aoc::test::test_part!(test2, part2, ?);

fn read_grid() -> Grid<char> {
	read_lines("input/12.txt")
		.map(|line| line.chars().collect())
		.collect()
}

pub fn part1() -> usize {
	let mut graph = Graph::new();
	let mut plant_perimeters = HashMap::new();
	let grid = read_grid();
	grid.tiles().for_each(|(coords, &plant)| {
		graph.insert_edge(coords, coords, 0);
		let neighbor_coords = grid
			.four_neighbors(coords)
			.filter(|(_, &neighbor_plant)| neighbor_plant == plant)
			.map(|(n_coords, _)| n_coords)
			.collect::<Vec<_>>();
		plant_perimeters.insert(coords, 4 - neighbor_coords.len());
		for n_coords in neighbor_coords {
			graph.insert_edge(coords, n_coords, 1);
		}
	});
	let graphs = graph.into_connected_components();
	graphs
		.into_iter()
		.map(|graph| {
			let region = graph.get_nodes();
			let area = region.len();
			let perimeter: usize =
				region.iter().map(|coords| plant_perimeters[coords]).sum();
			area * perimeter
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
