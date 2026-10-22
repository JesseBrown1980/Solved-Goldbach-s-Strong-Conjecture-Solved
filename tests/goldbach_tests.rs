use goldbach_3adic_spectral::{
    find_goldbach_partition, is_prime, prism, to_balanced_ternary, unprism, Zero, K, P,
};

#[test]
fn primes_ge_5_have_strictly_nonzero_lowest_trit() {
    for n in 5..=2000 {
        if is_prime(n) {
            let trits = to_balanced_ternary(n as i64);
            assert_ne!(
                trits[0],
                Zero::Nil,
                "Prime {n} had Zero::Nil at rank 0, which violates p ≢ 0 mod 3"
            );
            assert!(
                trits[0] == Zero::Pos || trits[0] == Zero::Neg,
                "Prime {n} rank 0 must be +1 or -1"
            );
        }
    }
}

#[test]
fn even_integers_have_even_net_trit_sum() {
    for n in 2..=1000 {
        let even_val = 2 * n;
        let trits = to_balanced_ternary(even_val as i64);
        let sum: i64 = trits.iter().map(|t| t.direction() as i64).sum();
        assert_eq!(
            sum % 2,
            0,
            "Even integer {even_val} must have even trit sum, got {sum}"
        );
    }
}

#[test]
fn goldbach_pairs_strictly_satisfy_3adic_conjugate_selection_rule() {
    for n in 3..=2500 {
        let even_val = 2 * n;
        let rem_3 = (even_val % 3) as i64;
        let expected_trit_sum = if rem_3 == 2 { -1 } else { rem_3 };

        if let Some((p1, p2)) = find_goldbach_partition(even_val) {
            if p1 >= 5 && p2 >= 5 {
                let t1 = to_balanced_ternary(p1 as i64)[0].direction() as i64;
                let t2 = to_balanced_ternary(p2 as i64)[0].direction() as i64;
                let actual_sum = (t1 + t2).rem_euclid(3);
                let target_mod = expected_trit_sum.rem_euclid(3);

                assert_eq!(
                    actual_sum, target_mod,
                    "3-adic selection rule violated for 2n={even_val} = {p1} + {p2}: t1={t1}, t2={t2}"
                );

                if rem_3 == 0 {
                    assert!(
                        (t1 == 1 && t2 == -1) || (t1 == -1 && t2 == 1),
                        "2n ≡ 0 mod 3 must pair Pos with Neg"
                    );
                } else if rem_3 == 1 {
                    assert_eq!(t1, -1);
                    assert_eq!(t2, -1);
                } else if rem_3 == 2 {
                    assert_eq!(t1, 1);
                    assert_eq!(t2, 1);
                }
            }
        } else {
            panic!("Goldbach partition not found for {even_val}!");
        }
    }
}

#[test]
fn goldbach_ntt_spectral_convolution_and_uncomputed_center() {
    let mut prime_frame = [0u64; K];
    let mut count_primes = 0u64;

    for i in 0..K {
        let val = i as u64;
        if is_prime(val) {
            prime_frame[i] = 1;
            count_primes += 1;
        }
    }

    let a_spec = prism(&prime_frame);
    assert_eq!(
        a_spec[0], count_primes,
        "A[0] must equal exact prime count mod P"
    );

    let mut s_spec = [0u64; K];
    for j in 0..K {
        s_spec[j] = ((a_spec[j] as u128 * a_spec[j] as u128) % P as u128) as u64;
    }

    let expected_s0 = ((count_primes as u128 * count_primes as u128) % P as u128) as u64;
    assert_eq!(s_spec[0], expected_s0);
    assert!(
        s_spec[0] > 0,
        "DC spectral center S[0] must be strictly non-vanishing"
    );

    let r_conv = unprism(&s_spec);

    let mut sum_r = 0u64;
    for i in 0..K {
        sum_r = (sum_r + r_conv[i]) % P;
    }
    assert_eq!(
        sum_r, s_spec[0],
        "Conservation law: sum of convolution terms must equal uncomputed center S[0]"
    );

    assert!(
        r_conv[10] > 0,
        "Target 10 must have non-vanishing spectral overlap"
    );
    assert!(
        r_conv[12] > 0,
        "Target 12 must have non-vanishing spectral overlap"
    );
    assert!(
        r_conv[16] > 0,
        "Target 16 must have non-vanishing spectral overlap"
    );
}

#[test]
fn goldbach_exhaustive_convergence_up_to_10_000() {
    for n in 2..=5000 {
        let even_val = 2 * n;
        let partition = find_goldbach_partition(even_val);
        assert!(
            partition.is_some(),
            "Goldbach failed for even integer {even_val}"
        );
    }
}

#[test]
fn goldbach_large_integer_spectral_invariance() {
    let large_targets = [
        65536u64, 131072, 262144, 524288, 1_000_000, 1_000_082,
    ];

    for &even_val in &large_targets {
        let partition = find_goldbach_partition(even_val);
        assert!(
            partition.is_some(),
            "Goldbach partition failed for large target {even_val}"
        );
        let (p1, p2) = partition.unwrap();
        assert_eq!(p1 + p2, even_val);
        assert!(is_prime(p1));
        assert!(is_prime(p2));
    }
}
