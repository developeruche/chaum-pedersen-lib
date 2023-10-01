use num_bigint::BigUint;
use rand::Rng;
use sha256::digest;









pub fn hash_str(message: &str) -> String {
     digest(message)
}


pub fn exponentiate(num: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    num.modpow(exp,p)
}


fn generate_random_32_bytes() -> BigUint {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    BigUint::from_bytes_be(&bytes)
}



