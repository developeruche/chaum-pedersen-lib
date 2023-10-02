use num_bigint::BigUint;
use rand::Rng;
use sha256::digest;









pub fn hash_str(message: &str) -> String {
     digest(message)
}


pub fn exponentiate(num: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    num.modpow(exp,p)
}


pub fn generate_random_32_bytes() -> BigUint {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    BigUint::from_bytes_be(&bytes)
}


/// This function is used to solve this challenge the verifier sends to the prover
/// s = (k - c * x) mod p
pub fn solve_challenge(
    rand_prover_k: &BigUint,
    secret_from_prover: &BigUint,
    rand_verifier_c: &BigUint,
    modulus: &BigUint
) -> BigUint {
    let c_mul_x = rand_verifier_c * secret_from_prover;

    if *rand_prover_k > c_mul_x {
        (rand_prover_k - c_mul_x).modpow(&BigUint::one(), modulus)
    } else {
        q - (c_mul_x - rand_prover_k).modpow(&BigUint::one(), modulus)
    }
}

pub fn verify_claim(
    alpha: &BigUint,
    beta: &BigUint,
    r_one: &BigUint,
    r_two: &BigUint,
    y_one: &BigUint,
    y_two: &BigUint,
    solution_from_prover: &BigUint,
    rand_verifier_c: &BigUint,
    prime: &BigUint,
) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {

        let hash_one = hash_str("hello");
        let hash_two = hash_str("hello");
        let hash_three = hash_str("developeruche");

        assert_eq!(hash_one, hash_two);
        assert_ne!(hash_one, hash_three);
    }

    #[test]
    fn test_expo() {
        let num_ = BigUint::from(99u32);
        let exp_ = BigUint::from(4u32);
        let p_ = BigUint::from(1100u32);
        let ex_res = exponentiate(&num_, &exp_, &p_);

        assert_eq!(ex_res, BigUint::from(1001u32));
    }


    #[test]
    fn test_randomness() {
        let rand_one = generate_random_32_bytes();
        let rand_two = generate_random_32_bytes();


        assert_ne!(rand_one, rand_two);
    }
}
