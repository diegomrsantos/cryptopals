use crate::basics::hex_to_base64::{ConversionError, hex_to_bytes};

use std::collections::HashMap;

// Frequency of letters in English language (in percentage)
const fn get_frequency(c: char) -> f32 {
    match c {
        'a' => 8.2,
        'b' => 1.5,
        'c' => 2.8,
        'd' => 4.3,
        'e' => 13.0,
        'f' => 2.2,
        'g' => 2.0,
        'h' => 6.1,
        'i' => 7.0,
        'j' => 0.15,
        'k' => 0.77,
        'l' => 4.0,
        'm' => 2.4,
        'n' => 6.7,
        'o' => 7.5,
        'p' => 1.9,
        'q' => 0.095,
        'r' => 6.0,
        's' => 6.3,
        't' => 9.1,
        'u' => 2.8,
        'v' => 0.98,
        'w' => 2.4,
        'x' => 0.15,
        'y' => 2.0,
        'z' => 0.074,
        ' ' => 13.0, // Space is very common in English text
        _ => 0.0,    // Return 0.0 for characters not in the frequency list
    }
}


pub fn calculate_score(text: &str) -> f32 {
    let mut score = 0.0;
    let mut letter_counts = HashMap::new();
    let text_len = text.len() as f32;

    for c in text.chars() {
        *letter_counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }

    for (letter, &count) in &letter_counts {
        let expected_freq = get_frequency(*letter);
        if expected_freq > 0.0 {
            let observed_freq = (count as f32) / text_len * 100.0;
            score += 1.0 / ((observed_freq - expected_freq).abs() + 1.0);
        }
    }

    score
}

pub fn best_score_single_byte_xor(hex_text: &str) -> Result<(String, f32), ConversionError> {
    let mut best_score = f32::MIN;
    let mut best_string = String::new();

    let bytes = hex_to_bytes(hex_text)?;
    for codepoint in 0x20..=0x7E {
        let mut candidate = Vec::new();
        let byte = codepoint as u8;
        for &b in &bytes {
            candidate.push(b ^ byte);
        }

        if let Ok(decoded_string) = String::from_utf8(candidate) {
            let score = calculate_score(&decoded_string);
            if score > best_score {
                best_score = score;
                best_string = decoded_string;
            }
        }
    }
    Ok((best_string, best_score))
}

pub fn break_single_byte_xor(ciphertext: &str) -> Result<String, ConversionError> {
    let (best_string, _) = best_score_single_byte_xor(ciphertext)?;
    Ok(best_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte_xor_cypher() {
        let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let decrypted = break_single_byte_xor(encrypted);
        assert_eq!(decrypted.unwrap(), "Cooking MC's like a pound of bacon");
    }
}
