extern crate curve25519_dalek;
//extern crate rand_core;
extern crate sha2;
//extern crate gmp-mpfr-sys;
extern crate rand;
//extern crate bls12_381;
//extern crate curve25519_dalek;
//extern crate chacha20;
//extern crate rug;

use sha2::Sha512;
use rand::RngCore;
//use std::convert::TryInto;
//use bls12_381::{G1Projective, Scalar};
use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar as RistrettoScalar};
//use chacha20::ChaCha20;
//use chacha20::stream_cipher::{NewStreamCipher, SyncStreamCipher};
//use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
//use hex_literal::hex;
//use std::env;

use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;


// use rand::prelude::*;
use sha2::{Sha256, Digest};
//use rand::thread_rng;
use rand_core::OsRng;
use std::env;


// Schnorr Signatures

//fn Schnorr_Signatures() {

  //  let mut message="password";
    //let args: Vec<String> = env::args().collect();
  
   // if args.len() >1 { message = args[1].as_str();}

    // Base point
    //let G = &constants::ED25519_BASEPOINT_POINT;

    //Private key (r)
    //let r = Scalar::random(&mut OsRng);

    //Public key (rG)
    //let U = r*G;


    //Create a random nonce (rt) for each proof
    //let rt = Scalar::random(&mut OsRng);

    //let Ut = rt*G;




    // Challenge generation
    //let mut temp: [u8;32] = [0u8;32];
    //let mut hasher = Sha256::new();
    //hasher.update(message.as_bytes());
   // hasher.update(Ut.compress().to_bytes());
   // temp.copy_from_slice(hasher.finalize().as_slice());

    //let c = Scalar::from_bytes_mod_order(temp);

    //let rz = rt + c*r;

   // println!("Message:={:}",message);

    //println!("\nG={:}",hex::encode(G.compress().as_bytes()));

    //println!("-- Let's pick the private key (r) -- ");
    //println!("r= {:}",hex::encode(r.as_bytes()));
    //println!("-- Let's generate the public key (rG) -- ");
    //println!("\nU=rG= {:}",hex::encode(U.compress().as_bytes()));

    //println!("\n-- Let's pick a random value (rt) -- ");
    //println!("rt= {:}",hex::encode(rt.as_bytes()));

    //println!("Ut=rtG= {:}",hex::encode(Ut.compress().as_bytes()));

    //println!("\n-- Let's generate the challenge (c) and response (rz) -- ");
    //println!("c= {:}",hex::encode(c.as_bytes()));
    //println!("rz=rt+c*r {:}",hex::encode(rz.as_bytes()));

    //let p1=rz*G;
    //let p2=Ut+c*U;
    //println!("\n-- Computing the proof -- ");
    //println!("\nrz*G={:}",hex::encode(p1.compress().as_bytes()));
    //println!("Ut+c*U{:}",hex::encode(p2.compress().as_bytes()));

    //if (p1==p2) {
      //  println!("Proven!!");   
   // }
    //else {
      //  println!("Not proven");
   // }

//}

// Schnorr Signatures
//1. Generates a random private key, generates the corresponding public key, 
//2. signs a random message using Schnorr signatures with 'secp256k1',
//3.  verifies the Schnorrsignature.

// key genration of the Schnorr Signatures
pub fn schnorr_keygen() -> (Scalar, RistrettoPoint) {
    let mut rng = rand::thread_rng();
    let private_key = Scalar::random(&mut rng);
    let public_key = private_key * RistrettoPoint::default();
    (private_key, public_key)
}

// signs random message of the Schnorr Signatures

pub fn schnorr_sign(private_key: Scalar, message: &[u8]) -> (RistrettoPoint, Scalar) 

{
    // Challenge generation
   let mut temp: [u8;32] = [0u8;32];
   // let mut hasher = Sha256::new();
   // hasher.update(message.as_bytes());
  // hasher.update(Ut.compress().to_bytes());
 //  temp.copy_from_slice(hasher.finalize().as_slice());

   // let c = Scalar::from_bytes_mod_order(temp);

    let mut rng = rand::thread_rng();
    let nonce = RistrettoScalar::random(&mut rng);
    let r = nonce * RistrettoPoint::default();
   // pub fn from_bytes_mod_order(bytes: [u8; 32]);
    let e = Scalar::from_bytes_mod_order(temp);
    let s = nonce + (e * private_key);
    (r, s)
}

// the verifies of random message Schnorr signature.

pub fn schnorr_verify(public_key: RistrettoPoint, signature: (RistrettoPoint, Scalar), message: &[u8]) -> bool 

{
    let mut temp: [u8;32] = [0u8;32];
    let (r, s) = signature;
    let e = Scalar::from_bytes_mod_order(temp);
    let rv = s * RistrettoPoint::default() - e * public_key;
    rv.compress() == r.compress()
}

fn main() {
    let num_players = 5;
     let threshold = 3;
    let num_bits = 128;
  
    let random_bits = secure_random_bits(num_players, threshold, num_bits);
    println!("Random Bits: {:?}", random_bits);
  }