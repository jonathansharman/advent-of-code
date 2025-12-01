use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
	grid::{Grid, Point, Vector},
	input,
};

aoc::test::test_part!(test1, part1, 449);
aoc::test::test_part!(test2, part2, 443);

struct Map {
	start: Point,
	end: Point,
	heights: Grid<u32>,
}

fn read_map() -> Map {
	let (mut start, mut end) = (Point::zero(), Point::zero());
	let heights = input!()
		.lines()
		.enumerate()
		.map(|(r, line)| {
			line.as_bytes()
				.iter()
				.enumerate()
				.map(|(c, b)| {
					let coords = (r, c).into();
					let h = match b {
						b'S' => {
							start = coords;
							0
						}
						b'E' => {
							end = coords;
							b'z' - b'a'
						}
						_ => b - b'a',
					};
					h as u32
				})
				.collect()
		})
		.collect();
	Map {
		start,
		end,
		heights,
	}
}

type Graph = HashMap<Point, HashSet<Point>>;

fn build_graph(map: &Map) -> Graph {
	let mut graph = Graph::new();
	for (coords, &h) in &map.heights {
		for (neighbor_coords, &nh) in map.heights.four_neighbors(coords) {
			if nh <= h + 1 {
				graph.entry(coords).or_default().insert(neighbor_coords);
			}
		}
	}
	graph
}

fn flip_graph(graph: Graph) -> Graph {
	let mut flipped = Graph::new();
	for (node, neighbors) in graph {
		for neighbor in neighbors {
			flipped.entry(neighbor).or_default().insert(node);
		}
	}
	flipped
}

fn bfs(start: Point, size: Vector, graph: &Graph) -> Grid<u32> {
	let mut distances = Grid::new(size, u32::MAX);
	let mut visited = Grid::new(size, false);
	distances[start] = 0;
	visited[start] = true;

	let mut queue = VecDeque::new();
	queue.push_back(start);
	while let Some(current) = queue.pop_front() {
		if let Some(neighbors) = graph.get(&current) {
			for &neighbor in neighbors {
				if visited[neighbor] {
					continue;
				}
				visited[neighbor] = true;
				let candidate = distances[current] + 1;
				if candidate < distances[neighbor] {
					distances[neighbor] = candidate;
					queue.push_back(neighbor);
				}
			}
		}
	}
	distances
}

pub fn part1() -> u32 {
	let map = read_map();
	let graph = build_graph(&map);
	bfs(map.start, map.heights.size(), &graph)[map.end]
}

pub fn part2() -> u32 {
	let map = read_map();
	let graph = flip_graph(build_graph(&map));
	let distances = bfs(map.end, map.heights.size(), &graph);
	map.heights
		.iter()
		.filter(|&(_, &h)| h == 0)
		.map(|(coords, _)| distances[coords])
		.min()
		.unwrap()
}
