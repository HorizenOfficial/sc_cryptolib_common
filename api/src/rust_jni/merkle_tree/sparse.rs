use std::collections::{HashMap, HashSet};
use jni::{objects::JMap, sys::jlongArray};

use super::*;

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeInit(
        _env: JNIEnv,
        _class: JClass,
        _height: jint,
    ) -> jobject {
        // Create new BaseSparseMerkleTree Rust side
        return_jobject(
            &_env,
            new_ginger_sparse_mht(_height as u8),
            "io/horizen/common/merkletreenative/BaseSparseMerkleTree"
        ).into_inner()
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeAddLeaves(
        _env: JNIEnv,
        _tree: JObject,
        _leaves: JObject,
    ) {
        //Read _leaves as HashMap<u32, FieldElement>
        let leaves_map = JMap::from_env(&_env, _leaves)
            .expect("Should be able to construct JMap from _leaves JObject");
        let mut leaves = HashMap::new();

        for (pos, fe) in leaves_map
            .iter()
            .expect("Should be able to get JMap iterator")
        {
            // Read FieldElement
            let field = parse_rust_struct_from_jobject::<FieldElement>(&_env, fe, "fieldElementPointer");

            // Read position
            let position = parse_long_from_jobject(&_env, pos, "value") as u32;

            leaves.insert(position, *field);
        }

        // Read tree
        let tree = parse_mut_rust_struct_from_jobject::<GingerSparseMHT>(&_env, _tree, "merkleTreePointer");

        // Update the tree with leaves
        ok_or_throw_exc!(
            _env,
            insert_leaves_to_ginger_sparse_mht(tree, leaves),
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to insert leaves to BaseSparseMerkleTree"
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeRemoveLeaves(
        _env: JNIEnv,
        _tree: JObject,
        _positions: jlongArray,
    ) {

        //Read _positions as an array of jlongs
        let positions_len = _env
            .get_array_length(_positions)
            .expect("Should be able to read positions array size");
        let mut positions = HashSet::new();

        // Array can be empty
        for i in 0..positions_len {
            let long_obj = _env
                .get_object_array_element(_positions, i)
                .unwrap_or_else(|_| {
                    panic!("Should be able to read elem {} of the positions array", i)
                });

            // Read position
            let position = parse_long_from_jobject(&_env, long_obj, "value") as u32;

            positions.insert(position);
        }

        // Read tree
        let tree = parse_mut_rust_struct_from_jobject::<GingerSparseMHT>(&_env, _tree, "merkleTreePointer");

        // Update the tree with leaves
        ok_or_throw_exc!(
            _env,
            remove_leaves_from_ginger_sparse_mht(tree, positions),
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to remove leaves from BaseSparseMerkleTree"
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeIsPositionEmpty(
        _env: JNIEnv,
        _tree: JObject,
        _position: jlong,
    ) -> jboolean {
        // Read tree
        let tree = parse_rust_struct_from_jobject::<GingerSparseMHT>(&_env, _tree, "merkleTreePointer");

        // Call corresponding function and return result if Ok(), otherwise throw Exception
        map_to_jboolean_or_throw_exc(
            _env,
            is_leaf_empty_in_ginger_sparse_mht(tree, _position as u32),
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to check for empty position in BaseSparseMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeFinalize(
        _env: JNIEnv,
        _tree: JObject,
    ) -> jobject {
        let tree = parse_rust_struct_from_jobject::<GingerSparseMHT>(
            &_env,
            _tree,
            "merkleTreePointer",
        );

        map_to_jobject_or_throw_exc(
            _env,
            finalize_ginger_sparse_mht(tree),
            "io/horizen/common/merkletreenative/BaseSparseMerkleTree",
            "io/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize BaseSparseMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeFinalizeInPlace(
        _env: JNIEnv,
        _tree: JObject,
    ) {
        let tree = parse_mut_rust_struct_from_jobject::<GingerSparseMHT>(
            &_env,
            _tree,
            "merkleTreePointer",
        );

        ok_or_throw_exc!(
            _env,
            finalize_ginger_sparse_mht_in_place(tree),
            "io/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize BaseSparseMerkleTree in place"
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeRoot(
        _env: JNIEnv,
        _tree: JObject,
    ) -> jobject {
        let tree = parse_rust_struct_from_jobject::<GingerSparseMHT>(
            &_env,
            _tree,
            "merkleTreePointer",
        );

        map_to_jobject_or_throw_exc(
            _env,
            get_ginger_sparse_mht_root(tree),
            "io/horizen/common/librustsidechains/FieldElement",
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get BaseSparseMerkleTree root",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeGetMerklePath(
        _env: JNIEnv,
        _tree: JObject,
        _leaf_index: jlong,
    ) -> jobject {
        let tree = parse_rust_struct_from_jobject::<GingerSparseMHT>(
            &_env,
            _tree,
            "merkleTreePointer",
        );

        map_to_jobject_or_throw_exc(
            _env,
            get_ginger_sparse_mht_path(tree, _leaf_index as u32),
            "io/horizen/common/merkletreenative/FieldBasedMerklePath",
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get MerklePath from BaseSparseMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeGetLeafIndex(
        _env: JNIEnv,
        _tree: JObject,
        _leaf: JObject,
    ) -> jlong {
        let leaf =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _leaf, "fieldElementPointer");

        let tree = parse_rust_struct_from_jobject::<GingerSparseMHT>(
            &_env,
            _tree,
            "merkleTreePointer",
        );

        // Check if element is in tree and if yes get its index
        let maybe_idx = ok_or_throw_exc!(
            &_env,
            get_leaf_index_from_ginger_sparse_mht(tree, leaf),
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get leaf index",
            -1
        );

        match maybe_idx {
            Some(idx) => idx as jlong,
            None => -1,
        }
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeReset(
        _env: JNIEnv,
        _tree: JObject,
    ) {
        let tree = parse_mut_rust_struct_from_jobject::<GingerSparseMHT>(
            &_env,
            _tree,
            "merkleTreePointer",
        );

        reset_ginger_sparse_mht(tree);
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseSparseMerkleTree_nativeFreeBaseSparseMerkleTree(
        _env: JNIEnv,
        _tree: JObject,
    ) {
        drop_rust_struct_from_jobject::<GingerSparseMHT>(
            _env,
            _tree,
            "merkleTreePointer",
        )
    }
);