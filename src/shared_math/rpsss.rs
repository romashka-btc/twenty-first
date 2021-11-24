use super::prime_field_element_big::{PrimeFieldBig, PrimeFieldElementBig};
use super::stark::Stark;
use crate::shared_math::rescue_prime_stark::RescuePrime;
use crate::util_types::proof_stream::ProofStream;
use crate::utils::blake3_digest;
use rand::RngCore;
use std::error::Error;

const DOCUMENT_HASH_LENGTH: usize = 32usize;

#[derive(Clone, Debug)]
pub struct SecretKey<'a> {
    pub value: PrimeFieldElementBig<'a>,
}

#[derive(Clone, Debug)]
pub struct PublicKey<'a> {
    pub value: PrimeFieldElementBig<'a>,
}

#[derive(Clone, Debug)]
pub struct Signature {
    pub proof: Vec<u8>,
}

pub struct RPSSS<'a> {
    pub field: PrimeFieldBig,
    pub rp: RescuePrime<'a>,
    pub stark: Stark<'a>,
}

impl<'a> RPSSS<'a> {
    pub fn keygen(&'a self) -> (SecretKey<'a>, PublicKey<'a>) {
        let mut prng = rand::thread_rng();
        let mut bytes = vec![0u8; 17];
        prng.fill_bytes(&mut bytes);
        let sk: SecretKey = SecretKey {
            value: self.field.from_bytes(&bytes),
        };
        let pk = PublicKey {
            value: self.rp.hash(&sk.value),
        };
        (sk, pk)
    }

    pub fn verify(&self, public_key: &PublicKey, signature: &Signature, document: &[u8]) -> bool {
        // Verify that the signature is prepended with the hash of the document
        let document_hash = blake3_digest(document);
        if signature.proof[0..DOCUMENT_HASH_LENGTH] != document_hash {
            return false;
        }

        let mut proof_stream: ProofStream = signature.proof.clone().into();
        proof_stream.set_index(DOCUMENT_HASH_LENGTH);

        let boundary_constraints = self.rp.get_boundary_constraints(&public_key.value);
        let transition_constraints = self.rp.get_air_constraints(&self.stark.omicron);
        let res = self.stark.verify(
            &mut proof_stream,
            transition_constraints,
            boundary_constraints,
        );

        res.is_ok()
    }

    pub fn sign(&self, sk: &SecretKey, document: &[u8]) -> Result<Signature, Box<dyn Error>> {
        let (output, trace) = self.rp.eval_and_trace(&sk.value);
        let document_hash = blake3_digest(document);
        let mut proof_stream = ProofStream::new_with_prefix(&document_hash);
        self.stark.prove(
            trace,
            self.rp.get_air_constraints(&self.stark.omicron),
            self.rp.get_boundary_constraints(&output),
            &mut proof_stream,
        )?;

        Ok(Signature {
            proof: proof_stream.serialize(),
        })
    }
}

#[cfg(test)]
mod test_rpsss {
    use std::time::Instant;

    use super::super::stark::test_stark;
    use super::*;
    use crate::shared_math::prime_field_element_big::PrimeFieldBig;
    use num_bigint::BigInt;

    #[test]
    fn sign_verify_test() {
        let modulus: BigInt = (407u128 * (1 << 119) + 1).into();
        let field = PrimeFieldBig::new(modulus);
        let (stark, rp) = test_stark::get_tutorial_stark(&field);
        let rpsss = RPSSS {
            field: field.clone(),
            stark,
            rp,
        };
        let document_string: String = "Hello Neptune!".to_string();
        let document: Vec<u8> = document_string.clone().into_bytes();

        let (sk, pk) = rpsss.keygen();
        println!("secret key = {}, public key = {}", sk.value, pk.value);
        let mut start = Instant::now();
        let mut signature: Signature = rpsss.sign(&sk, &document).unwrap();
        let proof_duration = start.elapsed();
        start = Instant::now();
        assert!(rpsss.verify(&pk, &signature, &document));
        let verify_duration = start.elapsed();
        println!("Proof time: {:?}", proof_duration);
        println!("Verification time: {:?}", verify_duration);

        if let Some(last) = signature.proof.last_mut() {
            *last = *last ^ 0x01;
        }

        assert!(!rpsss.verify(&pk, &signature, &document));

        // Set the last byte of the proof stream back and change the
        // first byte and verify that it still fails
        if let Some(last) = signature.proof.last_mut() {
            *last = *last ^ 0x01;
        }
        if let Some(first) = signature.proof.first_mut() {
            *first = *first ^ 0x01;
        }

        assert!(!rpsss.verify(&pk, &signature, &document));

        // A valid proof is rejected if the `document` argument is wrong
        if let Some(first) = signature.proof.first_mut() {
            *first = *first ^ 0x01;
        }
        let bad_document_string: String = "Hello Saturn!".to_string();
        let bad_document: Vec<u8> = bad_document_string.clone().into_bytes();
        assert!(!rpsss.verify(&pk, &signature, &bad_document));
        assert!(rpsss.verify(&pk, &signature, &document));
    }
}
