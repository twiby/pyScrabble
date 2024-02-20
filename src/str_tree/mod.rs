mod read_file;
pub use read_file::cnt_lines;
pub use read_file::read_lines;

mod tree_building;
pub use tree_building::StrTree;

pub use crate::board::WordError;
use crate::board::WordToFill;
pub use crate::board::SIDE;

#[derive(Copy, Clone, Debug, Default)]
pub struct StaticWord {
    w: [char; SIDE],
    l: usize,
}
impl StaticWord {
    pub(crate) fn str(&self) -> String {
        self.w.iter().take(self.l).collect()
    }
    fn push(&mut self, c: char) {
        self.w[self.l] = c;
        self.l += 1;
    }
    pub(crate) fn as_slice(&mut self) -> &[char] {
        &self.w[0..self.l]
    }
    pub(crate) fn len(&self) -> usize {
        self.l
    }
    pub(crate) fn pop(&mut self) -> Option<char> {
        if self.l > 0 {
            self.l -= 1;
            Some(self.w[self.l])
        } else {
            None
        }
    }
    pub(crate) fn swap_remove(&mut self, index: usize) -> Option<char> {
        self.w.swap(index, self.l - 1);
        self.pop()
    }
}
impl From<Vec<char>> for StaticWord {
    fn from(val: Vec<char>) -> Self {
        let mut ret = StaticWord::default();
        for c in val {
            ret.push(c);
        }
        ret
    }
}
impl std::ops::Index<usize> for StaticWord {
    type Output = char;
    fn index(&self, index: usize) -> &char {
        &self.w[index]
    }
}

pub fn initiate_word_buf(n: usize) -> Vec<StaticWord> {
    Vec::with_capacity(n)
}

pub trait Dictionnary {
    fn build_dict_from_file(filename: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    fn get_anagrams(
        &self,
        letter_set: &str,
        words_buf: &mut Vec<StaticWord>,
        nb_letter: Option<Vec<u8>>,
        letter_constraints: Option<Vec<(u8, char)>>,
        word_constraint: Option<Vec<(u8, WordToFill)>>,
    ) -> Result<(), WordError>;

    fn add_word(&mut self, new_word: &str);
    fn is_word(&self, word: &str) -> bool;
}

pub fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
    return StrTree::build_dict_from_file(filename);
}
