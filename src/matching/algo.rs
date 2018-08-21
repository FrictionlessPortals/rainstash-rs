/// Implementation of the fuzzy matching algorithms.
/// This code was modified from - https://github.com/batuwa/fuzzymatch
use std::cmp::max;

pub fn fuzzy_match_simple(pattern: &str, document: &str) -> bool {
    if pattern.len() == 0 || document.len() == 0 {
        return false;
    }

    let mut pattern_chars_lower = pattern.chars().flat_map(char::to_lowercase);
    let mut doc_chars_lower = document.chars().flat_map(char::to_lowercase);

    'outer: for pattern_ch in &mut pattern_chars_lower {
        'inner: for doc_ch in &mut doc_chars_lower {
            if pattern_ch == doc_ch {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

pub fn fuzzy_match(pattern: &str, document: &str) -> (bool, i32, Vec<usize>) {
    if pattern.len() == 0 || document.len() == 0 {
        return (false, 0, vec![]);
    }

    // Score factors
    let adjacency_bonus: i32 = 5;                   // bonus for adjacent matches
    let separator_bonus: i32 = 10;                  // bonus if match occurs after a separator
    let camel_bonus: i32 = 10;                      // bonus if match is uppercase and prev is lower
    let leading_letter_penalty: i32 = -3;           // penalty applied for every letter in str before the first match
    let max_leading_letter_penalty: i32 = -9;      // maximum penalty for leading letters
    let unmatched_letter_penalty: i32 = -1;        // penalty for every letter that doesn't matter

    // Loop variables
    let mut score: i32 = 0;
    let mut prev_matched = false;
    let mut prev_lower = false;
    let mut prev_separator = true;             // true here so if first letter matches it gets separator bonus

    // Use "best" matched letter if multiple string letters match the pattern
    let mut best_letter = String::new();
    let mut best_lower = String::new();
    let mut best_letter_idx: usize = 0;
    let mut best_letter_score = 0;

    let mut matched_indices = Vec::new();    // To enable highlighting to matched characters
    let mut pattern_idx_max: usize = 0;
    let mut doc_chars = document.chars();
    let mut doc_idx: usize = 0;

    // Loop through each character of document string to calculate score
    'outer: for (pattern_idx, pattern_ch) in pattern.chars().enumerate() {
        let pattern_ch_lower: String = pattern_ch.to_lowercase().collect();

        'inner: for doc_ch in &mut doc_chars {
            let doc_ch_lower: String = doc_ch.to_lowercase().collect();
            let doc_ch_upper: String = doc_ch.to_uppercase().collect();

            let mut next_match = false;
            let mut rematch = false;
            let mut advanced = false;
            let mut pattern_repeat = false;

            if pattern_idx_max < pattern.len() {
                next_match = pattern_ch_lower == doc_ch_lower;
            }

            if best_letter.len() > 0 {
                rematch = best_lower == doc_ch_lower;
                advanced = next_match;

                if pattern_idx_max < pattern.len() {
                    pattern_repeat = best_lower == pattern_ch_lower;
                }
            }

            if advanced || pattern_repeat {
                score += best_letter_score;
                matched_indices.push(best_letter_idx);
                best_letter = String::new();
                best_lower = String::new();
                best_letter_idx = 0;
                best_letter_score = 0;
            }

            if next_match || rematch {
                let mut new_score = 0;
                // Apply penalty for each letter before the first match
                // using max because penalties are negative (so max = smallest)
                if pattern_idx == 0 {
                    score += max((doc_idx as i32) * leading_letter_penalty, max_leading_letter_penalty);
                }

                // Apply bonus for consecutive matches
                if prev_matched {
                    new_score += adjacency_bonus;
                }

                // Apply bonus for matches after a separator
                if prev_separator {
                    new_score += separator_bonus;
                }

                // Apply bonus across camelCase boundaries
                if prev_lower && (doc_ch.to_string() == doc_ch_upper) && (doc_ch_lower != doc_ch_upper) {
                    new_score += camel_bonus;
                }

                // Update best letter match (may be next or rematch)
                if new_score >= best_letter_score {
                    // Apply penalty for now-skipped letter
                    if best_letter.len() > 0 {
                        score += unmatched_letter_penalty;
                    }

                    best_letter = doc_ch.to_string();
                    best_lower = best_letter.to_lowercase();
                    best_letter_score = new_score;
                    if pattern_idx_max < pattern.len() {
                        best_letter_idx = doc_idx;
                    }
                }
                prev_matched = true;

                // Update pattern index if the next pattern letter was matched
                if next_match {
                    pattern_idx_max +=1;
                    prev_lower = doc_ch.to_string() == doc_ch_lower && doc_ch_lower != doc_ch_upper;
                    prev_separator = doc_ch == '_' || doc_ch == ' ';

                    if pattern_idx_max < pattern.len() {
                        doc_idx += 1;
                        continue 'outer;
                    }
                }
            }
                else {
                    score += unmatched_letter_penalty;
                    prev_matched = false;
                }

            prev_lower = doc_ch.to_string() == doc_ch_lower && doc_ch_lower != doc_ch_upper;
            prev_separator = doc_ch == '_' || doc_ch == ' ';

            doc_idx += 1;
        }
    }

    // Apply score for last match
    if best_letter.len() > 0 {
        score += best_letter_score;
        matched_indices.push(best_letter_idx);
    }

    return (pattern_idx_max == pattern.len(), score, matched_indices);
}