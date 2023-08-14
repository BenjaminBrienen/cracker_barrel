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
	let mut board = [
        [I, I, I, I, X],
        [I, I, I, X, X],
        [I, I, X, O, X],
        [I, X, X, X, X],
        [X, X, X, X, X]
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
