use {
	crate::{
		direction::{
			Direction,
			DIRECTIONS,
		},
		play::Play,
		position::Position,
	},
	std::collections::HashSet,
};

pub type Board<const ROWS: usize, const COLUMNS: usize> = [[Option<bool>; COLUMNS]; ROWS];

/// Hole
pub const O: Option<bool> = Some(false);

/// Peg
pub const X: Option<bool> = Some(true);

/// Not part of the board
pub const I: Option<bool> = None;

pub fn solve<const ROWS: usize, const COLUMNS: usize>(
	board: &mut Board<ROWS, COLUMNS>,
	plays: &mut Vec<Play<ROWS, COLUMNS>>,
	visited: &mut HashSet<Board<ROWS, COLUMNS>>,
) -> bool
{
	if board
		.iter()
		.flatten()
		.filter(|cell| cell.is_some_and(|value| value))
		.count()
		== 1
	{
		return true
	}

	// Check if the current board configuration has been visited before
	if visited.contains(board)
	{
		return false
	}

	// Iterate over each peg cell in the board
	let valid_moves: Vec<_> = board
		.iter()
		.flatten()
		.enumerate()
		.filter(|(_, value)| X == **value)
		.flat_map(|(index, _)| DIRECTIONS.map(|direction| (index, direction)))
		// Iterate over possible plays
		.map(|(index, direction)| cell_and_direction_to_play(index / COLUMNS, index % COLUMNS, direction))
		// Check if the play is valid
		.filter_map(filter_valid_play::<ROWS, COLUMNS>)
		.filter(|(_, middle, end)| board[middle.0][middle.1] == X && board[end.0][end.1] == O)
		.collect();
	for (start, middle, end) in valid_moves
	{
		// Play the move
		play_move(board, start, middle, end);
		plays.push(Play { start, end, state: *board });
		// Recursively solve the remaining board
		if solve(board, plays, visited)
		{
			visited.insert(*board);
			return true
		}
		unplay_move(board, start, middle, end);
		// Undo the play
		plays.pop();
	}
	visited.insert(*board);
	false
}

fn filter_valid_play<const ROWS: usize, const COLUMNS: usize>((start, middle, end): (Position<isize>, Position<isize>, Position<isize>))
	-> Option<(Position<usize>, Position<usize>, Position<usize>)>
{
	let max = Position(ROWS, COLUMNS);
	let min = Position(0, 0);
	if let Some(middle) = middle.check_bounds::<usize>(&min, &max)
	&& let Some(end) = end.check_bounds::<usize>(&min, &max)
	&& let Some(start) = start.check_bounds::<usize>(&min, &max)
	{
		Some((start, middle, end))
	}
	else {
		None
	}
}

fn cell_and_direction_to_play(
	row: usize,
	column: usize,
	direction: Direction,
) -> (Position<isize>, Position<isize>, Position<isize>)
{
	let delta = direction.to_delta();
	#[allow(clippy::cast_possible_wrap)]
	let start = Position(row as isize, column as isize);
	let middle = start + delta;
	let end = start + delta * 2;
	(start, middle, end)
}

fn play_move<const ROWS: usize, const COLUMNS: usize>(
	board: &mut Board<ROWS, COLUMNS>,
	start: Position<usize>,
	middle: Position<usize>,
	end: Position<usize>,
)
{
	board[start.0][start.1] = O;
	board[middle.0][middle.1] = O;
	board[end.0][end.1] = X;
}

fn unplay_move<const ROWS: usize, const COLUMNS: usize>(
	board: &mut Board<ROWS, COLUMNS>,
	start: Position<usize>,
	middle: Position<usize>,
	end: Position<usize>,
)
{
	board[start.0][start.1] = X;
	board[middle.0][middle.1] = X;
	board[end.0][end.1] = O;
}

#[cfg(test)]
mod tests
{
	use {
		crate::*,
		test_case::test_case,
	};

	#[test_case(&mut [[None]] => false)]
	#[test_case(&mut [[None, None]] => false)]
	#[test_case(&mut [[None, None, None]] => false)]
	#[test_case(&mut [[None, None, Some(false)]] => false)]
	#[test_case(&mut [[None, None, Some(true)]] => true)]
	#[test_case(&mut [[None, Some(false), Some(true)]] => true)]
	#[test_case(&mut [[Some(false), Some(false), Some(true)]] => true)]
	#[test_case(&mut [[None], [None], [None]] => false ; "vertical not solvable")]
	#[test_case(&mut [[X], [X], [X]] => false)]
	#[test_case(&mut [[O, X, X]] => true ; "horizontal solvable left")]
	#[test_case(&mut [[X, X, O]] => true ; "horizontal solvable right")]
	#[test_case(&mut [[X], [X], [O]] => true)]
	#[test_case(&mut [[O], [X], [X]] => true)]
	#[test_case(&mut [
		[I, I, O],
		[I, O, X],
		[O, O, X],
	] => true)]
	#[test_case(&mut [
		[I, I, O],
		[I, O, X],
		[X, X, O],
	] => true)]
	#[test_case(&mut [
		[I, I, X],
		[I, X, X],
		[O, X, O],
	] => true)]
	#[test_case(&mut [
		[I, I, I, O],
		[I, I, O, O],
		[I, O, O, O],
		[X, X, O, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, O],
        [I, I, O, O],
        [I, O, O, O],
        [X, O, X, X],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, O],
        [I, I, O, X],
        [I, O, O, X],
        [X, O, X, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, O],
        [I, I, O, O],
        [I, O, X, X],
        [X, X, X, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, X],
        [I, I, O, X],
        [I, O, X, O],
        [X, X, X, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, X],
        [I, I, X, X],
        [I, X, X, O],
        [O, X, X, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, X],
        [I, I, X, O],
        [I, X, X, X],
        [O, X, X, X],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, O],
        [I, I, O, X],
        [I, X, X, O],
        [X, O, X, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, X],
        [I, I, X, X],
        [I, O, X, O],
        [X, O, X, O],
        ] => true)]
	#[test_case(&mut [
        [I, I, I, I, O],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, X, X, X, O],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [O, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, O, X],
        [I, X, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, O, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, O, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, O, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, O],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, O, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, O],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, O, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, X, X, O, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, O, X, X],
        [I, X, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, O],
        [I, X, X, X, X],
        [X, X, X, X, X],
    ] => true)]
	#[test_case(&mut [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, X, X],
        [I, X, X, X, X],
        [X, X, O, X, X],
    ] => true)]
	fn test_solve<const ROWS: usize, const COLUMNS: usize>(board: &mut Board<ROWS, COLUMNS>) -> bool
	{
		let mut moves = Vec::new();
		let mut visited: HashSet<Board<ROWS, COLUMNS>> = HashSet::new();
		solve(board, &mut moves, &mut visited)
	}
}
