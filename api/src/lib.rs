#![deny(
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]
#![deny(
    unused_qualifications,
    variant_size_differences,
    stable_features,
    unreachable_pub
)]
#![deny(
    non_shorthand_field_patterns,
    unused_attributes,
    unused_imports,
    unused_extern_crates
)]
#![deny(
    renamed_and_removed_lints,
    stable_features,
    unused_allocation,
    unused_comparisons,
    bare_trait_objects
)]
#![deny(
    const_err,
    unused_must_use,
    unused_mut,
    unused_unsafe,
    private_in_public,
)]

use algebra::{SemanticallyValid, serialize::*};
use std::any::type_name;

pub mod ginger_calls;
use ginger_calls::*;

#[macro_use]
pub mod exception;
use exception::*;

#[macro_use]
pub mod type_mappings;
use type_mappings::*;


pub fn read_raw_pointer<'a, T>(env: &JNIEnv, input: *const T) -> &'a T {
    if input.is_null() {
        throw_and_exit!(env, "java/lang/NullPointerException", "Received null pointer");
    }
    unsafe { &* input }
}

pub fn read_mut_raw_pointer<'a, T>(env: &JNIEnv, input: *mut T) -> &'a mut T {
    if input.is_null() {
        throw_and_exit!(env, "java/lang/NullPointerException", "Received null pointer");
    }
    unsafe { &mut *input }
}

pub fn read_nullable_raw_pointer<'a, T>(input: *const T) -> Option<&'a T> {
    unsafe { input.as_ref() }
}

pub fn serialize_from_raw_pointer<T: CanonicalSerialize>(
    _env:       &JNIEnv,
    to_write:   *const T,
    compressed: Option<bool>,
) -> Vec<u8> {
    serialize_to_buffer(read_raw_pointer(&_env, to_write), compressed)
        .expect(format!("unable to write {} to buffer", type_name::<T>()).as_str())
}

pub fn return_jobject<'a, T: Sized>(_env: &'a JNIEnv, obj: T, class_path: &str) -> JObject<'a>
{
    //Return field element
    let obj_ptr: jlong = jlong::from(Box::into_raw(Box::new(obj)) as i64);

    let obj_class = _env.find_class(class_path).expect("Should be able to find class");

    _env.new_object(obj_class, "(J)V", &[JValue::Long(obj_ptr)])
        .expect("Should be able to create new jobject")
}

pub fn deserialize_to_jobject<T: CanonicalDeserialize + SemanticallyValid>(
    _env: &JNIEnv,
    obj_bytes: jbyteArray,
    checked: Option<jboolean>, // Can be none for types with trivial checks or without themn
    compressed: Option<jboolean>, // Can be none for uncompressable types
    class_path: &str,
) -> jobject
{
    let obj_bytes = _env.convert_byte_array(obj_bytes)
        .expect("Cannot read bytes.");

    let obj = deserialize_from_buffer::<T>(
        obj_bytes.as_slice(),
        checked.map(|jni_bool| jni_bool == JNI_TRUE),
        compressed.map(|jni_bool| jni_bool == JNI_TRUE)
    );

    match obj {
        Ok(obj) => *return_jobject(&_env, obj, class_path),
        Err(_) => std::ptr::null::<jobject>() as jobject,
    }
}

pub fn serialize_from_jobject<T: CanonicalSerialize>(
    _env: &JNIEnv,
    obj: JObject,
    ptr_name: &str,
    compressed: Option<jboolean>, // Can be none for uncompressable types
) -> jbyteArray
{
    let pointer = _env.get_field(obj, ptr_name, "J")
        .expect("Cannot get object raw pointer.");

    let obj_bytes = serialize_from_raw_pointer(_env, pointer.j().unwrap() as *const T, compressed.map(|jni_bool| jni_bool == JNI_TRUE));

    _env.byte_array_from_slice(obj_bytes.as_slice())
        .expect("Cannot write object.")
}

pub fn parse_jbyte_array_to_vec(_env: &JNIEnv, java_byte_array: &jbyteArray, length: usize) -> Vec<u8> {
    let vec = _env.convert_byte_array(*java_byte_array)
        .expect("Should be able to convert to Rust array");

    if vec.len() != length {
        panic!("Retrieved array size {} expected to be {}.", vec.len(), length);
    }

    vec
}

