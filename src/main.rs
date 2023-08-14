#![feature(let_chains)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

mod position;
use position::Position;

mod direction;
use direction::Direction;

mod play;
use play::Play;

type Board = Vec<Vec<Option<bool>>>;

const O: Option<bool> = Some(false);
const X: Option<bool> = Some(true);
const I: Option<bool> = None;

fn main()
{
	// Define the initial board configuration
	#[rustfmt::skip]
	let mut board = vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, O, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X]
    ];

	let mut moves = Vec::new();

	// Create a HashMap to store visited board configurations
	let mut visited = HashSet::new();

	// Solve the game and print the result
	let solved = solve(&mut board, &mut moves, &mut visited);
	if solved
	{
		println!("Solution found:");
		print_board(&board);
		print_moves(&moves);
        println!("# of visited boards: {}", visited.len());
	}
	else
	{
		println!("No solution found.");
	}
}

fn solve(
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

	// Iterate over each cell in the board
	for row in 0..rows
	{
		for column in 0..columns
		{
			// Check if the current cell contains a peg
			if let X = board[row][column]
			{
				// Iterate over possible moves: up, down, left, right
				for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::To, Direction::Fro]
				{
					let delta = direction.to_delta();
					#[allow(clippy::cast_possible_wrap)]
					let start_position = Position(row as isize, column as isize);
					let mid_position = start_position + delta;
					let end_position = start_position + delta * Position(2, 2);

					// Check if the move is valid and update the board
					if let Some(mid_position) = mid_position.check_bounds::<usize>(&min, &max)
                    && let Some(end_position) = end_position.check_bounds::<usize>(&min, &max)
                    && let Some(start_position) = start_position.check_bounds::<usize>(&min, &max)
                    && board[mid_position.0][mid_position.1] == X
                    && board[end_position.0][end_position.1] == O
					{
						jump_take(board, start_position, mid_position, end_position);
                        let current_move = Play { start_position, end_position, end_board: board.clone() };
                        moves.push(current_move);
						// Recursively solve the remaining board
						if solve(board, moves, visited)
						{
							visited.insert(board.clone());
							return true
						}
                        // Undo the move
                        moves.pop();
                        board[start_position.0][start_position.1] = X;
                        board[mid_position.0][mid_position.1] = X;
                        board[end_position.0][end_position.1] = O;
					}
				}
			}
		}
	}

	// Mark the current board as unsolvable and return
	visited.insert(board.clone());
	false
}

