// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // 1. Decode the hexadecimal string into a vector of bytes.
    //    If decoding fails (e.g., invalid characters), map the error to a String and return it.
    let tx_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    // 2. The version field is 4 bytes long. Check if the decoded data is at least this long.
    if tx_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    // 3. Extract the first 4 bytes which represent the version number.
    //    We convert the slice to a fixed-size array `[u8; 4]`.
    //    The `unwrap()` here is safe because we have already checked the length.
    let version_bytes: [u8; 4] = tx_bytes[..4].try_into().unwrap();

    // 4. Convert the 4-byte array from little-endian byte order to a u32 integer.
    let version = u32::from_le_bytes(version_bytes);

    // 5. Return the extracted version number.
    Ok(version)
}
