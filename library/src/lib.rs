use num::One;
use num::Num;
use num::traits::FromBytes;
use num_bigint::BigUint;
use crate::utils::{exponentiate, generate_random_32_bytes};

pub mod utils; // this would hold the utils functions used through out the life-cycle of this lib



#[derive(Debug)]
pub struct SystemParameter {
    alpha: BigUint,
    beta: BigUint,
    modulus: BigUint
}



#[derive(Debug)]
pub struct InteractionOne {
    y_one: BigUint,
    y_two: BigUint,
    r_one: BigUint,
    r_two: BigUint
}



pub fn generate_initial_system_constants() -> SystemParameter {
    SystemParameter {
        alpha: generate_random_32_bytes(),
        beta: generate_random_32_bytes(),
        modulus: BigUint::from_str_radix("8F5BE75E0E12C042C7E6E41FDB5A9D9D4ACD0E6C2EF3FDD0EEDD1E7E6C0BAC73667DE4DB2D15BFB2A53599CCDE29A3A3C073D9FA442946F6CE9214173526F1A5682150A14FBF4F542F4C03FCE3E27BCE89A176957D7F80D39FEEA4E0F0E42E48C5EDB1DFA9640D9E78339FBCD9F7EF6E525EA05A44E2BBA56E446F6D12E3F6D07F865C68F964AB59EC3E8896DCD093E98AFB3AB15E99CCF8D5511DDE1F09E72DD3D", 16u32).unwrap()
    }
}

pub fn generate_interaction_one(
    system_parameter: &SystemParameter,
    secret_x: &BigUint,
    rand_k: &BigUint
) -> InteractionOne {
    InteractionOne {
        y_one: exponentiate(&system_parameter.alpha, secret_x, &system_parameter.modulus),
        y_two: exponentiate(&system_parameter.beta, secret_x, &system_parameter.modulus),
        r_one: exponentiate(&system_parameter.alpha, rand_k, &system_parameter.modulus),
        r_two: exponentiate(&system_parameter.beta, rand_k, &system_parameter.modulus),
    }
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
        modulus - (c_mul_x - rand_prover_k).modpow(&BigUint::one(), modulus)
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
    let first_condition = *r_one == (alpha.modpow(solution_from_prover, prime) * y_one.modpow(rand_verifier_c, prime)).modpow(&BigUint::one(), prime);
    let second_condition = *r_two == (beta.modpow(solution_from_prover, prime) * y_two.modpow(rand_verifier_c, prime)).modpow(&BigUint::one(), prime);
    first_condition && second_condition
}