fn jump_take(
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

fn print_board(board: &Board)
{
	// Print the current board configuration
	for row in board
	{
		for cell in row
		{
			match cell
			{
				Some(true) => print!("O "),
				Some(false) => print!(". "),
				None => print!("  "),
			}
		}
		println!();
	}
	println!();
}

fn print_moves(moves: &[Play])
{
	for m in moves
	{
		println!("start: {:?}", m.start_position);
		println!("end: {:?}", m.end_position);
		println!("board:");
		print_board(&m.end_board);
	}
	println!();
}

#[cfg(test)]
mod tests
{
	use {
		crate::*,
		test_case::test_case,
	};

	#[test_case(Position(0, 0), Direction::Up => Position(-1, 0))]
	#[test_case(Position(0, 0), Direction::Down => Position(1, 0))]
	#[test_case(Position(0, 0), Direction::Left => Position(0, -1))]
	#[test_case(Position(0, 0), Direction::Right => Position(0, 1))]
	#[test_case(Position(0, 0), Direction::To => Position(-1, 1))]
	#[test_case(Position(0, 0), Direction::Fro => Position(1, -1))]
	#[test_case(Position(3, 3), Direction::Up => Position(2, 3))]
	#[test_case(Position(3, 3), Direction::Down => Position(4, 3))]
	#[test_case(Position(3, 3), Direction::Left => Position(3, 2))]
	#[test_case(Position(3, 3), Direction::Right => Position(3, 4))]
	#[test_case(Position(3, 3), Direction::To => Position(2, 4))]
	#[test_case(Position(3, 3), Direction::Fro => Position(4, 2))]
	fn test_add_position(
		start_position: Position<isize>,
		direction: Direction,
	) -> Position<isize>
	{
		start_position + direction.to_delta()
	}

	#[test_case(Position(0, 0), Direction::Up, 2 => Position(-2, 0))]
	#[test_case(Position(0, 0), Direction::Down, 2 => Position(2, 0))]
	#[test_case(Position(0, 0), Direction::Left, 2 => Position(0, -2))]
	#[test_case(Position(0, 0), Direction::Right, 2 => Position(0, 2))]
	#[test_case(Position(0, 0), Direction::To, 2 => Position(-2, 2))]
	#[test_case(Position(0, 0), Direction::Fro, 2 => Position(2, -2))]
	#[test_case(Position(3, 3), Direction::Up, 2 => Position(1, 3))]
	#[test_case(Position(3, 3), Direction::Down, 2 => Position(5, 3))]
	#[test_case(Position(3, 3), Direction::Left, 2 => Position(3, 1))]
	#[test_case(Position(3, 3), Direction::Right, 2 => Position(3, 5))]
	#[test_case(Position(3, 3), Direction::To, 2 => Position(1, 5))]
	#[test_case(Position(3, 3), Direction::Fro, 2 => Position(5, 1))]
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

	#[test_case(&mut vec![vec![O, X, X]] => true)]
	#[test_case(&mut vec![vec![X, X, O]] => true)]
	#[test_case(&mut vec![vec![X], vec![X], vec![O]] => true)]
	#[test_case(&mut vec![vec![O], vec![X], vec![X]] => true)]
	#[test_case(&mut vec![vec![X], vec![X], vec![X]] => false)]
	#[test_case(&mut vec![
		vec![I, I, O],
		vec![I, O, X],
		vec![O, O, X],
	] => true)]
	#[test_case(&mut vec![
		vec![I, I, O],
		vec![I, O, X],
		vec![X, X, O],
	] => true)]
	#[test_case(&mut vec![
		vec![I, I, X],
		vec![I, X, X],
		vec![O, X, O],
	] => true)]
	#[test_case(&mut vec![
		vec![I, I, I, O],
		vec![I, I, O, O],
		vec![I, O, O, O],
		vec![X, X, O, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, O],
        vec![I, I, O, O],
        vec![I, O, O, O],
        vec![X, O, X, X],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, O],
        vec![I, I, O, X],
        vec![I, O, O, X],
        vec![X, O, X, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, O],
        vec![I, I, O, O],
        vec![I, O, X, X],
        vec![X, X, X, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, X],
        vec![I, I, O, X],
        vec![I, O, X, O],
        vec![X, X, X, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, X],
        vec![I, I, X, X],
        vec![I, X, X, O],
        vec![O, X, X, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, X],
        vec![I, I, X, O],
        vec![I, X, X, X],
        vec![O, X, X, X],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, O],
        vec![I, I, O, X],
        vec![I, X, X, O],
        vec![X, O, X, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, X],
        vec![I, I, X, X],
        vec![I, O, X, O],
        vec![X, O, X, O],
        ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, O],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, O],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![O, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, O, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, O, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, O, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, O, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, O],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, O, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, O],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, O, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, X, X, O, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, O, X, X],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, O],
        vec![I, X, X, X, X],
        vec![X, X, X, X, X],
    ] => true)]
	#[test_case(&mut vec![
        vec![I, I, I, I, X],
        vec![I, I, I, X, X],
        vec![I, I, X, X, X],
        vec![I, X, X, X, X],
        vec![X, X, O, X, X],
    ] => true)]
	fn test_solve(board: &mut Board) -> bool
	{
		let mut moves = Vec::new();
		let mut visited: HashSet<Board> = HashSet::new();
		solve(board, &mut moves, &mut visited)
	}
}
