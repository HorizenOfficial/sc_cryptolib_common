use algebra::{
    ProjectiveCurve, AffineCurve, UniformRand,
};
use primitives::{
    crh::FieldBasedHash,
    signature::{FieldBasedSignatureScheme, schnorr::field_based_schnorr::FieldBasedSchnorrPk}
};

use rand::{
    SeedableRng, rngs::OsRng,
};
use rand_xorshift::XorShiftRng;
use crate::type_mappings::*;

//*******************************Serialization/Deserialization functions ***********************/
use algebra::{serialize::*, SemanticallyValid};
use std::{path::Path, fs::File, io::{Read, BufReader, BufWriter, Cursor, Error as IoError, ErrorKind}};

fn _deserialize_inner<R: Read, T: CanonicalDeserialize + SemanticallyValid>(
    reader:                 R,
    semantic_checks:        Option<bool>,
    compressed:             Option<bool>,
) ->  Result<T, SerializationError>
{
    let semantic_checks = semantic_checks.unwrap_or(false);
    let compressed = compressed.unwrap_or(false);

    let t = if compressed {
        T::deserialize_unchecked(reader)
    } else {
        T::deserialize_uncompressed_unchecked(reader)
    }?;

    if semantic_checks && !t.is_valid() {
        return Err(SerializationError::InvalidData)
    }

    Ok(t)
}

/// Deserialize from `buffer` a compressed or uncompressed element, depending on the value of
/// `compressed` flag, and perform checks on it, depending on the value of `semantic_checks` flag.
/// `compressed` can be optional, due to some types being uncompressable;
/// `semantic_checks` can be optional, due to some types having no checks to be performed,
/// or trivial checks already performed a priori during serialization.
pub fn deserialize_from_buffer<T: CanonicalDeserialize + SemanticallyValid>(
    buffer:                 &[u8],
    semantic_checks:        Option<bool>,
    compressed:             Option<bool>,
) ->  Result<T, SerializationError>
{
    _deserialize_inner(buffer, semantic_checks, compressed)
}

/// Deserialize from `buffer` a compressed or uncompressed element, depending on the value of
/// `compressed` flag, and perform checks on it, depending on the value of `semantic_checks` flag.
/// `compressed` can be optional, due to some types being uncompressable;
/// `semantic_checks` can be optional, due to some types having no checks to be performed,
/// or trivial checks already performed a priori during serialization.
/// If there are still bytes to read in `buffer` after deserializing T, this function returns an error.
pub fn deserialize_from_buffer_strict<T: CanonicalDeserialize + SemanticallyValid>(
    buffer:                 &[u8],
    semantic_checks:        Option<bool>,
    compressed:             Option<bool>,
) ->  Result<T, SerializationError>
{
    // Wrap buffer in a cursor
    let buff_len = buffer.len() as u64;
    let mut buffer = Cursor::new(buffer);

    // Deserialize t
    let t = _deserialize_inner(&mut buffer, semantic_checks, compressed)?;

    let position = buffer.position();
    if position != buff_len {
        return Err(SerializationError::IoError(IoError::new(
            ErrorKind::InvalidInput,
            format!("Oversized data. Read {} but buff len is {}", position, buff_len)
        )));
    }

    Ok(t)
}

/// Serialize to buffer, choosing whether to use compressed representation or not,
/// depending on the value of `compressed` flag.
/// `compressed` can be optional, due to some types being uncompressable.
pub fn serialize_to_buffer<T: CanonicalSerialize>(
    to_write:               &T,
    compressed:             Option<bool>,
) ->  Result<Vec<u8>, SerializationError>
{
    let compressed = compressed.unwrap_or(false);

    let mut buffer;
    if compressed {
        buffer = Vec::with_capacity(to_write.serialized_size());
        CanonicalSerialize::serialize(to_write, &mut buffer)?;
    } else {
        buffer = Vec::with_capacity(to_write.uncompressed_size());
        CanonicalSerialize::serialize_uncompressed(to_write, &mut buffer)?;
    }
    Ok(buffer)
}

pub const DEFAULT_BUF_SIZE: usize = 1 << 20;

