use {
    algebra::{
        curves::tweedle::dum::{Affine as DumAffine, Projective as DumProjective, TweedledumParameters},
    },
    primitives::crh::poseidon::parameters::tweedle_dee::{TweedleFrPoseidonHash, TweedleFrBatchPoseidonHash},
    primitives::merkle_tree::tweedle_dee::TWEEDLE_DEE_MHT_POSEIDON_PARAMETERS,
};

use algebra::{Field, FpParameters, ModelParameters, PrimeField,};
use primitives::{
    merkle_tree::*,
    signature::{
        schnorr::field_based_schnorr::{
            FieldBasedSchnorrSignatureScheme, FieldBasedSchnorrSignature,
        },
    }
};

pub type Error = Box<dyn std::error::Error>;

generate_algebraic_types!(DumAffine, TweedledumParameters);
generate_poseidon_hash_types!(TweedleFrPoseidonHash, TweedleFrBatchPoseidonHash);
generate_merkle_tree_types!(TWEEDLE_DEE_MHT_POSEIDON_PARAMETERS, 2);
generate_schnorr_signature_types!(DumProjective, DumAffine);