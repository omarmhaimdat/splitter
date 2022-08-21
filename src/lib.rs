mod language_model;

use lazy_static::lazy_static;
use std::collections::HashMap;

use pyo3::prelude::*;

lazy_static! {
    static ref COST_DICT: (HashMap<String, f32>, i32) = get_cost_dict();
}

/// Return a Vec containing all the words in the corpus
fn lines_from_file() -> Vec<String> {
    let my_str = include_str!("corpus.txt");
    my_str.lines().map(|l| l.to_string()).collect()
}

/// Calculate the cost of each word in the corpus
/// Return a Tuple containing a HashMap of words and their costs
/// as values and the maximum cost as second value
fn get_cost_dict() -> (HashMap<String, f32>, i32) {
    let mut dict = HashMap::new();
    let words = lines_from_file();
    let words_length = words.len() as f32;
    let mut max_word = 0;
    for (idx, word) in words.iter().enumerate() {
        let a = (idx + 1) as f32;
        let c = a * words_length.ln();
        let z = c.ln();
        dict.insert(word.to_string(), z);
    }
    words.iter().for_each(|word| {
        let word_cost = word.chars().count() as i32;
        if word_cost > max_word {
            max_word = word_cost;
        }
    });
    return (dict, max_word);
}

fn best_match(i: i32, text: String, cost: &mut Vec<f32>) -> (f32, f32) {
    let max = vec![0, i - COST_DICT.1].into_iter().max().unwrap() as usize;
    let mut slice: Vec<f32> = cost[max..i as usize].to_vec();
    slice.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut array_min: Vec<(f32, f32)> = Vec::new();
    for (k, c) in slice.iter().enumerate() {
        let word_cost = COST_DICT
            .0
            .get(
                &text[(i - k as i32 - 1) as usize..i as usize]
                    .to_string()
                    .to_lowercase(),
            )
            .map_or(f32::MAX, |x| *x);
        array_min.push((c + word_cost, k as f32 + 1.0));
    }
    return array_min
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
}

/// Calculate the best match for a given text
/// # Arguments
/// * `text` - The text to be matched
/// * `cost` - The cost of each word in the corpus
/// * `text_length` - The length of the text
/// # Returns
/// A Vec of f32 containing the best match costs for each word in the corpus
fn build_cost_array(text_length: u32, text: String, cost: &mut Vec<f32>) {
    for i in 1..(text_length + 1) {
        let (c, _k) = best_match(i as i32, text.clone(), cost);
        cost.push(c);
    }
}

/// Calculate the optimal cost of a text
/// # Arguments
/// * `text` - The text to calculate the cost of
/// * `cost` - The cost of each word in the corpus
/// * `text_length` - The length of the text
/// # Returns
/// A Vec of strings containing the minimum costing words
fn minimal_cost(text: String, cost: &mut Vec<f32>, text_length: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut i = text_length;
    while i > 0 {
        let (_c, k) = best_match(i as i32, text.clone(), cost);
        let mut new_token: bool = true;
        if text[(i - k as u32) as usize..i as usize] != "'".to_string() {
            let result_length = result.len();
            if result_length > 0 {
                if &result[&result.len() - 1] == &"'s".to_string()
                    || (text[(i - k as u32) as usize..i as usize]
                        .to_string()
                        .chars()
                        .next()
                        .unwrap()
                        .is_digit(10)
                        && result[result.len() - 1]
                            .chars()
                            .next()
                            .unwrap()
                            .is_digit(10))
                {
                    let mut test = text[(i - k as u32) as usize..i as usize].to_string();
                    test.push_str(&result[result_length - 1].to_string());
                    result[result_length - 1] = test;
                    new_token = false;
                }
            }
        }
        if new_token {
            result.push(text[(i - k as u32) as usize..i as usize].to_string());
        }
        i -= k as u32;
    }
    return result;
}

/// A wrapper function for the split function
/// # Arguments
/// * `text` - The text to be split
/// # Returns
/// A String object containing the split text
/// # Examples
/// ```
/// use split_rust::split_wrapper;
/// let text = "Thequickbrownfoxjumpsoverthelazydog";
/// split_wrapper(text);
/// ```
/// Result: "The quick brown fox jumps over the lazy dog"
fn split_wrapper(text: String) -> String {
    let mut cost: Vec<f32> = Vec::new();
    cost.push(0.0);
    let text_length = text.chars().count() as u32;
    build_cost_array(text_length, text.clone(), &mut cost);
    let texts = minimal_cost(text.clone(), &mut cost, text_length);
    return texts.into_iter().rev().collect::<Vec<String>>().join(" ");
}

/// A wrapper function for the split function
/// # Arguments
/// * `text` - The text to be split
/// # Returns
/// A String object containing the split text
/// # Examples
/// ```
/// import rsplitter
/// text = "Thequickbrownfoxjumpsoverthelazydog";
/// rsplitter.split(text);
/// ```
/// Result: "The quick brown fox jumps over the lazy dog"
#[pyfunction]
fn split(text: String) -> PyResult<String> {
    Ok(split_wrapper(text))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rsplitter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<language_model::LanguageModel>()?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let text = "Thequickbrownfoxjumpsoverthelazydog";
        let result = split_wrapper(text.to_string());
        assert_eq!(result, "The quick brown fox jumps over the lazy dog");
    }

    #[test]
    fn test_split() {
        let text = "Thequickbrownfoxjumpsoverthelazydog";
        let mut language_model = language_model::LanguageModel {
            corpus_path: "/Users/omarmhaimdat/Documents/splitter/src/corpus.txt".to_string(),
            cost_dict: None,
        };
        let result = language_model.split_wrapper(text.to_string());
        assert_eq!(result, "The quick brown fox jumps over the lazy dog");
    }
}
