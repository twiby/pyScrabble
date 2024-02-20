use crate::str_tree;
use crate::str_tree::{Dictionnary, StaticWord};
use std::collections::HashSet;

fn to_string_vec(words: &Vec<StaticWord>) -> Vec<String> {
    words.iter().map(|w| w.str()).collect()
}

fn get_anagrams(letters: &str) -> Vec<String> {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let mut vec = str_tree::initiate_word_buf(0);
    tree.get_anagrams(letters, &mut vec, None, None, None)
        .unwrap();
    to_string_vec(&vec)
}

fn found_in_vec<T>(el: &T, vec: &Vec<T>) -> bool
where
    T: std::cmp::PartialEq,
{
    for el2 in vec.into_iter() {
        if *el == *el2 {
            return true;
        }
    }
    return false;
}

fn unordered_equal<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool
where
    T: std::cmp::PartialEq,
{
    for el in v1.into_iter() {
        if !found_in_vec(el, v2) {
            return false;
        }
    }
    for el in v2.into_iter() {
        if !found_in_vec(el, v1) {
            return false;
        }
    }
    return true;
}

fn assert_unordered_equal<T: std::cmp::Eq + std::fmt::Debug + std::hash::Hash>(
    v1: Vec<T>,
    v2: Vec<T>,
) {
    assert_eq!(
        v1.into_iter().collect::<HashSet<T>>(),
        v2.into_iter().collect::<HashSet<T>>()
    );
}

#[test]
fn load_success() {
    let _ = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
}

#[test]
#[should_panic]
fn load_fail() {
    let _ = str_tree::build_dict_from_file("prout.prout").expect("File not found");
}

#[test]
fn existing_words() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    assert!(tree.is_word("arbre"));
    assert!(tree.is_word("bar"));
    assert!(tree.is_word("barre"));
    assert!(tree.is_word("barbe"));
    assert!(tree.is_word("mazout"));
    assert!(tree.is_word("unmotduscrabble"));
    assert!(tree.is_word("rzzzzzzzz"));
    assert!(tree.is_word("ezzzzzzzz"));
    assert!(tree.is_word("bezzzzzzz"));
    assert!(!tree.is_word("erreur"));
}

#[test]
fn tree_iterator() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let walker = tree
        .iter_words()
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let mut count = 0;
    for word in walker {
        count += 1;
        println!("{:?}", word);
        assert!(tree.is_word(&word));

        if count > 500_000 {
            panic!("probable infinite recursion");
        }
    }

    assert_eq!(count, 9);
}

#[test]
fn tree_anagrammer() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let walker = tree
        .anagrams("arbre")
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    let correct_answer = vec!["arbre".to_string(), "bar".to_string(), "barre".to_string()];
    assert_unordered_equal(anagrams, correct_answer);
}

#[test]
fn tree_anagrammer_with_joker() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let walker = tree
        .anagrams("arbr0")
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    let correct_answer = vec![
        "arbrE".to_string(),
        "bar".to_string(),
        "Bar".to_string(),
        "bAr".to_string(),
        "baR".to_string(),
        "barrE".to_string(),
    ];
    assert_unordered_equal(anagrams, correct_answer);
}

#[test]
fn tree_anagrammer_with_2_joker() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let walker = tree
        .anagrams("ar00e")
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    let correct_answer = vec![
        "Bar".to_string(),
        "BAr".to_string(),
        "BaR".to_string(),
        "arBRe".to_string(),
        "aRBre".to_string(),
        "BarBe".to_string(),
        "BaRre".to_string(),
        "BarRe".to_string(),
    ];
    assert_unordered_equal(anagrams, correct_answer);
}

#[test]
fn tree_anagrammer_nb_letters_constraints() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let mut correct_answer = vec!["arbre".to_string(), "barre".to_string()];

    let walker = tree
        .anagrams("ar00e")
        .with_nb_letters(vec![])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, Vec::<String>::new());

    let walker = tree
        .anagrams("arbre")
        .with_nb_letters(vec![3])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, vec!["bar".to_string()]);

    let walker = tree
        .anagrams("arbre")
        .with_nb_letters(vec![5])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, correct_answer.clone());

    let walker = tree
        .anagrams("arbre")
        .with_nb_letters(vec![3, 5])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());

    let anagrams = walker.collect::<Vec<_>>();
    correct_answer.push("bar".to_string());
    assert_unordered_equal(anagrams, correct_answer);
}

