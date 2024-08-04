use std::fmt;
/// Custom error type for hex and Base64 conversion.
#[derive(Debug, PartialEq)]
pub enum ConversionError {
    OddLength,
    InvalidHexCharacter(char),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::OddLength => write!(f, "Hex string has an odd length"),
            ConversionError::InvalidHexCharacter(c) => write!(f, "Invalid hex character: {}", c),
        }
    }
}

impl std::error::Error for ConversionError {}

const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// Function to convert a hex string to a Base64 string
pub fn hex_to_base64(hex: &str) -> Result<String, ConversionError> {
    let bytes = hex_to_bytes(hex)?;
    Ok(bytes_to_base64(&bytes))
}

// Function to convert a hex string to a vector of bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ConversionError> {
    // Check if the length of the hex string is odd
    if hex.len() % 2 != 0 {
        return Err(ConversionError::OddLength);
    }

    // Convert the hex string to bytes
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            let hex_pair = &hex[i..i + 2];
            u8::from_str_radix(hex_pair, 16)
                .map_err(|_| ConversionError::InvalidHexCharacter(hex_pair.chars().find(|c| !c.is_digit(16)).unwrap()))
        })
        .collect()
}

// Function to convert a vector of bytes to a Base64 string
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let mut base64_string = String::new();
    let mut buffer: u32 = 0;
    let mut bits_in_buffer = 0;

    for &byte in bytes {
        buffer = buffer << 8 | (byte as u32);
        bits_in_buffer += 8;

        while bits_in_buffer >= 6 {
            // What is left is the number of bits we ignore for now and will be used in the next iteration
            bits_in_buffer -= 6;
            // Ignore bits_in_buffer bits from the left and take the last 6 bits, & 0x3F ignores the most significant bits
            // and keeps the least significant 6 bits.
            let index: usize = ((buffer >> bits_in_buffer) & 0x3F) as usize;
            // Then use this as an index to get the corresponding character from BASE64_CHARS
            base64_string.push(BASE64_CHARS.chars().nth(index).unwrap());
        }

    }

    // If there are bits left in the buffer, we need to pad the buffer with 0s to make it a multiple of 6
    if bits_in_buffer > 0 {
        buffer <<= 6 - bits_in_buffer;
        let index: usize = (buffer & 0x3F) as usize;
        base64_string.push(BASE64_CHARS.chars().nth(index).unwrap());
    }

    // Add padding to make the length of the result a multiple of 4
    while base64_string.len() % 4 != 0 {
        base64_string.push('=');
    }

    base64_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64_conversion() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex_string).unwrap(), base64_string);
    }

    #[test]
    fn test_hex_to_base64_empty_string() {
        let hex_string = "";
        let base64_string = "";
        assert_eq!(hex_to_base64(hex_string).unwrap(), base64_string);
    }

    #[test]
    fn test_hex_to_base64_single_byte() {
        let hex_string = "4d";
        let base64_string = "TQ==";
        assert_eq!(hex_to_base64(hex_string).unwrap(), base64_string);
    }

    #[test]
    fn test_hex_to_base64_two_bytes() {
        let hex_string = "4d61";
        let base64_string = "TWE=";
        assert_eq!(hex_to_base64(hex_string).unwrap(), base64_string);
    }

    #[test]
    fn test_hex_to_bytes_invalid_hex() {
        let hex_string = "4d6g";
        let result = hex_to_bytes(hex_string);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid hex character: g");
    }

    #[test]
    fn test_hex_to_bytes_odd_length() {
        let hex_string = "4d6";
        assert_eq!(hex_to_bytes(hex_string).unwrap_err(), ConversionError::OddLength);
    }

    #[test]
    fn test_bytes_to_base64_padding() {
        let bytes = [77, 97, 110]; // "Man" in ASCII
        let base64_string = "TWFu";
        // "Man" in ASCII is [77, 97, 110]. Encoded as:
        // 'M' -> "01001101", 'a' -> "01100001", 'n' -> "01101110"
        // Combined -> "010011010110000101101110"
        // Split into 6-bit chunks -> "010011" "010110" "000101" "101110"
        // Mapped to Base64 -> "T" "W" "F" "u"
        assert_eq!(bytes_to_base64(&bytes), base64_string);

        let bytes = [77]; // "M" in ASCII
        let base64_string = "TQ==";
        // "M" in ASCII is [77]. Encoded as:
        // 'M' -> "01001101"
        // Combined -> "01001101"
        // Split into 6-bit chunks -> "010011" "010000"
        // Mapped to Base64 -> "T" "Q"
        // Padding needed to make length multiple of 4 -> "TQ=="
        assert_eq!(bytes_to_base64(&bytes), base64_string);

        let bytes = [77, 97]; // "Ma" in ASCII
        let base64_string = "TWE=";
        // "Ma" in ASCII is [77, 97]. Encoded as:
        // 'M' -> "01001101", 'a' -> "01100001"
        // Combined -> "0100110101100001"
        // Split into 6-bit chunks -> "010011" "010110" "000100"
        // Mapped to Base64 -> "T" "W" "E"
        // Padding needed to make length multiple of 4 -> "TWE="
        assert_eq!(bytes_to_base64(&bytes), base64_string);
    }
}
