// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {

    // Convert the raw transaction hex into bytes
    let raw_tx_bytes = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    // Check if we have at least 4 bytes for the version
    if raw_tx_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    let version = u32::from_le_bytes([
        raw_tx_bytes[0],
        raw_tx_bytes[1],
        raw_tx_bytes[2],
        raw_tx_bytes[3],
    ]);

    Ok(version)
}
