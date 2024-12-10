use std::ops::{
	Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

/// Row-column grid coordinates.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
	pub row: i64,
	pub col: i64,
}

impl<T: TryInto<i64>> From<(T, T)> for Point
where
	<T as TryInto<i64>>::Error: std::fmt::Debug,
{
	fn from((x, y): (T, T)) -> Self {
		Self {
			row: x.try_into().unwrap(),
			col: y.try_into().unwrap(),
		}
	}
}

impl AddAssign for Point {
	fn add_assign(&mut self, rhs: Self) {
		self.row += rhs.row;
		self.col += rhs.col;
	}
}

impl Add<Point> for Point {
	type Output = Point;

	fn add(self, rhs: Self) -> Point {
		Point {
			row: self.row + rhs.row,
			col: self.col + rhs.col,
		}
	}
}

impl SubAssign for Point {
	fn sub_assign(&mut self, rhs: Self) {
		self.row -= rhs.row;
		self.col -= rhs.col;
	}
}

impl Sub for Point {
	type Output = Point;

	fn sub(self, rhs: Self) -> Point {
		Point {
			row: self.row - rhs.row,
			col: self.col - rhs.col,
		}
	}
}

impl MulAssign<i64> for Point {
	fn mul_assign(&mut self, rhs: i64) {
		self.row *= rhs;
		self.col *= rhs;
	}
}

impl Mul<i64> for Point {
	type Output = Point;

	fn mul(self, rhs: i64) -> Point {
		Point {
			row: self.row * rhs,
			col: self.col * rhs,
		}
	}
}

impl Mul<Point> for i64 {
	type Output = Point;

	fn mul(self, rhs: Point) -> Point {
		Point {
			row: self * rhs.row,
			col: self * rhs.col,
		}
	}
}

impl DivAssign<i64> for Point {
	fn div_assign(&mut self, rhs: i64) {
		self.row /= rhs;
		self.col /= rhs;
	}
}

impl Div<i64> for Point {
	type Output = Point;

	fn div(self, rhs: i64) -> Point {
		Point {
			row: self.row / rhs,
			col: self.col / rhs,
		}
	}
}

/// A rectangular grid of tiles.
#[derive(Clone, Debug)]
pub struct Grid<T> {
	tiles: Vec<Vec<T>>,
}

impl<T> Grid<T> {
	pub fn tiles(&self) -> impl Iterator<Item = (Point, &T)> {
		self.rows().enumerate().flat_map(|(i, row)| {
			row.iter()
				.enumerate()
				.map(move |(j, tile)| (Point::from((i, j)), tile))
		})
	}

	/// An iterator over the rows of the grid.
	pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
		self.tiles.iter()
	}

	/// A reference to the element at `coords`, if in bounds.
	pub fn get(&self, coords: Point) -> Option<&T> {
		let row: usize = coords.row.try_into().ok()?;
		let col: usize = coords.col.try_into().ok()?;
		self.tiles.get(row).and_then(|row| row.get(col))
	}

	/// Whether `coords` is in bounds.
	pub fn contains(&self, coords: Point) -> bool {
		self.get(coords).is_some()
	}

	/// The number of rows in the grid.
	pub fn height(&self) -> i64 {
		self.tiles.len() as i64
	}

	/// The number of columns in the grid. The grid must have at least one row.
	/// This assumes that all rows have the same width.
	pub fn width(&self) -> i64 {
		self.tiles[0].len() as i64
	}
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
	fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
		Self {
			tiles: iter
				.into_iter()
				.map(|row| row.into_iter().collect())
				.collect(),
		}
	}
}
