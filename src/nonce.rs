use rand::{RngExt, distr::Alphanumeric, rng};
use sstr::Str;

/// Generates a secure nonce.
pub fn generate_nonce() -> Str<17> {
    rng()
        .sample_iter(&Alphanumeric)
        .take(17)
        .map(char::from)
        .fold(Str::<17>::empty(), |mut acc, char| {
            unsafe { acc.push_unchecked(char) }
            acc
        })
}
