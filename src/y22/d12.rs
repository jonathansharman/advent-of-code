use std::collections::{HashMap, HashSet, VecDeque};

use crate::{io::read_lines, neighbors};

crate::test::test_part!(test1, part1, 449);
crate::test::test_part!(test2, part2, 443);

type Coords = (usize, usize);

struct Map {
	start: Coords,
	end: Coords,
	nrows: usize,
	ncols: usize,
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
		nrows: heights.len(),
		ncols: heights[0].len(),
		heights,
	}
}

type Graph = HashMap<Coords, HashSet<Coords>>;

fn build_graph(map: &Map) -> Graph {
	let mut graph = Graph::new();
	for r in 0..map.nrows {
		for c in 0..map.ncols {
			for (nr, nc) in neighbors::four(map.nrows, map.ncols, r, c) {
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

fn bfs(
	start: Coords,
	nrows: usize,
	ncols: usize,
	graph: &Graph,
) -> Vec<Vec<u32>> {
	let mut distances = Vec::new();
	let mut visited = Vec::new();
	for r in 0..nrows {
		distances.push(Vec::new());
		visited.push(Vec::new());
		for c in 0..ncols {
			if (r, c) == start {
				distances[r].push(0);
				visited[r].push(true);
			} else {
				distances[r].push(u32::MAX);
				visited[r].push(false);
			}
		}
	}
	let mut queue = VecDeque::new();
	queue.push_back(start);
	while let Some(current) = queue.pop_front() {
		if let Some(neighbors) = graph.get(&current) {
			for neighbor in neighbors {
				if visited[neighbor.0][neighbor.1] {
					continue;
				}
				visited[neighbor.0][neighbor.1] = true;
				let candidate = distances[current.0][current.1] + 1;
				if candidate < distances[neighbor.0][neighbor.1] {
					distances[neighbor.0][neighbor.1] = candidate;
					queue.push_back(*neighbor);
				}
			}
		}
	}
	distances
}

pub fn part1() -> u32 {
	let map = read_map();
	let graph = build_graph(&map);
	bfs(map.start, map.nrows, map.ncols, &graph)[map.end.0][map.end.1]
}

pub fn part2() -> u32 {
	let map = read_map();
	let graph = flip_graph(build_graph(&map));
	let distances = bfs(map.end, map.nrows, map.ncols, &graph);
	let mut m = u32::MAX;
	for (r, height_row) in map.heights.iter().enumerate() {
		for (c, h) in height_row.iter().enumerate() {
			if *h == 0 {
				m = m.min(distances[r][c]);
			}
		}
	}
	m
}