#[test]
fn tree_anagrammer_letters_constraints() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

    let walker = tree
        .anagrams("arbe")
        .with_letter_constraints(vec![(2, 'z')])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());
    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, Vec::<String>::new());

    let walker = tree
        .anagrams("rbre")
        .with_letter_constraints(vec![(0, 'a')])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());
    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, vec!["_rbre".to_string()]);

    let walker = tree
        .anagrams("arbe")
        .with_letter_constraints(vec![(1, 'r')])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());
    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, vec!["a_bre".to_string()]);

    let walker = tree
        .anagrams("arbe")
        .with_letter_constraints(vec![(3, 'r')])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());
    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(anagrams, vec!["arb_e".to_string(), "bar_e".to_string()]);

    let walker = tree
        .anagrams("arbr")
        .with_letter_constraints(vec![(4, 'e')])
        .map(|mut static_word| static_word.into_word().iter().collect::<String>());
    let anagrams = walker.collect::<Vec<_>>();
    assert_unordered_equal(
        anagrams,
        vec!["arbr_".to_string(), "barr_".to_string(), "bar".to_string()],
    );
}

#[test]
fn add_word() {
    let mut tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    assert!(!tree.is_word("erreur"));
    tree.add_word("erreur");
    assert!(tree.is_word("erreur"));
}

#[test]
fn no_double_without_joker() {
    let anagrams = get_anagrams("arbre");

    for i1 in 0..anagrams.len() {
        for i2 in 0..anagrams.len() {
            if i1 == i2 {
                continue;
            }
            assert_ne!(anagrams[i1], anagrams[i2]);
        }
    }
}

#[test]
fn all_anagrams_without_joker() {
    let anagrams = get_anagrams("arbre");
    let correct_answer = vec!["arbre".to_string(), "bar".to_string(), "barre".to_string()];

    assert!(unordered_equal(&anagrams, &correct_answer));
}

#[test]
fn all_anagrams_with_joker() {
    let anagrams = get_anagrams("arbr0");
    let correct_answer = vec![
        "arbrE".to_string(),
        "bar".to_string(),
        "Bar".to_string(),
        "bAr".to_string(),
        "baR".to_string(),
        "barrE".to_string(),
    ];

    assert!(unordered_equal(&anagrams, &correct_answer));
}

#[test]
fn no_anagrams() {
    let empty = &Vec::<String>::new();
    assert_eq!(&get_anagrams(""), empty);
    assert_eq!(&get_anagrams("zzz"), empty);
    assert_eq!(&get_anagrams("00"), empty);
}

#[test]
fn nb_letters_constraints() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let mut correct_answer = vec!["arbre".to_string(), "barre".to_string()];

    let mut words = str_tree::initiate_word_buf(0);

    tree.get_anagrams("arbre", &mut words, Some(vec![]), None, None)
        .unwrap();
    assert!(unordered_equal(
        &to_string_vec(&words),
        &Vec::<String>::new()
    ));

    tree.get_anagrams("arbre", &mut words, Some(vec![3]), None, None)
        .unwrap();
    assert!(unordered_equal(
        &to_string_vec(&words),
        &vec!["bar".to_string()]
    ));

    tree.get_anagrams("arbre", &mut words, Some(vec![5]), None, None)
        .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));

    correct_answer.push("bar".to_string());
    tree.get_anagrams("arbre", &mut words, Some(vec![3, 5]), None, None)
        .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));
}

#[test]
fn no_letter_actually_used() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let empty = &Vec::<String>::new();

    let mut words = str_tree::initiate_word_buf(0);

    tree.get_anagrams("", &mut words, Some(vec![0]), None, None)
        .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), empty));
    tree.get_anagrams(
        "",
        &mut words,
        Some(vec![0]),
        Some(vec![(0, 'b'), (1, 'a'), (2, 'r')]),
        None,
    )
    .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), empty));
}

#[test]
fn nb_letters_does_not_include_constraints() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let correct_answer = vec!["___re".to_string()];

    let mut words = str_tree::initiate_word_buf(0);

    tree.get_anagrams(
        "re",
        &mut words,
        Some(vec![2]),
        Some(vec![(0, 'b'), (1, 'a'), (2, 'r')]),
        None,
    )
    .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));
}

