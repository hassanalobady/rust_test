extern crate curve25519_dalek;
extern crate rand;
extern crate rand_core;
extern crate sha2;
//extern crate gmp-mpfr-sys;
extern crate bls12_381;
//extern crate curve25519_dalek;
//extern crate chacha20;
//extern crate rug;



//use std::convert::TryInto;
use bls12_381::{G1Projective, Scalar};
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar as RistrettoScalar};
//use chacha20::ChaCha20;
//use chacha20::stream_cipher::{NewStreamCipher, SyncStreamCipher};
//use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
//use hex_literal::hex;
//use std::env;

use curve25519_dalek::scalar::Scalar;
// use rand::prelude::*;


use rand_core::OsRng;


// Shamir's Secret Sharing (SSS)
pub fn generate_shares(secret: Scalar, num_players: usize, threshold: usize) -> Vec<Scalar> {
    let _rng = rand::thread_rng();
    let mut coefficients = Vec::with_capacity(threshold - 1);
    let mut shares = Vec::with_capacity(num_players);

    for _ in 0..(threshold - 1) {
        coefficients.push(Scalar::random(&mut OsRng));
    }

    for player in 0..num_players {
        let share = secret;

        for (_exp, _coeff) in coefficients.iter().enumerate() {
            let _player_scalar = Scalar::from(player as u64 + 1);
           // share += coeff * player_scalar::Scalar(&[exp u64 + 1]);
           // share += coeff * player_scalar.pow(&[exp as u64 + 1]);

        }

        shares.push(share);
    }

    shares
}



// EC cryptography with the BLS12-381 curve
pub fn encrypt_share(share: Scalar, public_key: G1Projective) -> G1Projective {
  let share_point = share * G1Projective::generator();
  share_point + public_key
}

pub fn decrypt_share(encrypted_share: G1Projective, private_key: Scalar) -> Scalar {
  let decrypted_share_point = encrypted_share - (private_key * G1Projective::generator());
  decrypted_share_point.into_affine().scalar
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

