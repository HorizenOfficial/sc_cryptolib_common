use super::*;
use crate::ginger_calls::poseidon_hash::*;

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeGetHashSize(
        _env: JNIEnv,
        _class: JClass,
    ) -> jint {
        FIELD_SIZE as jint
    }
);

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeGetConstantLengthPoseidonHash(
        _env: JNIEnv,
        _class: JClass,
        _input_size: jint,
        _personalization: jobjectArray,
    ) -> jobject {
        //Read _personalization as array of FieldElement
        let personalization_len = _env
            .get_array_length(_personalization)
            .expect("Should be able to read personalization array size");
        let mut personalization = vec![];

        // Array can be empty
        for i in 0..personalization_len {
            let field_obj = _env.get_object_array_element(_personalization, i).expect(
                format!(
                    "Should be able to read elem {} of the personalization array",
                    i
                )
                .as_str(),
            );

            let field = parse_rust_struct_from_jobject::<FieldElement>(
                &_env,
                field_obj,
                "fieldElementPointer",
            );

            personalization.push(field);
        }

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
        return_jobject(&_env, h, "com/horizen/poseidonnative/PoseidonHash").into_inner()
    }
);

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeGetVariableLengthPoseidonHash(
        _env: JNIEnv,
        _class: JClass,
        _mod_rate: jboolean,
        _personalization: jobjectArray,
    ) -> jobject {
        //Read _personalization as array of FieldElement
        let personalization_len = _env
            .get_array_length(_personalization)
            .expect("Should be able to read personalization array size");
        let mut personalization = vec![];

        // Array can be empty
        for i in 0..personalization_len {
            let field_obj = _env.get_object_array_element(_personalization, i).expect(
                format!(
                    "Should be able to read elem {} of the personalization array",
                    i
                )
                .as_str(),
            );

            let field = parse_rust_struct_from_jobject::<FieldElement>(
                &_env,
                field_obj,
                "fieldElementPointer",
            );

            personalization.push(field);
        }

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
        return_jobject(&_env, h, "com/horizen/poseidonnative/PoseidonHash").into_inner()
    }
);

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeUpdate(
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
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeFinalize(
        _env: JNIEnv,
        _h: JObject,
    ) -> jobject {
        //Read PoseidonHash instance
        let digest = parse_rust_struct_from_jobject::<FieldHash>(&_env, _h, "poseidonHashPointer");

        //Get digest
        map_to_jobject_or_throw_exc(
            _env,
            finalize_poseidon_hash(digest),
            "com/horizen/librustsidechains/FieldElement",
            "com/horizen/poseidonnative/PoseidonHashException",
            "Unable to finalize hash",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeReset(
        _env: JNIEnv,
        _h: JObject,
        _personalization: jobjectArray,
    ) {
        //Read PoseidonHash instance
        let digest =
            parse_mut_rust_struct_from_jobject::<FieldHash>(&_env, _h, "poseidonHashPointer");

        //Read _personalization as array of FieldElement
        let personalization_len = _env
            .get_array_length(_personalization)
            .expect("Should be able to read personalization array size");
        let mut personalization = vec![];

        // Array can be empty
        for i in 0..personalization_len {
            let field_obj = _env.get_object_array_element(_personalization, i).expect(
                format!(
                    "Should be able to read elem {} of the personalization array",
                    i
                )
                .as_str(),
            );

            let field = parse_rust_struct_from_jobject::<FieldElement>(
                &_env,
                field_obj,
                "fieldElementPointer",
            );

            personalization.push(field);
        }

        let personalization = if personalization.is_empty() {
            None
        } else {
            Some(personalization)
        };

        reset_poseidon_hash(digest, personalization)
    }
);

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeFreePoseidonHash(
        _env: JNIEnv,
        _h: JObject,
    ) {
        drop_rust_struct_from_jobject::<FieldHash>(_env, _h, "poseidonHashPointer")
    }
);