#[test]
fn letters_constraints() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

    let mut words = str_tree::initiate_word_buf(0);

    tree.get_anagrams("arbe", &mut words, None, Some(vec![(2, 'z')]), None)
        .unwrap();
    assert!(unordered_equal(
        &to_string_vec(&words),
        &Vec::<String>::new()
    ));

    tree.get_anagrams("rbre", &mut words, None, Some(vec![(0, 'a')]), None)
        .unwrap();
    assert!(unordered_equal(
        &to_string_vec(&words),
        &vec!["_rbre".to_string()]
    ));
    tree.get_anagrams("arbe", &mut words, None, Some(vec![(1, 'r')]), None)
        .unwrap();
    assert!(unordered_equal(
        &to_string_vec(&words),
        &vec!["a_bre".to_string()]
    ));

    tree.get_anagrams("arbe", &mut words, None, Some(vec![(3, 'r')]), None)
        .unwrap();
    println!("{:?}", words);
    assert!(unordered_equal(
        &to_string_vec(&words),
        &vec!["arb_e".to_string(), "bar_e".to_string()]
    ));

    tree.get_anagrams("arbr", &mut words, None, Some(vec![(4, 'e')]), None)
        .unwrap();
    assert!(unordered_equal(
        &to_string_vec(&words),
        &vec!["arbr_".to_string(), "barr_".to_string(), "bar".to_string()]
    ));
}

#[test]
fn words_constraint() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

    let mut words = str_tree::initiate_word_buf(0);

    let mut correct_answer = vec!["bar".to_string()];
    let mut constraints = Some(vec![(
        2,
        crate::constraints::WordToFill::new("ba".to_string(), "re".to_string()).unwrap(),
    )]);
    tree.get_anagrams(
        "arbre",
        &mut words,
        Some(vec![2, 3]),
        None,
        constraints.clone(),
    )
    .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));

    correct_answer = vec!["barre".to_string(), "bar".to_string()];
    constraints = Some(vec![(
        2,
        crate::constraints::WordToFill::new("ba".to_string(), "re".to_string()).unwrap(),
    )]);
    tree.get_anagrams("arbre", &mut words, None, None, constraints.clone())
        .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));

    correct_answer = vec!["arbre".to_string()];
    constraints = Some(vec![(
        2,
        crate::constraints::WordToFill::new("ar".to_string(), "re".to_string()).unwrap(),
    )]);
    tree.get_anagrams("arbre", &mut words, None, None, constraints.clone())
        .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));
}

#[test]
fn all_constraints() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");
    let mut words = str_tree::initiate_word_buf(0);
    let mut correct_answer = vec!["b_r".to_string()];

    tree.get_anagrams(
        "rbre",
        &mut words,
        Some(vec![2]),
        Some(vec![(1, 'a')]),
        Some(vec![(
            2,
            crate::constraints::WordToFill::new("a".to_string(), "bre".to_string()).unwrap(),
        )]),
    )
    .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));

    correct_answer.push("b_rre".to_string());

    tree.get_anagrams(
        "rbre",
        &mut words,
        None,
        Some(vec![(1, 'a')]),
        Some(Vec::<(u8, crate::constraints::WordToFill)>::new()),
    )
    .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));

    tree.get_anagrams("rbre", &mut words, None, Some(vec![(1, 'a')]), None)
        .unwrap();
    assert!(unordered_equal(&to_string_vec(&words), &correct_answer));
}

use crate::board;
use crate::board::BoardService;
use crate::board::DeserializingError;

use crate::board::transposition::*;

use crate::constraints;
use crate::constraints::{PotentialWordConditions, PotentialWordConditionsBuilder, WordToFill};

