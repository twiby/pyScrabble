use crate::board::WordToFill;

use crate::str_tree::WordError;
use crate::str_tree::SIDE;
use crate::str_tree::{cnt_lines, read_lines};

use crate::str_tree::{Dictionnary, StaticWord};

#[derive(Clone, Debug)]
struct WordConstraint<'a> {
    node: &'a StrTree,
    end: String,
}
trait WordConstraintTrait {
    fn valid(&self, c: char) -> bool;
}
impl<'a> WordConstraint<'a> {
    fn new(tree: &'a StrTree, to_fill: WordToFill) -> Result<Self, WordError> {
        let Some(node) = tree.get_node(&to_fill.beginning) else {
            return Err(WordError::UnknownConstraint(format!(
                "Constraint node doesn't exist: {:?}",
                to_fill.beginning
            )));
        };
        Ok(Self {
            node,
            end: to_fill.end,
        })
    }
}
impl<'a> WordConstraintTrait for Option<WordConstraint<'a>> {
    fn valid(&self, c: char) -> bool {
        match self {
            None => true,
            Some(w) => w
                .node
                .get_child(c)
                .map(|n| n.is_word(&w.end))
                .unwrap_or(false),
        }
    }
}

#[derive(Debug, Default)]
struct LetterSet {
    data: StaticWord,
}
impl LetterSet {
    fn from_letters(letters: Vec<char>) -> Self {
        Self {
            data: letters.clone().into(),
        }
    }
    fn remove(&mut self, val: char) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] == val {
                self.data.swap_remove(i);
                return true;
            }
        }
        return false;
    }
    fn insert(&mut self, val: char) {
        self.data.push(val)
    }
}

#[derive(Debug, PartialEq)]
enum LevelState {
    UsingJoker,
    UsingLetter,
    ConstraintLetter(char),
}

#[derive(Debug, Default)]
pub(crate) struct TreeAnagrammer<'a> {
    // Iterator state
    states: Vec<LevelState>,
    parents: Vec<&'a StrTree>,
    cursor: Vec<std::slice::Iter<'a, StrTree>>,

    // letter info
    word: StaticWord,
    set: LetterSet,
    nb_available_jokers: usize,

    // Constraints type
    nb_letters: [bool; SIDE],
    letter_constraints: [Option<char>; SIDE],
    word_constraints: [Option<WordConstraint<'a>>; SIDE],
}

impl<'a> TreeAnagrammer<'a> {
    fn new(tree: &'a StrTree, set: Vec<char>) -> Self {
        let mut ret = Self {
            nb_available_jokers: set.iter().filter(|&&c| c == '0').count(),
            set: LetterSet::from_letters(set),
            nb_letters: [true; SIDE],
            ..Default::default()
        };
        ret.push_child(tree);
        ret
    }
    pub(crate) fn with_nb_letters(mut self, nb_letters_in: Vec<u8>) -> Self {
        self.nb_letters = [false; SIDE];
        for pos in nb_letters_in {
            self.nb_letters[pos as usize] = true;
        }
        self
    }
    pub(crate) fn with_letter_constraints(mut self, letter_constraints: Vec<(u8, char)>) -> Self {
        for (pos, c) in letter_constraints {
            self.letter_constraints[pos as usize] = Some(c);
        }

        if let Some(c) = self.letter_constraints[0] {
            self.states
                .first_mut()
                .map(|state| *state = LevelState::ConstraintLetter(c));
        }

        self
    }
    pub(crate) fn with_word_constraints(
        mut self,
        word_constraints: Vec<(u8, WordToFill)>,
    ) -> Result<Self, WordError> {
        for (pos, to_fill) in word_constraints {
            self.word_constraints[pos as usize] =
                Some(WordConstraint::new(self.parents[0], to_fill)?);
        }
        Ok(self)
    }

    fn push_child(&mut self, child: &'a StrTree) {
        if let Some(mut c) = child.data {
            c = match self.states.last() {
                Some(&LevelState::UsingLetter) => c,
                Some(&LevelState::UsingJoker) => c.to_ascii_uppercase(),
                Some(&LevelState::ConstraintLetter(_)) => '_',
                None => unreachable!(),
            };
            self.word.push(c);
        }

        let new_state = if let Some(c) = self.letter_constraints[self.word.len()] {
            LevelState::ConstraintLetter(c)
        } else if self.nb_available_jokers == 0 {
            LevelState::UsingLetter
        } else {
            self.nb_available_jokers -= 1;
            LevelState::UsingJoker
        };

        self.cursor.push(child.children.iter());
        self.parents.push(child);
        self.states.push(new_state);
    }
    fn pop_child(&mut self) {
        self.cursor.pop();
        self.parents.pop();
        self.states.pop();
        if let Some(c) = self.word.pop() {
            if c.is_ascii_lowercase() {
                self.set.insert(c);
            }
        }
    }

