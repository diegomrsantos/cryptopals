use crate::basics::hex_to_base64::{ConversionError, hex_to_bytes};

/// Enum representing possible errors that can occur during XOR operations.
#[derive(Debug)]
pub enum XorError {
    /// Error during conversion from hex to bytes.
    Conversion,
    /// Error when the lengths of the input strings do not match.
    LengthMismatch,
}

impl From<ConversionError> for XorError {
    /// Converts a `ConversionError` into an `XorError`.
    ///
    /// # Arguments
    ///
    /// * `_` - The `ConversionError` to convert.
    ///
    /// # Returns
    ///
    /// * `XorError::Conversion` - The converted error.
    fn from(_: ConversionError) -> Self {
        XorError::Conversion
    }
}

/// Performs a fixed XOR operation on two hexadecimal strings.
///
/// # Arguments
///
/// * `hex1` - The first hexadecimal string.
/// * `hex2` - The second hexadecimal string.
///
/// # Returns
///
/// * `Result<String, XorError>` - The result of the XOR operation as a hexadecimal string, or an error.
pub fn fixed_xor(hex1: &str, hex2: &str) -> Result<String, XorError> {
    if hex1.len() != hex2.len() {
        return Err(XorError::LengthMismatch)
    }

    let bytes1 = hex_to_bytes(hex1)?;
    let bytes2 = hex_to_bytes(hex2)?;

    let bytes_xor = bytes1.iter().zip(bytes2.iter()).map(|(x, y)| x ^ y).collect::<Vec<_>>();
    Ok(bytes_to_hex(&bytes_xor))
}

/// Converts a byte slice to a hexadecimal string.
///
/// # Arguments
///
/// * `bytes` - The byte slice to convert.
///
/// # Returns
///
/// * `String` - The resulting hexadecimal string.
pub fn bytes_to_hex<T: AsRef<[u8]>>(bytes: T) -> String {
    const HEX_CHARS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    let bytes = bytes.as_ref();
    let mut hex_string = String::new();
    for &byte in bytes {
        hex_string.push(HEX_CHARS[(byte >> 4) as usize]);
        hex_string.push(HEX_CHARS[(byte &0xf) as usize]);
    }
    hex_string
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `fixed_xor` function with valid input.
    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let result = fixed_xor(a, b).expect("XOR operation failed");
        assert_eq!(result, expected);
    }

    /// Tests the `fixed_xor` function with input strings of different lengths.
    #[test]
    fn test_fixed_xor_length_mismatch() {
        let a = "1c0111001f010100061a024b53535009181caa";
        let b = "686974207468652062756c6c277320657965";
        let result = fixed_xor(a, b);
        assert!(matches!(result, Err(XorError::LengthMismatch)));
    }
}