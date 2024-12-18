/// Declares a 2D point type and an associated vector type, with customizable
/// type names, field names, and data type.
#[macro_export]
macro_rules! define_point_and_vector {
	($point: ident, $vector: ident, $field1: ident, $field2: ident, $t:ty) => {
		/// A vector of stringify!($part)
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
		pub struct $vector {
			pub $field1: $t,
			pub $field2: $t,
		}

		impl $vector {
			pub const fn new($field1: $t, $field2: $t) -> $vector {
				$vector {
					$field1: $field1,
					$field2: $field2,
				}
			}

			pub const fn zero() -> $vector {
				$vector {
					$field1: 0,
					$field2: 0,
				}
			}
		}

		impl<T: TryInto<$t>> From<(T, T)> for $vector
		where
			<T as TryInto<$t>>::Error: std::fmt::Debug,
		{
			fn from((x, y): (T, T)) -> Self {
				Self {
					$field1: x.try_into().unwrap(),
					$field2: y.try_into().unwrap(),
				}
			}
		}

		impl std::ops::AddAssign for $vector {
			fn add_assign(&mut self, rhs: Self) {
				self.$field1 += rhs.$field1;
				self.$field2 += rhs.$field2;
			}
		}

		impl std::ops::Add<$vector> for $vector {
			type Output = $vector;

			fn add(self, rhs: Self) -> $vector {
				$vector {
					$field1: self.$field1 + rhs.$field1,
					$field2: self.$field2 + rhs.$field2,
				}
			}
		}

		impl std::ops::SubAssign for $vector {
			fn sub_assign(&mut self, rhs: Self) {
				self.$field1 -= rhs.$field1;
				self.$field2 -= rhs.$field2;
			}
		}

		impl std::ops::Sub for $vector {
			type Output = $vector;

			fn sub(self, rhs: Self) -> $vector {
				$vector {
					$field1: self.$field1 - rhs.$field1,
					$field2: self.$field2 - rhs.$field2,
				}
			}
		}

		impl std::ops::MulAssign<$t> for $vector {
			fn mul_assign(&mut self, rhs: $t) {
				self.$field1 *= rhs;
				self.$field2 *= rhs;
			}
		}

		impl std::ops::Mul<$t> for $vector {
			type Output = $vector;

			fn mul(self, rhs: $t) -> $vector {
				$vector {
					$field1: self.$field1 * rhs,
					$field2: self.$field2 * rhs,
				}
			}
		}

		impl std::ops::Mul<$vector> for $t {
			type Output = $vector;

			fn mul(self, rhs: $vector) -> $vector {
				$vector {
					$field1: self * rhs.$field1,
					$field2: self * rhs.$field2,
				}
			}
		}

		impl std::ops::DivAssign<$t> for $vector {
			fn div_assign(&mut self, rhs: $t) {
				self.$field1 /= rhs;
				self.$field2 /= rhs;
			}
		}

		impl std::ops::Div<$t> for $vector {
			type Output = $vector;

			fn div(self, rhs: $t) -> $vector {
				$vector {
					$field1: self.$field1 / rhs,
					$field2: self.$field2 / rhs,
				}
			}
		}

		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
		pub struct $point {
			pub $field1: $t,
			pub $field2: $t,
		}

		impl $point {
			pub const fn new($field1: $t, $field2: $t) -> $point {
				$point {
					$field1: $field1,
					$field2: $field2,
				}
			}

			pub const fn zero() -> $point {
				$point {
					$field1: 0,
					$field2: 0,
				}
			}
		}

		impl<T: TryInto<$t>> From<(T, T)> for $point
		where
			<T as TryInto<$t>>::Error: std::fmt::Debug,
		{
			fn from((x, y): (T, T)) -> Self {
				Self {
					$field1: x.try_into().unwrap(),
					$field2: y.try_into().unwrap(),
				}
			}
		}

		impl std::ops::AddAssign<$vector> for $point {
			fn add_assign(&mut self, rhs: $vector) {
				self.$field1 += rhs.$field1;
				self.$field2 += rhs.$field2;
			}
		}

		impl std::ops::Add<$vector> for $point {
			type Output = $point;

			fn add(self, rhs: $vector) -> $point {
				$point {
					$field1: self.$field1 + rhs.$field1,
					$field2: self.$field2 + rhs.$field2,
				}
			}
		}

		impl std::ops::Add<$point> for $vector {
			type Output = $point;

			fn add(self, rhs: $point) -> $point {
				$point {
					$field1: self.$field1 + rhs.$field1,
					$field2: self.$field2 + rhs.$field2,
				}
			}
		}

		impl std::ops::SubAssign<$vector> for $point {
			fn sub_assign(&mut self, rhs: $vector) {
				self.$field1 -= rhs.$field1;
				self.$field2 -= rhs.$field2;
			}
		}

		impl std::ops::Sub<$vector> for $point {
			type Output = $point;

			fn sub(self, rhs: $vector) -> $point {
				$point {
					$field1: self.$field1 - rhs.$field1,
					$field2: self.$field2 - rhs.$field2,
				}
			}
		}

		impl std::ops::Sub<$point> for $point {
			type Output = $vector;

			fn sub(self, rhs: $point) -> $vector {
				$vector {
					$field1: self.$field1 - rhs.$field1,
					$field2: self.$field2 - rhs.$field2,
				}
			}
		}
	};
}
