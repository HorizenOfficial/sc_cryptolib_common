use super::*;

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeGetHashSize(
        _env: JNIEnv,
        _class: JClass,
    ) -> jint {
        FIELD_SIZE as jint
    }
);

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeGetConstantLengthPoseidonHash(
        _env: JNIEnv,
        _class: JClass,
        _input_size: jint,
        _personalization: jobjectArray,
    ) -> jobject {
        // Read personalization as vector of field element
        let mut personalization = vec![];

        parse_rust_struct_vec_from_jobject_array!(
            _env,
            _personalization,
            personalization,
            "personalization",
            "fieldElementPointer"
        );

        //Instantiate PoseidonHash
        let h = get_poseidon_hash_constant_length(
            _input_size as usize,
            if personalization.is_empty() {
                None
            } else {
                Some(personalization)
            },
        );

        //Return PoseidonHash instance
        return_jobject(&_env, h, "com/horizen/common/poseidonnative/PoseidonHash").into_inner()
    }
);

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeGetVariableLengthPoseidonHash(
        _env: JNIEnv,
        _class: JClass,
        _mod_rate: jboolean,
        _personalization: jobjectArray,
    ) -> jobject {
        // Read personalization as vector of field element
        let mut personalization = vec![];

        parse_rust_struct_vec_from_jobject_array!(
            _env,
            _personalization,
            personalization,
            "personalization",
            "fieldElementPointer"
        );

        //Instantiate PoseidonHash
        let h = get_poseidon_hash_variable_length(
            _mod_rate == JNI_TRUE,
            if personalization.is_empty() {
                None
            } else {
                Some(personalization)
            },
        );

        //Return PoseidonHash instance
        return_jobject(&_env, h, "com/horizen/common/poseidonnative/PoseidonHash").into_inner()
    }
);

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeUpdate(
        _env: JNIEnv,
        _h: JObject,
        _input: JObject,
    ) {
        //Read PoseidonHash instance
        let digest =
            parse_mut_rust_struct_from_jobject::<FieldHash>(&_env, _h, "poseidonHashPointer");

        //Read input
        let input =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _input, "fieldElementPointer");

        update_poseidon_hash(digest, input);
    }
);

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeFinalize(
        _env: JNIEnv,
        _h: JObject,
    ) -> jobject {
        //Read PoseidonHash instance
        let digest = parse_rust_struct_from_jobject::<FieldHash>(&_env, _h, "poseidonHashPointer");

        //Get digest
        map_to_jobject_or_throw_exc(
            _env,
            finalize_poseidon_hash(digest),
            "com/horizen/common/librustsidechains/FieldElement",
            "com/horizen/common/librustsidechains/FinalizationException",
            "Unable to finalize hash",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeReset(
        _env: JNIEnv,
        _h: JObject,
        _personalization: jobjectArray,
    ) {
        //Read PoseidonHash instance
        let digest =
            parse_mut_rust_struct_from_jobject::<FieldHash>(&_env, _h, "poseidonHashPointer");

        // Read personalization as vector of field element
        let mut personalization = vec![];

        parse_rust_struct_vec_from_jobject_array!(
            _env,
            _personalization,
            personalization,
            "personalization",
            "fieldElementPointer"
        );

        let personalization = if personalization.is_empty() {
            None
        } else {
            Some(personalization)
        };

        reset_poseidon_hash(digest, personalization)
    }
);

ffi_export!(
    fn Java_com_horizen_common_poseidonnative_PoseidonHash_nativeFreePoseidonHash(
        _env: JNIEnv,
        _h: JObject,
    ) {
        drop_rust_struct_from_jobject::<FieldHash>(_env, _h, "poseidonHashPointer")
    }
);
