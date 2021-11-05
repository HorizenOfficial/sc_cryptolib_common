use super::*;
use algebra::{AffineCurve, ProjectiveCurve};
use primitives::{schnorr::field_based_schnorr::*, signature::FieldBasedSignatureScheme};
use rand::rngs::OsRng;

pub fn schnorr_generate_key() -> (SchnorrPk, SchnorrSk) {
    let mut rng = OsRng;
    let (pk, sk) = SchnorrSigScheme::keygen(&mut rng);
    (pk.0.into_affine(), sk)
}

pub fn schnorr_get_public_key(sk: &SchnorrSk) -> SchnorrPk {
    SchnorrSigScheme::get_public_key(sk).0.into_affine()
}

pub fn schnorr_verify_public_key(pk: &SchnorrPk) -> bool {
    SchnorrSigScheme::keyverify(&FieldBasedSchnorrPk(pk.into_projective()))
}

pub fn schnorr_sign(
    msg: &FieldElement,
    sk: &SchnorrSk,
    pk: &SchnorrPk,
) -> Result<SchnorrSig, Error> {
    let mut rng = OsRng;
    SchnorrSigScheme::sign(
        &mut rng,
        &FieldBasedSchnorrPk(pk.into_projective()),
        sk,
        *msg,
    )
}

pub fn schnorr_verify_signature(
    msg: &FieldElement,
    pk: &SchnorrPk,
    signature: &SchnorrSig,
) -> Result<bool, Error> {
    SchnorrSigScheme::verify(&FieldBasedSchnorrPk(pk.into_projective()), *msg, signature)
}
