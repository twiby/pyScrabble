use crate::constraints::{ConstraintLetters, ConstraintNbLetters, ConstraintWords};
use crate::constraints::{PotentialWordConditions, PotentialWordConditionsBuilder};

type CNbL = Option<Vec<u8>>;
type CL = Option<Vec<(u8, char)>>;
type CW = Option<Vec<(u8, WordToFill)>>;

#[derive(Clone, PartialEq)]
pub struct WordToFill {
    pub(crate) beginning: String,
    pub(crate) end: String,
}
#[derive(Debug)]
pub struct NoWordToFillError;
impl WordToFill {
    pub(crate) fn new(begin: String, end: String) -> Result<Self, NoWordToFillError> {
        if begin == "" && end == "" {
            return Err(NoWordToFillError);
        }
        return Ok(Self {
            beginning: begin.clone(),
            end: end.clone(),
        });
    }

    pub(crate) fn complete(&self, c: char) -> String {
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

impl ConstraintWords for CW {}
impl ConstraintNbLetters for CNbL {}
impl ConstraintLetters for CL {}

pub struct PotentialWord {
    nb_letters_constraint: Vec<u8>,
    letter_constraints: Vec<(u8, char)>,
    word_constraints: Vec<(u8, WordToFill)>,
}

impl PotentialWordConditionsBuilder for PotentialWord {
    fn new() -> Self {
        return PotentialWord {
            nb_letters_constraint: Vec::new(),
            letter_constraints: Vec::new(),
            word_constraints: Vec::new(),
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
