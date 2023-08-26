use crate::constraints::{ConstraintNbLetters, ConstraintLetters, ConstraintWords};
use crate::constraints::{PotentialWordConditions, PotentialWordConditionsBuilder};

type CNbL = Option<Vec<u8>>;
type CL = Option<Vec<(u8, char)>>;
type CW = Option<Vec<(u8, WordToFill)>>;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct WordToFill {
	beginning: String,
	end: String
}
#[derive(Debug)]
pub struct NoWordToFillError;
impl WordToFill {
	pub fn new(begin: String, end: String) -> Result<Self, NoWordToFillError> {
		if begin == "" && end == "" {
			return Err(NoWordToFillError);
		}
		return Ok(Self{beginning: begin.clone(), end: end.clone()});
	}

	pub fn complete(&self, c: char) -> String {
		let mut ret = self.beginning.clone();
		ret.push(c);
		ret.push_str(&self.end);
		return ret;
	}
}
impl std::fmt::Debug for WordToFill {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut string = self.beginning.clone();
		string.push('_');
		string.push_str(&self.end);
		f.write_str(&string)
	}
}

impl ConstraintWords for CW {
	fn sort_and_fuse(&mut self) {
		match self {
			None => (),
			Some(ref mut vec) => {
				vec.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
				vec.reverse();
				vec.dedup_by(|a, b| a.0.eq(&b.0));
			}
		}
	}

	fn decrease(&mut self, c: char) -> Option<String> {
		match self {
			None => None,
			Some(ref mut vec) => {
				if vec.is_empty() {
					return None;
				}
				let (i,w) = (*vec.last().unwrap()).clone();
				let ret = match i {
					0 => {vec.pop(); Some(w.complete(c))},
					_ => None
				};

				for (idx, _) in vec.into_iter() {
					*idx -= 1;
				}
				return ret;
			}
		}
	}
}

impl ConstraintNbLetters for CNbL {
	fn sort_and_fuse(&mut self) {
		match self {
			None => (),
			Some(ref mut vec) => {
				vec.sort_unstable();
				vec.reverse();
				vec.dedup();
				if vec.last() == Some(&0) {
					vec.pop();
				}
			}
		};
	}

	fn decrease(&mut self) -> bool {
		match self {
			None => true,
			Some(ref mut vec) => {
				let ret = match vec.last() {
					Some(&0) => {vec.pop(); true}
					_ => false
				};
				for el in vec.into_iter() {
					*el -= 1;
				}
				ret
			}
		}
	}
}

impl ConstraintLetters for CL {
	fn sort_and_fuse(&mut self) {
		match self {
			None => (),
			Some(ref mut vec) => {
				vec.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
				vec.reverse();
				vec.dedup_by(|a, b| a.0.eq(&b.0));
			}
		}
	}

	fn decrease(&mut self) -> Option<char> {
		match self {
			None => None,
			Some(ref mut vec) => {
				if vec.is_empty() {
					return None;
				}
				let (i,c) = *vec.last().unwrap();
				let ret = match i {
					0 => {vec.pop(); Some(c)},
					_ => None
				};

				for (idx, _) in vec.into_iter() {
					*idx -= 1;
				}
				return ret;
			}
		}
	}
}

pub struct PotentialWord {
	nb_letters_constraint: Vec<u8>,
	letter_constraints: Vec<(u8, char)>,
	word_constraints: Vec<(u8, WordToFill)>
}

impl PotentialWordConditionsBuilder for PotentialWord {
	fn new() -> Self {
		return PotentialWord{
			nb_letters_constraint: Vec::new(), 
			letter_constraints: Vec::new(),
			word_constraints: Vec::new()
		};
	}
	fn reset(&mut self) {
		self.nb_letters_constraint.clear();
		self.letter_constraints.clear();
		self.word_constraints.clear();
	}
	fn add_nb_letters(&mut self, n: u8) {
		self.nb_letters_constraint.push(n);
	}
	fn add_letter(&mut self, c: char, pos: u8) {
		self.letter_constraints.push((pos, c));
	}
	fn add_word(&mut self, w: WordToFill, pos: u8) {
		self.word_constraints.push((pos, w));
	}
}

impl PotentialWordConditions<CNbL, CL, CW> for PotentialWord {
	fn get_constraint_nb_letters(&self) -> Option<Vec<u8>> {
		return Some(self.nb_letters_constraint.clone());
	}
	fn get_constraint_letters(&self) -> Option<Vec<(u8, char)>> {
		return Some(self.letter_constraints.clone());
	}
	fn get_constraint_words(&self) -> Option<Vec<(u8, WordToFill)>> {
		return Some(self.word_constraints.clone());
	}
}
