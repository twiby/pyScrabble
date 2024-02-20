mod constraints;
pub use constraints::PotentialWord;
pub use constraints::WordToFill;

pub trait ConstraintNbLetters: Clone {}
pub trait ConstraintLetters: Clone {}
pub trait ConstraintWords: Clone {}

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
