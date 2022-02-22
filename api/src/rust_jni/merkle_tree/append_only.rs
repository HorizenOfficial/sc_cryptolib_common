use super::*;

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeInit(
        _env: JNIEnv,
        _class: JClass,
        _height: jint,
        _processing_step: jlong,
    ) -> jobject {
        // Create new BaseAppendOnlyMerkleTree Rust side
        map_to_jobject_or_throw_exc(
            _env,
            new_ginger_mht(_height as usize, _processing_step as usize),
            "io/horizen/common/merkletreenative/BaseAppendOnlyMerkleTree",
            "io/horizen/common/librustsidechains/InitializationException",
            "Unable to inizialize BaseAppendOnlyMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeAppend(
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
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to append leaf to BaseAppendOnlyMerkleTree"
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeFinalize(
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
            "io/horizen/common/merkletreenative/BaseAppendOnlyMerkleTree",
            "io/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize BaseAppendOnlyMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeFinalizeInPlace(
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
            "io/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize BaseAppendOnlyMerkleTree in place"
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeRoot(
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
            "io/horizen/common/librustsidechains/FieldElement",
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get BaseAppendOnlyMerkleTree root",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeGetMerklePath(
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
            "io/horizen/common/merkletreenative/FieldBasedMerklePath",
            "io/horizen/common/merkletreenative/MerkleTreeException",
            "Unable to get MerklePath from BaseAppendOnlyMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeGetLeafIndex(
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
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeSerialize(
        _env: JNIEnv,
        _tree: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<GingerMHT>(_env, _tree, "inMemoryOptimizedMerkleTreePointer", None)
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeDeserialize(
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
            "io/horizen/common/merkletreenative/BaseAppendOnlyMerkleTree",
            "io/horizen/common/librustsidechains/DeserializationException",
            "Unable to deserialize BaseAppendOnlyMerkleTree",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeReset(
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
    fn Java_io_horizen_common_merkletreenative_BaseAppendOnlyMerkleTree_nativeFreeMerkleTree(
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