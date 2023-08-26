#![allow(non_snake_case)]

mod str_tree;

mod board;

mod constraints;

mod solver;
use solver::{WithTimer};

fn main() {
	let tree = str_tree::build_dict_from_file("../pyScrabble/scrabbleWords.txt").unwrap();

	let mut str_board = "".to_string();
	str_board.push_str("6__2___6___2__6");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("2__5___2___5__2");
	str_board.push_str("____5_____5____");
	str_board.push_str("_3___3___3___3_");
	str_board.push_str("__2___2_2___2__");
	str_board.push_str("6__2___a___2__6");
	str_board.push_str("__2___2r2___2__");
	str_board.push_str("_3___3_be3___3_");
	str_board.push_str("____5__R__5____");
	str_board.push_str("2__5___e___5__2");
	str_board.push_str("__5___2_2___5__");
	str_board.push_str("_5___3___3___5_");
	str_board.push_str("6__2___6___2__6");

	let board = board::deserialize(&str_board).expect("Error when deserializing board message");

	let mut duration = std::time::Instant::now().elapsed();
	for n in 0..50 {
		use std::time::Instant;
		let now = Instant::now();
		println!("{:?}", solver::find_best_word::<WithTimer, _, _>("syste00", &board, &tree, None));
		duration += now.elapsed();
		println!("Elapsed: {:.2?}\n", duration/(n+1));
	}
}
