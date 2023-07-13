//extern crate rand;
//extern crate rand_core;
//extern crate sha2;
//extern crate gmp-mpfr-sys;
//extern crate bls12_381;
//extern crate curve25519_dalek;
//extern crate chacha20;
//extern crate rug;
//extern crate bls12_381;
//extern crate rand;

//use bls12_381::{G1Affine, G1Projective, Scalar};
//use rand::RngCore;

//extern crate bls12_381;
//extern crate rand;

//use bls12_381::{Scalar, G1Affine, G1Projective};
//use rand::{Rng, RngCore};

//use std::convert::TryInto;
//use bls12_381::{G1Projective, Scalar};
//use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar as RistrettoScalar};

//use chacha20::ChaCha20;
//use chacha20::stream_cipher::{NewStreamCipher, SyncStreamCipher};
//use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
//use hex_literal::hex;
//use std::env;
// use rand::prelude::*;

//use rand_core::OsRng;

//use bls12_381::Scalar;
//use rand::Rng;

//extern crate curve25519_dalek;
extern crate bls12_381;
extern crate rand;
use ff::Field;


use bls12_381::{G1Affine, G1Projective, Scalar};
//use bls12_381::{G1Affine, G1Projective};
//use bls12_381::Scalar;
//use bls12_381::random;

//extern crate bls12_381;

use rand::RngCore;

//use curve25519_dalek::scalar::Scalar;
use std::env;

//use rand::RngCore;
use rand::{thread_rng, Rng};
//use rand::RngCore;

use std::iter::FromIterator;
use std::collections::VecDeque;
//extern crate bls12_381;
//extern crate rand;

//use bls12_381::{Scalar, G1Projective, G1Affine};
//use rand::{thread_rng, Rng};

//extern crate bls12_381;
//extern crate rand;

//use bls12_381::{G1Affine, G1Projective, Scalar};
//use rand::RngCore;

///use ff::Field;
//use bls12_381::{Scalar, G1Projective, G1Affine};
//use rand::Rng;
//use std::iter::FromIterator;

fn generate_random_seeds(num_players: usize) -> Vec<Scalar> {
    let mut rng = rand::thread_rng();
    (0..num_players).map(|_| Scalar::random(&mut rng)).collect()
}

fn compute_curve_points(seeds: &[Scalar]) -> Vec<G1Projective> {
    seeds.iter().map(|seed| G1Projective::generator() * seed).collect()
}

fn generate_commitments(points: &[G1Projective]) -> Vec<G1Projective> {
    let generator = G1Projective::generator();
    points.iter().map(|point| generator + point).collect()
}

fn verify_commitments(commitments: &[G1Projective], points: &[G1Projective]) -> bool {
    let generator = G1Projective::generator();
    commitments.iter().zip(points).all(|(commitment, point)| {
        let challenge = Scalar::random(&mut rand::thread_rng());
        let left = generator * challenge;
        let right = *commitment + (point * challenge);
        G1Affine::from(left) == G1Affine::from(right)
    })
}

fn reveal_ciphertexts(ciphertexts: &[Scalar]) -> Vec<u8> {
    let mut result = Vec::new();
    for scalar in ciphertexts {
        result.extend_from_slice(&scalar.to_bytes());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_seeds() {
        let num_players = 5;
        let random_seeds = generate_random_seeds(num_players);
        assert_eq!(random_seeds.len(), num_players);
    }

    #[test]
    fn test_compute_curve_points() {
        let num_players = 5;
        let random_seeds = generate_random_seeds(num_players);
        let curve_points = compute_curve_points(&random_seeds);
        assert_eq!(curve_points.len(), num_players);
    }

    #[test]
    fn test_generate_commitments() {
        let num_players = 5;
        let random_seeds = generate_random_seeds(num_players);
        let curve_points = compute_curve_points(&random_seeds);
        let commitments = generate_commitments(&curve_points);
        assert_eq!(commitments.len(), num_players);
    }

    #[test]
    fn test_verify_commitments() {
        let num_players = 5;
        let random_seeds = generate_random_seeds(num_players);
        let curve_points = compute_curve_points(&random_seeds);
        let commitments = generate_commitments(&curve_points);
        assert!(verify_commitments(&commitments, &curve_points));
    }

    #[test]
    fn test_reveal_ciphertexts() {
        let num_players = 5;
        let random_seeds = generate_random_seeds(num_players);
        let ciphertexts = compute_curve_points(&random_seeds);
        let revealed = reveal_ciphertexts(&ciphertexts);
        assert_eq!(revealed.len(), 32 * num_players);
    }
}

fn main() {
    let num_players = 5;
    let random_seeds = generate_random_seeds(num_players);
    let curve_points = compute_curve_points(&random_seeds);
    let commitments = generate_commitments(&curve_points);
    let verified = verify_commitments(&commitments, &curve_points);
    let ciphertexts = curve_points; // Just for testing, using curve points as ciphertexts
    let final_randomness = reveal_ciphertexts(&ciphertexts);

    println!("Verified: {}", verified);
    println!("Final Randomness: {:?}", final_randomness);
}
