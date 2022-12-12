use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{io::read_lines, neighbors};

crate::test::test_part!(test1, part1, 449);
crate::test::test_part!(test2, part2, 443);

type Coords = (usize, usize);

struct Map {
	start: Coords,
	end: Coords,
	heights: Vec<Vec<u32>>,
}

fn read_map() -> Map {
	let mut heights: Vec<Vec<u32>> = Vec::new();
	let (mut start, mut end) = ((0, 0), (0, 0));
	for (r, line) in read_lines("input/2022/12.txt").enumerate() {
		heights.push(Vec::new());
		for (c, b) in line.as_bytes().iter().enumerate() {
			let h = match b {
				b'S' => {
					start = (r, c);
					0
				}
				b'E' => {
					end = (r, c);
					b'z' - b'a'
				}
				_ => b - b'a',
			};
			heights[r].push(h as u32);
		}
	}
	Map {
		start,
		end,
		heights,
	}
}

type Graph = HashMap<Coords, HashSet<Coords>>;

fn build_graph(map: &Map) -> Graph {
	let mut graph = Graph::new();
	let (nrows, ncols) = (map.heights.len(), map.heights[0].len());
	for r in 0..nrows {
		for c in 0..ncols {
			for (nr, nc) in neighbors::four(nrows, ncols, r, c) {
				if map.heights[nr][nc] as i32 - map.heights[r][c] as i32 <= 1 {
					graph
						.entry((r, c))
						.or_insert_with(HashSet::new)
						.insert((nr, nc));
				}
			}
		}
	}
	graph
}

fn flip_graph(graph: Graph) -> Graph {
	let mut flipped = Graph::new();
	for (node, neighbors) in graph {
		for neighbor in neighbors {
			flipped
				.entry(neighbor)
				.or_insert_with(HashSet::new)
				.insert(node);
		}
	}
	flipped
}

fn shortest_distances(
	start: Coords,
	nrows: usize,
	ncols: usize,
	graph: &Graph,
) -> HashMap<Coords, u32> {
	let mut previous = HashMap::new();
	let mut distances = HashMap::new();
	let mut queue = BinaryHeap::new();
	for r in 0..nrows {
		for c in 0..ncols {
			let distance = if (r, c) == start { 0 } else { u32::MAX };
			distances.insert((r, c), distance);
			queue.push((Reverse(distance), (r, c)))
		}
	}
	while let Some((priority, current)) = queue.pop() {
		if priority.0 != distances[&current] {
			continue;
		}
		if let Some(neighbors) = graph.get(&current) {
			for neighbor in neighbors {
				let candidate = distances[&current] + 1;
				if candidate < distances[neighbor] {
					distances.insert(*neighbor, candidate);
					previous.insert(neighbor, current);
					queue.push((Reverse(candidate), *neighbor));
				}
			}
		}
	}
	distances
}

pub fn part1() -> u32 {
	let map = read_map();
	let graph = build_graph(&map);
	shortest_distances(
		map.start,
		map.heights.len(),
		map.heights[0].len(),
		&graph,
	)[&map.end]
}

pub fn part2() -> u32 {
	let map = read_map();
	let graph = flip_graph(build_graph(&map));
	let distances = shortest_distances(
		map.end,
		map.heights.len(),
		map.heights[0].len(),
		&graph,
	);
	let mut m = u32::MAX;
	for r in 0..map.heights.len() {
		for c in 0..map.heights[r].len() {
			if map.heights[r][c] == 0 {
				m = m.min(distances[&(r, c)]);
			}
		}
	}
	println!("{distances:?}");
	m
}
