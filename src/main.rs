#![feature(exact_size_is_empty)]
#![feature(let_chains)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

mod position;
use position::Position;

mod direction;

mod play;
use play::Play;

mod board;
use board::{
	solve,
	Board,
	I,
	O,
	X,
};

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
	let mut visited = HashSet::new();
	if solve(&mut board, &mut moves, &mut visited)
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

fn print_board(board: &Board)
{
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
		direction::Direction::{
			self,
			Down,
			Fro,
			Left,
			Right,
			To,
			Up,
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

	#[test_case(&mut vec![vec![None]] => false)]
	#[test_case(&mut vec![vec![None, None]] => false)]
	#[test_case(&mut vec![vec![None, None, None]] => false)]
	#[test_case(&mut vec![vec![None, None, Some(false)]] => false)]
	#[test_case(&mut vec![vec![None, None, Some(true)]] => true)]
	#[test_case(&mut vec![vec![None, Some(false), Some(true)]] => true)]
	#[test_case(&mut vec![vec![Some(false), Some(false), Some(true)]] => true)]
	#[test_case(&mut vec![vec![None], vec![None], vec![None]] => false)]
	#[test_case(&mut vec![vec![X], vec![X], vec![X]] => false)]
	#[test_case(&mut vec![vec![O, X, X]] => true)]
	#[test_case(&mut vec![vec![X, X, O]] => true)]
	#[test_case(&mut vec![vec![X], vec![X], vec![O]] => true)]
	#[test_case(&mut vec![vec![O], vec![X], vec![X]] => true)]
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