    fn valid_return(&self) -> bool {
        if !self.parents.last().unwrap().is_word {
            return false;
        }

        if let Some(LevelState::ConstraintLetter(_)) = self.states.last() {
            return false;
        }

        let nb_letters = self
            .word
            .as_slice()
            .iter()
            .filter(|c| c.is_alphabetic())
            .count();
        if !self.nb_letters[nb_letters] || nb_letters == 0 {
            return false;
        }

        return true;
    }
}
impl<'a> Iterator for TreeAnagrammer<'a> {
    type Item = StaticWord;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.cursor.is_empty() {
            debug_assert_eq!(self.parents.len(), self.cursor.len());
            debug_assert_eq!(self.states.len(), self.cursor.len());
            let last = self.parents.len() - 1;

            match self.states[last] {
                LevelState::UsingJoker => {
                    if let Some(child) = self.cursor[last]
                        .find(|c| self.word_constraints[self.word.len()].valid(c.data.unwrap()))
                    {
                        self.push_child(child);
                        if self.valid_return() {
                            return Some(self.word);
                        }
                    } else {
                        self.cursor[last] = self.parents[last].children.iter();
                        self.states[last] = LevelState::UsingLetter;
                        self.nb_available_jokers += 1;
                    }
                }

                LevelState::UsingLetter => {
                    if let Some(child) = self.cursor[last].find(|c| {
                        self.word_constraints[self.word.len()].valid(c.data.unwrap())
                            && self.set.remove(c.data.unwrap())
                    }) {
                        self.push_child(child);
                        if self.valid_return() {
                            return Some(self.word);
                        }
                    } else {
                        self.pop_child();
                    }
                }

                LevelState::ConstraintLetter(c) => {
                    if let Some(child) = self.cursor[last].find(|child| child.data == Some(c)) {
                        self.push_child(child);
                        if self.valid_return() {
                            return Some(self.word);
                        }
                    } else {
                        self.pop_child();
                    }
                }
            }
        }

        None
    }
}

pub struct StrTree {
    data: Option<char>,
    is_word: bool,
    children: Vec<StrTree>,
}

impl StrTree {
    #[cfg(test)]
    pub(crate) fn anagrams(&self, letters: &str) -> TreeAnagrammer {
        let letter_set_vec: Vec<char> = letters.chars().collect();
        TreeAnagrammer::new(self, letter_set_vec)
    }
}

impl std::fmt::Debug for StrTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = match self.data {
            None => "Head of tree".to_string(),
            Some(c) => c.to_string(),
        };
        string.push_str(" - ");
        if self.is_word {
            string.push_str("word - ");
        } else {
            string.push_str("not a word - ");
        }
        string.push_str(&self.children.len().to_string());
        string.push_str(" children");
        f.write_str(&string)
    }
}

impl Dictionnary for StrTree {
    fn build_dict_from_file(filename: &str) -> std::io::Result<StrTree> {
        let mut ret = StrTree::init();
        ret.fill_with_file(filename)?;
        Ok(ret)
    }

    fn get_anagrams(
        &self,
        letter_set: &str,
        words_buf: &mut Vec<StaticWord>,
        nb_letters: Option<Vec<u8>>,
        letter_constraints: Option<Vec<(u8, char)>>,
        word_constraints: Option<Vec<(u8, WordToFill)>>,
    ) -> Result<(), WordError> {
        let letter_set_vec: Vec<char> = letter_set.chars().collect();
        let mut anagrammer = TreeAnagrammer::new(self, letter_set_vec);
        if let Some(nb_letters) = nb_letters {
            anagrammer = anagrammer.with_nb_letters(nb_letters);
        }
        if let Some(letter_constraints) = letter_constraints {
            anagrammer = anagrammer.with_letter_constraints(letter_constraints);
        }
        if let Some(word_constraints) = word_constraints {
            anagrammer = anagrammer.with_word_constraints(word_constraints)?;
        }

        words_buf.clear();
        words_buf.extend(anagrammer);
        Ok(())
    }

    fn add_word(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.get_or_make_child(c);
        }
        node.is_word = true;
    }

    fn is_word(&self, word: &str) -> bool {
        match self.get_node(word) {
            None => return false,
            Some(node) => return node.is_word,
        };
    }
}

impl StrTree {
    fn init() -> Self {
        return Self {
            data: None,
            is_word: false,
            children: Vec::new(),
        };
    }

    fn get_child_idx(&self, c: char) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if child.data == Some(c) {
                return Some(i);
            }
        }
        return None;
    }

    fn get_child(&self, c: char) -> Option<&StrTree> {
        Some(&self.children[self.get_child_idx(c)?])
    }

    fn get_or_make_child(&mut self, c: char) -> &mut StrTree {
        match self.get_child_idx(c) {
            Some(idx) => return &mut self.children[idx],
            None => return self.add_child(c),
        };
    }

    fn add_child(&mut self, c: char) -> &mut StrTree {
        let new_tree = StrTree {
            data: Some(c),
            is_word: false,
            children: Vec::new(),
        };
        self.children.push(new_tree);
        return self.children.last_mut().unwrap();
    }

    fn get_node(&self, word: &str) -> Option<&StrTree> {
        let mut node = self;
        for c in word.chars() {
            node = node.get_child(c)?;
        }
        return Some(node);
    }

    // The output is wrapped in a Result to allow matching on errors
    fn fill_with_file(&mut self, filename: &str) -> std::io::Result<u32> {
        let nb_lines = cnt_lines(&filename)?;
        println!("reading {} words from file", nb_lines);

        let reader = read_lines(&filename)?;
        let mut nb_words: u32 = 0;
        for line in reader {
            if let Ok(word) = line {
                self.add_word(&word);
                nb_words += 1;
            }
        }

        return Ok(nb_words);
    }
}
