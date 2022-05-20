use std::collections::HashSet;
use std::ops::Index;

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

#[derive(Debug, Clone)]
pub struct CheckResult {
    letters: Vec<Letter>,
}

impl CheckResult {
    pub fn new(letters: Vec<Letter>) -> Self {
        Self { letters }
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
    let target = target.as_slice();
    let mut result = Vec::with_capacity(target.len());
    let mut used = HashSet::new();
    for (i, c) in word.into_iter().enumerate() {
        result.push(if target[i] == c {
            Letter::Green
        } else if !used.contains(&c) && target.contains(&c) {
            Letter::Yellow
        } else {
            Letter::Gray
        });
        used.insert(c);
    }

    CheckResult::new(result)
}

pub fn filter_word(word: &str, results: &Vec<(String, CheckResult)>) -> bool {
    let word = str_to_chars(word);
    let word = word.as_slice();
    for (current, result) in results {
        let current = str_to_chars(current.as_str());
        let current = current.as_slice();
        for (i, letter) in result.letters.iter().enumerate() {
            match letter {
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
    }

    true
}

fn str_to_chars(word: &str) -> Vec<char> {
    word.chars().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{check_word, Letter};

    // ТКАНЬ tested here: https://wordle.belousov.one/?word_id=XgT7TH8clN1

    #[test]
    fn check_word_test() {
        let result = check_word("сдоба", "ткань");
        assert_eq!(
            vec![
                Letter::Gray,
                Letter::Gray,
                Letter::Gray,
                Letter::Gray,
                Letter::Yellow,
            ],
            result.letters
        );
        assert_eq!("⬜️️⬜️️⬜️️⬜️️🟨", result.as_string());
        assert_eq!(false, result.success());
    }

    #[test]
    fn check_second_occurrence_is_gray() {
        let result = check_word("канал", "ткань");
        assert_eq!(
            vec![
                Letter::Yellow,
                Letter::Yellow,
                Letter::Yellow,
                Letter::Gray,
                Letter::Gray,
            ],
            result.letters,
        );
        assert_eq!("🟨🟨🟨⬜️️⬜️️", result.as_string());
        assert_eq!(false, result.success());
        assert_eq!(Letter::Yellow, result[0]);
    }

    // TODO: second letter

    #[test]
    fn check_success() {
        let result = check_word("ткань", "ткань");
        assert_eq!(
            vec![
                Letter::Green,
                Letter::Green,
                Letter::Green,
                Letter::Green,
                Letter::Green,
            ],
            result.letters,
        );
        assert_eq!("🟩🟩🟩🟩🟩", result.as_string());
        assert_eq!(true, result.success());
    }
}
