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

use {
	once_cell::sync::Lazy,
	std::time::Instant,
};

pub static CLOCK: Lazy<Instant> = Lazy::new(Instant::now);

pub fn solve<const ROWS: usize, const COLUMNS: usize>(
	board: &mut Board<ROWS, COLUMNS>,
	plays: &mut Vec<Play<ROWS, COLUMNS>>,
	visited: &mut HashSet<Board<ROWS, COLUMNS>>,
	best_min: &mut usize,
) -> bool
{
	let count = board
		.iter()
		.flatten()
		.filter(|cell| cell.is_some_and(|value| value))
		.count();
	if count < *best_min
	{
		println!("Best min: {best_min}");
		*best_min = count;
	}
	if visited.len() % 1_000_000 == 0
	{
		println!("Elapsed seconds: {}", CLOCK.elapsed().as_secs_f32());
		println!("# of visited boards: {}", visited.len());
	}
	if count == 1
	{
		return true
	}

	if visited.contains(board)
	{
		return false
	}
	visited.insert(*board);

	let is_peg = |&(_, value): &(usize, &Option<bool>)| -> bool { X == *value };
	let with_directions = |(index, _)| DIRECTIONS.map(|direction| (index, direction));
	let valid_moves: Vec<_> = board
		.iter()
		.flatten()
		.enumerate()
		.filter(is_peg)
		.flat_map(with_directions)
		.map(cell_and_direction_to_play::<COLUMNS>)
		.filter_map(|play| filter_valid_play::<ROWS, COLUMNS>(board, play))
		.collect();

	for (start, middle, end) in valid_moves
	{
		play_move(board, start, middle, end);
		plays.push(Play { start, end, state: *board });

		if solve(board, plays, visited, best_min)
		{
			visited.insert(*board);
			return true
		}

		unplay_move(board, start, middle, end);
		plays.pop();
	}
	false
}

fn filter_valid_play<const ROWS: usize, const COLUMNS: usize>(
	board: &Board<ROWS, COLUMNS>,
	(start, middle, end): (Position<isize>, Position<isize>, Position<isize>),
) -> Option<(Position<usize>, Position<usize>, Position<usize>)>
{
	let max = Position(ROWS, COLUMNS);
	let min = Position(0, 0);
	// Play is on the board
	if let Some(middle) = middle.check_bounds::<usize>(&min, &max)
	&& let Some(end) = end.check_bounds::<usize>(&min, &max)
	&& let Some(start) = start.check_bounds::<usize>(&min, &max)
	// Jump over peg and land in hole
	&& board[middle.0][middle.1] == X
	&& board[end.0][end.1] == O
	{
		Some((start, middle, end))
	}
	else {
		None
	}
}

fn cell_and_direction_to_play<const COLUMNS: usize>((index, direction): (usize, Direction)) -> (Position<isize>, Position<isize>, Position<isize>)
{
	let delta = direction.to_delta();
	#[allow(clippy::cast_possible_wrap)]
	let start = Position((index / COLUMNS) as isize, (index % COLUMNS) as isize);
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
		let mut best_min = 0;
		solve(board, &mut moves, &mut visited, &mut best_min)
	}
}
