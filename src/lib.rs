use pyo3::{prelude::*, exceptions::PyNotImplementedError};
use rphonetic::{Encoder, Soundex, DoubleMetaphone, Nysiis, Metaphone, MatchRatingApproach, Caverphone1, Caverphone2, Phonex, Cologne};


fn get_encoder(encoder: &str) -> Result<Box<dyn Encoder>, PyErr> {
    match encoder {
        "soundex" => Ok(Box::new(Soundex::default())),
        "refined_soundex" => Ok(Box::new(Soundex::default())),
        "metaphone" => Ok(Box::new(Metaphone::default())),
        "double_metaphone" => Ok(Box::new(DoubleMetaphone::default())),
        "nysiis" => Ok(Box::new(Nysiis::default())),
        "match_rating" => Ok(Box::new(MatchRatingApproach)),
        "caverphone_1" => Ok(Box::new(Caverphone1)),
        "caverphone_2" => Ok(Box::new(Caverphone2)),
        "phonex" => Ok(Box::new(Phonex::default())),
        "cologne" => Ok(Box::new(Cologne)),
        _ => Err(PyNotImplementedError::new_err("Encoder not implemented")),
    }
}

#[pyfunction]
fn get_encoders() -> PyResult<Vec<String>> {
    let mut encoders: Vec<String> = Vec::new();
    encoders.push("soundex".to_string());
    encoders.push("refined_soundex".to_string());
    encoders.push("metaphone".to_string());
    encoders.push("double_metaphone".to_string());
    encoders.push("nysiis".to_string());
    encoders.push("match_rating".to_string());
    encoders.push("caverphone_1".to_string());
    encoders.push("caverphone_2".to_string());
    encoders.push("phonex".to_string());
    encoders.push("cologne".to_string());
    Ok(encoders)
}


#[pyfunction]
fn encode_words(words: Vec<&str>, encoder: &str) -> PyResult<Vec<String>> {
    let mut encoded_words: Vec<String> = Vec::new();
    let encoder_klass = get_encoder(encoder)?;

    for word in words {
        encoded_words.push(encoder_klass.encode(word));
    }

    Ok(encoded_words)
}

#[pyfunction]
fn encode_word(word: &str, encoder: &str) -> PyResult<String> {
    let encoder_klass = get_encoder(encoder)?;

    Ok(encoder_klass.encode(word))

}

/// A Python module implemented in Rust.
#[pymodule]
fn fast_phonetic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_encoders, m)?)?;
    m.add_function(wrap_pyfunction!(encode_words, m)?)?;
    m.add_function(wrap_pyfunction!(encode_word, m)?)?;
    Ok(())
}