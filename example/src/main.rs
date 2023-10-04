use chaum_pedersen_lib::*;
use chaum_pedersen_lib::utils::generate_random_32_bytes;


fn main() {
    let system_default = generate_initial_system_constants();
    let secret_x = BigUint::from(100u32);
    let rand_k = generate_random_32_bytes();

    let params_one = generate_interaction_one(
        &system_default,
        &secret_x,
        &rand_k
    );

    let rand_c = generate_random_32_bytes();


    let solution = solve_challenge(
        &rand_k,
        &secret_x,
        &rand_c,
        &system_default.order
    );


    let verify = verify_claim(
        &system_default.alpha,
        &system_default.beta,
        &params_one.r_one,
        &params_one.r_two,
        &params_one.y_one,
        &params_one.y_two,
        &solution,
        &rand_c,
        &system_default.modulus
    );



    dbg!("==============================");
    println!(" Here come the verification:::------->   {}", verify);
    dbg!("==============================");
}