use crate::str_tree::WordError;
use crate::str_tree::SIDE;
use crate::str_tree::{cnt_lines, read_lines};
use crate::str_tree::{ConstraintLetters, ConstraintNbLetters, ConstraintWords};
use crate::str_tree::{Dictionnary, StaticWord};

struct TreeIter<'a> {
    cursor: Vec<std::slice::Iter<'a, StrTree>>,
    word: StaticWord,
}

impl<'a> TreeIter<'a> {
    fn new(tree: &'a StrTree) -> Self {
        Self {
            cursor: vec![tree.children.iter()],
            word: Default::default(),
        }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = (&'a StrTree, StaticWord);
    fn next(&mut self) -> Option<Self::Item> {
        let Some(mut it) = self.cursor.pop() else {
            return None;
        };

        let Some(child) = it.next() else {
            self.word.pop();
            return self.next();
        };

        self.word.push(child.data.unwrap());
        self.cursor.push(it);
        self.cursor.push(child.children.iter());
        Some((child, self.word))
    }
}

#[derive(Debug)]
struct LetterSet {
    data: Vec<char>,
}
impl LetterSet {
    fn from_letters(letters: Vec<char>) -> Self {
        let mut ret = Self {
            data: Vec::with_capacity(letters.len()),
        };
        ret.data
            .extend(letters.into_iter().filter(|c| c.is_ascii_lowercase()));
        ret
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
    JokerUsed,
}

#[derive(Debug)]
struct TreeAnagrammer<'a> {
    // Itertaor state
    states: Vec<LevelState>,
    parents: Vec<&'a StrTree>,
    cursor: Vec<std::slice::Iter<'a, StrTree>>,

    // letter info
    word: StaticWord,
    set: LetterSet,
    nb_available_jokers: usize,
    // Constraints type
    // nb_letters: CNbL,
    // letter_constraints: CL,
    // word_constraints: CW,
}
impl<'a> TreeAnagrammer<'a> {
    fn new(tree: &'a StrTree, set: Vec<char>) -> Self {
        let mut ret = Self {
            states: vec![],
            parents: vec![],
            cursor: vec![],
            word: Default::default(),
            nb_available_jokers: set.iter().filter(|&&c| c == '0').count(),
            set: LetterSet::from_letters(set),
        };

        ret.cursor.push(tree.children.iter());
        ret.parents.push(tree);
        if ret.nb_available_jokers == 0 {
            ret.states.push(LevelState::JokerUsed);
        } else {
            ret.states.push(LevelState::UsingJoker);
            ret.nb_available_jokers -= 1;
        }

        ret
    }
    fn push_child(&mut self, child: &'a StrTree) {
        let mut c = child.data.unwrap();
        if self.states.last() == Some(&LevelState::UsingJoker) {
            c = c.to_ascii_uppercase();
        }
        self.word.push(c);
        self.cursor.push(child.children.iter());
        self.parents.push(child);

        if self.nb_available_jokers == 0 {
            self.states.push(LevelState::JokerUsed);
        } else {
            self.states.push(LevelState::UsingJoker);
            self.nb_available_jokers -= 1;
        }
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
    fn next_child(&mut self) -> Option<&'a StrTree> {
        self.cursor.last_mut()?.next()
    }
    fn next_child_from_set(&mut self) -> Option<&'a StrTree> {
        while let Some(c) = self.next_child() {
            if self.set.remove(c.data?) {
                return Some(c);
            }
        }
        None
    }

    /// Final check before returning
    fn return_state_if_valid(&mut self) -> Option<StaticWord> {
        if self.parents.last()?.is_word {
            Some(self.word)
        } else {
            self.next()
        }
    }
}
impl<'a> Iterator for TreeAnagrammer<'a> {
    type Item = StaticWord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }

        debug_assert_eq!(self.parents.len(), self.cursor.len());
        debug_assert_eq!(self.states.len(), self.cursor.len());
        let last = self.parents.len() - 1;

        match self.states.last()? {
            LevelState::UsingJoker => {
                if let Some(child) = self.next_child() {
                    self.push_child(child);
                    self.return_state_if_valid()
                } else {
                    // wind back the last cursor
                    self.cursor[last] = self.parents[last].children.iter();
                    self.states[last] = LevelState::JokerUsed;
                    self.nb_available_jokers += 1;
                    self.next()
                }
            }

            LevelState::JokerUsed => {
                if let Some(child) = self.next_child_from_set() {
                    self.push_child(child);
                    self.return_state_if_valid()
                } else {
                    self.pop_child();
                    self.next()
                }
            }
        }
    }
}

pub struct StrTree {
    data: Option<char>,
    is_word: bool,
    children: Vec<StrTree>,
}

