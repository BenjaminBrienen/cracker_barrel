#![feature(exact_size_is_empty)]
#![feature(let_chains)]
#![warn(clippy::pedantic)]

use {
	board::{
		solve,
		Board,
		I,
		O,
		X,
	},
	play::Play,
	position::Position,
	std::collections::HashSet,
};

mod board;
mod direction;
mod play;
mod position;

fn main()
{
	// Define the initial board configuration
	#[rustfmt::skip]
	let mut board = [
		[I, I, X, X, X, I, I],
		[I, X, X, X, X, X, I],
		[X, X, X, X, X, X, X],
		[X, X, X, O, X, X, X],
		[X, X, X, X, X, X, X],
		[I, X, X, X, X, X, I],
		[I, I, X, X, X, I, I],
    ];

	let mut moves = Vec::new();
	let mut visited = HashSet::new();
	let mut best_min = usize::MAX;
	if solve(&mut board, &mut moves, &mut visited, &mut best_min)
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

fn print_board<const ROWS: usize, const COLUMNS: usize>(board: &Board<ROWS, COLUMNS>)
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

fn print_moves<const ROWS: usize, const COLUMNS: usize>(moves: &[Play<ROWS, COLUMNS>])
{
	for m in moves
	{
		println!("start: {:?}", m.start);
		println!("end: {:?}", m.end);
		println!("board:");
		print_board(&m.state);
	}
	println!();
}
