use super::*;

pub mod field_element;
pub mod merkle_tree;
pub mod poseidon_hash;
pub mod schnorr_signature;
pub mod serialization;
pub mod vrf;

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
    use algebra::{Field, UniformRand};
    use field_element::*;
    use rand::rngs::OsRng;
    use serialization::*;

    #[test]
    fn sample_calls_schnorr_sig_prove_verify() {
        use schnorr_signature::*;

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
        let pk_deserialized: SchnorrPk =
            deserialize_from_buffer(&pk_serialized, Some(true), Some(true)).unwrap();
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
        use merkle_tree::*;

        let height = 6;
        let leaves_num = 2usize.pow(height as u32);

        // Get GingerMHT
        let mut mht = new_ginger_mht(height, leaves_num).unwrap();

        // Add leaves
        let mut mht_leaves = Vec::with_capacity(leaves_num);
        for i in 0..leaves_num / 2 {
            let leaf = get_random_field_element(i as u64);
            mht_leaves.push(leaf);
            append_leaf_to_ginger_mht(&mut mht, &leaf).unwrap();
        }
        for _ in leaves_num / 2..leaves_num {
            mht_leaves.push(FieldElement::zero());
        }

        // Compute the root
        finalize_ginger_mht_in_place(&mut mht).unwrap();
        let mht_root = get_ginger_mht_root(&mht).expect("Tree must've been finalized");

        for i in 0..leaves_num {
            //Create and verify merkle paths for each leaf
            let path = get_ginger_mht_path(&mht, i as u64).unwrap();
            assert!(verify_ginger_merkle_path_without_length_check(
                &path,
                &mht_leaves[i],
                &mht_root
            ));

            // Check leaf index is the correct one
            assert_eq!(i as u64, get_leaf_index_from_path(&path));

            if i == 0 {
                // leftmost check
                assert!(is_path_leftmost(&path));
            } else if i == (leaves_num / 2) - 1 {
                // non-empty rightmost check
                assert!(are_right_leaves_empty(&path));
            } else if i == leaves_num - 1 {
                //rightmost check
                assert!(is_path_rightmost(&path));
            } else {
                // Other cases check
                assert!(!is_path_leftmost(&path));
                assert!(!is_path_rightmost(&path));

                if i < (leaves_num / 2) - 1 {
                    assert!(!are_right_leaves_empty(&path));
                }
            }

            // Serialization/deserialization test
            let path_serialized = serialize_to_buffer(&path, None).unwrap();
            let path_deserialized: GingerMHTPath =
                deserialize_from_buffer(&path_serialized, Some(true), None).unwrap();
            assert_eq!(path, path_deserialized);
        }
    }

    #[test]
    fn sample_calls_poseidon_hash() {
        use poseidon_hash::*;

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

    #[test]
    fn sample_calls_vrf_prove_verify() {
        use vrf::*;

        let mut rng = OsRng;
        let msg = FieldElement::rand(&mut rng);
        {
            let msg_bytes = serialize_to_buffer(&msg, None).unwrap();
            println!("msg bytes: {:?}", into_i8(msg_bytes.clone()));
        }

        let (pk, sk) = vrf_generate_key(); //Keygen
        assert_eq!(vrf_get_public_key(&sk), pk); //Get pk
        assert!(vrf_verify_public_key(&pk)); //Verify pk

        //Serialize/deserialize pk
        let pk_serialized = serialize_to_buffer(&pk, Some(true)).unwrap();
        assert_eq!(pk_serialized.len(), VRF_PK_SIZE);
        let pk_deserialized: VRFPk =
            deserialize_from_buffer(&pk_serialized, Some(true), Some(true)).unwrap();
        assert_eq!(pk, pk_deserialized);

        //Serialize/deserialize sk
        let sk_serialized = serialize_to_buffer(&sk, None).unwrap();
        assert_eq!(sk_serialized.len(), VRF_SK_SIZE);
        println!("sk bytes: {:?}", into_i8(sk_serialized.clone()));
        let sk_deserialized = deserialize_from_buffer(&sk_serialized, None, None).unwrap();
        assert_eq!(sk, sk_deserialized);

        let (vrf_proof, vrf_out) = vrf_prove(&msg, &sk, &pk).unwrap(); //Create vrf proof for msg
        assert!(is_valid(&vrf_proof));

        //Serialize/deserialize vrf proof
        let proof_serialized = serialize_to_buffer(&vrf_proof, Some(true)).unwrap();
        assert_eq!(proof_serialized.len(), VRF_PROOF_SIZE);
        println!("proof bytes: {:?}", into_i8(proof_serialized.clone()));
        let proof_deserialized =
            deserialize_from_buffer(&proof_serialized, Some(true), Some(true)).unwrap();
        assert_eq!(vrf_proof, proof_deserialized);

        //Serialize/deserialize vrf out (i.e. a field element)
        let vrf_out_serialized = serialize_to_buffer(&vrf_out, None).unwrap();
        println!("vrf out bytes: {:?}", into_i8(vrf_out_serialized.clone()));
        let vrf_out_deserialized =
            deserialize_from_buffer(&vrf_out_serialized, None, None).unwrap();
        assert_eq!(vrf_out, vrf_out_deserialized);

        let vrf_out_dup = vrf_proof_to_hash(&msg, &pk, &vrf_proof).unwrap(); //Verify vrf proof and get vrf out for msg
        assert_eq!(vrf_out, vrf_out_dup);

        //Negative case
        let wrong_msg = FieldElement::rand(&mut rng);
        assert!(vrf_proof_to_hash(&wrong_msg, &pk, &vrf_proof).is_err());
    }
}
