use quickcheck::TestResult;
use rand;

use crate::scores;
use crate::scores::Score;

#[test]
fn test_crate_consts() {
    assert_eq!(scores::VALID, scores::VALID);
    assert_eq!(scores::INVALID, scores::INVALID);
}

#[test]
fn test_score_to_char() {
    assert_eq!(Score::to_char(Score::Here), '🟢');
    assert_eq!(Score::to_char(Score::Somewhere), '🟡');
    assert_eq!(Score::to_char(Score::Nope), '⚫');
}

#[test]
fn create_winning_scored() {
    let guess = "guess";
    let answer = "guess";

    let scored = scores::Scored::new(guess, answer);

    assert_eq!(scored.valid, true);
    assert_eq!(scored.done, true);
    assert_eq!(scored.guess.len(), scored.answer.len());
    assert_eq!(scored.scores, vec![Score::Here; answer.chars().count()]);
}

#[test]
fn create_invalid_scored() {
    let guess = "Guess";
    let answer = "Answer"; // Not same length (unicode)

    let scored = scores::Scored::new(guess, answer);

    assert_eq!(scored.valid, false);
    assert_eq!(scored.done, false);
}

fn check(guess: &str, answer: &str, valid: bool, done: bool, scores: &Vec<Score>) -> bool {
    let scored = scores::Scored::new(guess, answer);

    println!("Checking: {}", scored);

    assert_eq!(valid, scored.valid);
    assert_eq!(done, scored.done);
    assert_eq!(scores, &scored.scores);

    true
}

#[test]
fn winning_tests() {
    let words: Vec<&str> = vec!["hello", "one", "🟢🟡🟡✅YEAH"];

    for word in words.iter() {
        check(
            word,
            word,
            true,
            true,
            &vec![Score::Here; word.chars().count()],
        );
    }
}

fn random_word_excluding(word: &str) -> String {
    let mut num_chars = word.chars().count();

    let mut random_word = String::new();

    while num_chars != 0 {
        let mut ch = rand::random::<char>();
        while word.contains(ch) {
            ch = rand::random::<char>();
        }
        random_word.push(ch);
        num_chars -= 1;
    }

    random_word
}

#[test]
fn totally_wrong_tests() {
    let words: Vec<&str> = vec!["hello", "one", "🟢🟡🟡✅YEAH", "BBqyz"];

    for word in words.iter() {
        check(
            word,
            &random_word_excluding(word),
            true,
            false,
            &vec![Score::Nope; word.chars().count()],
        );
    }
}

#[test]
fn half_right_tests() {
    let words: Vec<&str> = vec!["piechart", "four", "🟢🟡🟡✅YEAH", "bbqs"];

    for word in words.iter() {
        let num_chars = word.chars().count();

        let wrong_word = random_word_excluding(&word);

        let first_half_chars = word.chars().take(num_chars / 2).collect::<String>();
        let second_half_chars = wrong_word.chars().skip(num_chars / 2).collect::<String>();

        let half_right_word: String = first_half_chars + &second_half_chars;

        let mut expected_scores = vec![Score::Here; num_chars / 2];
        expected_scores.append(&mut vec![Score::Nope; num_chars / 2]);

        check(&half_right_word, &word, true, false, &expected_scores);
    }
}

struct Scenario {
    guess: String,
    answer: String,
    expected: Vec<Score>,
}

impl Scenario {
    pub fn new(guess: &str, answer: &str, expected: Vec<Score>) -> Self {
        Scenario {
            guess: String::from(guess),
            answer: String::from(answer),
            expected,
        }
    }
}

macro_rules! to_expected {
    (H) => {Score::Here};
    (S) => {Score::Somewhere};
    (N) => {Score::Nope};
    ($($c:tt)+) => {vec![
        $(to_expected!($c)),+
    ]}
}

#[test]
fn other_scenarios() {
    let tricky = vec![
        Scenario::new("cacca", "acccc", to_expected![S S H H N]),
        Scenario::new("aabbb", "acccc", to_expected![H N N N N]),
        Scenario::new("baabb", "acccc", to_expected![N S N N N]),
        Scenario::new("bbaab", "acccc", to_expected![N N S N N]),
        Scenario::new("bbbaa", "acccc", to_expected![N N N S N]),
        Scenario::new("bbbba", "acccc", to_expected![N N N N S]),
        Scenario::new("bbbbc", "acccc", to_expected![N N N N H]),
        Scenario::new("abcde", "bcdea", to_expected![S S S S S]),
    ];

    tricky.iter().for_each(|sc| {
        check(&sc.guess, &sc.answer, true, false, &sc.expected);
    });
}

#[quickcheck]
fn qc_winning_tests(word: String) -> TestResult {
    if word.len() == 0 || word.contains('\0') {
        return TestResult::discard();
    }

    TestResult::from_bool(check(
        &word,
        &word,
        true,
        true,
        &vec![Score::Here; word.chars().count()],
    ))
}

#[quickcheck]
fn qc_totally_wrong_tests(word: String) -> TestResult {
    if word.len() == 0 || word.contains('\0') {
        return TestResult::discard();
    }

    TestResult::from_bool(check(
        &word,
        &random_word_excluding(&word),
        true,
        false,
        &vec![Score::Nope; word.chars().count()],
    ))
}

#[quickcheck]
fn qc_half_right_tests(word: String) -> TestResult {
    let num_chars = word.chars().count();

    if num_chars == 0 || word.contains('\0') {
        return TestResult::discard();
    }

    let wrong_word = random_word_excluding(&word);

    let first_half_chars = word.chars().take(num_chars / 2).collect::<String>();
    let second_half_chars = wrong_word.chars().skip(num_chars / 2).collect::<String>();

    let half_right_word: String = first_half_chars + &second_half_chars;

    let mut expected_scores = vec![Score::Here; num_chars / 2];
    expected_scores.append(&mut vec![Score::Nope; num_chars / 2 + num_chars % 2]);

    check(&half_right_word, &word, true, false, &expected_scores);

    expected_scores.reverse();

    TestResult::from_bool(check(
        &half_right_word.chars().rev().collect::<String>(),
        &word.chars().rev().collect::<String>(),
        true,
        false,
        &expected_scores,
    ))
}
