use anyhow::Result;
use mecab::mecab_tokenize;

#[cfg(test)]
mod tests;

const JAPANESE_STOPWORDS: &'static str = include_str!("Japanese.txt");

#[derive(Debug, Clone, PartialEq)]
pub enum Pos {
    Noun,
    Verb,
    Adjective,
    Adverb,
    AuxiliaryVerb,
    Pronoun,
    ProperNoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    Filler,
    Other,
}

impl Pos {
    pub fn from_mecab(pos_1: &str, pos_2: &str) -> Pos {
        match pos_1 {
            "名詞" => match pos_2 {
                "一般" => Pos::Noun,
                "固有名詞" => Pos::ProperNoun,
                "代名詞" => Pos::Pronoun,
                _ => Pos::Noun,
            },
            "動詞" => Pos::Verb,
            "形容詞" => Pos::Adjective,
            "副詞" => Pos::Adverb,
            "助詞" => Pos::Preposition,
            "接続詞" => Pos::Conjunction,
            "助動詞" => Pos::AuxiliaryVerb,
            "連体詞" => Pos::Determiner,
            "感動詞" => Pos::Interjection,
            "フィラー" => Pos::Filler,
            _ => Pos::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub surface: String,
    pub stem: String,
    pub pos: Pos,
    pub is_stopword: bool,
}

pub trait Tokenizer {
    fn tokenize(&self, text: String) -> Result<Vec<Token>>;
    fn wakachi(&self, text: String) -> Result<String>;
    fn is_stopword(&self, surface: String, pos: Pos) -> Result<bool>;
}

pub struct JapaneseTokenizer {
    pub max_word_length: usize,
    pub stopwords_list: Vec<String>,
}

impl JapaneseTokenizer {
    pub fn new(max_word_length: usize) -> JapaneseTokenizer {
        let stopwords_list = JAPANESE_STOPWORDS
            .split("\n")
            .map(|s| s.to_string())
            .collect();
        return JapaneseTokenizer {
            max_word_length: max_word_length,
            stopwords_list: stopwords_list,
        };
    }
}

impl Tokenizer for JapaneseTokenizer {
    fn tokenize(&self, text: String) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mecab_tokens = mecab_tokenize(&text);
        for mecab_token in mecab_tokens {
            let token = Token {
                surface: mecab_token.surface.clone(),
                stem: mecab_token.feature.clone(),
                pos: Pos::from_mecab(&mecab_token.pos1.clone(), &mecab_token.pos2.clone()),
                is_stopword: self.is_stopword(
                    mecab_token.surface.clone(),
                    Pos::from_mecab(&mecab_token.pos1.clone(), &mecab_token.pos2.clone()),
                )?,
            };
            tokens.push(token);
        }
        return Ok(tokens);
    }

    fn wakachi(&self, text: String) -> Result<String> {
        let tokens = self.tokenize(text)?;
        let mut wakachi = Vec::new();
        for token in tokens {
            wakachi.push(token.surface);
        }
        return Ok(wakachi.join(" "));
    }

    fn is_stopword(&self, surface: String, pos: Pos) -> Result<bool> {
        let is_containd_in_list = self.stopwords_list.contains(&surface);
        if is_containd_in_list {
            return Ok(true);
        }
        // TODO: Add more rules to determine if a word is a stopword
        let is_stopword = match pos {
            Pos::Noun => false,
            Pos::Verb => false,
            Pos::Adjective => false,
            Pos::Adverb => false,
            Pos::AuxiliaryVerb => false,
            Pos::Pronoun => false,
            Pos::ProperNoun => false,
            Pos::Preposition => false,
            Pos::Conjunction => false,
            Pos::Interjection => false,
            Pos::Determiner => false,
            Pos::Filler => true,
            Pos::Other => true,
        };
        return Ok(is_stopword);
    }
}
