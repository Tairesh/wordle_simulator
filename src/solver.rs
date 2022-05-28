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

pub type Attempt = (&'static str, Matches);
pub type Attempts = Vec<Attempt>;

fn select_next_word(
    words: &[&'static str],
    preferred_words: &[&'static str],
    prioritized_words: &[&'static str],
) -> &'static str {
    let mut frequency_positions: Vec<HashMap<char, usize>> =
        vec![HashMap::with_capacity(32); words[0].chars().count()];
    let mut frequency: HashMap<char, usize> = HashMap::with_capacity(32);
    for w in words.iter() {
        w.chars().enumerate().for_each(|(i, c)| {
            frequency_positions[i].entry(c).or_insert(1).add_assign(1);
            frequency.entry(c).or_insert(1).add_assign(1);
        });
    }

    let mut words_with_score: Vec<(&str, usize)> = words
        .iter()
        .map(|w| {
            let mut score = w
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    (frequency.get(&c).copied().unwrap_or(0)
                        + frequency_positions[i].get(&c).copied().unwrap_or(0))
                        / w.chars().filter(|cc| *cc == c).count()
                })
                .sum();
            if prioritized_words.contains(w) {
                score *= 10;
            }
            if preferred_words.contains(w) {
                score *= 5;
            }
            (*w, score)
        })
        .collect();
    words_with_score.sort_by(|(_, score1), (_, score2)| score1.cmp(score2));

    words_with_score.pop().unwrap().0
}

pub fn solve(
    target: &str,
    words: &[&'static str],
    attempts_limit: usize,
    start_word: Option<&'static str>,
    preferred_words: &[&'static str],
    prioritized_words: &[&'static str],
) -> (Solution, Attempts) {
    let mut attempts = Vec::new();
    let mut words: Vec<&str> = words
        .iter()
        .filter(|w| w.len() == target.len())
        .copied()
        .collect();

    while !words.is_empty() {
        let current_word = if attempts.is_empty() {
            start_word
                .unwrap_or_else(|| select_next_word(&words, preferred_words, prioritized_words))
        } else {
            select_next_word(&words, preferred_words, prioritized_words)
        };

        let diff = wordle::diff(current_word, target);
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
