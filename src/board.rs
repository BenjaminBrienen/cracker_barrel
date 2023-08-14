use std::collections::HashSet;

use crate::{play::Play, position::Position, direction::Direction};

pub type Board = Vec<Vec<Option<bool>>>;

/// Hole
pub const O: Option<bool> = Some(false);

/// Peg
pub const X: Option<bool> = Some(true);

/// Not part of the board
pub const I: Option<bool> = None;

pub fn solve(
	board: &mut Board,
	moves: &mut Vec<Play>,
	visited: &mut HashSet<Board>,
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

	let rows = board.len();
	let columns = board[0].len();

	let max = Position(rows, columns);
	let min = Position(0, 0);
	// Iterate over each peg cell in the board
	for (index, _) in board
		.clone()
		.iter()
		.flatten()
		.enumerate()
		.filter(|(_, &value)| X == value)
	{
		// Iterate over possible moves
		for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::To, Direction::Fro]
		{
			let delta = direction.to_delta();
			#[allow(clippy::cast_possible_wrap)]
			let start_position = Position((index / columns) as isize, (index % columns) as isize);
			let mid_position = start_position + delta;
			let end_position = start_position + delta * Position(2, 2);

			// Check if the move is valid
			if let Some(mid_position) = mid_position.check_bounds::<usize>(&min, &max)
            && let Some(end_position) = end_position.check_bounds::<usize>(&min, &max)
            && let Some(start_position) = start_position.check_bounds::<usize>(&min, &max)
            && board[mid_position.0][mid_position.1] == X
            && board[end_position.0][end_position.1] == O
            {
                // Play the move
                play_move(board, start_position, mid_position, end_position);
                moves.push(Play { start_position, end_position, end_board: board.clone() });
                // Recursively solve the remaining board
                if solve(board, moves, visited)
                {
                    visited.insert(board.clone());
                    return true
                }
                unplay_move(board, start_position, mid_position, end_position);
                // Undo the move
                moves.pop();
            }
		}
	}
	visited.insert(board.clone());
	false
}

fn play_move(
	board: &mut Board,
	start_position: Position<usize>,
	mid_position: Position<usize>,
	end_position: Position<usize>,
)
{
	board[start_position.0][start_position.1] = O;
	board[mid_position.0][mid_position.1] = O;
	board[end_position.0][end_position.1] = X;
}

fn unplay_move(
	board: &mut [Vec<Option<bool>>],
	start_position: Position<usize>,
	mid_position: Position<usize>,
	end_position: Position<usize>,
)
{
    board[start_position.0][start_position.1] = X;
    board[mid_position.0][mid_position.1] = X;
    board[end_position.0][end_position.1] = O;
}