/// Deserialize from the file at `file_path` a compressed or uncompressed element,
/// depending on the value of `compressed` flag, and perform checks on it, depending
/// on the value of `semantic_checks` flag.
/// `compressed` can be optional, due to some types being uncompressable;
/// `semantic_checks` can be optional, due to some types having no checks to be performed,
/// or trivial checks already performed a priori during serialization.
pub fn read_from_file<T: CanonicalDeserialize + SemanticallyValid>(
    file_path:              &Path,
    semantic_checks:        Option<bool>,
    compressed:             Option<bool>,
) ->  Result<T, SerializationError>
{
    let fs = File::open(file_path)
        .map_err(|e| SerializationError::IoError(e))?;
    let reader = BufReader::with_capacity(DEFAULT_BUF_SIZE, fs);

    _deserialize_inner(reader, semantic_checks, compressed)
}

/// Serialize to file, choosing whether to use compressed representation or not,
/// depending on the value of `compressed` flag.
/// `compressed` can be optional, due to some types being uncompressable.
pub fn write_to_file<T: CanonicalSerialize>(
    to_write:               &T,
    file_path:              &Path,
    compressed:             Option<bool>,
) ->  Result<(), SerializationError>
{
    let compressed = compressed.unwrap_or(false);

    let fs = File::create(file_path)
        .map_err(|e| SerializationError::IoError(e))?;
    let mut writer = BufWriter::with_capacity(DEFAULT_BUF_SIZE, fs);

    if compressed {
        CanonicalSerialize::serialize(to_write, &mut writer)?;
    } else {
        CanonicalSerialize::serialize_uncompressed(to_write, &mut writer)?;
    }

    writer.flush().map_err(|e| SerializationError::IoError(e))?;
    Ok(())
}

pub fn is_valid<T: SemanticallyValid>(to_check: &T) -> bool {
    T::is_valid(to_check)
}

//*******************************Poseidon Hash functions****************************************


pub fn get_poseidon_hash_constant_length(input_size: usize, personalization: Option<Vec<&FieldElement>>) -> FieldHash {
    if personalization.is_some() {
        FieldHash::init_constant_length(
            input_size,
            Some(personalization.unwrap().into_iter().map(|fe| *fe).collect::<Vec<_>>().as_slice())
        )
    } else {
        FieldHash::init_constant_length(input_size, None)
    }
}

pub fn get_poseidon_hash_variable_length(mod_rate: bool, personalization: Option<Vec<&FieldElement>>) -> FieldHash {
    if personalization.is_some() {
        FieldHash::init_variable_length(
            mod_rate,
            Some(personalization.unwrap().into_iter().map(|fe| *fe).collect::<Vec<_>>().as_slice())
        )
    } else {
        FieldHash::init_variable_length(mod_rate, None)
    }
}

pub fn update_poseidon_hash(hash: &mut FieldHash, input: &FieldElement){
    hash.update(*input);
}

pub fn reset_poseidon_hash(hash: &mut FieldHash, personalization: Option<Vec<&FieldElement>>){
    if personalization.is_some() {
        hash.reset(
            Some(personalization.unwrap().into_iter().map(|fe| *fe).collect::<Vec<_>>().as_slice())
        )
    } else {
        hash.reset(None)
    };
}

pub fn finalize_poseidon_hash(hash: &FieldHash) -> Result<FieldElement, Error> {
    let result = hash.finalize()?;
    Ok(result)
}

// ******************************Merkle Tree functions******************************************

use primitives::{FieldBasedMerkleTree, FieldBasedMerkleTreePath};

pub fn new_ginger_mht(height: usize, processing_step: usize) -> Result<GingerMHT, Error> {
    GingerMHT::init(height, processing_step)
}

pub fn append_leaf_to_ginger_mht(tree: &mut GingerMHT, leaf: &FieldElement) -> Result<(), Error> {
    let _ = tree.append(*leaf)?;
    Ok(())
}

pub fn finalize_ginger_mht(tree: &GingerMHT) -> Result<GingerMHT, Error> {
    tree.finalize()
}

pub fn finalize_ginger_mht_in_place(tree: &mut GingerMHT) -> Result<(), Error>{
    tree.finalize_in_place()?;
    Ok(())
}

