use rand::{seq::SliceRandom, thread_rng};

mod wordle;

const ATTEMPTS: usize = 6;
const WORD_BYTES: usize = wordle::WORD_LEN * 2; // Cyrillic chars
const BASE_WORDS: &[&'static str] = &[
    "сдоба",
    "белка",
    "нитка",
    "полка",
    "пакет",
    "багет",
    "лимон",
];

fn wordle(target: String, mut words: Vec<String>) -> (bool, Vec<(String, wordle::Check)>) {
    let mut attempts = Vec::new();
    let mut rng = thread_rng();

    while !words.is_empty() {
        let current_word = if attempts.is_empty() {
            BASE_WORDS
                .choose(&mut rng)
                .unwrap() // Unreachable
                .to_string()
        } else {
            words
                .choose(&mut rng) // TODO: smarter choose
                .unwrap() // Unreachable
                .to_string()
        };

        let check = wordle::check_word(&current_word, &target);

        attempts.push((current_word, check.clone()));

        if check.success() {
            return (true, attempts);
        }

        if attempts.len() >= ATTEMPTS {
            return (false, attempts);
        }

        words = words
            .into_iter()
            .filter(|w| wordle::filter_word(w, &attempts))
            .collect();
    }

    (false, attempts)
}

fn main() {
    let target = match std::env::args().nth(1) {
        None => return println!("No word provided"),
        Some(target) if target.len() != WORD_BYTES => return println!("Invalid argument"),
        Some(target) => target,
    };
    let words =
        serde_json::from_str::<Vec<String>>(include_str!("../words.json")).expect("Corrupted json");

    let (success, attempts) = wordle(target, words);

    if success {
        println!("Wordle {}/6", attempts.len());
    } else {
        println!("Wordle x/6");
    }
    println!();
    for (word, result) in attempts {
        println!("{} ||{}||", result.to_string(), word);
    }
}
