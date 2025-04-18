use ark_bn254::Fr;
use ark_ec::pairing::Pairing;
use ark_ec::CurveGroup;
use ark_std::time::Instant;
use ark_std::One;
use ark_std::UniformRand;
use lrs_v2::cc::helpers::mimc_constants_round91;
use lrs_v2::cc::helpers::multi_mimc7;
use std::ops::Mul;

fn main() {
    type E = ark_bn254::Bn254;
    type F = <E as Pairing>::ScalarField;
    type G1 = <E as Pairing>::G1Affine;
    type G2 = <E as Pairing>::G2Affine;

    let field_iters = 1000000; // 1e6
    let mimc_iters = 1000000; // 1e6

    let rng = &mut ark_std::test_rng();
    let a = Fr::rand(rng);
    let b = Fr::rand(rng);
    let mut c = Fr::rand(rng);
    let start = Instant::now();
    for _ in 0..field_iters {
        c = a * b + c;
    }
    println!("Field c: {:?}", c);
    let field_time = start.elapsed();
    println!("Repeat {:?}: Field time: {:?}", field_iters, field_time);

    let mut mimc_input = vec![F::one(), F::one()];
    let start = Instant::now();
    let c = mimc_constants_round91::<E>();
    for _ in 0..mimc_iters {
        let result = multi_mimc7::<E>(&mimc_input, 2, &c);
        mimc_input[0] = result;
    }
    let mimc_time = start.elapsed();
    println!("Repeat {:?}: MiMC time: {:?}", mimc_iters, mimc_time);

    let exp_iters = 1000000; // 1e6
    let mut base = G1::rand(rng);
    let mut exp = Fr::rand(rng);
    let mut res = G1::rand(rng);
    let start = Instant::now();
    for _ in 0..exp_iters {
        res = base.mul(exp).into_affine();
        base = res;
        exp = exp + Fr::one();
    }
    let exp_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: Exponentiation G1 time: {:?}",
        exp_iters, exp_time
    );

    let exp_iters = 1000000; // 1e6
    let mut base = G2::rand(rng);
    let mut exp = Fr::rand(rng);
    let mut res = G2::rand(rng);
    let start = Instant::now();
    for _ in 0..exp_iters {
        res = base.mul(exp).into_affine();
        base = res;
        exp = exp + Fr::one();
    }
    let exp_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: Exponentiation G2 time: {:?}",
        exp_iters, exp_time
    );
}
