use std::fmt;

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

    // First pass - chars in correct place
    for (i, c) in guess.chars().enumerate() {
        if c.to_lowercase().to_string() == ans[i].to_lowercase().to_string() {
            scores[i] = Score::Here;
            ans[i] = USED_CHAR;
        }
    }

    for (i, c) in guess.chars().enumerate() {
        if scores[i] == Score::Nope {
            if let Some(pos) = ans
                .iter()
                .position(|&v| v.to_lowercase().to_string() == c.to_lowercase().to_string())
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
}

impl fmt::Display for Scored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scores = self.scores.iter().map(|s| s.to_char()).collect::<String>();

        write!(
            f,
            "{{{}, {}, g={}, a={}, {}}}",
            if self.valid { VALID } else { INVALID },
            if self.done { "✅" } else { "❌" },
            self.guess,
            self.answer,
            scores,
        )
    }
}
