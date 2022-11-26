#![no_main]

pub extern crate externc_libm as libm;
use risc0_zkvm_guest::env;
risc0_zkvm_guest::entry!(main);

use ark_ec::{ProjectiveCurve, AffineCurve, PairingEngine};
use ark_ff::{PrimeField, Field};
// We'll use the BLS12-381 pairing-friendly group for this example.
use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, G1Affine, G2Affine, Fr as ScalarField};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_std::{Zero, UniformRand};

pub fn main() {
    let mut rng = ark_std::test_rng();
    // Let's sample uniformly random field elements:
    let a: G1Affine = G1::rand(&mut rng).into();
    //let b: G2Affine = G2::rand(&mut rng).into();
    // We can compute the pairing of `a` and `b`:
    //let c = Bls12_381::pairing(a, b);
    
    // We can serialize with compression...
    let mut compressed_bytes = Vec::new();
    a.serialize(&mut compressed_bytes).unwrap();
    env::commit(&compressed_bytes);
}