pub fn get_ginger_mht_root(tree: &GingerMHT) -> Option<FieldElement> {
    tree.root()
}

pub fn get_ginger_mht_path(tree: &GingerMHT, leaf_index: u64) -> Option<GingerMHTPath> {
    tree.get_merkle_path(leaf_index as usize)
}

pub fn reset_ginger_mht(tree: &mut GingerMHT){
    tree.reset();
}

pub fn verify_ginger_merkle_path(
    path: &GingerMHTPath,
    height: usize,
    leaf: &FieldElement,
    root: &FieldElement
) -> Result<bool, Error> {
    path.verify(height, leaf, root)
}

pub fn verify_ginger_merkle_path_without_length_check(
    path: &GingerMHTPath,
    leaf: &FieldElement,
    root: &FieldElement
) -> bool {
    path.verify_without_length_check(leaf, root)
}

pub fn is_path_leftmost(path: &GingerMHTPath) -> bool {
    path.is_leftmost()
}

pub fn is_path_rightmost(path: &GingerMHTPath) -> bool {
    path.is_rightmost()
}

pub fn are_right_leaves_empty(path: &GingerMHTPath) -> bool { path.are_right_leaves_empty() }

pub fn get_leaf_index_from_path(path: &GingerMHTPath) -> u64 {
    path.leaf_index() as u64
}

pub fn get_root_from_path(path: &GingerMHTPath, leaf: &FieldElement) -> FieldElement
{
    path.compute_root(leaf)
}

//*******************************Generic functions**********************************************

// NOTE: This function relies on a non-cryptographically safe RNG, therefore it
// must be used ONLY for testing purposes
pub fn get_random_field_element(seed: u64) -> FieldElement {
    let mut rng = XorShiftRng::seed_from_u64(seed);
    FieldElement::rand(&mut rng)
}

//***************************Schnorr types and functions********************************************

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

pub fn schnorr_sign(msg: &FieldElement, sk: &SchnorrSk, pk: &SchnorrPk) -> Result<SchnorrSig, Error> {
    let mut rng = OsRng;
    SchnorrSigScheme::sign(&mut rng, &FieldBasedSchnorrPk(pk.into_projective()), sk, msg.clone())
}

pub fn schnorr_verify_signature(msg: &FieldElement, pk: &SchnorrPk, signature: &SchnorrSig) -> Result<bool, Error> {
    SchnorrSigScheme::verify(&FieldBasedSchnorrPk(pk.into_projective()), msg.clone(), signature)
}

// Test functions

