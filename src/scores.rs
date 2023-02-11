use std::fmt;

const USED_CHAR: char = ' ';

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Score {
    Here,
    Somewhere,
    Nope,
}

#[readonly::make]
pub struct Scored {
    pub valid: bool,
    pub done: bool,
    pub input: String,
    pub answer: String,
    pub scores: Vec<Score>,
}

impl Score {
    pub fn to_char(&self) -> char {
        match self {
            Score::Here => '🟢',
            Score::Somewhere => '🟡',
            Score::Nope => '⚫',
        }
    }
}

fn calc_scores(input: &str, answer: &str, scores: &mut Vec<Score>) -> bool {
    let mut ans: Vec<char> = answer.chars().collect();

    // First pass - chars in correct place
    for (i, c) in input.char_indices() {
        if c == ans[i] {
            scores[i] = Score::Here;
            ans[i] = USED_CHAR;
        }
    }

    for (i, c) in input.char_indices() {
        if scores[i] == Score::Nope {
            if let Some(pos) = ans.iter().position(|&v| v == c) {
                scores[i] = Score::Somewhere;
                ans[pos] = USED_CHAR;
            }
        }
    }

    scores.iter().all(|&v| v == Score::Here)
}

impl Scored {
    pub fn new(input: &str, answer: &str) -> Self {
        let valid = input.len() == answer.len();
        let mut scores = vec![Score::Nope; std::cmp::max(input.len(), answer.len())];

        let mut done = false;

        if valid {
            done = calc_scores(&input, &answer, &mut scores);
        }

        Scored {
            valid,
            done,
            input: String::from(input),
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
            "{{{}, {}, {}, {}, {}}}",
            if self.valid { "OK" } else { "INVALID" },
            if self.done { "✅" } else { "❌" },
            self.input,
            self.answer,
            scores,
        )
    }
}
