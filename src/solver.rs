use crate::str_tree;
use crate::str_tree::{Dictionnary, StaticWord};

use crate::board::transposition::*;
use crate::board::BoardService;
use crate::board::WordError;

use crate::constraints::{PotentialWord, PotentialWordConditions, PotentialWordConditionsBuilder};

use pyo3::prelude::{pyclass, pymethods};

type WordSearchResult = Result<Option<BestWord>, WordError>;
type SearchResult = Result<(), WordError>;

#[derive(Debug, PartialEq)]
#[pyclass]
pub struct BestWord {
    #[pyo3(get)]
    pub vertical: bool,
    #[pyo3(get)]
    pub coord: (usize, usize),
    #[pyo3(get)]
    pub word: String,
    #[pyo3(get)]
    pub score: usize,
}
#[pymethods]
impl BestWord {
    fn __str__(&self) -> pyo3::PyResult<String> {
        let mut ret = "[".to_string();
        ret.push_str(&self.word);
        ret.push(']');
        ret.push_str(" at: (");
        ret.push_str(&self.coord.0.to_string());
        ret.push_str(", ");
        ret.push_str(&self.coord.1.to_string());
        ret.push_str(") ");
        if self.vertical {
            ret.push_str("vertically -> ");
        } else {
            ret.push_str("horizontally -> ");
        }
        ret.push_str(&self.score.to_string());
        Ok(ret)
    }
}

pub trait TransposedBool {
    fn get_transposition_as_orientation() -> bool;
}
impl TransposedBool for Transposed {
    fn get_transposition_as_orientation() -> bool {
        true
    }
}
impl TransposedBool for NotTransposed {
    fn get_transposition_as_orientation() -> bool {
        false
    }
}

fn _find_best_word_at<T, B, D>(
    letter_set: &str,
    x: usize,
    y: usize,
    board: &B,
    dict: &D,
    words_buf: &mut Vec<StaticWord>,
    pw: &mut PotentialWord,
    timer: &mut Option<&mut std::time::Duration>,
) -> WordSearchResult
where
    B: BoardService,
    D: Dictionnary,
    T: TransposedState + TransposedBool,
{
    let mut best_word: String = "".to_string();
    let mut best_score = 0;

    board.get_conditions::<T, _>(x, y, pw);

    let now = std::time::Instant::now();
    dict.get_anagrams(
        letter_set,
        words_buf,
        pw.get_constraint_nb_letters(),
        pw.get_constraint_letters(),
        pw.get_constraint_words(),
    )?;
    if let Some(timer_uw) = timer {
        **timer_uw += now.elapsed();
    }

    for word in words_buf {
        let score = board.get_score::<T>(word.as_slice(), x, y)?;
        if score > best_score {
            best_score = score;
            best_word = word.str();
        }
    }

    match best_score {
        0 => Ok(None),
        _ => Ok(Some(BestWord {
            vertical: T::get_transposition_as_orientation(),
            coord: T::transposed_coord(x, y),
            word: best_word,
            score: best_score,
        })),
    }
}

fn find_best_word_at<B, D>(
    letter_set: &str,
    x: usize,
    y: usize,
    board: &B,
    dict: &D,
    words_buf: &mut Vec<StaticWord>,
    pw: &mut PotentialWord,
    timer: &mut Option<&mut std::time::Duration>,
) -> WordSearchResult
where
    B: BoardService,
    D: Dictionnary,
{
    let bw_horizontal = _find_best_word_at::<NotTransposed, _, _>(
        letter_set, x, y, board, dict, words_buf, pw, timer,
    )?;
    let bw_vertical = _find_best_word_at::<Transposed, _, _>(
        letter_set, x, y, board, dict, words_buf, pw, timer,
    )?;

    match (&bw_horizontal, &bw_vertical) {
        (None, None) => Ok(None),
        (None, _) => Ok(bw_vertical),
        (_, None) => Ok(bw_horizontal),
        (Some(ref b1), Some(ref b2)) => {
            if b1.score > b2.score {
                Ok(bw_horizontal)
            } else {
                Ok(bw_vertical)
            }
        }
    }
}

pub struct WithTimer;
pub struct WithoutTimer;
pub trait Timer {
    fn timer(t: &mut std::time::Duration) -> Option<&mut std::time::Duration>;
    fn print(t: &std::time::Duration);
}
impl Timer for WithTimer {
    fn timer(t: &mut std::time::Duration) -> Option<&mut std::time::Duration> {
        Some(t)
    }
    fn print(t: &std::time::Duration) {
        println!("Anagram time: {t:?}");
    }
}
impl Timer for WithoutTimer {
    fn timer(_: &mut std::time::Duration) -> Option<&mut std::time::Duration> {
        None
    }
    fn print(_: &std::time::Duration) {}
}

pub fn get_anagrams<D>(
    letter_set: &str,
    dict: &D,
    mut words_buf_opt: Option<&mut Vec<StaticWord>>,
) -> SearchResult
where
    D: Dictionnary,
{
    let mut small_buffer = str_tree::initiate_word_buf(1);
    let words_buf = match words_buf_opt {
        None => &mut small_buffer,
        Some(ref mut wb) => wb,
    };

    dict.get_anagrams(letter_set, words_buf, None, None, None)?;
    Ok(())
}

pub fn find_best_first_word<B, D>(
    letter_set: &str,
    board: &B,
    dict: &D,
    mut words_buf_opt: Option<&mut Vec<StaticWord>>,
) -> WordSearchResult
where
    B: BoardService,
    D: Dictionnary,
{
    let mut best_word: Option<BestWord> = None;

    let mut small_buffer = str_tree::initiate_word_buf(1);
    let words_buf = match words_buf_opt {
        None => &mut small_buffer,
        Some(ref mut wb) => wb,
    };

    dict.get_anagrams(letter_set, words_buf, None, None, None)?;

    let mut best_score = 0;
    for y in 0..7 {
        for word in &mut *words_buf {
            if y + word.len() - 1 < 7 {
                continue;
            }
            let score = board.get_score::<NotTransposed>(word.as_slice(), 7, y)?;

            if score > best_score {
                best_score = score;
                best_word = Some(BestWord {
                    vertical: false,
                    coord: (7, y),
                    word: word.str(),
                    score: best_score,
                });
            }
        }
    }

    Ok(best_word)
}

pub fn find_best_word<T: Timer, B, D>(
    letter_set: &str,
    board: &B,
    dict: &D,
    mut words_buf_opt: Option<&mut Vec<StaticWord>>,
) -> WordSearchResult
where
    B: BoardService,
    D: Dictionnary,
{
    let mut best_word: Option<BestWord> = None;
    let mut pw = PotentialWord::new();

    let mut small_buffer = str_tree::initiate_word_buf(1);
    let words_buf = match words_buf_opt {
        None => &mut small_buffer,
        Some(ref mut wb) => wb,
    };

    let mut base_time = std::time::Instant::now().elapsed();
    let mut timer = T::timer(&mut base_time);

    for x in 0..crate::board::SIDE {
        for y in 0..crate::board::SIDE {
            if let Some(bw) = find_best_word_at(
                letter_set, x, y, board, dict, words_buf, &mut pw, &mut timer,
            )? {
                best_word = match best_word {
                    None => Some(bw),
                    Some(ref word) => {
                        if word.score < bw.score {
                            Some(bw)
                        } else {
                            best_word
                        }
                    }
                };
            }
        }
    }

    T::print(&base_time);

    Ok(best_word)
}
