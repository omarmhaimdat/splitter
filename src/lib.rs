use lazy_static::lazy_static;
use std::collections::HashMap;

use pyo3::prelude::*;

lazy_static! {
    static ref COST_DICT: (HashMap<String, f32>, i32) = get_cost_dict();
}

fn lines_from_file() -> Vec<String> {
    let my_str = include_str!("corpus.txt");
    my_str.lines().map(|l| l.to_string()).collect()
}

/// Get the cost dictionary from a list of words
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
            .get(&text[(i - k as i32 - 1) as usize..i as usize].to_string())
            .map_or(f32::MAX, |x| *x);
        array_min.push((c + word_cost, k as f32 + 1.0));
    }
    return array_min
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
}

fn build_cost_array(text_length: u32, text: String, cost: &mut Vec<f32>) {
    for i in 1..(text_length + 1) {
        let (c, _k) = best_match(i as i32, text.clone(), cost);
        cost.push(c);
    }
}

fn minimal_cost(text: String, cost: &mut Vec<f32>, text_length: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut i = text_length;
    while i > 0 {
        let (_c, k) = best_match(i as i32, text.clone(), cost);
        result.push(text[(i - k as u32) as usize..i as usize].to_string());
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
    m.add_function(wrap_pyfunction!(split, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let text = "helloworld";
        let result = split_wrapper(text.to_string());
        assert_eq!(result, "hello world");
    }
}
