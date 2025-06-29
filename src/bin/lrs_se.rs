use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_std::thread::sleep;
use lrs_v2::constants::*;
use lrs_v2::lrs::setup;
use lrs_v2::lrs::sign;
use lrs_v2::lrs::verify;
use lrs_v2::lrs::CircDescriptor;
use lrs_v2::lrs::SignTime;
use lrs_v2::lrs::VerifyTime;
use lrs_v2::sma::ring_gen;
use std::env::args;

type F = <Bn254 as Pairing>::ScalarField;

fn lrs_signature(n_iters: usize, log2_low: usize, log2_high: usize) {
    let security_par = "128".to_string();
    let msg = "test message";

    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_LRS_SE,
        num_commit_witness: NUM_COMMIT_WITNESS_LRS_SE,
        ioputs_name: IOPUTS_NAME_LRS_SE.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_LRS_SE.to_string(),
        circuit_name: CIRCUIT_NAME_LRS_SE.to_string(),
    };

    let signer_idx = 1usize;
    let rng = &mut ark_std::test_rng();

    for ring_size_max_log in log2_low..=log2_high {
        let ring_size_max = 1 << ring_size_max_log;
        println!("Ring size: {}", ring_size_max);
        let (lrs_pvkey, circuit) =
            setup::setup::<ark_bn254::Bn254>(security_par.clone(), ring_size_max, &circ_desc);

        println!("Public inputs:");
        for i in 0..circ_desc.num_pub_io {
            let value: F = F::from_bigint(circuit.instance[i].into_bigint()).unwrap();
            println!("{}: {}", circ_desc.ioputs_name[i], value);
        }

        println!("Commit witness:");
        for i in 0..circ_desc.num_commit_witness {
            let value: F = F::from_bigint(circuit.commit_witness[i].into_bigint()).unwrap();
            println!(
                "{}: {}",
                circ_desc.ioputs_name[circ_desc.num_pub_io + i],
                value
            );
        }

        let mut ring = ring_gen::<_, ark_bn254::Bn254>(ring_size_max, signer_idx, rng);
        ring[signer_idx] = circuit.commit_witness[0];

        // Average the time taken for n_iters iterations
        // for signature generation and verification
        let mut sign_times = Vec::new();
        let mut verify_times = Vec::new();
        for _iter in 0..n_iters {
            let is_print = if _iter == 1 { Some(true) } else { None };

            let mut sign_time = SignTime::new();
            let signature = sign::sign::<ark_bn254::Bn254>(&lrs_pvkey, &circuit, &ring, msg, &mut sign_time);

            let mut verify_time = VerifyTime::new();
            let result =
                verify::verify::<ark_bn254::Bn254>(&lrs_pvkey, &ring, msg, &signature, &mut verify_time);
            assert!(result, "Signature verification failed");

            if is_print.unwrap_or(false) {
                println!("Signature generation time: {:?}", sign_time);
                println!("Signature verification time: {:?}", verify_time);
            }

            sign_times.push(sign_time);
            verify_times.push(verify_time);

            sleep(std::time::Duration::from_millis(100));
        }

        let sign_time_avg = sign_times.iter().fold(SignTime::new(), |acc, x| {
            SignTime {
                cc: acc.cc + x.cc,
                sma: acc.sma + x.sma,
                link: acc.link + x.link,
                sign: acc.sign + x.sign,
            }
        }) / n_iters as u32;

        let verify_time_avg = verify_times.iter().fold(VerifyTime::new(), |acc, x| {
            VerifyTime {
                cc: acc.cc + x.cc,
                sma: acc.sma + x.sma,
                link: acc.link + x.link,
                verify: acc.verify + x.verify,
            }
        }) / n_iters as u32;
        
        println!(
            "Average Sign time for 2^{:?} ring: {:?}",
            ring_size_max_log, sign_time_avg
        );
        println!(
            "Average Verification for 2^{:?} ring: {:?}\n\n",
            ring_size_max_log, verify_time_avg
        );
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let n_iters: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    println!("n_iters: {}", n_iters);
    lrs_signature(n_iters, 3, 20);
}

#[test]
fn test_lrs_signature() {
    lrs_signature(1, 10, 10);
}
