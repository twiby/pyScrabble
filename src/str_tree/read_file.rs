use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn cnt_lines<P>(filename: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    let mut cnt: u32 = 0;
    for _ in io::BufReader::new(file).lines() {
        cnt += 1;
    }
    Ok(cnt)
}
