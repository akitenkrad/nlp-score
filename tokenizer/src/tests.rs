use super::*;
use std::time;

#[test]
fn test_japanese_tokenize_1() {
    let st = time::Instant::now();
    let tokenizer = JapaneseTokenizer::new(10);
    let text = "すもももももももものうち";
    let tokens = tokenizer.tokenize(text.to_string()).unwrap();
    println!("Elapsed: {:?}", st.elapsed());

    println!("{:?}", tokens);
    assert!(tokens.len() > 0);
}
