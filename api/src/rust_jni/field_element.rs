use super::*;

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeSerializeFieldElement(
        _env: JNIEnv,
        _field_element: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<FieldElement>(_env, _field_element, "fieldElementPointer", None)
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeDeserializeFieldElement(
        _env: JNIEnv,
        _class: JClass,
        _field_element_bytes: jbyteArray,
    ) -> jobject {
        let fe_bytes = _env
            .convert_byte_array(_field_element_bytes)
            .expect("Cannot read field element bytes.");
        map_to_jobject_or_throw_exc::<FieldElement, _>(
            _env,
            read_field_element_from_buffer_with_padding(fe_bytes.as_slice()),
            "io/horizen/common/librustsidechains/FieldElement",
            "io/horizen/common/librustsidechains/DeserializationException",
            "Unable to deserialize FieldElement",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeCreateRandom(
        _env: JNIEnv,
        _class: JClass,
        _seed: jlong,
    ) -> jobject {
        //Create random field element
        let fe = get_random_field_element(_seed as u64);

        return_field_element(_env, fe)
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeCreateSecureRandom(
        _env: JNIEnv,
        _class: JClass,
    ) -> jobject {
        //Create secure random field element
        let fe = get_secure_random_field_element();

        return_field_element(_env, fe)
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeCreateFromLong(
        _env: JNIEnv,
        _class: JClass,
        _long: jlong,
    ) -> jobject {
        //Create field element from _long
        let fe = FieldElement::from(_long as u64);

        return_field_element(_env, fe)
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeClone(
        _env: JNIEnv,
        _field_element: JObject,
    ) -> jobject {
        let fe = parse_rust_struct_from_jobject::<FieldElement>(
            &_env,
            _field_element,
            "fieldElementPointer",
        );
        return_field_element(_env, *fe)
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativePrintFieldElementBytes(
        _env: JNIEnv,
        _field_element: JObject,
    ) {
        let pointer = _env
            .get_field(_field_element, "fieldElementPointer", "J")
            .expect("Cannot get object raw pointer.");

        let obj_bytes =
            serialize_from_raw_pointer(&_env, pointer.j().unwrap() as *const FieldElement, None);

        println!("{:?}", into_i8(obj_bytes));
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeFreeFieldElement(
        _env: JNIEnv,
        _fe: JObject,
    ) {
        drop_rust_struct_from_jobject::<FieldElement>(_env, _fe, "fieldElementPointer")
    }
);

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_FieldElement_nativeEquals(
        _env: JNIEnv,
        _field_element_1: JObject,
        _field_element_2: JObject,
    ) -> jboolean {
        //Read field_1
        let field_1 = parse_rust_struct_from_jobject::<FieldElement>(
            &_env,
            _field_element_1,
            "fieldElementPointer",
        );
        let field_2 = parse_rust_struct_from_jobject::<FieldElement>(
            &_env,
            _field_element_2,
            "fieldElementPointer",
        );

        match field_1 == field_2 {
            true => JNI_TRUE,
            false => JNI_FALSE,
        }
    }
);
