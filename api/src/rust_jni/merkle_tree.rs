use super::*;
use crate::ginger_calls::merkle_tree::*;
use algebra::{CanonicalDeserialize, SemanticallyValid};

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeVerify(
        _env: JNIEnv,
        _path: JObject,
        _height: jint,
        _leaf: JObject,
        _root: JObject,
    ) -> jboolean {
        let leaf =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _leaf, "fieldElementPointer");

        let root =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _root, "fieldElementPointer");

        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        if !path.is_valid() {
            return JNI_FALSE;
        }

        map_to_jboolean_or_throw_exc(
            _env,
            verify_ginger_merkle_path(path, _height as usize, leaf, root),
            "com/horizen/common/merkletreenative/MerklePathException",
            "Unable to verify MerklePath",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeVerifyWithoutLengthCheck(
        _env: JNIEnv,
        _path: JObject,
        _leaf: JObject,
        _root: JObject,
    ) -> jboolean {
        let leaf =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _leaf, "fieldElementPointer");

        let root =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _root, "fieldElementPointer");

        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        if !path.is_valid() {
            return JNI_FALSE;
        }

        if verify_ginger_merkle_path_without_length_check(path, leaf, root) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeApply(
        _env: JNIEnv,
        _path: JObject,
        _leaf: JObject,
    ) -> jobject {
        let leaf =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _leaf, "fieldElementPointer");

        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        let root = get_root_from_path(path, leaf);

        return_field_element(_env, root)
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeIsLeftmost(
        _env: JNIEnv,
        _path: JObject,
    ) -> jboolean {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        is_path_leftmost(path) as jboolean
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeIsRightmost(
        _env: JNIEnv,
        _path: JObject,
    ) -> jboolean {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        is_path_rightmost(path) as jboolean
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeAreRightLeavesEmpty(
        _env: JNIEnv,
        _path: JObject,
    ) -> jboolean {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        are_right_leaves_empty(path) as jboolean
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeLeafIndex(
        _env: JNIEnv,
        _path: JObject,
    ) -> jlong {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        get_leaf_index_from_path(path) as jlong
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeSerialize(
        _env: JNIEnv,
        _path: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<GingerMHTPath>(_env, _path, "merklePathPointer", None)
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeDeserialize(
        _env: JNIEnv,
        _class: JClass,
        _path_bytes: jbyteArray,
        _checked: jboolean,
    ) -> jobject {
        deserialize_to_jobject::<GingerMHTPath>(
            _env,
            _path_bytes,
            Some(_checked),
            None,
            "com/horizen/common/merkletreenative/FieldBasedMerklePath",
            "com/horizen/common/librustsidechains/DeserializationException",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_FieldBasedMerklePath_nativeFreeMerklePath(
        _env: JNIEnv,
        _path: JObject,
    ) {
        drop_rust_struct_from_jobject::<GingerMHTPath>(_env, _path, "merklePathPointer")
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeInit(
        _env: JNIEnv,
        _class: JClass,
        _height: jint,
        _processing_step: jlong,
    ) -> jobject {
        // Create new BaseMerkleTree Rust side
        map_to_jobject_or_throw_exc(
            _env,
            new_ginger_mht(_height as usize, _processing_step as usize),
            "com/horizen/common/merkletreenative/BaseMerkleTree",
            "com/horizen/common/librustsidechains/InitializationException",
            "Unable to inizialize MerkleTree",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeAppend(
        _env: JNIEnv,
        _tree: JObject,
        _leaf: JObject,
    ) {
        let leaf =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _leaf, "fieldElementPointer");

        let tree = parse_mut_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        ok_or_throw_exc!(
            _env,
            append_leaf_to_ginger_mht(tree, leaf),
            "com/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to append leaf to MerkleTree"
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeFinalize(
        _env: JNIEnv,
        _tree: JObject,
    ) -> jobject {
        let tree = parse_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        map_to_jobject_or_throw_exc(
            _env,
            finalize_ginger_mht(tree),
            "com/horizen/common/merkletreenative/BaseMerkleTree",
            "com/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize MerkleTree",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeFinalizeInPlace(
        _env: JNIEnv,
        _tree: JObject,
    ) {
        let tree = parse_mut_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        ok_or_throw_exc!(
            _env,
            finalize_ginger_mht_in_place(tree),
            "com/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize MerkleTree in place"
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeRoot(
        _env: JNIEnv,
        _tree: JObject,
    ) -> jobject {
        let tree = parse_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        map_to_jobject_or_throw_exc(
            _env,
            get_ginger_mht_root(tree),
            "com/horizen/common/librustsidechains/FieldElement",
            "com/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get MerkleTree root",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeGetMerklePath(
        _env: JNIEnv,
        _tree: JObject,
        _leaf_index: jlong,
    ) -> jobject {
        let tree = parse_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        map_to_jobject_or_throw_exc(
            _env,
            get_ginger_mht_path(tree, _leaf_index as u64),
            "com/horizen/common/merkletreenative/FieldBasedMerklePath",
            "com/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get MerklePath",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeGetLeafIndex(
        _env: JNIEnv,
        _tree: JObject,
        _leaf: JObject,
    ) -> jlong {
        let leaf =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _leaf, "fieldElementPointer");

        let tree = parse_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        // Check if element is in tree and if yes get its index
        match get_leaf_index(tree, leaf) {
            Some(idx) => idx as jlong,
            None => -1,
        }
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeSerialize(
        _env: JNIEnv,
        _tree: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<GingerMHT>(_env, _tree, "inMemoryOptimizedMerkleTreePointer", None)
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeDeserialize(
        _env: JNIEnv,
        _class: JClass,
        _tree_bytes: jbyteArray,
    ) -> jobject {
        // Not really necessary to do deep checks: no one can trick a node into using different _tree_bytes
        // possibly deserializing an unconsistent tree that may lead to internal crashes (DOS).
        // TODO: Is this true ?
        let obj_bytes = _env
            .convert_byte_array(_tree_bytes)
            .expect("Cannot read tree bytes.");

        map_to_jobject_or_throw_exc(
            _env,
            <GingerMHT as CanonicalDeserialize>::deserialize(obj_bytes.as_slice()),
            "com/horizen/common/merkletreenative/BaseMerkleTree",
            "com/horizen/common/librustsidechains/DeserializationException",
            "Unable to deserialize MerkleTree",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeReset(
        _env: JNIEnv,
        _tree: JObject,
    ) {
        let tree = parse_mut_rust_struct_from_jobject::<GingerMHT>(
            &_env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        );

        reset_ginger_mht(tree);
    }
);

ffi_export!(
    fn Java_com_horizen_common_merkletreenative_BaseMerkleTree_nativeFreeMerkleTree(
        _env: JNIEnv,
        _tree: JObject,
    ) {
        drop_rust_struct_from_jobject::<GingerMHT>(
            _env,
            _tree,
            "inMemoryOptimizedMerkleTreePointer",
        )
    }
);
