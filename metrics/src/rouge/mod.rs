#[cfg(test)]
mod tests;

pub struct Rouge {
    pub stemming: bool,
    pub stopwords: bool,
    pub max_word_length: u32,
    pub language: String,
}
