use rand::Rng;

pub fn generate_qrng_key() -> Vec<u8> {
    let mut key = vec![0u8; 128];  // Example 1024-bit key (128 bytes)
    rand::thread_rng().fill(&mut key[..]);
    key
}
