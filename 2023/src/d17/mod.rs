use aoc::{
	graph::Digraph,
	grid::{Grid, Point},
	input,
};

aoc::test::test_part!(test1, part1, 970);
aoc::test::test_part!(test2, part2, 1149);

pub fn part1() -> usize {
	solve(1, 3)
}

pub fn part2() -> usize {
	solve(4, 10)
}

type Coords = (i64, i64, i64);

fn solve(min: i64, max: i64) -> usize {
	let costs = read_costs();
	let graph = get_graph(&costs, min, max);
	let pred = |c: &Coords| -> bool {
		c.0 == costs.row_count() - 1 && c.1 == costs.col_count() - 1
	};
	graph
		.shortest_distance((0, 0, 0), pred)
		.min(graph.shortest_distance((0, 0, 1), pred))
		.unwrap()
}

fn read_costs() -> Grid<usize> {
	input!()
		.lines()
		.map(|line| line.bytes().map(|b| (b - b'0') as usize))
		.collect()
}

fn get_graph(costs: &Grid<usize>, min: i64, max: i64) -> Digraph<Coords> {
	// Model the graph as two planes. Each move must move between min and max
	// spaces vertically or horizontally (depending on the current plane) and
	// move to the other plane, to force alternating horizontal and vertical
	// move sequences.
	let mut graph = Digraph::new();
	for (p, _) in costs {
		let n0 = (p.row, p.col, 0);
		let mut cost = 0;
		for row in (p.row - max..p.row).rev() {
			if let Some(c) = costs.get(Point::new(row, p.col)) {
				cost += c;
				if p.row - row >= min {
					graph.insert_edge(n0, (row, p.col, 1), cost);
				}
			}
		}
		let mut cost = 0;
		for row in p.row + 1..=p.row + max {
			if let Some(c) = costs.get(Point::new(row, p.col)) {
				cost += c;
				if row - p.row >= min {
					graph.insert_edge(n0, (row, p.col, 1), cost);
				}
			}
		}
		let n1 = (p.row, p.col, 1);
		let mut cost = 0;
		for col in (p.col - max..p.col).rev() {
			if let Some(c) = costs.get(Point::new(p.row, col)) {
				cost += c;
				if p.col - col >= min {
					graph.insert_edge(n1, (p.row, col, 0), cost);
				}
			}
		}
		let mut cost = 0;
		for col in p.col + 1..=p.col + max {
			if let Some(c) = costs.get(Point::new(p.row, col)) {
				cost += c;
				if col - p.col >= min {
					graph.insert_edge(n1, (p.row, col, 0), cost);
				}
			}
		}
	}
	graph
}
