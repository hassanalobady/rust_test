extern crate curve25519_dalek;
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

use bls12_381::{Scalar, G1Affine, G1Projective};
use rand::{Rng, RngCore};

//use std::convert::TryInto;
//use bls12_381::{G1Projective, Scalar};
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar as RistrettoScalar};

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
//pub fn generate_random_seeds(num_players: usize) -> Vec<Scalar> 
pub fn generate_random_seed() -> Scalar
{
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

 // let mut rng = rand::thread_rng();

  let mut bytes = [0u8; 32];
  rng.fill_bytes(&mut bytes);
  Scalar::from_bytes(&bytes)
}
  //(0..num_players).map(|_| Scalar::random(&mut rng)).collect()

//  (0..num_players).map(|_| Scalar::random(&mut rng)).collect()

//}

// Compute curve points using random seeds

pub fn compute_curve_points(seeds: &[Scalar]) -> Vec<G1Projective> {
  seeds.iter().map(|seed| seed * G1Projective::generator()).collect()
}

// Generate cryptographic commitments for each player
pub fn generate_commitments(points: &[G1Projective]) -> Vec<G1Projective> {
  let mut rng = rand::thread_rng();
  points.iter().map(|point| point * Scalar::random(&mut rng)).collect()
}

// Verify commitments
pub fn verify_commitments(commitments: &[G1Projective], points: &[G1Projective]) -> bool {
  let mut rng = rand::thread_rng();
  commitments.iter().zip(points).all(|(commitment, point)| {
      let challenge = Scalar::random(&mut rng);
      let left = commitment * challenge;
      let right = point * challenge;
      left == right
  })
}

// Reveal ciphertexts and compute final randomness
pub fn compute_final_randomness(commitments: &[G1Projective], seeds: &[Scalar]) -> [u8; 48] {
  let mut rng = rand::thread_rng();
  let challenge = Scalar::random(&mut rng);
  let mut combined_commitment = G1Projective::identity();
  let mut combined_seed = Scalar::zero();

  for (commitment, seed) in commitments.iter().zip(seeds) {
      combined_commitment += commitment * challenge;
      combined_seed += seed * challenge;
  }

  let combined_commitment_affine = G1Affine::from(combined_commitment);
  let combined_seed_bytes = combined_seed.to_bytes();
  let mut final_randomness = [0u8; 48];

  final_randomness[..16].copy_from_slice(&combined_commitment_affine.get_u().to_bytes());
  final_randomness[16..32].copy_from_slice(&combined_commitment_affine.get_v().to_bytes());
  final_randomness[32..].copy_from_slice(&combined_seed_bytes);

  final_randomness
}


// Schnorr Signatures
//1. Generates a random private key, generates the corresponding public key, 
//2. signs a random message using Schnorr signatures with 'secp256k1',
//3.  verifies the Schnorrsignature.

// key genration of the Schnorr Signatures
pub fn schnorr_keygen() -> (Scalar, RistrettoPoint)
 {
    let _rng = rand::thread_rng();
    let private_key = Scalar::random(&mut OsRng);
    let public_key = private_key * RistrettoPoint::default();
    (private_key, public_key)
}

// signs random message of the Schnorr Signatures

pub fn schnorr_sign(private_key: Scalar, _message: &[u8]) -> (RistrettoPoint, Scalar) 

{
    // Challenge generation
   let temp: [u8;32] = [0u8;32];
    let _os_rng = rand::thread_rng();
    let nonce = RistrettoScalar::random(&mut OsRng);
    let r = nonce * RistrettoPoint::default();
    let e = Scalar::from_bytes_mod_order(temp);
    let s = nonce + (e * private_key);
    (r, s)
}

// the verifies of random message Schnorr signature.

pub fn schnorr_verify(public_key: RistrettoPoint, signature: (RistrettoPoint, Scalar), _message: &[u8]) -> bool 

{
    let temp: [u8;32] = [0u8;32];
    let (r, s) = signature;
    let e = Scalar::from_bytes_mod_order(temp);
    let rv = s * RistrettoPoint::default() - e * public_key;
    rv.compress() == r.compress()
}


#[cfg(test)]

#[test]
fn test_schnorr_sign_verify() {
  let (private_key, public_key) = schnorr_keygen();
  let message = b"test message";

  let signature = schnorr_sign(private_key, message);
  assert!(schnorr_verify(public_key, signature, message));
}

fn main() {
//    let num_players = 5;
  //   let threshold = 3;
    //let num_bits = 128;
  
    //let random_bits = secure_random_bits(num_players, threshold, num_bits);
    //println!("Random Bits: {:?}", random_bits);
  }

