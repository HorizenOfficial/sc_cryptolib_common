macro_rules! generate_algebraic_types {
    ($curve: ident, $curve_parameters: ty) => {
        // Basic algebraic types

        pub type FieldElement = <$curve_parameters as ModelParameters>::BaseField;

        pub type ScalarFieldElement = <$curve_parameters as ModelParameters>::ScalarField;

        pub type Group = $curve;

        pub type BigInteger = <<FieldElement as Field>::BasePrimeField as PrimeField>::BigInt;

        // Basic constants

        pub const FIELD_CAPACITY: usize = <FieldElement as PrimeField>::Params::CAPACITY as usize;
        pub const FIELD_SIZE: usize = ((<FieldElement as PrimeField>::Params::MODULUS_BITS
            + <FieldElement as PrimeField>::Params::REPR_SHAVE_BITS)
            as usize)
            / 8;
        pub const SCALAR_FIELD_SIZE: usize =
            ((<ScalarFieldElement as PrimeField>::Params::MODULUS_BITS
                + <ScalarFieldElement as PrimeField>::Params::REPR_SHAVE_BITS)
                / 8) as usize;
        pub const GROUP_SIZE: usize = 2 * FIELD_SIZE + 1;
        // ceil ((MODULUS_BITS + 2 (1 bit for infinity and 1 for odd/even y))/8)
        pub const GROUP_COMPRESSED_SIZE: usize = FIELD_SIZE
            + if <FieldElement as PrimeField>::Params::REPR_SHAVE_BITS >= 2 {
                0
            } else {
                1
            };
    };
}

macro_rules! generate_poseidon_hash_types {
    ($field_hash: ident, $batch_field_hash: ident) => {
        pub type FieldHash = $field_hash;
        pub type BatchFieldHash = $batch_field_hash;
    };
}

macro_rules! generate_merkle_tree_types {
    ($tree_params: ident, $tree_arity: expr) => {
        #[derive(Clone, Debug)]
        pub struct GingerMHTParams;

        impl FieldBasedMerkleTreeParameters for GingerMHTParams {
            type Data = FieldElement;
            type H = FieldHash;
            const MERKLE_ARITY: usize = $tree_arity;
            const ZERO_NODE_CST: Option<
                FieldBasedMerkleTreePrecomputedZeroConstants<'static, Self::H>,
            > = Some($tree_params);
        }

        impl BatchFieldBasedMerkleTreeParameters for GingerMHTParams {
            type BH = BatchFieldHash;
        }

        pub type GingerMHT = FieldBasedOptimizedMHT<GingerMHTParams>;
        pub type GingerMHTPath = FieldBasedBinaryMHTPath<GingerMHTParams>;
    };
}

macro_rules! generate_schnorr_signature_types {
    ($projective_curve: ident, $affine_curve: ident) => {
        pub const SCHNORR_PK_SIZE: usize = GROUP_COMPRESSED_SIZE;
        pub const SCHNORR_SK_SIZE: usize = SCALAR_FIELD_SIZE;
        pub const SCHNORR_SIG_SIZE: usize = 2 * FIELD_SIZE;

        pub type SchnorrSigScheme =
            FieldBasedSchnorrSignatureScheme<FieldElement, $projective_curve, FieldHash>;
        pub type SchnorrSig = FieldBasedSchnorrSignature<FieldElement, $projective_curve>;
        pub type SchnorrPk = $affine_curve;
        pub type SchnorrSk = ScalarFieldElement;
    };
}