impl StrTree {
    pub(crate) fn iter_words(&self) -> impl Iterator<Item = StaticWord> + '_ {
        TreeIter::new(self)
            .filter(|(node, _)| node.is_word)
            .map(|(_, word)| word)
    }

    pub(crate) fn anagrams(&self, letters: &str) -> impl Iterator<Item = StaticWord> + '_ {
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

    fn get_anagrams<CNbL, CL, CW>(
        &self,
        letter_set: &str,
        words_buf: &mut Vec<StaticWord>,
        mut nb_letters: CNbL,
        mut letter_constraints: CL,
        mut word_constraints: CW,
    ) -> Result<(), WordError>
    where
        CNbL: ConstraintNbLetters,
        CL: ConstraintLetters,
        CW: ConstraintWords,
    {
        let mut letter_set_vec: Vec<char> = letter_set.chars().collect();
        letter_set_vec.sort_unstable();
        nb_letters.sort_and_fuse();
        letter_constraints.sort_and_fuse();
        word_constraints.sort_and_fuse();

        // We reformulate constraints in more memory efficient layouts
        let mut max_nb_letters = 0;
        let mut valid_nb_letter = [false; SIDE];
        let mut obligatory_letters: [Option<char>; SIDE] = [None; SIDE];
        let mut words_to_fill: [Option<(&StrTree, String)>; SIDE] = Default::default();
        for i in 0..SIDE {
            if nb_letters.decrease() {
                valid_nb_letter[i] = true;
                max_nb_letters = i;
            }
            obligatory_letters[i] = letter_constraints.decrease();
            words_to_fill[i] = self.get_next_word_to_fill(word_constraints.decrease('_'))?;
        }

        let mut letter_set = StaticWord {
            w: Default::default(),
            l: 0,
        };
        let mut current_word_buf = StaticWord {
            w: Default::default(),
            l: 0,
        };
        for c in letter_set_vec.iter() {
            letter_set.push(*c);
        }

        words_buf.clear();
        self.get_anagrams_internal(
            0,
            letter_set.into_word(),
            &mut current_word_buf,
            max_nb_letters,
            &valid_nb_letter,
            &obligatory_letters,
            &words_to_fill,
            words_buf,
        );
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

    fn get_next_word_to_fill<'a, 'b: 'a>(
        &'b self,
        wtf: Option<String>,
    ) -> Result<Option<(&'a StrTree, String)>, WordError> {
        if wtf.is_none() {
            return Ok(None);
        }
        let binding = wtf.unwrap();
        let segments: Vec<&str> = binding.split('_').collect();
        let node = match self.get_node(segments[0]) {
            Some(node) => node,
            None => {
                return Err(WordError::UnknownConstraint(format!(
                    "Constraint word doesn't exist: {}",
                    segments[0]
                )))
            }
        };
        Ok(Some((&node, segments[1].to_string())))
    }

    fn get_anagrams_internal(
        &self,
        depth: usize,
        letter_set: &mut [char],
        current_word: &mut StaticWord,
        max_nb_letters: usize,
        valid_nb_letter: &[bool; SIDE],
        obligatory_letters: &[Option<char>; SIDE],
        words_to_fill: &[Option<(&StrTree, String)>; SIDE],
        words: &mut Vec<StaticWord>,
    ) {
        let length = current_word.l;

        // Case the current node is supposed to complete a word on the board
        match self.data {
            None => (),
            Some(c) => {
                let valid_filled_word = match words_to_fill[length - 1] {
                    None => true,
                    Some((ref node, ref end)) => {
                        if let Some(child) = node.get_child(c) {
                            child.is_word(&end)
                        } else {
                            false
                        }
                    }
                };
                if !valid_filled_word {
                    return;
                }
            }
        };

        // Case the next letter is a constraint: continue only on that branch if it exists
        if let Some(constraint) = obligatory_letters[length] {
            let node = match self.get_child(constraint) {
                None => return,
                Some(node) => node,
            };
            current_word.push('_');
            return node.get_anagrams_internal(
                depth,
                letter_set,
                current_word,
                max_nb_letters,
                &valid_nb_letter,
                &obligatory_letters,
                &words_to_fill,
                words,
            );
        }

        if self.is_word && valid_nb_letter[depth] {
            words.push(*current_word);
        }

        // Case there is no higher up number of letters possible: exit
        let set_size = letter_set.len();
        if depth >= max_nb_letters || set_size == 0 {
            return;
        }

        // Case where there's at least one joker in set
        if letter_set[0] == '0' {
            for child in &self.children {
                current_word.push(child.data.unwrap().to_ascii_uppercase());
                child.get_anagrams_internal(
                    depth + 1,
                    &mut letter_set[1..],
                    current_word,
                    max_nb_letters,
                    &valid_nb_letter,
                    &obligatory_letters,
                    &words_to_fill,
                    words,
                );
                current_word.l = length;
            }
        }

        // Now take every letter in the set, and see if you can build a word from it
        for i in 0..set_size {
            // This avoids repetition coming from identitical letters
            if i > 0 && letter_set[0] == letter_set[i] {
                continue;
            }

            letter_set.swap(0, i);

            match self.get_child(letter_set[0]) {
                None => continue,
                Some(node) => {
                    current_word.push(node.data.unwrap());
                    node.get_anagrams_internal(
                        depth + 1,
                        &mut letter_set[1..],
                        current_word,
                        max_nb_letters,
                        &valid_nb_letter,
                        &obligatory_letters,
                        &words_to_fill,
                        words,
                    );
                    current_word.l = length;
                }
            };
        }

        let temp = letter_set[0];
        for i in 0..set_size - 1 {
            letter_set[i] = letter_set[i + 1];
        }
        letter_set[set_size - 1] = temp;
    }
}
