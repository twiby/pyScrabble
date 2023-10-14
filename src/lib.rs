#![allow(non_snake_case)]

#[cfg(test)]
mod test;

mod str_tree;
use str_tree::Dictionnary;

mod constraints;

mod board;
use board::DeserializingError;
use board::DeserializingError::*;
use board::WordError;
use board::WordError::*;

mod solver;
use solver::BestWord;
use solver::WithoutTimer;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

trait ErrorTypeToString {
    fn str() -> String;
}
fn py_value_error<ErrorType: ErrorTypeToString>(msg: &str) -> pyo3::PyErr {
    let mut string = ErrorType::str();
    string.push_str(": ");
    string.push_str(msg);
    PyErr::new::<PyValueError, _>(string)
}

// TODO: these error messages should be with the classe declaration and flow more naturally in PyErr
impl ErrorTypeToString for WordError {
    fn str() -> String {
        "WordError".to_string()
    }
}
impl From<WordError> for pyo3::PyErr {
    fn from(e: WordError) -> Self {
        match e {
            TileOccupied(s) => py_value_error::<WordError>(&s),
            UnknownChar(s) => py_value_error::<WordError>(&s),
            UnexpectedUnderscore(s) => py_value_error::<WordError>(&s),
            UnknownConstraint(s) => py_value_error::<WordError>(&s),
        }
    }
}

impl ErrorTypeToString for DeserializingError {
    fn str() -> String {
        "DeserializingError".to_string()
    }
}
impl From<DeserializingError> for pyo3::PyErr {
    fn from(e: DeserializingError) -> Self {
        match e {
            WrongLength(s) => py_value_error::<DeserializingError>(&s),
            UnknownSymbol(s) => py_value_error::<DeserializingError>(&s),
        }
    }
}

#[pyclass]
struct WordFinder {
    _tree: str_tree::StrTree,
    _word_buffer: Vec<str_tree::StaticWord>,
}

#[pymethods]
impl WordFinder {
    #[new]
    fn new(filename: &str) -> PyResult<Self> {
        match str_tree::build_dict_from_file(filename) {
            Err(e) => Err(PyErr::new::<PyValueError, _>(e)),
            Ok(tree) => Ok(WordFinder {
                _tree: tree,
                _word_buffer: str_tree::initiate_word_buf(1000),
            }),
        }
    }

    fn is_word(&self, word: &str) -> bool {
        return self._tree.is_word(word);
    }

    fn get_best_first_play(&mut self, word: &str, board_msg: &str) -> PyResult<Option<BestWord>> {
        let board = board::deserialize(board_msg)?;
        let bw =
            solver::find_best_first_word(word, &board, &self._tree, Some(&mut self._word_buffer))?;
        return Ok(bw);
    }

    fn get_best_play(&mut self, word: &str, board_msg: &str) -> PyResult<Option<BestWord>> {
        let board = board::deserialize(board_msg)?;
        let bw = solver::find_best_word::<WithoutTimer, _, _>(
            word,
            &board,
            &self._tree,
            Some(&mut self._word_buffer),
        )?;
        return Ok(bw);
    }
}

#[pymodule]
fn rsScrabble(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WordFinder>()?;
    m.add_class::<BestWord>()?;
    return Ok(());
}
