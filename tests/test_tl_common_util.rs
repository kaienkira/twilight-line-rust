#[test]
fn test_sha256_sum() {
    let hash = hex::encode(tl_common::util::sha256_sum(b"hello world"));
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}
