use super::*;
use crate::ginger_calls::{serialization::is_valid, vrf::*};

// VRF related functions
ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFPublicKey_nativeGetPublicKeySize(
        _env: JNIEnv,
        _vrf_public_key_class: JClass,
    ) -> jint {
        VRF_PK_SIZE as jint
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFPublicKey_nativeSerializePublicKey(
        _env: JNIEnv,
        _vrf_public_key: JObject,
        _compressed: jboolean,
    ) -> jbyteArray {
        serialize_from_jobject::<VRFPk>(
            _env,
            _vrf_public_key,
            "publicKeyPointer",
            Some(_compressed),
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFPublicKey_nativeDeserializePublicKey(
        _env: JNIEnv,
        _vrf_public_key_class: JClass,
        _public_key_bytes: jbyteArray,
        _check_public_key: jboolean,
        _compressed: jboolean,
    ) -> jobject {
        deserialize_to_jobject::<VRFPk>(
            _env,
            _public_key_bytes,
            Some(_check_public_key),
            Some(_compressed),
            "com/horizen/common/vrfnative/VRFPublicKey",
            "com/horizen/common/vrfnative/VRFException",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFPublicKey_nativeFreePublicKey(
        _env: JNIEnv,
        _vrf_public_key: JObject,
    ) {
        drop_rust_struct_from_jobject::<VRFPk>(_env, _vrf_public_key, "publicKeyPointer")
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFPublicKey_nativeVerifyKey(
        _env: JNIEnv,
        _vrf_public_key: JObject,
    ) -> jboolean {
        let public_key =
            parse_rust_struct_from_jobject::<VRFPk>(&_env, _vrf_public_key, "publicKeyPointer");

        if vrf_verify_public_key(public_key) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFPublicKey_nativeProofToHash(
        _env: JNIEnv,
        _vrf_public_key: JObject,
        _proof: JObject,
        _message: JObject,
    ) -> jobject {
        // Read data
        let public_key =
            parse_rust_struct_from_jobject::<VRFPk>(&_env, _vrf_public_key, "publicKeyPointer");
        let message =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _message, "fieldElementPointer");
        let proof = parse_rust_struct_from_jobject::<VRFProof>(&_env, _proof, "proofPointer");

        //Verify vrf proof and get vrf output
        map_to_jobject_or_throw_exc(
            _env,
            vrf_proof_to_hash(message, public_key, proof),
            "com/horizen/common/librustsidechains/FieldElement",
            "com/horizen/common/vrfnative/VRFException",
            "Unable to get vrf output",
        )
    }
);

//Secret VRF key utility functions
ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFSecretKey_nativeGetSecretKeySize(
        _env: JNIEnv,
        _vrf_secret_key_class: JClass,
    ) -> jint {
        VRF_SK_SIZE as jint
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFSecretKey_nativeSerializeSecretKey(
        _env: JNIEnv,
        _vrf_secret_key: JObject,
    ) -> jbyteArray {
        serialize_from_jobject::<VRFSk>(_env, _vrf_secret_key, "secretKeyPointer", None)
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFSecretKey_nativeDeserializeSecretKey(
        _env: JNIEnv,
        _vrf_secret_key_class: JClass,
        _secret_key_bytes: jbyteArray,
    ) -> jobject {
        deserialize_to_jobject::<VRFSk>(
            _env,
            _secret_key_bytes,
            None,
            None,
            "com/horizen/common/vrfnative/VRFSecretKey",
            "com/horizen/common/vrfnative/VRFException",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFSecretKey_nativeFreeSecretKey(
        _env: JNIEnv,
        _vrf_secret_key: JObject,
    ) {
        drop_rust_struct_from_jobject::<VRFSk>(_env, _vrf_secret_key, "secretKeyPointer")
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFSecretKey_nativeGetPublicKey(
        _env: JNIEnv,
        _vrf_secret_key: JObject,
    ) -> jobject {
        let sk =
            parse_rust_struct_from_jobject::<VRFSk>(&_env, _vrf_secret_key, "secretKeyPointer");
        let pk = vrf_get_public_key(sk);
        return_jobject(&_env, pk, "com/horizen/common/vrfnative/VRFPublicKey").into_inner()
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFProof_nativeGetProofSize(
        _env: JNIEnv,
        _class: JClass,
    ) -> jint {
        VRF_PROOF_SIZE as jint
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFProof_nativeSerializeProof(
        _env: JNIEnv,
        _proof: JObject,
        _compressed: jboolean,
    ) -> jbyteArray {
        serialize_from_jobject::<VRFProof>(_env, _proof, "proofPointer", Some(_compressed))
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFProof_nativeDeserializeProof(
        _env: JNIEnv,
        _class: JClass,
        _proof_bytes: jbyteArray,
        _check_proof: jboolean,
        _compressed: jboolean,
    ) -> jobject {
        deserialize_to_jobject::<VRFProof>(
            _env,
            _proof_bytes,
            Some(_check_proof),
            Some(_compressed),
            "com/horizen/common/vrfnative/VRFProof",
            "com/horizen/common/vrfnative/VRFException",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFProof_nativeIsValidVRFProof(
        _env: JNIEnv,
        _vrf_proof: JObject,
    ) -> jboolean {
        let proof = parse_rust_struct_from_jobject::<VRFProof>(&_env, _vrf_proof, "proofPointer");

        if is_valid(proof) {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFProof_nativeFreeProof(_env: JNIEnv, _proof: JObject) {
        drop_rust_struct_from_jobject::<VRFProof>(_env, _proof, "proofPointer")
    }
);

//VRF functions
ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFKeyPair_nativeGenerate(
        _env: JNIEnv,
        // this is the class that owns our
        // static method. Not going to be
        // used, but still needs to have
        // an argument slot
        _class: JClass,
    ) -> jobject {
        let (pk, sk) = vrf_generate_key();

        let secret_key_object =
            return_jobject(&_env, sk, "com/horizen/common/vrfnative/VRFSecretKey");
        let public_key_object =
            return_jobject(&_env, pk, "com/horizen/common/vrfnative/VRFPublicKey");

        let class = _env
            .find_class("com/horizen/common/vrfnative/VRFKeyPair")
            .expect("Should be able to find VRFKeyPair class");

        let result = _env.new_object(
        class,
        "(Lcom/horizen/common/vrfnative/VRFSecretKey;Lcom/horizen/common/vrfnative/VRFPublicKey;)V",
        &[JValue::Object(secret_key_object), JValue::Object(public_key_object)]
    ).expect("Should be able to create new (VRFSecretKey, VRFPublicKey) object");

        *result
    }
);

ffi_export!(
    fn Java_com_horizen_common_vrfnative_VRFKeyPair_nativeProve(
        _env: JNIEnv,
        _vrf_key_pair: JObject,
        _message: JObject,
    ) -> jobject {
        //Read sk
        let sk_object = _env
            .get_field(
                _vrf_key_pair,
                "secretKey",
                "Lcom/horizen/common/vrfnative/VRFSecretKey;",
            )
            .expect("Should be able to get field vrfKey")
            .l()
            .unwrap();

        let secret_key =
            parse_rust_struct_from_jobject::<VRFSk>(&_env, sk_object, "secretKeyPointer");

        //Read pk
        let pk_object = _env
            .get_field(
                _vrf_key_pair,
                "publicKey",
                "Lcom/horizen/common/vrfnative/VRFPublicKey;",
            )
            .expect("Should be able to get field publicKey")
            .l()
            .unwrap();

        let public_key =
            parse_rust_struct_from_jobject::<VRFPk>(&_env, pk_object, "publicKeyPointer");

        //Read message
        let message =
            parse_rust_struct_from_jobject::<FieldElement>(&_env, _message, "fieldElementPointer");

        //Compute vrf proof
        let (proof, vrf_out) = ok_or_throw_exc!(
            &_env,
            vrf_prove(message, secret_key, public_key),
            "com/horizen/common/vrfnative/VRFException",
            "Unable to compute VRF proof",
            JNI_NULL
        );

        //Create and return VRFProveResult instance
        let class = _env
            .find_class("com/horizen/common/vrfnative/VRFProveResult")
            .expect("Should be able to find VRFProveResult class");

        let result = _env.new_object(
        class,
        "(Lcom/horizen/common/vrfnative/VRFProof;Lcom/horizen/common/librustsidechains/FieldElement;)V",
        &[
            JValue::Object(return_jobject(&_env, proof, "com/horizen/common/vrfnative/VRFProof")),
            JValue::Object(return_jobject(&_env, vrf_out, "com/horizen/common/librustsidechains/FieldElement"))
        ]
    ).expect("Should be able to create new VRFProveResult:(VRFProof, FieldElement) object");

        *result
    }
);
