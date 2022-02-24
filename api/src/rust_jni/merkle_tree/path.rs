use super::*;

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeGetLength(
        _env: JNIEnv,
        _path: JObject,
    ) -> jint 
    {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        path.get_length() as jint
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeVerify(
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
            throw!(&_env, "io/horizen/common/merkletreenative/MerklePathException", "Invalid Path", JNI_FALSE);
        }

        verify_ginger_merkle_path_without_length_check(path, leaf, root) as jboolean
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeApply(
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
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeIsLeftmost(
        _env: JNIEnv,
        _path: JObject,
    ) -> jboolean {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        is_path_leftmost(path) as jboolean
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeIsRightmost(
        _env: JNIEnv,
        _path: JObject,
    ) -> jboolean {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        is_path_rightmost(path) as jboolean
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeAreRightLeavesEmpty(
        _env: JNIEnv,
        _path: JObject,
    ) -> jboolean {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        are_right_leaves_empty(path) as jboolean
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeLeafIndex(
        _env: JNIEnv,
        _path: JObject,
    ) -> jlong {
        let path =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path, "merklePathPointer");

        get_leaf_index_from_path(path) as jlong
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeSerialize(
        _env: JNIEnv,
        _path: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<GingerMHTPath>(_env, _path, "merklePathPointer", None)
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeDeserialize(
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
            "io/horizen/common/merkletreenative/FieldBasedMerklePath",
            "io/horizen/common/librustsidechains/DeserializationException",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeEquals(
        _env: JNIEnv,
        _path_1: JObject,
        _path_2: JObject,
    ) -> jboolean {
        let path_1 =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path_1, "merklePathPointer");

        let path_2 =
            parse_rust_struct_from_jobject::<GingerMHTPath>(&_env, _path_2, "merklePathPointer");

        match path_1 == path_2 {
            true => JNI_TRUE,
            false => JNI_FALSE,
        }
    }
);

ffi_export!(
    fn Java_io_horizen_common_merkletreenative_FieldBasedMerklePath_nativeFreeMerklePath(
        _env: JNIEnv,
        _path: JObject,
    ) {
        drop_rust_struct_from_jobject::<GingerMHTPath>(_env, _path, "merklePathPointer")
    }
);