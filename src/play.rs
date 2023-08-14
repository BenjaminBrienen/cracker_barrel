use crate::{
	Board,
	Position,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Play<const ROWS: usize, const COLUMNS: usize>
{
	pub start: Position<usize>,
	pub end:   Position<usize>,
	pub state: Board<ROWS, COLUMNS>,
}
