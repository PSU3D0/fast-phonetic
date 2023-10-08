use pyo3::prelude::*;
use rphonetic::{Encoder, Soundex, DoubleMetaphone, Nysiis, Metaphone};


#[pyfunction]
fn encode_words(words: Vec<&str>, encoder: &str) -> PyResult<Vec<String>> {
    let mut encoded_words: Vec<String> = Vec::new();
    match encoder {
        "soundex" => {
            let soundex = Soundex::default();
            for word in words {
                encoded_words.push(soundex.encode(word));
            }
        }
        "metaphone" => {
            let metaphone = Metaphone::default();
            for word in words {
                encoded_words.push(metaphone.encode(word));
            }
        }
        "double_metaphone" => {
            let double_metaphone = DoubleMetaphone::default();
            for word in words {
                encoded_words.push(double_metaphone.encode(word));
            }
        }
        "nysiis" => {
            let nysiis = Nysiis::default();
            for word in words {
                encoded_words.push(nysiis.encode(word));
            }
        }
        _ => {
            let soundex = Soundex::default();
            for word in words {
                encoded_words.push(soundex.encode(word));
            }
        }
    }
    Ok(encoded_words)
}

#[pyfunction]
fn encode_word(word: &str, encoder: &str) -> PyResult<String> {
    match encoder {
        "soundex" => {
            let soundex = Soundex::default();
            Ok(soundex.encode(word))
        }
        "metaphone" => {
            let metaphone = Metaphone::default();
            Ok(metaphone.encode(word))
        }
        "double_metaphone" => {
            let double_metaphone = DoubleMetaphone::default();
            Ok(double_metaphone.encode(word))
        }
        "nysiis" => {
            let nysiis = Nysiis::default();
            Ok(nysiis.encode(word))
        }
        _ => {
            let soundex = Soundex::default();
            Ok(soundex.encode(word))
        }
    }
}


#[pyfunction]
fn encode_word_soundex(word: &str) -> PyResult<String> {
    let soundex = Soundex::default();
    Ok(soundex.encode(word))
}

#[pyfunction]
fn encode_word_metaphone(word: &str) -> PyResult<String> {
    let metaphone = Metaphone::default();
    Ok(metaphone.encode(word))
}


#[pyfunction]
fn encode_word_double_metaphone(word: &str) -> PyResult<String> {
    let double_metaphone = DoubleMetaphone::default();
    Ok(double_metaphone.encode(word))
}

#[pyfunction]
fn encode_word_nysiis(word: &str) -> PyResult<String> {
    let nysiis = Nysiis::default();
    Ok(nysiis.encode(word))
}

/// A Python module implemented in Rust.
#[pymodule]
fn fast_phonetic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_words, m)?)?;
    m.add_function(wrap_pyfunction!(encode_word, m)?)?;
    m.add_function(wrap_pyfunction!(encode_word_soundex, m)?)?;
    m.add_function(wrap_pyfunction!(encode_word_metaphone, m)?)?;
    m.add_function(wrap_pyfunction!(encode_word_double_metaphone, m)?)?;
    m.add_function(wrap_pyfunction!(encode_word_nysiis, m)?)?;
    Ok(())
}