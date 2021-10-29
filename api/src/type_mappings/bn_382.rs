use {
    algebra::curves::bn_382::{
        g::{
            Affine as Bn382DualAffine, Bn382GParameters as Bn382DualParameters,
            Projective as Bn382DualProjective,
        },
        Bn382
    },
    primitives::crh::poseidon::parameters::bn382::{BN382FrBatchPoseidonHash, BN382FrPoseidonHash},
    primitives::merkle_tree::bn382::BN382_MHT_POSEIDON_PARAMETERS,
    type_mappings::*,
};

generate_all_types_and_functions!(
    Bn382DualAffine,
    Bn382DualProjective,
    Bn382DualParameters,
    BN382FrPoseidonHash,
    BN382FrBatchPoseidonHash,
    BN382_MHT_POSEIDON_PARAMETERS,
    2
);

#[cfg(feature = "groth16")]
generate_groth16_functions!(Bn382);