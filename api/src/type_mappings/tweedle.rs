use {
    algebra::curves::tweedle::dum::{
        Affine as DumAffine, Projective as DumProjective, TweedledumParameters,
    },
    primitives::crh::poseidon::parameters::tweedle_dee::{
        TweedleFrBatchPoseidonHash, TweedleFrPoseidonHash,
    },
    primitives::merkle_tree::tweedle_dee::TWEEDLE_DEE_MHT_POSEIDON_PARAMETERS,
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
    DumAffine, DumProjective, TweedledumParameters, TweedleFrPoseidonHash,
    TweedleFrBatchPoseidonHash, TWEEDLE_DEE_MHT_POSEIDON_PARAMETERS, 2
);