pub(crate) fn into_i8(v: Vec<u8>) -> Vec<i8> {
    // first, make sure v's destructor doesn't free the data
    // it thinks it owns when it goes out of scope
    let mut v = std::mem::ManuallyDrop::new(v);

    // then, pick apart the existing Vec
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    // finally, adopt the data into a new Vec
    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

#[cfg(test)]
mod test {
    use super::*;
    use algebra::Field;

    #[test]
    fn sample_calls_schnorr_sig_prove_verify(){
        let mut rng = OsRng;
        let msg = FieldElement::rand(&mut rng);
        {
            let msg_bytes = serialize_to_buffer(&msg, None).unwrap();
            println!("msg bytes: {:?}", into_i8(msg_bytes.clone()));
        }

        let (pk, sk) = schnorr_generate_key(); //Keygen
        assert_eq!(schnorr_get_public_key(&sk), pk); //Get pk
        assert!(schnorr_verify_public_key(&pk)); //Verify pk

        //Serialize/deserialize pk
        let pk_serialized = serialize_to_buffer(&pk, Some(true)).unwrap();
        assert_eq!(pk_serialized.len(), SCHNORR_PK_SIZE);
        let pk_deserialized: SchnorrPk = deserialize_from_buffer(&pk_serialized, Some(true), Some(true)).unwrap();
        assert_eq!(pk, pk_deserialized);

        //Serialize/deserialize sk
        let sk_serialized = serialize_to_buffer(&sk, None).unwrap();
        assert_eq!(sk_serialized.len(), SCHNORR_SK_SIZE);
        println!("sk bytes: {:?}", into_i8(sk_serialized.clone()));
        let sk_deserialized = deserialize_from_buffer(&sk_serialized, None, None).unwrap();
        assert_eq!(sk, sk_deserialized);

        let sig = schnorr_sign(&msg, &sk, &pk).unwrap(); //Sign msg
        assert!(is_valid(&sig));

        //Serialize/deserialize sig
        let sig_serialized = serialize_to_buffer(&sig, None).unwrap();
        println!("sig bytes: {:?}", into_i8(sig_serialized.clone()));
        assert_eq!(sig_serialized.len(), SCHNORR_SIG_SIZE);
        let sig_deserialized = deserialize_from_buffer(&sig_serialized, Some(true), None).unwrap();
        assert_eq!(sig, sig_deserialized);

        assert!(schnorr_verify_signature(&msg, &pk, &sig).unwrap()); //Verify sig

        //Negative case
        let wrong_msg = FieldElement::rand(&mut rng);
        assert!(!schnorr_verify_signature(&wrong_msg, &pk, &sig).unwrap());
    }

    #[test]
    fn sample_calls_merkle_path() {
        let height = 6;
        let leaves_num = 2usize.pow(height as u32);

        // Get GingerMHT
        let mut mht = new_ginger_mht(height, leaves_num).unwrap();

        // Add leaves
        let mut mht_leaves = Vec::with_capacity(leaves_num);
        for i in 0..leaves_num/2 {
            let leaf = get_random_field_element(i as u64);
            mht_leaves.push(leaf);
            append_leaf_to_ginger_mht(&mut mht, &leaf).unwrap();
        }
        for _ in leaves_num/2..leaves_num {
            mht_leaves.push(FieldElement::zero());
        }

        // Compute the root
        finalize_ginger_mht_in_place(&mut mht).unwrap();
        let mht_root = get_ginger_mht_root(&mht).expect("Tree must've been finalized");

        for i in 0..leaves_num {

            //Create and verify merkle paths for each leaf
            let path = get_ginger_mht_path(&mht, i as u64).unwrap();
            assert!(verify_ginger_merkle_path_without_length_check(&path,&mht_leaves[i], &mht_root));

            // Check leaf index is the correct one
            assert_eq!(i as u64, get_leaf_index_from_path(&path));

            if i == 0 { // leftmost check
                assert!(is_path_leftmost(&path));
            }
            else if i == (leaves_num / 2) - 1 { // non-empty rightmost check
                assert!(are_right_leaves_empty(&path));
            }
            else if i == leaves_num - 1 { //rightmost check
                assert!(is_path_rightmost(&path));
            }
            else { // Other cases check
                assert!(!is_path_leftmost(&path));
                assert!(!is_path_rightmost(&path));

                if i < (leaves_num / 2) - 1 {
                    assert!(!are_right_leaves_empty(&path));
                }
            }

            // Serialization/deserialization test
            let path_serialized = serialize_to_buffer(&path, None).unwrap();
            let path_deserialized: GingerMHTPath = deserialize_from_buffer(&path_serialized, Some(true), None).unwrap();
            assert_eq!(path, path_deserialized);
        }
    }

    #[test]
    fn sample_calls_poseidon_hash(){
        let mut rng = OsRng;
        let hash_input = vec![FieldElement::rand(&mut rng); 2];
        let mut h = get_poseidon_hash_variable_length(false, None);

        //Compute poseidon hash
        update_poseidon_hash(&mut h, &hash_input[0]);
        update_poseidon_hash(&mut h, &hash_input[1]);
        let h_output = finalize_poseidon_hash(&h).unwrap();

        //Call to finalize keeps the state
        reset_poseidon_hash(&mut h, None);
        update_poseidon_hash(&mut h, &hash_input[0]);
        finalize_poseidon_hash(&h).unwrap(); //Call to finalize() keeps the state
        update_poseidon_hash(&mut h, &hash_input[1]);
        assert_eq!(h_output, finalize_poseidon_hash(&h).unwrap());

        //finalize() is idempotent
        assert_eq!(h_output, finalize_poseidon_hash(&h).unwrap());
    }
}