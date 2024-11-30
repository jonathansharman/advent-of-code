use std::collections::HashSet;

use rayon::prelude::*;

use crate::{graph::Graph, io::read_lines, neighbors};

aoc::test::test_part!(test1, part1, 2414);
aoc::test::test_part!(test2, part2, 6598);

enum Tile {
	Path,
	Forest,
	Up,
	Down,
	Left,
	Right,
}

impl Tile {
	fn allows_up(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Down)
	}

	fn allows_down(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Up)
	}

	fn allows_left(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Right)
	}

	fn allows_right(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Up)
	}
}

pub fn part1() -> usize {
	let tiles: Vec<Vec<Tile>> = read_lines("input/23.txt")
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'.' => Tile::Path,
					'#' => Tile::Forest,
					'^' => Tile::Up,
					'v' => Tile::Down,
					'<' => Tile::Left,
					'>' => Tile::Right,
					_ => panic!("invalid character"),
				})
				.collect()
		})
		.collect();
	longest_slippery_walk(&tiles, (0, 1), (1, 1), 1)
}

// This assumes corridors that all eventually lead to the goal and don't loop.
fn longest_slippery_walk(
	tiles: &[Vec<Tile>],
	(i_last, j_last): (usize, usize),
	(i, j): (usize, usize),
	steps: usize,
) -> usize {
	if i == tiles.len() - 1 && j == tiles[0].len() - 2 {
		return steps;
	}
	let mut result = 0;
	if i > 0 && i_last != i - 1 && tiles[i - 1][j].allows_up() {
		result = result.max(longest_slippery_walk(
			tiles,
			(i, j),
			(i - 1, j),
			steps + 1,
		));
	}
	if i < tiles.len() - 1 && i_last != i + 1 && tiles[i + 1][j].allows_down() {
		result = result.max(longest_slippery_walk(
			tiles,
			(i, j),
			(i + 1, j),
			steps + 1,
		));
	}
	if j > 0 && j_last != j - 1 && tiles[i][j - 1].allows_left() {
		result = result.max(longest_slippery_walk(
			tiles,
			(i, j),
			(i, j - 1),
			steps + 1,
		));
	}
	if j < tiles[0].len() - 1
		&& j_last != j + 1
		&& tiles[i][j + 1].allows_right()
	{
		result = result.max(longest_slippery_walk(
			tiles,
			(i, j),
			(i, j + 1),
			steps + 1,
		));
	}
	result
}

pub fn part2() -> usize {
	let tiles = read_tiles();
	let graph = get_graph(&tiles);
	// Uncomment to generate Graphviz/DOT file:
	// std::fs::write("src/y23/23.dot", graph.graphviz_undirected()).unwrap();
	let goal = (tiles.len() - 1, tiles[0].len() - 2);
	longest_walk(&graph, HashSet::new(), (0, 1), goal, 0)
}

fn read_tiles() -> Vec<Vec<bool>> {
	read_lines("input/23.txt")
		.map(|line| line.chars().map(|c| c != '#').collect())
		.collect()
}

fn get_graph(tiles: &[Vec<bool>]) -> Graph<(usize, usize)> {
	let (n, m) = (tiles.len(), tiles[0].len());
	let mut graph = Graph::new();
	let start = (0, 1);
	let mut nodes = HashSet::from([start]);
	let mut visited = HashSet::from([start]);
	let mut queue = vec![(start, (1, 1), 0)];
	while let Some((node, current, len)) = queue.pop() {
		if !visited.insert(current) {
			continue;
		}
		let mut node_neighbors = Vec::new();
		let mut open_neighbors = Vec::new();
		for n in neighbors::four(n, m, current.0, current.1) {
			if n != node && nodes.contains(&n) {
				node_neighbors.push(n);
			}
			if !visited.contains(&n) && tiles[n.0][n.1] {
				open_neighbors.push(n);
			}
		}
		match open_neighbors.len() {
			0 => {
				if node_neighbors.is_empty() {
					// Dead end. In practice, I think this can only be the goal.
					nodes.insert(current);
					graph.insert_edge(node, current, len);
				}
			}
			1 => {
				// Still in the hallway.
				queue.push((node, open_neighbors[0], len + 1))
			}
			_ => {
				// This is a new node.
				nodes.insert(current);
				graph.insert_edge(node, current, len);
				queue.extend(
					open_neighbors.into_iter().map(|n| (current, n, 0)),
				);
			}
		}
		for n in node_neighbors {
			graph.insert_edge(node, n, len + 1);
		}
	}
	graph
}

fn longest_walk(
	graph: &Graph<(usize, usize)>,
	mut visited: HashSet<(usize, usize)>,
	from: (usize, usize),
	to: (usize, usize),
	steps: usize,
) -> usize {
	if from == to {
		return steps;
	}
	visited.insert(from);
	graph
		.edges_from(&from)
		.unwrap()
		.par_iter()
		.filter(|(n, _)| !visited.contains(n))
		.map(|(&n, weight)| {
			longest_walk(graph, visited.clone(), n, to, steps + 1 + weight)
		})
		.max()
		.unwrap_or_default()
}
