pub fn custom_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encrypted_data = data.to_vec();
    for (i, byte) in encrypted_data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];  // XOR encryption (placeholder for custom cipher)
    }
    encrypted_data
}
