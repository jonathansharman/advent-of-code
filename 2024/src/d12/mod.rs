use std::collections::{BTreeSet, HashMap, HashSet};

use aoc::{
	graph::Graph,
	grid::{Grid, Point, Vector},
	input,
};

aoc::test::test_part!(test1, part1, 1437300);
aoc::test::test_part!(test2, part2, 849332);

fn read_grid() -> Grid<char> {
	input!()
		.lines()
		.map(|line| line.chars().collect())
		.collect()
}

pub fn part1() -> usize {
	let mut graph = Graph::new();
	let mut plant_perimeters = HashMap::new();
	let grid = read_grid();
	for (coords, &plant) in &grid {
		graph.insert_edge(coords, coords, 0);
		let neighbor_coords = grid
			.four_neighbors(coords)
			.filter(|&(_, &neighbor_plant)| neighbor_plant == plant)
			.map(|(n_coords, _)| n_coords)
			.collect::<Vec<_>>();
		plant_perimeters.insert(coords, 4 - neighbor_coords.len());
		for n_coords in neighbor_coords {
			graph.insert_edge(coords, n_coords, 1);
		}
	}
	let graphs = graph.into_connected_components();
	graphs
		.into_iter()
		.map(|graph| {
			let region = graph.nodes();
			let area = region.len();
			let perimeter: usize =
				region.iter().map(|coords| plant_perimeters[coords]).sum();
			area * perimeter
		})
		.sum()
}

fn sides(region: &HashSet<Point>) -> usize {
	sides_towards(region, (-1, 0).into())
		+ sides_towards(region, (1, 0).into())
		+ sides_towards(region, (0, -1).into())
		+ sides_towards(region, (0, 1).into())
}

fn sides_towards(region: &HashSet<Point>, offset: Vector) -> usize {
	let side_offset = Vector {
		row: offset.col,
		col: offset.row,
	};
	let mut sides = 0;
	let mut queue: BTreeSet<Point> = region.iter().cloned().collect();
	while let Some(coords) = queue.pop_first() {
		if !region.contains(&(coords + offset)) {
			sides += 1;
			for n in 1.. {
				let colinear = coords + n * side_offset;
				if region.contains(&(colinear))
					&& !region.contains(&(colinear + offset))
				{
					queue.remove(&colinear);
				} else {
					break;
				}
			}
			for n in 1.. {
				let colinear = coords - n * side_offset;
				if region.contains(&(colinear))
					&& !region.contains(&(colinear + offset))
				{
					queue.remove(&colinear);
				} else {
					break;
				}
			}
		}
	}
	sides
}

pub fn part2() -> usize {
	let mut graph = Graph::new();
	let grid = read_grid();
	for (coords, &plant) in &grid {
		graph.insert_edge(coords, coords, 0);
		grid.four_neighbors(coords)
			.filter(|&(_, &neighbor_plant)| neighbor_plant == plant)
			.map(|(n_coords, _)| n_coords)
			.for_each(|n_coords| {
				graph.insert_edge(coords, n_coords, 1);
			});
	}
	let graphs = graph.into_connected_components();
	graphs
		.into_iter()
		.map(|graph| {
			let region = graph.nodes();
			let area = region.len();
			area * sides(region)
		})
		.sum()
}
