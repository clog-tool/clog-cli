
pub fn get_short_hash(hash: &str) -> &str {
    hash.slice_chars(0,8)
}