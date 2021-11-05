use super::*;

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrPublicKey_nativeGetPublicKeySize(
        _env: JNIEnv,
        _schnorr_public_key_class: JClass,
    ) -> jint {
        SCHNORR_PK_SIZE as jint
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrPublicKey_nativeSerializePublicKey(
        _env: JNIEnv,
        _schnorr_public_key: JObject,
        _compressed: jboolean,
    ) -> jbyteArray {
        serialize_from_jobject::<SchnorrPk>(
            _env,
            _schnorr_public_key,
            "publicKeyPointer",
            Some(_compressed),
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrPublicKey_nativeDeserializePublicKey(
        _env: JNIEnv,
        _schnorr_public_key_class: JClass,
        _public_key_bytes: jbyteArray,
        _check_public_key: jboolean,
        _compressed: jboolean,
    ) -> jobject {
        deserialize_to_jobject::<SchnorrPk>(
            _env,
            _public_key_bytes,
            Some(_check_public_key),
            Some(_compressed),
            "io/horizen/common/schnorrnative/SchnorrPublicKey",
            "io/horizen/common/librustsidechains/DeserializationException",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrPublicKey_nativeFreePublicKey(
        _env: JNIEnv,
        _schnorr_public_key: JObject,
    ) {
        drop_rust_struct_from_jobject::<SchnorrPk>(_env, _schnorr_public_key, "publicKeyPointer")
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSecretKey_nativeGetSecretKeySize(
        _env: JNIEnv,
        _schnorr_secret_key_class: JClass,
    ) -> jint {
        SCHNORR_SK_SIZE as jint
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSecretKey_nativeSerializeSecretKey(
        _env: JNIEnv,
        _schnorr_secret_key: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<SchnorrSk>(_env, _schnorr_secret_key, "secretKeyPointer", None)
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSecretKey_nativeDeserializeSecretKey(
        _env: JNIEnv,
        _schnorr_secret_key_class: JClass,
        _secret_key_bytes: jbyteArray,
    ) -> jobject {
        deserialize_to_jobject::<SchnorrSk>(
            _env,
            _secret_key_bytes,
            None,
            None,
            "io/horizen/common/schnorrnative/SchnorrSecretKey",
            "io/horizen/common/librustsidechains/DeserializationException",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSecretKey_nativeFreeSecretKey(
        _env: JNIEnv,
        _schnorr_secret_key: JObject,
    ) {
        drop_rust_struct_from_jobject::<SchnorrSk>(_env, _schnorr_secret_key, "secretKeyPointer")
    }
);

//Schnorr signature utility functions
ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSignature_nativeGetSignatureSize(
        _env: JNIEnv,
        _class: JClass,
    ) -> jint {
        SCHNORR_SIG_SIZE as jint
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSignature_nativeSerializeSignature(
        _env: JNIEnv,
        _schnorr_sig: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<SchnorrSig>(_env, _schnorr_sig, "signaturePointer", None)
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSignature_nativeDeserializeSignature(
        _env: JNIEnv,
        _class: JClass,
        _sig_bytes: jbyteArray,
        _check_sig: jboolean,
    ) -> jobject {
        deserialize_to_jobject::<SchnorrSig>(
            _env,
            _sig_bytes,
            Some(_check_sig),
            None,
            "io/horizen/common/schnorrnative/SchnorrSignature",
            "io/horizen/common/librustsidechains/DeserializationException",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSignature_nativeIsValidSignature(
        _env: JNIEnv,
        _sig: JObject,
    ) -> jboolean {
        let sig = parse_rust_struct_from_jobject::<SchnorrSig>(&_env, _sig, "signaturePointer");

        if is_valid(sig) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSignature_nativefreeSignature(
        _env: JNIEnv,
        _sig: JObject,
    ) {
        drop_rust_struct_from_jobject::<SchnorrSig>(_env, _sig, "signaturePointer")
    }
);

//Schnorr signature functions
ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrKeyPair_nativeGenerate(
        _env: JNIEnv,
        // this is the class that owns our
        // static method. Not going to be
        // used, but still needs to have
        // an argument slot
        _class: JClass,
    ) -> jobject {
        let (pk, sk) = schnorr_generate_key();

        let secret_key_object = return_jobject(
            &_env,
            sk,
            "io/horizen/common/schnorrnative/SchnorrSecretKey",
        );
        let public_key_object = return_jobject(
            &_env,
            pk,
            "io/horizen/common/schnorrnative/SchnorrPublicKey",
        );

        let class = _env
            .find_class("io/horizen/common/schnorrnative/SchnorrKeyPair")
            .expect("Should be able to find SchnorrKeyPair class");

        let result = _env.new_object(
        class,
        "(Lio/horizen/common/schnorrnative/SchnorrSecretKey;Lio/horizen/common/schnorrnative/SchnorrPublicKey;)V",
        &[JValue::Object(secret_key_object), JValue::Object(public_key_object)]
    ).expect("Should be able to create new (SchnorrSecretKey, SchnorrPublicKey) object");

        *result
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrKeyPair_nativeSignMessage(
        _env: JNIEnv,
        _schnorr_key_pair: JObject,
        _message: JObject,
    ) -> jobject {
        //Read sk
        let sk_object = _env
            .get_field(
                _schnorr_key_pair,
                "secretKey",
                "Lio/horizen/common/schnorrnative/SchnorrSecretKey;",
            )
            .expect("Should be able to get field secretKey")
            .l()
            .unwrap();

        let secret_key =
            parse_rust_struct_from_jobject::<SchnorrSk>(&_env, sk_object, "secretKeyPointer");

        //Read pk
        let pk_object = _env
            .get_field(
                _schnorr_key_pair,
                "publicKey",
                "Lio/horizen/common/schnorrnative/SchnorrPublicKey;",
            )
            .expect("Should be able to get field publicKey")
            .l()
            .unwrap();

        let public_key =
            parse_rust_struct_from_jobject::<SchnorrPk>(&_env, pk_object, "publicKeyPointer");

        //Read message
        let message =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _message, "fieldElementPointer");

        //Sign message and return opaque pointer to sig
        map_to_jobject_or_throw_exc(
            _env,
            schnorr_sign(message, secret_key, public_key),
            "io/horizen/common/schnorrnative/SchnorrSignature",
            "io/horizen/common/schnorrnative/SchnorrSignatureException",
            "Unable to sign message",
        )
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrPublicKey_nativeVerifyKey(
        _env: JNIEnv,
        _public_key: JObject,
    ) -> jboolean {
        let public_key =
            parse_rust_struct_from_jobject::<SchnorrPk>(&_env, _public_key, "publicKeyPointer");

        if schnorr_verify_public_key(public_key) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrSecretKey_nativeGetPublicKey(
        _env: JNIEnv,
        _secret_key: JObject,
    ) -> jobject {
        let secret_key =
            parse_rust_struct_from_jobject::<SchnorrSk>(&_env, _secret_key, "secretKeyPointer");

        let pk = schnorr_get_public_key(secret_key);

        return_jobject(
            &_env,
            pk,
            "io/horizen/common/schnorrnative/SchnorrPublicKey",
        )
        .into_inner()
    }
);

ffi_export!(
    fn Java_io_horizen_common_schnorrnative_SchnorrPublicKey_nativeVerifySignature(
        _env: JNIEnv,
        _public_key: JObject,
        _signature: JObject,
        _message: JObject,
    ) -> jboolean {
        // Read pk
        let public_key =
            parse_rust_struct_from_jobject::<SchnorrPk>(&_env, _public_key, "publicKeyPointer");

        //Read message
        let message =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _message, "fieldElementPointer");

        //Read sig
        let signature =
            parse_rust_struct_from_jobject::<SchnorrSig>(&_env, _signature, "signaturePointer");

        //Verify sig
        map_to_jboolean_or_throw_exc(
            _env,
            schnorr_verify_signature(message, public_key, signature),
            "io/horizen/common/schnorrnative/SchnorrSignatureException",
            "Unable to verify signature",
        )
    }
);
