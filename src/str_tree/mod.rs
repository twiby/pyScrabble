mod read_file;
pub use read_file::cnt_lines;
pub use read_file::read_lines;

mod tree_building;
pub use tree_building::StrTree;

pub use crate::board::WordError;
pub use crate::board::SIDE;
pub use crate::constraints::{ConstraintLetters, ConstraintNbLetters, ConstraintWords};

#[derive(Copy, Clone, Debug, Default)]
pub struct StaticWord {
    w: [char; SIDE],
    l: usize,
}
impl StaticWord {
    pub fn str(&self) -> String {
        self.w.iter().take(self.l).collect()
    }
    fn push(&mut self, c: char) {
        self.w[self.l] = c;
        self.l += 1;
    }
    pub fn into_word(&mut self) -> &mut [char] {
        &mut self.w[0..self.l]
    }
    pub fn len(&self) -> usize {
        self.l
    }
    pub fn pop(&mut self) -> Option<char> {
        if self.l > 0 {
            self.l -= 1;
            Some(self.w[self.l])
        } else {
            None
        }
    }
}

pub fn initiate_word_buf(n: usize) -> Vec<StaticWord> {
    Vec::with_capacity(n)
}

pub trait Dictionnary {
    fn build_dict_from_file(filename: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    fn get_anagrams<CNbL, CL, CW>(
        &self,
        letter_set: &str,
        words_buf: &mut Vec<StaticWord>,
        nb_letter: CNbL,
        letter_constraints: CL,
        word_constraint: CW,
    ) -> Result<(), WordError>
    where
        CNbL: ConstraintNbLetters,
        CL: ConstraintLetters,
        CW: ConstraintWords;

    fn add_word(&mut self, new_word: &str);
    fn is_word(&self, word: &str) -> bool;
}

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
    return StrTree::build_dict_from_file(filename);
}
