mod solver;
mod wordle;

fn main() {
    let mut words = include_str!("../dictionary.txt")
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    let targets = include_str!("../targets.txt")
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    words.append(&mut targets.clone());

    for target in std::env::args().skip(1) {
        let (solution, attempts) = solver::solve(
            &target,
            &words,
            solver::MAX_ATTEMPTS,
            &if target.chars().count() == 5 {
                Some("тоска".to_string())
            } else {
                None
            },
            &targets,
        );
        println!("{solution}\n");
        for (word, matches) in attempts {
            println!("{matches} ||{word}||");
        }
    }
}
