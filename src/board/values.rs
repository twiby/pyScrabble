use crate::board::{WordError, WordError::*};

const VALUES: [u8; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 10, 1, 2, 1, 1, 3, 8, 1, 1, 1, 1, 4, 10, 10, 10, 10,
];

pub fn get_value_lowercase(c: char) -> usize {
    VALUES[(c as usize) - ('a' as usize)] as usize
}
pub fn get_value(c: char) -> Result<usize, WordError> {
    if c.is_ascii_lowercase() {
        Ok(get_value_lowercase(c))
    } else if c.is_ascii_uppercase() {
        Ok(0)
    } else {
        Err(UnknownChar("get_value: unknown char".to_string()))
    }
}
pub fn get_str_value(word: &str) -> Result<usize, WordError> {
    word.chars().map(get_value).sum()
}
