use super::*;
use std::time;

#[test]
fn test_sample() {
    let path = std::path::Path::new(".");
    println!("{:?}", path.display());
}

#[test]
fn test_mecab_tokenize_1() {
    let st = time::Instant::now();
    let tokens = mecab_tokenize("すもももももももものうち");
    println!("Elapsed: {:?}", st.elapsed());
    assert!(tokens.len() > 0);
}

#[test]
fn test_mecab_tokenize_2() {
    let st = time::Instant::now();
    let tokens = mecab_tokenize("今日は晴れです。");
    assert!(tokens.len() > 0);
    println!("Elapsed: {:?}", st.elapsed());
}

#[test]
fn test_mecab_tokenize_3() {
    let st = time::Instant::now();
    let tokens = mecab_tokenize("いつも自分はこうやって，何かが起こるのを待っている．");
    assert!(tokens.len() > 0);
    println!("{:?}", tokens);
    println!("Elapsed: {:?}", st.elapsed());
}
