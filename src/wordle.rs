use std::collections::HashSet;
use std::ops::Index;

pub const WORD_LEN: usize = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Letter {
    Green,
    Yellow,
    Gray,
}

impl From<Letter> for &str {
    fn from(r: Letter) -> Self {
        match r {
            Letter::Green => "🟩",
            Letter::Yellow => "🟨",
            Letter::Gray => "⬜️️",
        }
    }
}

pub struct CheckResult {
    letters: [Letter; WORD_LEN],
}

impl CheckResult {
    pub fn new(raw: [Letter; WORD_LEN]) -> Self {
        Self { letters: raw }
    }

    pub fn as_string(&self) -> String {
        let mut s = String::new();
        for r in self.letters.iter().copied() {
            s.push_str(r.into());
        }
        s
    }

    pub fn success(&self) -> bool {
        self.letters.iter().all(|r| matches!(r, Letter::Green))
    }
}

impl Index<usize> for CheckResult {
    type Output = Letter;

    fn index(&self, index: usize) -> &Self::Output {
        &self.letters[index]
    }
}

pub fn check_word(word: &str, target: &str) -> CheckResult {
    let word = str_to_chars(word);
    let target = str_to_chars(target);
    let mut result = [
        Letter::Gray,
        Letter::Gray,
        Letter::Gray,
        Letter::Gray,
        Letter::Gray,
    ];
    let mut used = HashSet::new();
    for (i, c) in word.iter().enumerate() {
        result[i] = if target[i] == *c {
            Letter::Green
        } else if !used.contains(c) && target.contains(c) {
            Letter::Yellow
        } else {
            Letter::Gray
        };
        used.insert(*c);
    }

    CheckResult::new(result)
}

pub fn filter_word(word: &str, current: &str, result: &CheckResult) -> bool {
    let current = str_to_chars(current);
    let word = str_to_chars(word);
    for i in 0..WORD_LEN {
        match result[i] {
            Letter::Green => {
                if word[i] != current[i] {
                    return false;
                }
            }
            Letter::Yellow => {
                if !word.contains(&current[i]) || word[i] == current[i] {
                    return false;
                }
            }
            Letter::Gray => {
                if word[i] == current[i] {
                    return false;
                }
            }
        }
    }

    true
}

fn str_to_chars(word: &str) -> [char; WORD_LEN] {
    word.chars()
        .into_iter()
        .take(WORD_LEN)
        .collect::<Vec<char>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{check_word, CheckResult, Letter};

    // ТКАНЬ tested here: https://wordle.belousov.one/?word_id=XgT7TH8clN1

    #[test]
    fn check_word_test() {
        let result = check_word("сдоба", "ткань");
        assert!(matches!(
            result,
            CheckResult {
                letters: [
                    Letter::Gray,
                    Letter::Gray,
                    Letter::Gray,
                    Letter::Gray,
                    Letter::Yellow,
                ]
            }
        ));
        assert_eq!("⬜️️⬜️️⬜️️⬜️️🟨", result.as_string());
        assert_eq!(false, result.success());
    }

    #[test]
    fn check_second_occurrence_is_gray() {
        let result = check_word("канал", "ткань");
        assert!(matches!(
            result,
            CheckResult {
                letters: [
                    Letter::Yellow,
                    Letter::Yellow,
                    Letter::Yellow,
                    Letter::Gray,
                    Letter::Gray,
                ]
            }
        ));
        assert_eq!("🟨🟨🟨⬜️️⬜️️", result.as_string());
        assert_eq!(false, result.success());
        assert_eq!(Letter::Yellow, result[0]);
    }

    #[test]
    fn check_success() {
        let result = check_word("ткань", "ткань");
        assert!(matches!(
            result,
            CheckResult {
                letters: [
                    Letter::Green,
                    Letter::Green,
                    Letter::Green,
                    Letter::Green,
                    Letter::Green,
                ]
            }
        ));
        assert_eq!("🟩🟩🟩🟩🟩", result.as_string());
        assert_eq!(true, result.success());
    }
}
