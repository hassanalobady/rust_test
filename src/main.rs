//extern crate curve25519_dalek;
extern crate rand;
extern crate rand_core;
extern crate sha2;
//extern crate gmp-mpfr-sys;
//extern crate bls12_381;
//extern crate curve25519_dalek;
//extern crate chacha20;
//extern crate rug;
extern crate bls12_381;
//extern crate rand;

use bls12_381::{G1Affine, G1Projective, Scalar};
use rand::RngCore;

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

//use curve25519_dalek::scalar::Scalar;
// use rand::prelude::*;

use rand_core::OsRng;

//use bls12_381::Scalar;
//use rand::Rng;

// Generate random seed for each player
fn generate_random_seeds(num_players: usize) -> Vec<Scalar> {
  let mut rng = rand::thread_rng();
  (0..num_players).map(|_| Scalar::random(&mut rng)).collect()
}

// Compute curve points using random seeds
fn compute_curve_points(seeds: &[Scalar]) -> Vec<G1Projective> {
  seeds.iter().map(|seed| G1Projective::generator() * seed).collect()
}

// Generate cryptographic commitments for each player
fn generate_commitments(points: &[G1Projective]) -> Vec<G1Projective> {
  let mut rng = rand::thread_rng();
  points.iter().map(|point| G1Projective::generator() * Scalar::random(&mut rng)).collect()
}

// Verify commitments
fn verify_commitments(commitments: &[G1Projective], points: &[G1Projective]) -> bool {
  commitments.iter().zip(points).all(|(commitment, point)| {
      let challenge = Scalar::random(&mut rand::thread_rng());
      let left = G1Projective::generator() * challenge;
      let right = commitment + (point * challenge);
      left == right
  })
}

// Reveal ciphertexts and compute final randomness
fn compute_final_randomness(commitments: &[G1Projective], seeds: &[Scalar]) -> [u8; 48] {
  let challenge = Scalar::random(&mut rand::thread_rng());
  let combined_commitment = commitments.iter().fold(G1Projective::identity(), |acc, commitment| acc + (commitment * challenge));
  let combined_commitment_affine = G1Affine::from(combined_commitment);
  let combined_seed = seeds.iter().fold(Scalar::zero(), |acc, seed| acc + (seed * challenge));
  let combined_seed_bytes = combined_seed.to_bytes();

  let mut final_randomness = [0u8; 48];
  final_randomness[..16].copy_from_slice(&combined_commitment_affine.to_compressed().as_ref()[..16]);
  final_randomness[16..32].copy_from_slice(&combined_commitment_affine.to_compressed().as_ref()[16..32]);
  final_randomness[32..].copy_from_slice(&combined_seed_bytes);

  final_randomness
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
  fn test_compute_final_randomness() {
      let num_players = 5;
      let random_seeds = generate_random_seeds(num_players);
      let curve_points = compute_curve_points(&random_seeds);
      let commitments = generate_commitments(&curve_points);
      let final_randomness = compute_final_randomness(&commitments, &random_seeds);
      assert_eq!(final_randomness.len(), 48);
  }
}

fn main() {
  let num_players = 5;
  let random_seeds = generate_random_seeds(num_players);
  let curve_points = compute_curve_points(&random_seeds);
  let commitments = generate_commitments(&curve_points);
  let verified = verify_commitments(&commitments, &curve_points);
  let final_randomness = compute_final_randomness(&commitments, &random_seeds);

  println!("Verified: {}", verified);
  println!("Final Randomness: {:?}", final_randomness);
}