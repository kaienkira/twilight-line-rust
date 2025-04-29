use sha2::Digest;
use sha2::Sha256;

pub fn sha256_sum(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    return result.as_slice().to_vec();
}
