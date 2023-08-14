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
