use {
    algebra::curves::tweedle::dum::{
        Affine as DumAffine, Projective as DumProjective, TweedledumParameters,
    },
    primitives::crh::poseidon::parameters::tweedle_dee::{
        TweedleFrBatchPoseidonHash, TweedleFrPoseidonHash,
    },
    primitives::merkle_tree::tweedle_dee::TWEEDLE_DEE_MHT_POSEIDON_PARAMETERS,
};

use crate::hash_to_curve;
use algebra::{AffineCurve, Field, FpParameters, ModelParameters, PrimeField, ProjectiveCurve};
use lazy_static::lazy_static;
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

pub type Error = Box<dyn std::error::Error>;

generate_all_algebraic_crypto_types!(
    DumAffine,
    DumProjective,
    TweedledumParameters,
    TweedleFrPoseidonHash,
    TweedleFrBatchPoseidonHash,
    TWEEDLE_DEE_MHT_POSEIDON_PARAMETERS,
    2
);
