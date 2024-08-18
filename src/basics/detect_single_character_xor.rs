use std::{fs, io};
use std::io::BufRead;
use crate::basics::hex_to_base64::ConversionError;
use crate::basics::single_byte_xor_cypher::best_score_single_byte_xor;

fn read_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn detect_single_character_xor(hex_texts: Vec<String>) -> Result<String, ConversionError> {
    let mut best_score = f32::MIN;
    let mut best_string = String::new();

    for line in hex_texts {
        let (best_string_local, best_score_local) = best_score_single_byte_xor(&line)?;
            if best_score_local > best_score {
                best_score = best_score_local;
                best_string = best_string_local.trim_end().to_string();
         }
    }
    Ok(best_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_single_character_xor() {
        let file_path = "src/basics/challenge-data/4.txt";
        let hex_lines = read_lines(file_path).expect("Error reading file");
        assert_eq!(detect_single_character_xor(hex_lines).unwrap(), "Now that the party is jumping");
    }
}