#[test]
fn board_serialization() {
    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___a___2__6");
    str_board.push_str("__2___2r2___2__");
    str_board.push_str("_3___3_b_3___3_");
    str_board.push_str("____5__R__5____");
    str_board.push_str("2__5___e___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__");

    let b = board::deserialize(&str_board).expect_err("Unlikely Success");
    match b {
        DeserializingError::WrongLength(_) => (),
        _ => panic!("Wrong error type"),
    }

    str_board.push('!');
    let b2 = board::deserialize(&str_board).expect_err("Unlikely success");
    match b2 {
        DeserializingError::UnknownSymbol(_) => (),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn get_conditions_vertical() {
    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___a___2__6");
    str_board.push_str("__2___2r2___2__");
    str_board.push_str("_3___3_b_3___3_");
    str_board.push_str("____5__R__5____");
    str_board.push_str("2__5___e___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");

    let board = board::deserialize(&str_board).expect("Error when deserializing board message");
    let mut pw = constraints::PotentialWord::new();

    board.get_conditions::<NotTransposed, _>(10, 0, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(vec![7]));
    assert_eq!(pw.get_constraint_letters(), Some(vec![(7, 'r')]));
    assert_eq!(
        pw.get_constraint_words(),
        Some(Vec::<(u8, WordToFill)>::new())
    );

    board.get_conditions::<NotTransposed, _>(11, 0, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(vec![7]));
    assert_eq!(pw.get_constraint_letters(), Some(vec![(7, 'e')]));
    assert_eq!(
        pw.get_constraint_words(),
        Some(Vec::<(u8, WordToFill)>::new())
    );

    board.get_conditions::<NotTransposed, _>(12, 0, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(vec![]));
    assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
    assert_eq!(pw.get_constraint_words(), Some(vec![]));

    board.get_conditions::<NotTransposed, _>(6, 0, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(vec![]));
    assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
    assert_eq!(pw.get_constraint_words(), Some(vec![]));

    board.get_conditions::<NotTransposed, _>(11, 7, &mut pw);
    assert_eq!(
        pw.get_constraint_nb_letters(),
        Some(vec![0, 1, 2, 3, 4, 5, 6, 7])
    );
    assert_eq!(pw.get_constraint_letters(), Some(vec![(0, 'e')]));
    assert_eq!(
        pw.get_constraint_words(),
        Some(Vec::<(u8, WordToFill)>::new())
    );

    board.get_conditions::<NotTransposed, _>(11, 8, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(Vec::<u8>::new()));
    assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
    assert_eq!(
        pw.get_constraint_words(),
        Some(Vec::<(u8, WordToFill)>::new())
    );
}

#[test]
fn get_conditions_horizontal() {
    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___arbre__6");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("____5_____5____");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");

    let board = board::deserialize(&str_board).expect("Error when deserializing board message");
    let mut pw = constraints::PotentialWord::new();

    board.get_conditions::<NotTransposed, _>(10, 0, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(Vec::<u8>::new()));
    assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
    assert_eq!(
        pw.get_constraint_words(),
        Some(Vec::<(u8, WordToFill)>::new())
    );

    board.get_conditions::<NotTransposed, _>(7, 0, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(vec![7]));
    assert_eq!(
        pw.get_constraint_letters(),
        Some(vec![(7, 'a'), (8, 'r'), (9, 'b'), (10, 'r'), (11, 'e')])
    );
    assert_eq!(
        pw.get_constraint_words(),
        Some(Vec::<(u8, WordToFill)>::new())
    );

    board.get_conditions::<NotTransposed, _>(8, 10, &mut pw);
    assert_eq!(pw.get_constraint_nb_letters(), Some(vec![1, 2, 3, 4, 5]));
    assert_eq!(pw.get_constraint_letters(), Some(Vec::<(u8, char)>::new()));
    assert_eq!(
        pw.get_constraint_words(),
        Some(vec![
            (0, WordToFill::new("r".to_string(), "".to_string()).unwrap()),
            (1, WordToFill::new("e".to_string(), "".to_string()).unwrap())
        ])
    );
}

#[test]
fn word_conditions_on_boundary() {
    let mut str_board = "".to_string();
    str_board.push_str("6__2b__6___2__6");
    str_board.push_str("_5__a3___3___5_");
    str_board.push_str("__5_n_2_2___5__");
    str_board.push_str("2__5c__2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("arbre__5___2__6");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("____5_____5____");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");

    let board = board::deserialize(&str_board).expect("Error when deserializing board message");
    let mut pw = constraints::PotentialWord::new();

    board.get_conditions::<NotTransposed, _>(4, 2, &mut pw);
    assert_eq!(
        pw.get_constraint_words(),
        Some(vec![(
            2,
            WordToFill::new("banc".to_string(), "".to_string()).unwrap()
        )])
    );

    board.get_conditions::<Transposed, _>(5, 6, &mut pw);
    assert_eq!(
        pw.get_constraint_words(),
        Some(vec![(
            1,
            WordToFill::new("arbre".to_string(), "".to_string()).unwrap()
        )])
    );
}

#[test]
fn get_score() {
    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___a___2__6");
    str_board.push_str("__2___2r2___2__");
    str_board.push_str("_3___3_bE3___3_");
    str_board.push_str("____5__R2_5____");
    str_board.push_str("2__5___e___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");

    let board = board::deserialize(&str_board).expect("Error when deserializing board message");

    assert_eq!(
        7,
        board
            .get_score::<NotTransposed>(&['t', 'e', '_', 's', 'e'], 10, 5)
            .unwrap()
    );
    assert_eq!(
        6,
        board
            .get_score::<NotTransposed>(&['t', 'E', '_', 's', 'e'], 10, 5)
            .unwrap()
    );
    assert_eq!(
        3,
        board
            .get_score::<NotTransposed>(&['t', 'e', '_', 'S', 'e'], 10, 5)
            .unwrap()
    );
    assert_eq!(
        14,
        board
            .get_score::<NotTransposed>(&['t', 'e', '_', 's', 'e', 's'], 10, 5)
            .unwrap()
    );
    assert_eq!(
        8,
        board
            .get_score::<NotTransposed>(&['t', 'e', '_', 'S', 'e', 's'], 10, 5)
            .unwrap()
    );
    assert_eq!(
        32,
        board
            .get_score::<NotTransposed>(&['t', 'e', '_', 'f', 'e', 's'], 10, 5)
            .unwrap()
    );
    assert_eq!(
        18,
        board
            .get_score::<Transposed>(&['m', 'e', 's', 's', 'e'], 9, 5)
            .unwrap()
    );
    assert_eq!(
        124,
        board
            .get_score::<Transposed>(&['s', 'y', 's', 't', 'e', 'm', 'e'], 9, 8)
            .unwrap()
    );
}

use crate::board::WordError;

#[test]
fn get_score_errors() {
    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___a___2__6");
    str_board.push_str("__2___2r2___2__");
    str_board.push_str("_3___3_bE3___3_");
    str_board.push_str("____5__R2_5____");
    str_board.push_str("2__5___e___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");

    let board = board::deserialize(&str_board).expect("Error when deserializing board message");

    match board.get_score::<NotTransposed>(&['t', 'e', 'r', 's', 'e'], 10, 5) {
        Err(WordError::TileOccupied(_)) => (),
        _ => panic!("Wrong error type"),
    };

    match board.get_score::<NotTransposed>(&['t', 'E', '_', 's', '_'], 10, 5) {
        Err(WordError::UnexpectedUnderscore(_)) => (),
        _ => panic!("Wrong error type"),
    }

    match board.get_score::<NotTransposed>(&['t', 'E', '_', 's', '!'], 10, 5) {
        Err(WordError::UnknownChar(_)) => (),
        _ => panic!("Wrong error type"),
    }
}

use crate::solver;
use crate::solver::WithoutTimer;

#[test]
fn complete_test() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___a___2__6");
    str_board.push_str("__2___2r2___2__");
    str_board.push_str("_3___3_be3___3_");
    str_board.push_str("____5__R__5____");
    str_board.push_str("2__5___e___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");
    let board = board::deserialize(&str_board).expect("Error when deserializing board message");

    let mut bw = solver::find_best_word::<WithoutTimer, _, _>("", &board, &tree, None);
    assert_eq!(bw, Ok(None));

    bw = solver::find_best_word::<WithoutTimer, _, _>("arbre", &board, &tree, None);
    assert_eq!(
        bw,
        Ok(Some(solver::BestWord {
            coord: (11, 3),
            word: "arbr_".to_string(),
            vertical: false,
            score: 12
        }))
    );
}

#[test]
fn complete_test_error() {
    let tree = str_tree::build_dict_from_file("src/test/words.txt").expect("File not found");

    let mut str_board = "".to_string();
    str_board.push_str("6__2___6___2__6");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("2__5___2___5__2");
    str_board.push_str("____5_____5____");
    str_board.push_str("_3___3___3___3_");
    str_board.push_str("__2___2_2___2__");
    str_board.push_str("6__2___a___2__6");
    str_board.push_str("__2___2r2___2__");
    str_board.push_str("_3___3_be3___3_");
    str_board.push_str("____5__R__5____");
    str_board.push_str("2__5___z___5__2");
    str_board.push_str("__5___2_2___5__");
    str_board.push_str("_5___3___3___5_");
    str_board.push_str("6__2___6___2__6");
    let board = board::deserialize(&str_board).expect("Error when deserializing board message");

    let bw = solver::find_best_word::<WithoutTimer, _, _>("arbre", &board, &tree, None);
    match bw {
        Err(WordError::UnknownConstraint(_)) => (),
        _ => panic!("Wrong error type"),
    }
}
