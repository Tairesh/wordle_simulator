mod wordle;

extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let words: Vec<String> = serde_json::from_str(include_str!("../words.json")).unwrap();
    let mut words: Vec<String> = words
        .into_iter()
        .filter(|w| w.len() == wordle::WORD_LEN * 2)
        .collect();
    let target = "ткань";
    let mut results = Vec::new();
    let mut rng = thread_rng();

    while !words.is_empty() {
        let current_word = if results.is_empty() {
            String::from(
                *[
                    "сдоба",
                    "белка",
                    "нитка",
                    "полка",
                    "пакет",
                    "багет",
                    "лимон",
                ]
                .choose(&mut rng)
                .unwrap(),
            )
        } else {
            words.choose(&mut rng).unwrap().clone()
        };

        let result = wordle::check_word(current_word.as_str(), target);
        results.push((current_word.clone(), result.as_string()));
        if result.success() {
            break;
        }

        words = words
            .into_iter()
            .filter(|w| wordle::filter_word(w, current_word.as_str(), &result))
            .collect();
    }

    for (word, result) in results {
        println!("{} {}", word, result);
    }
}
