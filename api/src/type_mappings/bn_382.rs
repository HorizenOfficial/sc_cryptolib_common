use {
    algebra::curves::bn_382::g::{
        Affine as Bn382DualAffine, Projective as Bn382DualProjective, Bn382GParameters as Bn382DualParameters
    },
    primitives::crh::poseidon::parameters::bn382::{BN382FrPoseidonHash, BN382FrBatchPoseidonHash},
    primitives::merkle_tree::bn382::BN382_MHT_POSEIDON_PARAMETERS,
};

use algebra::{Field, FpParameters, ModelParameters, PrimeField,};
use primitives::{
    merkle_tree::*,
    signature::{
    schnorr::field_based_schnorr::{
        FieldBasedSchnorrSignatureScheme, FieldBasedSchnorrSignature,
    },
}};

pub type Error = Box<dyn std::error::Error>;

generate_algebraic_types!(Bn382DualAffine, Bn382DualParameters);
generate_poseidon_hash_types!(BN382FrPoseidonHash, BN382FrBatchPoseidonHash);
generate_merkle_tree_types!(BN382_MHT_POSEIDON_PARAMETERS, 2);
generate_schnorr_signature_types!(Bn382DualProjective, Bn382DualAffine);
