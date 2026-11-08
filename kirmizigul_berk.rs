// Homework 3 — Word count with HashMap
// Author: Berk Kirmizigul

use std::collections::HashMap;

/// 1. Counts how many times each word appears in `text`.
/// Words are separated by whitespace; case does not matter.
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// 2. Returns how many times `word` appears (0 if it never appears).
fn count_of(counts: &HashMap<String, usize>, word: &str) -> usize {
    *counts.get(&word.to_lowercase()).unwrap_or(&0)
}

/// 3. Returns the total number of words.
fn total_words(counts: &HashMap<String, usize>) -> usize {
    counts.values().sum()
}

/// 4. Returns (word, count) pairs: most common first,
/// ties broken alphabetically.
fn sorted_words(counts: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut list: Vec<(String, usize)> = counts
        .iter()
        .map(|(word, &count)| (word.clone(), count))
        .collect();
    list.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    list
}

fn main() {
    let texts = ["the cat The dog THE end", "Rust is fun and Rust is fast", ""];

    for text in texts {
        let counts = count_words(text);
        println!("Text:   {:?}", text);
        println!("Total:  {}", total_words(&counts));
        println!("Sorted: {:?}", sorted_words(&counts));
        println!();
    }

    let counts = count_words("the cat The dog THE end");
    println!("count_of(\"the\")   = {}", count_of(&counts, "the"));
    println!("count_of(\"horse\") = {}", count_of(&counts, "horse"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs(v: &[(&str, usize)]) -> Vec<(String, usize)> {
        v.iter().map(|(w, c)| (w.to_string(), *c)).collect()
    }

    #[test]
    fn example_text() {
        let counts = count_words("the cat The dog THE end");
        assert_eq!(count_of(&counts, "the"), 3);
        assert_eq!(count_of(&counts, "cat"), 1);
        assert_eq!(count_of(&counts, "horse"), 0);
        assert_eq!(total_words(&counts), 6);
        assert_eq!(
            sorted_words(&counts),
            pairs(&[("the", 3), ("cat", 1), ("dog", 1), ("end", 1)])
        );
    }

    #[test]
    fn rust_text() {
        let counts = count_words("Rust is fun and Rust is fast");
        assert_eq!(total_words(&counts), 7);
        assert_eq!(
            sorted_words(&counts),
            pairs(&[("is", 2), ("rust", 2), ("and", 1), ("fast", 1), ("fun", 1)])
        );
    }

    #[test]
    fn empty_text() {
        let counts = count_words("");
        assert!(counts.is_empty());
        assert_eq!(total_words(&counts), 0);
        assert!(sorted_words(&counts).is_empty());
    }
}
