use std::{
	fmt::Debug,
	ops::{
		Add,
		Mul,
		Sub,
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position<T>(pub T, pub T);

impl<T: Add<Output = T> + Copy> Add for Position<T>
{
	type Output = Position<T>;

	fn add(
		self,
		rhs: Self,
	) -> Self::Output
	{
		Position(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl<T: Sub<Output = T> + Copy> Sub for Position<T>
{
	type Output = Position<T>;

	fn sub(
		self,
		rhs: Self,
	) -> Self::Output
	{
		Position(self.0 - rhs.0, self.1 - rhs.1)
	}
}

impl<T: Mul<Output = T> + Copy> Mul for Position<T>
{
	type Output = Position<T>;

	fn mul(
		self,
		rhs: Self,
	) -> Self::Output
	{
		Position(self.0 * rhs.0, self.1 * rhs.1)
	}
}

impl<T: Ord + Copy> Position<T>
{
	pub fn check_bounds<UT: TryFrom<T> + Ord>(
		self,
		min: &Position<UT>,
		max: &Position<UT>,
	) -> Option<Position<UT>>
	where
		<UT as std::convert::TryFrom<T>>::Error: std::fmt::Debug,
	{
		let row = self.0.try_into();
		let column = self.1.try_into();
		if let Ok(row) = row
        && let Ok(column) = column
        && row >= min.0 && column >= min.1
        && row < max.0 && column < max.1
		{
			Some(Position(row, column))
		}
		else
		{
			None
		}
	}
}

#[cfg(test)]
mod tests
{
	use {
		super::Position,
		crate::direction::Direction::{
			self,
			*,
		},
		test_case::test_case,
	};

	#[test_case(Position(0, 0), Up => Position(-1, 0))]
	#[test_case(Position(0, 0), Down => Position(1, 0))]
	#[test_case(Position(0, 0), Left => Position(0, -1))]
	#[test_case(Position(0, 0), Right => Position(0, 1))]
	#[test_case(Position(0, 0), To => Position(-1, 1))]
	#[test_case(Position(0, 0), Fro => Position(1, -1))]
	#[test_case(Position(3, 3), Up => Position(2, 3))]
	#[test_case(Position(3, 3), Down => Position(4, 3))]
	#[test_case(Position(3, 3), Left => Position(3, 2))]
	#[test_case(Position(3, 3), Right => Position(3, 4))]
	#[test_case(Position(3, 3), To => Position(2, 4))]
	#[test_case(Position(3, 3), Fro => Position(4, 2))]
	fn test_add_position(
		start_position: Position<isize>,
		direction: Direction,
	) -> Position<isize>
	{
		start_position + direction.to_delta()
	}

	#[test_case(Position(0, 0), Up, 2 => Position(-2, 0))]
	#[test_case(Position(0, 0), Down, 2 => Position(2, 0))]
	#[test_case(Position(0, 0), Left, 2 => Position(0, -2))]
	#[test_case(Position(0, 0), Right, 2 => Position(0, 2))]
	#[test_case(Position(0, 0), To, 2 => Position(-2, 2))]
	#[test_case(Position(0, 0), Fro, 2 => Position(2, -2))]
	#[test_case(Position(3, 3), Up, 2 => Position(1, 3))]
	#[test_case(Position(3, 3), Down, 2 => Position(5, 3))]
	#[test_case(Position(3, 3), Left, 2 => Position(3, 1))]
	#[test_case(Position(3, 3), Right, 2 => Position(3, 5))]
	#[test_case(Position(3, 3), To, 2 => Position(1, 5))]
	#[test_case(Position(3, 3), Fro, 2 => Position(5, 1))]
	fn test_mul_position(
		position: Position<isize>,
		direction: Direction,
		magnitude: isize,
	) -> Position<isize>
	{
		let delta = direction.to_delta();
		position + delta * Position(magnitude, magnitude)
	}

	#[test_case(Position(3, 3), 3, 3 => None)]
	#[test_case(Position(2, 2), 3, 3 => Some(Position(2, 2)))]
	#[test_case(Position(1, 1), 3, 3 => Some(Position(1, 1)))]
	#[test_case(Position(0, 0), 3, 3 => Some(Position(0, 0)))]
	#[test_case(Position(-1, -1), 3, 3 => None)]
	#[test_case(Position(4, 4), 4, 4 => None)]
	#[test_case(Position(3, 3), 4, 4 => Some(Position(3, 3)))]
	#[test_case(Position(2, 2), 4, 4 => Some(Position(2, 2)))]
	#[test_case(Position(1, 1), 4, 4 => Some(Position(1, 1)))]
	#[test_case(Position(0, 0), 4, 4 => Some(Position(0, 0)))]
	#[test_case(Position(-1, -1), 4, 4 => None)]
	#[test_case(Position(5, 5), 5, 5 => None)]
	#[test_case(Position(4, 4), 5, 5 => Some(Position(4, 4)))]
	#[test_case(Position(3, 3), 5, 5 => Some(Position(3, 3)))]
	#[test_case(Position(2, 2), 5, 5 => Some(Position(2, 2)))]
	#[test_case(Position(1, 1), 5, 5 => Some(Position(1, 1)))]
	#[test_case(Position(0, 0), 5, 5 => Some(Position(0, 0)))]
	#[test_case(Position(-1, -1), 5, 5 => None)]
	fn test_check_bounds(
		position: Position<isize>,
		rows: usize,
		columns: usize,
	) -> Option<Position<usize>>
	{
		let max = Position(rows, columns);
		let min = Position(0, 0);
		position.check_bounds::<usize>(&min, &max)
	}
}
