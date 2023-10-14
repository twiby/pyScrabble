mod constraints;
pub use constraints::PotentialWord;
pub use constraints::WordToFill;

pub trait ConstraintNbLetters: Clone {
    fn sort_and_fuse(&mut self);
    fn decrease(&mut self) -> bool;
}
pub trait ConstraintLetters: Clone {
    fn sort_and_fuse(&mut self);
    fn decrease(&mut self) -> Option<char>;
}
pub trait ConstraintWords: Clone {
    fn sort_and_fuse(&mut self);
    fn decrease(&mut self, c: char) -> Option<String>;
}

pub trait PotentialWordConditions<CNbL, CL, CW>
where
    CNbL: ConstraintNbLetters,
    CL: ConstraintLetters,
    CW: ConstraintWords,
{
    fn get_constraint_nb_letters(&self) -> CNbL;
    fn get_constraint_letters(&self) -> CL;
    fn get_constraint_words(&self) -> CW;
}
pub trait PotentialWordConditionsBuilder {
    fn new() -> Self;
    fn reset(&mut self);
    fn add_nb_letters(&mut self, n: u8);
    fn add_letter(&mut self, c: char, pos: u8);
    fn add_word(&mut self, w: WordToFill, pos: u8);
}
