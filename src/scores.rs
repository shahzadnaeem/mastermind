use std::fmt;

use owo_colors::OwoColorize;

pub const VALID: &str = "VALID";
pub const INVALID: &str = "INVALID";

const USED_CHAR: char = '\0';

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum Score {
    Here,
    Somewhere,
    Nope,
}

#[readonly::make]
pub struct Scored {
    pub valid: bool,
    pub done: bool,
    pub guess: String,
    pub answer: String,
    pub scores: Vec<Score>,
}

impl Score {
    pub fn to_char(self) -> char {
        match self {
            Score::Here => '🟢',
            Score::Somewhere => '🟡',
            Score::Nope => '⚫',
        }
    }
}

fn calc_scores(guess: &str, answer: &str, scores: &mut Vec<Score>) -> bool {
    let mut ans: Vec<char> = answer.chars().collect();

    // First pass - chars in correct place => Score::Here
    for (i, c) in guess.chars().enumerate() {
        if c.to_ascii_lowercase() == ans[i].to_ascii_lowercase() {
            scores[i] = Score::Here;
            ans[i] = USED_CHAR;
        }
    }

    // Second pass - chars in incorrect place => Score::Somewhere
    for (i, c) in guess.chars().enumerate() {
        if scores[i] == Score::Nope {
            if let Some(pos) = ans
                .iter()
                .position(|&v| v.to_ascii_lowercase() == c.to_ascii_lowercase())
            {
                scores[i] = Score::Somewhere;
                ans[pos] = USED_CHAR;
            }
        }
    }

    scores.iter().all(|&v| v == Score::Here)
}

impl Scored {
    pub fn new(guess: &str, answer: &str) -> Self {
        let guess_chars = guess.chars().count();
        let answer_chars = answer.chars().count();

        let valid = guess_chars == answer_chars;
        let mut scores = vec![Score::Nope; std::cmp::max(guess_chars, answer_chars)];

        let mut done = false;

        if valid {
            done = calc_scores(&guess, &answer, &mut scores);
        }

        Scored {
            valid,
            done,
            guess: String::from(guess),
            answer: String::from(answer),
            scores,
        }
    }

    pub fn score_result(&self) -> String {
        self.scores.iter().map(|s| s.to_char()).collect::<String>()
    }

    pub fn coloured_guess(&self) -> String {
        let mut answer = String::new();

        for (c, s) in self
            .guess
            .to_uppercase()
            .to_string()
            .chars()
            .zip(self.scores.iter())
        {
            match s {
                Score::Here => answer.push_str(&format!("{}", c.green())),
                Score::Somewhere => answer.push_str(&format!("{}", c.yellow())),
                Score::Nope => answer.push_str(&format!("{}", c.black())),
            }
        }

        answer
    }
}

impl fmt::Display for Scored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scores = self.score_result();

        write!(
            f,
            "{{{}, {}, g={}, a={}, {} {}}}",
            if self.valid { VALID } else { INVALID },
            if self.done { "✅" } else { "❌" },
            self.guess,
            self.answer,
            self.coloured_guess(),
            scores,
        )
    }
}
