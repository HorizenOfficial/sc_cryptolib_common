use {
    algebra::curves::bn_382::g::{
        Affine as Bn382DualAffine, Bn382GParameters as Bn382DualParameters,
        Projective as Bn382DualProjective,
    },
    primitives::crh::poseidon::parameters::bn382::{BN382FrBatchPoseidonHash, BN382FrPoseidonHash},
    primitives::merkle_tree::bn382::BN382_MHT_POSEIDON_PARAMETERS,
};

use algebra::{Field, FpParameters, ModelParameters, PrimeField, AffineCurve, ProjectiveCurve};
use primitives::{
    crh::{
        bowe_hopwood::{BoweHopwoodPedersenCRH, BoweHopwoodPedersenParameters},
        pedersen::PedersenWindow, 
    },
    merkle_tree::*,
    signature::schnorr::field_based_schnorr::{
        FieldBasedSchnorrSignature, FieldBasedSchnorrSignatureScheme,
    },
    vrf::ecvrf::{FieldBasedEcVrf, FieldBasedEcVrfProof},
};
use crate::hash_to_curve;
use lazy_static::lazy_static;

pub type Error = Box<dyn std::error::Error>;

generate_all_algebraic_crypto_types!(
    Bn382DualAffine, Bn382DualProjective, Bn382DualParameters, BN382FrPoseidonHash,
    BN382FrBatchPoseidonHash, BN382_MHT_POSEIDON_PARAMETERS, 2
);