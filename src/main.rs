use std::env;

mod scores;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Take each pair of guess, answer and score them

    if args.len() % 2 == 0 {
        for i in 0..(args.len() / 2) {
            let scored = scores::Scored::new(&args[i * 2], &args[i * 2 + 1]);

            println!("scored = {}", scored);
        }
    }
}
