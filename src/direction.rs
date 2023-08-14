use crate::position::Position;

pub const DIRECTIONS: [Direction; 6] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::To, Direction::Fro];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction
{
	Up,
	Down,
	Left,
	Right,
	To,
	Fro,
}

impl Direction
{
	pub const fn to_delta(self) -> Position<isize>
	{
		match self
		{
			Direction::Up => Position(-1, 0),
			Direction::Down => Position(1, 0),
			Direction::Left => Position(0, -1),
			Direction::Right => Position(0, 1),
			Direction::To => Position(-1, 1),
			Direction::Fro => Position(1, -1),
		}
	}
}
