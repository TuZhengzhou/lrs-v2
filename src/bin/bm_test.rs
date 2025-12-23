use ark_bn254::Fr;
use ark_ec::pairing::Pairing;
use ark_ec::CurveGroup;
use ark_std::time::Instant;
use ark_std::One;
use ark_std::UniformRand;
use lrs_v2::cc::helpers::mimc_constants_round91;
use lrs_v2::cc::helpers::multi_mimc7;
use std::ops::Mul;
use ark_std::Zero;

fn main() {
    type E = ark_bn254::Bn254;
    type F = <E as Pairing>::ScalarField;
    type G1 = <E as Pairing>::G1Affine;
    type G2 = <E as Pairing>::G2Affine;
    type GT = ark_bn254::Fq12;

    let field_iters = 100000; // 1e5
    let mimc_iters = 100000; // 1e5

    let rng = &mut ark_std::test_rng();
    let a = Fr::rand(rng);
    let mut b = Fr::rand(rng);
    let mut c = Fr::rand(rng);
    let start = Instant::now();
    for _ in 0..field_iters {
        c = a * b + c;
        b = b + a;
    }
    println!("Field c: {:?}", c);
    let field_time = start.elapsed();
    println!("Repeat {:?}: Field Mul time: {:?}", field_iters, field_time);

    let a = Fr::rand(rng);
    let mut b = Fr::rand(rng);
    let start = Instant::now();
    for _ in 0..field_iters {
        b = b + a;
    }
    println!("Field b: {:?}", b);
    let field_time = start.elapsed();
    println!("Repeat {:?}: Field Add time: {:?}", field_iters, field_time);

    let mut mimc_input = vec![F::one(), F::one()];
    let start = Instant::now();
    let c = mimc_constants_round91::<E>();
    for _ in 0..mimc_iters {
        let result = multi_mimc7::<E>(&mimc_input, 2, &c);
        mimc_input[0] = result;
    }
    let mimc_time = start.elapsed();
    println!("Repeat {:?}: MiMC time: {:?}", mimc_iters, mimc_time);

    let mut base = G1::rand(rng);
    let mut res = G1::rand(rng);
    let start = Instant::now();
    for _ in 0..field_iters {
        res = (res + base).into();
    }
    let exp_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: ADD G1 time: {:?}",
        field_iters, exp_time
    );

    let exp_iters = 100000; // 1e5
    let mut base = G1::rand(rng);
    let mut exp = Fr::rand(rng);
    let mut res = base.mul(exp);
    let mut add = exp;
    let start = Instant::now();
    for _ in 0..exp_iters {
        res = res + base.mul(add);
        add = add + exp;
    }
    let exp_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: Exponentiation G1 time: {:?}",
        exp_iters, exp_time
    );

    let mut base = G2::rand(rng);
    let mut res = G2::rand(rng);
    let start = Instant::now();
    for _ in 0..field_iters {
        res = (res + base).into();
    }
    let exp_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: ADD G2 time: {:?}",
        field_iters, exp_time
    );

    let exp_iters = 100000; // 1e5
    let mut base = G2::rand(rng);
    let mut exp = Fr::rand(rng);
    let mut res = G2::rand(rng);
    let mut add = exp;
    let start = Instant::now();
    for _ in 0..exp_iters {
        res = (res + base.mul(add)).into();
        add = add + exp;
    }
    let exp_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: Exponentiation G2 time: {:?}",
        exp_iters, exp_time
    );

    let pairing_iters = 100000; // 1e5
    
    let base1 = G1::rand(rng);
    let base = G2::rand(rng);
    let mut result = E::pairing(base1, base); // 初始化
    let start = Instant::now();
    for _ in 0..pairing_iters {
        result = E::pairing(base1, base) + result;
    }
    let pairing_time = start.elapsed();
    println!("Field res: {:?}", res);
    println!(
        "Repeat {:?}: Pairing time: {:?}",
        pairing_iters, pairing_time
    );
}