pub fn _get_byte_array(_env: &JNIEnv, java_byte_array: &jbyteArray, buffer: &mut [u8]) {
    let vec = _env.convert_byte_array(*java_byte_array)
        .expect("Should be able to convert to Rust array");

    for (pos, e) in vec.iter().enumerate() {
        buffer[pos] = *e;
    }
}

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jbyteArray, jboolean, jint, jlong, jobject, jobjectArray};
use jni::sys::{JNI_TRUE, JNI_FALSE};

//Field element related functions

pub fn return_field_element(_env: &JNIEnv, fe: FieldElement) -> jobject
{
    return_jobject(_env, fe, "com/horizen/librustsidechains/FieldElement")
        .into_inner()
}

ffi_export!(
    fn Java_com_horizen_librustsidechains_Library_nativePanickingFunction(
    _env: JNIEnv,
    _class: JClass,
) { panic!("Oh no ! A panic occured !") });

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeGetFieldElementSize(
    _env: JNIEnv,
    _field_element_class: JClass,
) -> jint { FIELD_SIZE as jint });

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeSerializeFieldElement(
    _env: JNIEnv,
    _field_element: JObject,
) -> jbyteArray
{
    serialize_from_jobject::<FieldElement>(
        &_env,
        _field_element,
        "fieldElementPointer",
        None
    )
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeDeserializeFieldElement(
    _env: JNIEnv,
    _class: JClass,
    _field_element_bytes: jbyteArray,
) -> jobject
{
    deserialize_to_jobject::<FieldElement>(
        &_env,
        _field_element_bytes,
        None,
        None,
        "com/horizen/librustsidechains/FieldElement",
    )
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeCreateRandom(
    _env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _seed: jlong,
) -> jobject
{
    //Create random field element
    let fe = get_random_field_element(_seed as u64);

    return_field_element(&_env, fe)
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeCreateFromLong(
    _env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    _long: jlong
) -> jobject
{
    //Create field element from _long
    let fe = FieldElement::from(_long as u64);

    return_field_element(&_env, fe)
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativePrintFieldElementBytes(
    _env: JNIEnv,
    _field_element: JObject,
)
{
    let pointer = _env.get_field(_field_element, "fieldElementPointer", "J")
        .expect("Cannot get object raw pointer.");

    let obj_bytes = serialize_from_raw_pointer(&_env,
        pointer.j().unwrap() as *const FieldElement,
        None,
    );

    println!("{:?}", into_i8(obj_bytes));
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeFreeFieldElement(
    _env: JNIEnv,
    _class: JClass,
    _fe: *mut FieldElement,
)
{
    if _fe.is_null()  { return }
    drop(unsafe { Box::from_raw(_fe) });
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeEquals(
    _env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _field_element_1: JObject,
    _field_element_2: JObject,
) -> jboolean
{
    //Read field_1
    let field_1 = {

        let f =_env.get_field(_field_element_1, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer_1");

        read_raw_pointer(&_env, f.j().unwrap() as *const FieldElement)
    };

    //Read field_2
    let field_2 = {

        let f =_env.get_field(_field_element_2, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer_2");

        read_raw_pointer(&_env, f.j().unwrap() as *const FieldElement)
    };

    match field_1 == field_2 {
        true => JNI_TRUE,
        false => JNI_FALSE,
    }
});

//Public Schnorr key utility functions
ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrPublicKey_nativeGetPublicKeySize(
    _env: JNIEnv,
    _schnorr_public_key_class: JClass,
) -> jint { SCHNORR_PK_SIZE as jint });

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrPublicKey_nativeSerializePublicKey(
    _env: JNIEnv,
    _schnorr_public_key: JObject,
    _compressed: jboolean,
) -> jbyteArray
{
    serialize_from_jobject::<SchnorrPk>(&_env, _schnorr_public_key, "publicKeyPointer", Some(_compressed))
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrPublicKey_nativeDeserializePublicKey(
    _env: JNIEnv,
    _schnorr_public_key_class: JClass,
    _public_key_bytes: jbyteArray,
    _check_public_key: jboolean,
    _compressed: jboolean,
) -> jobject
{
    deserialize_to_jobject::<SchnorrPk>(
        &_env,
        _public_key_bytes,
        Some(_check_public_key),
        Some(_compressed),
        "com/horizen/schnorrnative/SchnorrPublicKey"
    )
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrPublicKey_nativeFreePublicKey(
    _env: JNIEnv,
    _schnorr_public_key: JObject,
)
{
    let public_key_pointer = _env.get_field(_schnorr_public_key, "publicKeyPointer", "J")
        .expect("Cannot get public key pointer.");

    let public_key = public_key_pointer.j().unwrap() as *mut SchnorrPk;

    if public_key.is_null()  { return }
    drop(unsafe { Box::from_raw(public_key) });
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSecretKey_nativeGetSecretKeySize(
        _env: JNIEnv,
        _schnorr_secret_key_class: JClass,
    ) -> jint { SCHNORR_SK_SIZE as jint }
);

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSecretKey_nativeSerializeSecretKey(
    _env: JNIEnv,
    _schnorr_secret_key: JObject,
) -> jbyteArray
{
    serialize_from_jobject::<SchnorrSk>(
        &_env,
        _schnorr_secret_key,
        "secretKeyPointer",
        None
    )
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSecretKey_nativeDeserializeSecretKey(
        _env: JNIEnv,
        _schnorr_secret_key_class: JClass,
        _secret_key_bytes: jbyteArray,
    ) -> jobject
    {
        deserialize_to_jobject::<SchnorrSk>(
            &_env,
            _secret_key_bytes,
            None,
            None,
            "com/horizen/schnorrnative/SchnorrSecretKey",
        )
    }
);

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSecretKey_nativeFreeSecretKey(
    _env: JNIEnv,
    _schnorr_secret_key: JObject,
)
{
    let secret_key_pointer = _env.get_field(_schnorr_secret_key, "secretKeyPointer", "J")
        .expect("Cannot get secret key pointer.");

    let secret_key = secret_key_pointer.j().unwrap() as *mut SchnorrSk;

    if secret_key.is_null()  { return }
    drop(unsafe { Box::from_raw(secret_key) });
});

//Schnorr signature utility functions
ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSignature_nativeGetSignatureSize(
    _env: JNIEnv,
    _class: JClass,
) -> jint { SCHNORR_SIG_SIZE as jint });

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSignature_nativeSerializeSignature(
    _env: JNIEnv,
    _schnorr_sig: JObject,
) -> jbyteArray
{
    serialize_from_jobject::<SchnorrSig>(
        &_env,
        _schnorr_sig,
        "signaturePointer",
        None
    )
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSignature_nativeDeserializeSignature(
    _env: JNIEnv,
    _class: JClass,
    _sig_bytes: jbyteArray,
    _check_sig: jboolean,
) -> jobject
{
    deserialize_to_jobject::<SchnorrSig>(
        &_env,
        _sig_bytes,
        Some(_check_sig),
        None,
        "com/horizen/schnorrnative/SchnorrSignature"
    )
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSignature_nativeIsValidSignature(
    _env: JNIEnv,
    _sig: JObject,
) -> jboolean
{
    let sig = _env.get_field(_sig, "signaturePointer", "J")
        .expect("Should be able to get field signaturePointer").j().unwrap() as *const SchnorrSig;

    if is_valid(read_raw_pointer(&_env, sig)) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSignature_nativefreeSignature(
    _env: JNIEnv,
    _class: JClass,
    _sig: *mut SchnorrSig,
)
{
    if _sig.is_null()  { return }
    drop(unsafe { Box::from_raw(_sig) });
});

//Schnorr signature functions
ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrKeyPair_nativeGenerate(
    _env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
) -> jobject
{
    let (pk, sk) = schnorr_generate_key();

    let secret_key_object = return_jobject(&_env, sk, "com/horizen/schnorrnative/SchnorrSecretKey");
    let public_key_object = return_jobject(&_env, pk, "com/horizen/schnorrnative/SchnorrPublicKey");

    let class = _env.find_class("com/horizen/schnorrnative/SchnorrKeyPair")
        .expect("Should be able to find SchnorrKeyPair class");

    let result = _env.new_object(
        class,
        "(Lcom/horizen/schnorrnative/SchnorrSecretKey;Lcom/horizen/schnorrnative/SchnorrPublicKey;)V",
        &[JValue::Object(secret_key_object), JValue::Object(public_key_object)]
    ).expect("Should be able to create new (SchnorrSecretKey, SchnorrPublicKey) object");

    *result
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrKeyPair_nativeSignMessage(
    _env: JNIEnv,
    _schnorr_key_pair: JObject,
    _message: JObject,
) -> jobject {

    //Read sk
    let sk_object = _env.get_field(_schnorr_key_pair,
                                   "secretKey",
                                   "Lcom/horizen/schnorrnative/SchnorrSecretKey;"
    ).expect("Should be able to get field secretKey").l().unwrap();
    let secret_key = {

        let s =_env.get_field(sk_object, "secretKeyPointer", "J")
            .expect("Should be able to get field secretKeyPointer");

        read_raw_pointer(&_env, s.j().unwrap() as *const SchnorrSk)
    };

    //Read pk
    let pk_object = _env.get_field(_schnorr_key_pair,
                                   "publicKey",
                                   "Lcom/horizen/schnorrnative/SchnorrPublicKey;"
    ).expect("Should be able to get field publicKey").l().unwrap();

    let public_key = {

        let p = _env.get_field(pk_object, "publicKeyPointer", "J")
            .expect("Should be able to get field publicKeyPointer");

        read_raw_pointer(&_env, p.j().unwrap() as *const SchnorrPk)
    };

    //Read message
    let message = {

        let m =_env.get_field(_message, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, m.j().unwrap() as *const FieldElement)
    };

    //Sign message and return opaque pointer to sig
    let signature = match schnorr_sign(message, secret_key, public_key) {
        Ok(sig) => sig,
        Err(_) => return std::ptr::null::<jobject>() as jobject //CRYPTO_ERROR
    };

    return_jobject(&_env, signature, "com/horizen/schnorrnative/SchnorrSignature")
        .into_inner()
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrPublicKey_nativeVerifyKey(
    _env: JNIEnv,
    _public_key: JObject,
) -> jboolean
{
    let pk = _env.get_field(_public_key, "publicKeyPointer", "J")
        .expect("Should be able to get field publicKeyPointer").j().unwrap() as *const SchnorrPk;

    if schnorr_verify_public_key(read_raw_pointer(&_env, pk)) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrSecretKey_nativeGetPublicKey(
    _env: JNIEnv,
    _secret_key: JObject
) -> jobject {

    let sk = _env.get_field(_secret_key, "secretKeyPointer", "J")
        .expect("Should be able to get field secretKeyPointer").j().unwrap() as *const SchnorrSk;

    let secret_key = read_raw_pointer(&_env, sk);

    let pk = schnorr_get_public_key(secret_key);

    return_jobject(&_env, pk, "com/horizen/schnorrnative/SchnorrPublicKey")
        .into_inner()
});

ffi_export!(
    fn Java_com_horizen_schnorrnative_SchnorrPublicKey_nativeVerifySignature(
    _env: JNIEnv,
    _public_key: JObject,
    _signature: JObject,
    _message: JObject,
) -> jboolean {

    //Read pk
    let public_key = {

        let p = _env.get_field(_public_key, "publicKeyPointer", "J")
            .expect("Should be able to get field publicKeyPointer");

        read_raw_pointer(&_env, p.j().unwrap() as *const SchnorrPk)
    };

    //Read message
    let message = {

        let m =_env.get_field(_message, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, m.j().unwrap() as *const FieldElement)
    };

    //Read sig
    let signature = {
        let sig = _env.get_field(_signature, "signaturePointer", "J")
            .expect("Should be able to get field signaturePointer");

        read_raw_pointer(&_env, sig.j().unwrap() as *const SchnorrSig)
    };

    //Verify sig
    match schnorr_verify_signature(message, public_key, signature) {
        Ok(result) => if result {
            JNI_TRUE
        } else {
            JNI_FALSE
        },
        Err(_) => JNI_FALSE //CRYPTO_ERROR
    }
});

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeGetHashSize(
    _env: JNIEnv,
    _class: JClass,
) -> jint { FIELD_SIZE as jint });

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeGetConstantLengthPoseidonHash(
    _env: JNIEnv,
    _class: JClass,
    _input_size: jint,
    _personalization: jobjectArray,
) -> jobject
{
    //Read _personalization as array of FieldElement
    let personalization_len = _env.get_array_length(_personalization)
        .expect("Should be able to read personalization array size");
    let mut personalization = vec![];

    // Array can be empty
    for i in 0..personalization_len {
        let field_obj = _env.get_object_array_element(_personalization, i)
            .expect(format!("Should be able to read elem {} of the personalization array", i).as_str());

        let field = {

            let f =_env.get_field(field_obj, "fieldElementPointer", "J")
                .expect("Should be able to get field fieldElementPointer");

            read_raw_pointer(&_env, f.j().unwrap() as *const FieldElement)
        };

        personalization.push(field);
    }

    //Instantiate PoseidonHash
    let h = get_poseidon_hash_constant_length(
        _input_size as usize,
        if personalization.is_empty() { None } else { Some(personalization) }
    );

    //Return PoseidonHash instance
    return_jobject(&_env, h, "com/horizen/poseidonnative/PoseidonHash")
        .into_inner()
});

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeGetVariableLengthPoseidonHash(
    _env: JNIEnv,
    _class: JClass,
    _mod_rate: jboolean,
    _personalization: jobjectArray,
) -> jobject
{
    //Read _personalization as array of FieldElement
    let personalization_len = _env.get_array_length(_personalization)
        .expect("Should be able to read personalization array size");
    let mut personalization = vec![];

    // Array can be empty
    for i in 0..personalization_len {
        let field_obj = _env.get_object_array_element(_personalization, i)
            .expect(format!("Should be able to read elem {} of the personalization array", i).as_str());

        let field = {

            let f =_env.get_field(field_obj, "fieldElementPointer", "J")
                .expect("Should be able to get field fieldElementPointer");

            read_raw_pointer(&_env, f.j().unwrap() as *const FieldElement)
        };

        personalization.push(field);
    }

    //Instantiate PoseidonHash
    let h = get_poseidon_hash_variable_length(
        _mod_rate == JNI_TRUE,
        if personalization.is_empty() { None } else { Some(personalization) }
    );

    //Return PoseidonHash instance
    return_jobject(&_env, h, "com/horizen/poseidonnative/PoseidonHash")
        .into_inner()
});

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeUpdate(
    _env: JNIEnv,
    _h: JObject,
    _input: JObject,
){
    //Read PoseidonHash instance
    let digest = {

        let h = _env.get_field(_h, "poseidonHashPointer", "J")
            .expect("Should be able to get field poseidonHashPointer");

        read_mut_raw_pointer(&_env, h.j().unwrap() as *mut FieldHash)
    };

    //Read input
    let input = {

        let i =_env.get_field(_input, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, i.j().unwrap() as *const FieldElement)
    };

    update_poseidon_hash(digest, input);
});

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeFinalize(
    _env: JNIEnv,
    _h: JObject,
) -> jobject
{
    //Read PoseidonHash instance
    let digest = {

        let h = _env.get_field(_h, "poseidonHashPointer", "J")
            .expect("Should be able to get field poseidonHashPointer");

        read_raw_pointer(&_env, h.j().unwrap() as *const FieldHash)
    };

    //Get digest
    let fe = match finalize_poseidon_hash(digest) {
        Ok(fe) => fe,
        Err(_) => return std::ptr::null::<jobject>() as jobject //CRYPTO_ERROR
    };

    return_field_element(&_env, fe)
});

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeReset(
    _env: JNIEnv,
    _h: JObject,
    _personalization: jobjectArray,
){
    //Read PoseidonHash instance
    let digest = {

        let h = _env.get_field(_h, "poseidonHashPointer", "J")
            .expect("Should be able to get field poseidonHashPointer");

        read_mut_raw_pointer(&_env, h.j().unwrap() as *mut FieldHash)
    };

    //Read _personalization as array of FieldElement
    let personalization_len = _env.get_array_length(_personalization)
        .expect("Should be able to read personalization array size");
    let mut personalization = vec![];

    // Array can be empty
    for i in 0..personalization_len {
        let field_obj = _env.get_object_array_element(_personalization, i)
            .expect(format!("Should be able to read elem {} of the personalization array", i).as_str());

        let field = {

            let f =_env.get_field(field_obj, "fieldElementPointer", "J")
                .expect("Should be able to get field fieldElementPointer");

            read_raw_pointer(&_env, f.j().unwrap() as *const FieldElement)
        };

        personalization.push(field);
    }

    let personalization = if personalization.is_empty() { None } else { Some(personalization) };

    reset_poseidon_hash(digest, personalization)
});

ffi_export!(
    fn Java_com_horizen_poseidonnative_PoseidonHash_nativeFreePoseidonHash(
    _env: JNIEnv,
    _h: JObject,
)
{
    let h_pointer = _env.get_field(_h, "poseidonHashPointer", "J")
        .expect("Cannot get poseidonHashPointer");

    let h = h_pointer.j().unwrap() as *mut FieldHash;

    if h.is_null()  { return }
    drop(unsafe { Box::from_raw(h) });
});

//Merkle tree functions

//////////// MERKLE PATH

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeVerify(
    _env: JNIEnv,
    _path: JObject,
    _height: jint,
    _leaf: JObject,
    _root: JObject,
) -> jboolean
{
    let leaf = {

        let fe =_env.get_field(_leaf, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    let root = {

        let fe =_env.get_field(_root, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    let path = {

        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    if !path.is_valid() {
        return JNI_FALSE;
    }

    match verify_ginger_merkle_path(path, _height as usize, leaf, root) {
        Ok(result) => if result { JNI_TRUE } else { JNI_FALSE },
        Err(_) => JNI_FALSE // CRYPTO_ERROR
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeVerifyWithoutLengthCheck(
    _env: JNIEnv,
    _path: JObject,
    _leaf: JObject,
    _root: JObject,
) -> jboolean
{
    let leaf = {

        let fe =_env.get_field(_leaf, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    let root = {

        let fe =_env.get_field(_root, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    let path = {

        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    if !path.is_valid() {
        return JNI_FALSE;
    }

    if verify_ginger_merkle_path_without_length_check(path, leaf, root) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeApply(
    _env: JNIEnv,
    _path: JObject,
    _leaf: JObject,
) -> jobject
{
    let path = {
        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    let leaf = {

        let fe =_env.get_field(_leaf, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    let root = get_root_from_path(path, leaf);

    return_field_element(&_env, root)
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeIsLeftmost(
    _env: JNIEnv,
    _path: JObject,
) -> jboolean
{
    let path = {

        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    is_path_leftmost(path) as jboolean
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeIsRightmost(
    _env: JNIEnv,
    _path: JObject,
) -> jboolean
{
    let path = {

        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    is_path_rightmost(path) as jboolean
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeAreRightLeavesEmpty(
    _env: JNIEnv,
    _path: JObject,
) -> jboolean
{
    let path = {

        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    are_right_leaves_empty(path) as jboolean
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeLeafIndex(
    _env: JNIEnv,
    _path: JObject,
) -> jlong
{
    let path = {

        let t =_env.get_field(_path, "merklePathPointer", "J")
            .expect("Should be able to get field merklePathPointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHTPath)
    };

    get_leaf_index_from_path(path) as jlong
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeSerialize(
    _env: JNIEnv,
    _path: JObject,
) -> jbyteArray
{
    serialize_from_jobject::<GingerMHTPath>(
        &_env,
        _path,
        "merklePathPointer",
        None
    )
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeDeserialize(
    _env: JNIEnv,
    _class: JClass,
    _path_bytes: jbyteArray,
    _checked: jboolean,
) -> jobject
{
    deserialize_to_jobject::<GingerMHTPath>(
        &_env,
        _path_bytes,
        Some(_checked),
        None,
        "com/horizen/merkletreenative/FieldBasedMerklePath"
    )
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerklePath_nativeFreeMerklePath(
    _env: JNIEnv,
    _class: JClass,
    _path: *mut GingerMHTPath,
)
{
    if _path.is_null()  { return }
    drop(unsafe { Box::from_raw(_path) });
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeInit(
    _env: JNIEnv,
    _class: JClass,
    _height: jint,
    _processing_step: jlong,
) -> jobject
{
    // Create new FieldBasedMerkleTree Rust side
    let mt = new_ginger_mht(
        _height as usize,
        _processing_step as usize
    );

    // Create and return new FieldBasedMerkleTree Java side
    match mt {
        Ok(mt) => return_jobject(&_env, mt, "com/horizen/merkletreenative/FieldBasedMerkleTree").into_inner(),
        Err(_) => return std::ptr::null::<jobject>() as jobject //CRYPTO_ERROR
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeAppend(
    _env: JNIEnv,
    _tree: JObject,
    _leaf: JObject,
) -> jboolean
{
    let leaf = {

        let fe =_env.get_field(_leaf, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_mut_raw_pointer(&_env, t.j().unwrap() as *mut GingerMHT)
    };

    match append_leaf_to_ginger_mht(tree, leaf) {
        Ok(_) => JNI_TRUE,
        Err(_) => JNI_FALSE,
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeFinalize(
    _env: JNIEnv,
    _tree: JObject,
) -> jobject
{
    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHT)
    };

    match finalize_ginger_mht(tree) {
        Ok(tree_copy) => return_jobject(&_env, tree_copy, "com/horizen/merkletreenative/FieldBasedMerkleTree").into_inner(),
        Err(_) => return std::ptr::null::<jobject>() as jobject //CRYPTO_ERROR
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeFinalizeInPlace(
    _env: JNIEnv,
    _tree: JObject,
) -> jboolean
{
    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_mut_raw_pointer(&_env, t.j().unwrap() as *mut GingerMHT)
    };

    match finalize_ginger_mht_in_place(tree) {
        Ok(_) => JNI_TRUE,
        Err(_) => JNI_FALSE
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeRoot(
    _env: JNIEnv,
    _tree: JObject,
) -> jobject
{
    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHT)
    };

    match get_ginger_mht_root(tree) {
        Some(root) => return_field_element(&_env, root),
        None => std::ptr::null::<jobject>() as jobject
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeGetMerklePath(
    _env: JNIEnv,
    _tree: JObject,
    _leaf_index: jlong,
) -> jobject
{
    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_raw_pointer(&_env, t.j().unwrap() as *const GingerMHT)
    };

    match get_ginger_mht_path(tree, _leaf_index as u64) {
        Some(path) => return_jobject(&_env, path, "com/horizen/merkletreenative/FieldBasedMerklePath")
            .into_inner(),
        None => std::ptr::null::<jobject>() as jobject
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeGetLeafIndex(
    _env: JNIEnv,
    _tree: JObject,
    _leaf: JObject,
) -> jlong
{
    // Read leaf
    let leaf = {

        let fe =_env.get_field(_leaf, "fieldElementPointer", "J")
            .expect("Should be able to get field fieldElementPointer");

        read_raw_pointer(&_env, fe.j().unwrap() as *const FieldElement)
    };

    // Read field element
    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_mut_raw_pointer(&_env, t.j().unwrap() as *mut GingerMHT)
    };

    // Check if element is in tree and if yes get its index
    match get_leaf_index(tree, leaf) {
        Some(idx) => idx as jlong,
        None => -1
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeSerialize(
    _env: JNIEnv,
    _tree: JObject,
) -> jbyteArray
{
    serialize_from_jobject::<GingerMHT>(
        &_env,
        _tree,
        "inMemoryOptimizedMerkleTreePointer",
        None
    )
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeDeserialize(
    _env: JNIEnv,
    _class: JClass,
    _tree_bytes: jbyteArray,
) -> jobject
{
    // Not really necessary to do deep checks: no one can trick a node into using different _tree_bytes
    // possibly deserializing an unconsistent tree that may lead to internal crashes (DOS).
    // TODO: Is this true ?
    let obj_bytes = _env.convert_byte_array(_tree_bytes)
        .expect("Cannot read tree bytes.");

    let tree = <GingerMHT as CanonicalDeserialize>::deserialize(obj_bytes.as_slice());

    match tree {
        Ok(tree) => *return_jobject(&_env, tree, "com/horizen/merkletreenative/FieldBasedMerkleTree"),
        Err(e) => {
            eprintln!("{:?}", e);
            std::ptr::null::<jobject>() as jobject
        },
    }
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeReset(
    _env: JNIEnv,
    _tree: JObject,
)
{
    let tree = {

        let t =_env.get_field(_tree, "inMemoryOptimizedMerkleTreePointer", "J")
            .expect("Should be able to get field inMemoryOptimizedMerkleTreePointer");

        read_mut_raw_pointer(&_env, t.j().unwrap() as *mut GingerMHT)
    };

    reset_ginger_mht(tree);
});

ffi_export!(
    fn Java_com_horizen_merkletreenative_FieldBasedMerkleTree_nativeFreeMerkleTree(
    _env: JNIEnv,
    _class: JClass,
    _tree: *mut GingerMHT,
)
{
    if _tree.is_null()  { return }
    drop(unsafe { Box::from_raw(_tree) });
});