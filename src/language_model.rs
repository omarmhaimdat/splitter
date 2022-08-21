use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use pyo3::prelude::*;

#[pyclass]
pub struct LanguageModel {
    pub corpus_path: String,
    pub cost_dict: Option<(HashMap<String, f32>, i32)>,
}

#[pymethods]
impl LanguageModel {
    #[new]
    pub fn new(corpus_path: String) -> LanguageModel {
        LanguageModel {
            corpus_path,
            cost_dict: None,
        }
    }

    fn best_match(&mut self, i: i32, text: String, cost: Vec<f32>) -> (f32, f32) {
        if self.cost_dict.is_none() {
            self.cost_dict = Some(self.set_cost_dict());
        }
        let cost_dict = self.cost_dict.as_ref().unwrap();
        let max = vec![0, i - cost_dict.1].into_iter().max().unwrap() as usize;
        let mut slice: Vec<f32> = cost[max..i as usize].to_vec();
        slice.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let mut array_min: Vec<(f32, f32)> = Vec::new();
        for (k, c) in slice.iter().enumerate() {
            let word_cost = cost_dict
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
    fn build_cost_array(&mut self, text_length: u32, text: String, cost: Vec<f32>) -> Vec<f32> {
        let mut cost = cost.clone();
        for i in 1..(text_length + 1) {
            let (c, _k) = self.best_match(i as i32, text.clone(), cost.clone());
            cost.push(c);
        }
        return cost;
    }

    /// Return a Vec containing all the words in the corpus
    fn lines_from_file(&mut self) -> Vec<String> {
        // Read corpus file and split into lines
        let file = File::open(&self.corpus_path).unwrap();
        let buf_reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();
        for line in buf_reader.lines() {
            lines.push(line.unwrap());
        }
        return lines;
    }

    /// Calculate the optimal cost of a text
    /// # Arguments
    /// * `text` - The text to calculate the cost of
    /// * `cost` - The cost of each word in the corpus
    /// * `text_length` - The length of the text
    /// # Returns
    /// A Vec of strings containing the minimum costing words
    fn minimal_cost(&mut self, text: String, cost: Vec<f32>, text_length: u32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut i = text_length;
        while i > 0 {
            let (_c, k) = self.best_match(i as i32, text.clone(), cost.clone());
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

    /// Calculate the cost of each word in the corpus
    /// Return a Tuple containing a HashMap of words and their costs
    /// as values and the maximum cost as second value
    fn set_cost_dict(&mut self) -> (HashMap<String, f32>, i32) {
        let mut dict = HashMap::new();
        let words = self.lines_from_file();
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
    pub fn split_wrapper(&mut self, text: String) -> String {
        let mut cost: Vec<f32> = Vec::new();
        cost.push(0.0);
        let text_length = text.chars().count() as u32;
        let processed_cost = self.build_cost_array(text_length, text.clone(), cost);
        let texts = self.minimal_cost(text.clone(), processed_cost, text_length);
        return texts.into_iter().rev().collect::<Vec<String>>().join(" ");
    }

    pub fn split(&mut self, text: String) -> PyResult<String> {
        Ok(self.split_wrapper(text))
    }
}
