use num::PrimInt;

/// Given `a`, `b`, and `c` where ax + by = c, returns a function f: Z → (Z, Z)
/// that produces solutions to the equation, if such a function exists.
pub fn solve_diophantine<T: PrimInt>(
	a: T,
	b: T,
	c: T,
) -> Option<impl Fn(T) -> (T, T)> {
	// Use the extended Euclidean algorithm on a and b to find their GCD and
	// Bézout's coefficients x and y such that ax + by = gcd(x, y).
	let results = extended_euclidean(a, b);
	let (x, y) = results.bezout_coefficients;

	// If the GCD does not divide c, then there is no integer solution to the
	// original equation.
	if c % results.gcd != T::zero() {
		return None;
	}

	// Multiply both sides by c / gcd(x, y) to find one solution (x0, y0) to the
	// original equation, i.e. where a*x0 + b*y0 = c.
	let (x0, y0) = (x * c / results.gcd, y * c / results.gcd);

	// There are infinitely many additional solutions of the form
	// (x0 + bt, y0 - at), where t is an integer.
	Some(move |t| (x0 + b * t, y0 - a * t))
}

/// Outputs of the extended Euclidean algorithm.
struct ExtendedEuclidean<T: PrimInt> {
	pub bezout_coefficients: (T, T),
	pub gcd: T,
}

/// An implementation of the extended Euclidean algorithm, adapted from
/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm.
fn extended_euclidean<T: PrimInt>(a: T, b: T) -> ExtendedEuclidean<T> {
	let (mut old_r, mut r) = (a, b);
	let (mut old_s, mut s) = (T::one(), T::zero());
	let (mut old_t, mut t) = (T::zero(), T::one());

	while r != T::zero() {
		let quotient = old_r / r;
		(old_r, r) = (r, old_r - quotient * r);
		(old_s, s) = (s, old_s - quotient * s);
		(old_t, t) = (t, old_t - quotient * t);
	}

	ExtendedEuclidean {
		bezout_coefficients: (old_s, old_t),
		gcd: old_r,
	}
}

#[cfg(test)]
mod test_solve_diophantine {
	use super::*;

	#[test]
	fn solution() {
		let (a, b, c) = (1, 2, 3);
		let f = solve_diophantine(a, b, c).expect("expected integer solutions");
		for t in -10..=10 {
			let (x0, y0) = f(t);
			assert_eq!(a * (x0 + b * t) + b * (y0 - a * t), c);
		}
	}

	#[test]
	fn no_solution() {
		let (a, b, c) = (2, 4, 3);
		if solve_diophantine(a, b, c).is_some() {
			panic!("expected no integer solutions");
		};
	}
}
