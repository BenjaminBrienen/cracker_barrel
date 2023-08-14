use crate::{
	Board,
	Position,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Play
{
	pub start_position: Position<usize>,
	pub end_position:   Position<usize>,
	pub end_board:      Board,
}
