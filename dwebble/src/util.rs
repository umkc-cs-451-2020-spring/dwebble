use crypto::pbkdf2;
const PBKDF2_ITER: u32 = 100_000;

pub fn hash_password(password: &str) -> String {
    pbkdf2::pbkdf2_simple(password, PBKDF2_ITER).expect("Could not hash pasword")
}

pub fn verify_password(pw: &str, hash: &str) -> bool {
    pbkdf2::pbkdf2_check(pw, hash).expect("Could not verify password hash")
}

#[cfg(test)]
mod util_tests {
    use super::*;

    // #[test]
    // fn test_hash_password() {
    //     let salt = salt();
    //     let hash = hash_password("foobar", &);

    //     let res = pbkdf2::verify(DIGEST_ALG, PBKDF2_ITER, &salt, "foobar".as_bytes(), &hash);
    //     assert!(res.is_ok())
    // }

    // #[test]
    // fn test_verify() {
    //     let salt = salt();
    //     let hash = hash_password("foobar", &salt);
    //     assert!(verify_password(hash, &salt, "foobar"))
    // }
}
