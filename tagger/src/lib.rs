use std::collections::{HashMap, HashSet};

use anyhow::Result;

#[cfg(test)]
mod tests;

pub trait Tagger {
    fn tag(&self, tokens: &String) -> Result<Vec<String>>;
    fn tag_sentences(&self, sentences: &Vec<String>) -> Result<Vec<Vec<String>>> {
        let mut tagged_sentences = Vec::new();
        for sentence in sentences {
            let tagged = self.tag(sentence)?;
            tagged_sentences.push(tagged);
        }
        Ok(tagged_sentences)
    }
}

pub struct AveragePerceptron {
    pub weights: HashMap<String, HashMap<String, f64>>,
    pub classes: HashSet<String>,
    pub _totals: HashMap<(String, String), isize>,
    pub _timestamps: HashMap<(String, String), isize>,
    pub i: isize,
}

impl AveragePerceptron {
    pub fn new() -> Self {
        Self {
            weights: HashMap::new(),
            classes: HashSet::new(),
            _totals: HashMap::new(),
            _timestamps: HashMap::new(),
            i: 0,
        }
    }

    pub fn _softmax(&self, scores: &HashMap<String, f64>) -> Result<Vec<f64>> {
        let exps = scores.values().map(|v| v.exp()).collect::<Vec<f64>>();
        let sum = scores
            .values()
            .collect::<Vec<&f64>>()
            .into_iter()
            .sum::<f64>();
        let values = exps.iter().map(|v| v.clone() / sum).collect::<Vec<f64>>();
        return Ok(values);
    }

    pub fn predict(&self, features: &HashMap<String, f64>) -> Result<(String, f64)> {
        let mut scores: HashMap<String, f64> = HashMap::new();
        for (feat, value) in features.iter() {
            if !self.weights.contains_key(feat) || value == &0.0 {
                continue;
            }
            let weights = self.weights.get(feat).unwrap();
            for (label, weght) in weights.iter() {
                if !scores.contains_key(label) {
                    scores.insert(label.clone(), 0.0);
                }
                let score = scores.get_mut(label).unwrap();
                *score += value * weght;
            }
        }
        let best_label = scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        let conf = self
            ._softmax(&scores)?
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        return Ok((best_label.0.clone(), conf));
    }
}
