//! Synthetic corpus and reproducible, bounded mutation regression.
#[path = "support/fuzz_contract.rs"]
mod contract;

const SEEDS: &[&[u8]] = &[
    b"",
    b"\xff",
    b"%PDF-1.7\n%%EOF",
    b"%PDF-2.0\nstartxref\n0\n%%EOF",
    include_bytes!("../../../fuzz/corpus/pdf_bytes/synthetic-minimal.pdf"),
];

#[test]
fn synthetic_corpus_satisfies_boundary_invariants() {
    for seed in SEEDS {
        contract::check(seed);
    }
}

#[test]
fn bounded_mutations_satisfy_boundary_invariants() {
    let mut count = 0;
    for seed in SEEDS {
        let stride = (seed.len() / 32).max(1);
        for position in (0..=seed.len()).step_by(stride) {
            contract::check(&seed[..position]);
            count += 1;
            if position < seed.len() {
                for mask in [1_u8, 0x80, 0xff] {
                    let mut candidate = seed.to_vec();
                    candidate[position] ^= mask;
                    contract::check(&candidate);
                    count += 1;
                }
            }
        }
        for suffix in [b"\0".as_slice(), b"\xff", b"\n", b"{}"] {
            let mut candidate = seed.to_vec();
            candidate.extend_from_slice(suffix);
            contract::check(&candidate);
            count += 1;
        }
    }
    assert!(count > 100, "mutation corpus must execute");
}
