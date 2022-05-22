use crate::wordle::{self, Matches};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;

pub const MAX_ATTEMPTS: usize = 6;

#[derive(Debug)]
pub enum Solution {
    Success(usize),
    Failure,
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Success(tries) => {
                f.write_str(format!("Wordle {tries}/{MAX_ATTEMPTS}").as_str())
            }
            Solution::Failure => f.write_str(format!("Wordle x/{MAX_ATTEMPTS}").as_str()),
        }
    }
}

pub type Attempt = (String, Matches);
pub type Attempts = Vec<Attempt>;

fn select_next_word(words: &[String], preferred_words: &[String]) -> String {
    let mut frequency: HashMap<char, usize> = HashMap::with_capacity(32);
    for w in words.iter() {
        w.chars().for_each(|c| {
            frequency.entry(c).or_insert(1).add_assign(1);
        });
    }

    let mut words_with_score: Vec<(&String, usize)> = words
        .iter()
        .map(|w| {
            let mut score = w
                .chars()
                .map(|c| {
                    frequency.get(&c).copied().unwrap_or(0)
                        / w.chars().filter(|cc| *cc == c).count()
                })
                .sum();
            if preferred_words.contains(w) {
                score *= 2;
            }
            (w, score)
        })
        .collect();
    words_with_score.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));

    words_with_score.pop().unwrap().0.clone()
}

pub fn solve(
    target: &str,
    words: &[String],
    attempts_limit: usize,
    start_word: &Option<String>,
    preferred_words: &[String],
) -> (Solution, Attempts) {
    let mut attempts = Vec::new();
    let mut words: Vec<String> = words
        .iter()
        .filter(|w| w.len() == target.len())
        .cloned()
        .collect();

    while !words.is_empty() {
        let current_word = if attempts.is_empty() {
            start_word
                .as_ref()
                .cloned()
                .unwrap_or_else(|| select_next_word(&words, preferred_words))
        } else {
            select_next_word(&words, preferred_words)
        };

        let diff = wordle::diff(current_word.as_str(), target);
        attempts.push((current_word, diff.clone()));

        if diff.success() {
            return (Solution::Success(attempts.len()), attempts);
        }
        if attempts.len() == attempts_limit {
            return (Solution::Failure, attempts);
        }

        words = words
            .into_iter()
            .filter(|w| wordle::filter_word(w, &attempts))
            .collect();
    }

    (Solution::Failure, attempts)
}
