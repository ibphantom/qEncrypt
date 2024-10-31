use sha3::{Digest, Sha3_512};

pub fn calculate_hmac(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_512::new();
    hasher.update(key);
    hasher.update(data);
    hasher.finalize().to_vec()
}
