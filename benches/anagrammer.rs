use criterion::{criterion_group, criterion_main, Criterion};
use rsScrabble::build_dict_from_file;
use rsScrabble::initiate_word_buf;
use rsScrabble::Dictionnary;

pub fn no_joker(c: &mut Criterion) {
    let tree = build_dict_from_file("scrabbleWords.txt")
        .expect("File not found: please run the Python app once before benching");
    let mut words = initiate_word_buf(1000);

    c.bench_function("no_joker", |b| {
        b.iter(|| {
            let walker = tree
                .get_anagrams("arbrement", &mut words, None, None, None)
                .ok();
        })
    });
}
pub fn one_joker(c: &mut Criterion) {
    let tree = build_dict_from_file("scrabbleWords.txt")
        .expect("File not found: please run the Python app once before benching");
    let mut words = initiate_word_buf(1000);

    c.bench_function("one_joker", |b| {
        b.iter(|| {
            let walker = tree
                .get_anagrams("arbrement0", &mut words, None, None, None)
                .ok();
        })
    });
}
pub fn two_joker(c: &mut Criterion) {
    let tree = build_dict_from_file("scrabbleWords.txt")
        .expect("File not found: please run the Python app once before benching");
    let mut words = initiate_word_buf(1000);

    c.bench_function("two_joker", |b| {
        b.iter(|| {
            let walker = tree
                .get_anagrams("arbrement00", &mut words, None, None, None)
                .ok();
        })
    });
}

criterion_group!(anagrammer, no_joker, one_joker, two_joker);
criterion_main!(anagrammer);
