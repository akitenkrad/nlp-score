// use mecab;

pub enum Pos {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    Other,
}

pub struct Token {
    pub surface: String,
    pub stem: String,
    pub pos: Pos,
}

pub trait Tokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token>;
}

pub struct JapaneseTokenizer {
    pub stemming: bool,
    pub stopwords: bool,
    pub max_word_length: u32,
}
