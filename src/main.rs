#![feature(let_chains)]
#![warn(clippy::pedantic)]

use std::{
	collections::HashSet,
	fmt::Debug,
	ops::{
		Add,
		Mul,
		Sub,
	},
};

type Board = Vec<Vec<Option<bool>>>;

#[derive(Debug)]
struct Move 
{
    start_position: Position<usize>,
    end_position: Position<usize>,
    end_board: Board,
}

fn main()
{
	// Define the initial board configuration
	let mut board = vec![
		vec![None,       None,       None,       None,        Some(true)],
		vec![None,       None,       None,       Some(true),  Some(true)],
		vec![None,       None,       Some(true), Some(false), Some(true)],
		vec![None,       Some(true), Some(true), Some(true),  Some(true)],
		vec![Some(true), Some(true), Some(true), Some(true),  Some(true)],
	];

    let mut moves = Vec::new();

	// Create a HashMap to store visited board configurations
	let mut visited: HashSet<Board> = HashSet::new();

	// Solve the game and print the result
	let solved = solve(&mut board, &mut moves, &mut visited);
	if solved
	{
		println!("Solution found:");
		print_board(&board);
        print_moves(&moves);
	}
	else
	{
		println!("No solution found.");
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction
{
	Up,
	Down,
	Left,
	Right,
    To,
    Fro
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position<T>(T, T);

impl<T: Add<Output = T> + Copy> Add for Position<T>
{
	type Output = Position<T>;

	fn add(
		self,
		rhs: Self,
	) -> Self::Output
	{
		Position(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl<T: Sub<Output = T> + Copy> Sub for Position<T>
{
	type Output = Position<T>;

	fn sub(
		self,
		rhs: Self,
	) -> Self::Output
	{
		Position(self.0 - rhs.0, self.1 - rhs.1)
	}
}

impl<T: Mul<Output = T> + Copy> Mul for Position<T>
{
	type Output = Position<T>;

	fn mul(
		self,
		rhs: Self,
	) -> Self::Output
	{
		Position(self.0 * rhs.0, self.1 * rhs.1)
	}
}

impl<T: Ord + Copy> Position<T>
{
	pub fn check_bounds<UT: TryFrom<T> + Ord>(
		self,
		min: &Position<UT>,
		max: &Position<UT>,
	) -> Option<Position<UT>>
	where
		<UT as std::convert::TryFrom<T>>::Error: std::fmt::Debug,
	{
		let row = self.0.try_into();
		let column = self.1.try_into();
		if let Ok(row) = row
        && let Ok(column) = column
        && row >= min.0 && column >= min.1
        && row < max.0 && column < max.1
		{
			Some(Position(row, column))
		}
		else
		{
			None
		}
	}
}

const fn direction_to_delta(direction: Direction) -> Position<isize>
{
	match direction
	{
		Direction::Up => Position(-1, 0),
		Direction::Down => Position(1, 0),
		Direction::Left => Position(0, -1),
		Direction::Right => Position(0, 1),
        Direction::To => Position(-1, 1),
        Direction::Fro => Position(1, -1),
	}
}

fn solve(
	board: &mut Board,
    moves: &mut Vec<Move>,
	visited: &mut HashSet<Board>,
) -> bool
{
    if board.iter().flatten().filter(|cell| cell.is_some_and(|value| value)).count() == 1
    {
        return true;
    }
	// Check if the current board configuration has been visited before
	if visited.contains(board)
	{
		return false;
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
			if let Some(true) = board[row][column]
			{
				// Iterate over possible moves: up, down, left, right
				for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::To, Direction::Fro]
				{
					let delta = direction_to_delta(direction);
                    #[allow(clippy::cast_possible_wrap)]
					let start_position = Position(row as isize, column as isize);
					let mid_position = start_position + delta;
					let end_position = start_position + delta * Position(2, 2);

					// Check if the move is valid and update the board
					if let Some(mid_position) = mid_position.check_bounds::<usize>(&min, &max)
                    && let Some(end_position) = end_position.check_bounds::<usize>(&min, &max)
                    && let Some(start_position) = start_position.check_bounds::<usize>(&min, &max)
                    && board[mid_position.0][mid_position.1] == Some(true)
                    && board[end_position.0][end_position.1] == Some(false)
					{
						jump_take(board, start_position, mid_position, end_position);
                        let current_move = Move { start_position, end_position, end_board: board.clone() };
                        moves.push(current_move);
						// Recursively solve the remaining board
						if solve(board, moves, visited)
						{
							visited.insert(board.clone());
							return true
						}
                        // Undo the move
                        moves.pop();
                        board[start_position.0][start_position.1] = Some(true);
                        board[mid_position.0][mid_position.1] = Some(true);
                        board[end_position.0][end_position.1] = Some(false);
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
	board[start_position.0][start_position.1] = Some(false);
	board[mid_position.0][mid_position.1] = Some(false);
	board[end_position.0][end_position.1] = Some(true);
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

fn print_moves(moves: &[Move]) {
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
    use crate::*;
    use test_case::test_case;

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
    fn test_add_position(start_position: Position<isize>, direction: Direction) -> Position<isize>
    {
        start_position + direction_to_delta(direction)
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
    fn test_mul_position(position: Position<isize>, direction: Direction, magnitude: isize) -> Position<isize>
    {
        let delta = direction_to_delta(direction);
        position + delta * Position(magnitude, magnitude)
    }

    #[test_case(Position(3isize, 3isize), 3, 3 => None)]
    #[test_case(Position(2isize, 2isize), 3, 3 => Some(Position(2usize, 2usize)))]
    #[test_case(Position(1isize, 1isize), 3, 3 => Some(Position(1usize, 1usize)))]
    #[test_case(Position(0isize, 0isize), 3, 3 => Some(Position(0usize, 0usize)))]
    #[test_case(Position(-1isize, -1isize), 3, 3 => None)]
    #[test_case(Position(4isize, 4isize), 4, 4 => None)]
    #[test_case(Position(3isize, 3isize), 4, 4 => Some(Position(3usize, 3usize)))]
    #[test_case(Position(2isize, 2isize), 4, 4 => Some(Position(2usize, 2usize)))]
    #[test_case(Position(1isize, 1isize), 4, 4 => Some(Position(1usize, 1usize)))]
    #[test_case(Position(0isize, 0isize), 4, 4 => Some(Position(0usize, 0usize)))]
    #[test_case(Position(-1isize, -1isize), 4, 4 => None)]
    #[test_case(Position(5isize, 5isize), 5, 5 => None)]
    #[test_case(Position(4isize, 4isize), 5, 5 => Some(Position(4usize, 4usize)))]
    #[test_case(Position(3isize, 3isize), 5, 5 => Some(Position(3usize, 3usize)))]
    #[test_case(Position(2isize, 2isize), 5, 5 => Some(Position(2usize, 2usize)))]
    #[test_case(Position(1isize, 1isize), 5, 5 => Some(Position(1usize, 1usize)))]
    #[test_case(Position(0isize, 0isize), 5, 5 => Some(Position(0usize, 0usize)))]
    #[test_case(Position(-1isize, -1isize), 5, 5 => None)]
    fn test_check_bounds<T: Ord + Copy>(position: Position<T>, rows: usize, columns: usize) -> Option<Position<usize>>
    where usize: std::convert::TryFrom<T>,
        <usize as std::convert::TryFrom<T>>::Error: Debug
    {
        let max = Position(rows, columns);
        let min = Position(0, 0);
        position.check_bounds::<usize>(&min, &max)
    }

    #[test_case(&mut vec![vec![Some(false), Some(true), Some(true)]] => true)]
    #[test_case(&mut vec![vec![Some(true), Some(true), Some(false)]] => true)]
    #[test_case(&mut vec![vec![Some(true)], vec![Some(true)], vec![Some(false)]] => true)]
    #[test_case(&mut vec![vec![Some(false)], vec![Some(true)], vec![Some(true)]] => true)]
    #[test_case(&mut vec![vec![Some(true)], vec![Some(true)], vec![Some(true)]] => false)]
    #[test_case(&mut vec![
		vec![None, None, Some(false)],
		vec![None, Some(false), Some(true)],
		vec![Some(false), Some(false), Some(true)],
	] => true)]
    #[test_case(&mut vec![
		vec![None, None, Some(false)],
		vec![None, Some(false), Some(true)],
		vec![Some(true), Some(true), Some(false)],
	] => true)]
    #[test_case(&mut vec![
		vec![None, None, Some(true)],
		vec![None, Some(true), Some(true)],
		vec![Some(false), Some(true), Some(false)],
	] => true)]
    #[test_case(&mut vec![
		vec![None, None, None, Some(false)],
		vec![None, None, Some(false), Some(false)],
		vec![None, Some(false), Some(false), Some(false)],
		vec![Some(true), Some(true), Some(false), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(false)],
        vec![None, None, Some(false), Some(false)],
        vec![None, Some(false), Some(false), Some(false)],
        vec![Some(true), Some(false), Some(true), Some(true)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(false)],
        vec![None, None, Some(false), Some(true)],
        vec![None, Some(false), Some(false), Some(true)],
        vec![Some(true), Some(false), Some(true), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(false)],
        vec![None, None, Some(false), Some(false)],
        vec![None, Some(false), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(true)],
        vec![None, None, Some(false), Some(true)],
        vec![None, Some(false), Some(true), Some(false)],
        vec![Some(true), Some(true), Some(true), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(true)],
        vec![None, None, Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(false)],
        vec![Some(false), Some(true), Some(true), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(true)],
        vec![None, None, Some(true), Some(false)],
        vec![None, Some(true), Some(true), Some(true)],
        vec![Some(false), Some(true), Some(true), Some(true)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(false)],
        vec![None, None, Some(false), Some(true)],
        vec![None, Some(true), Some(true), Some(false)],
        vec![Some(true), Some(false), Some(true), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, Some(true)],
        vec![None, None, Some(true), Some(true)],
        vec![None, Some(false), Some(true), Some(false)],
        vec![Some(true), Some(false), Some(true), Some(false)],
        ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(false)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(false)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(false), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(false), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(false), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(false), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(false), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(false)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(false), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(false)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(false), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(false), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(false), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(false)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(true), Some(true), Some(true)],
    ] => true)]
    #[test_case(&mut vec![
        vec![None, None, None, None, Some(true)],
        vec![None, None, None, Some(true), Some(true)],
        vec![None, None, Some(true), Some(true), Some(true)],
        vec![None, Some(true), Some(true), Some(true), Some(true)],
        vec![Some(true), Some(true), Some(false), Some(true), Some(true)],
    ] => true)]
    fn test_solve(board: &mut Board) -> bool
    {
        let mut moves = Vec::new();
        let mut visited: HashSet<Board> = HashSet::new();
        solve(board, &mut moves, &mut visited)
    }
}
