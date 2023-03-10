use std::env;
use std::io;
use std::io::Write;

use rand;
use rand::Rng;

const WORDS: &str = include_str!("./words.txt");

mod scores;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Take each pair of guess, answer and score them

    if args.len() > 0 && args.len() % 2 == 0 {
        for i in 0..(args.len() / 2) {
            let scored = scores::Scored::new(&args[i * 2], &args[i * 2 + 1]);

            println!("scored = {}", scored);
        }
    }

    if args.len() == 0 || args.len() == 1 {
        let answer = if args.len() == 0 {
            println!("\nLet's play!");

            let words: Vec<_> = WORDS.lines().collect();
            let num_words = words.len();
            let mut rng = rand::thread_rng();

            let idx = rng.gen_range(0..num_words);

            words[idx]
        } else {
            &args[0]
        };

        let l: usize = answer.len();

        let mut coloured_guess = String::from("-".repeat(l));

        loop {
            print!("Guess {}: ", coloured_guess);
            if let Ok(_) = io::stdout().flush() {
                let mut guess = String::new();
                if let Ok(_) = io::stdin().read_line(&mut guess) {
                    let scored = scores::Scored::new(&guess[..guess.len() - 1], &answer);

                    coloured_guess = scored.coloured_guess();

                    if scored.done {
                        print!("\nSuccess!: {}\n", coloured_guess);

                        break;
                    }
                }
            }
        }
    }
}
