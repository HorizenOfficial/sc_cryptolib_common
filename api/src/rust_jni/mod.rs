use super::*;
use jni::{
    objects::{JClass, JObject, JValue},
    sys::{jbyteArray, jint, jboolean, jlong, jobject, jobjectArray, JNI_TRUE, JNI_FALSE},
    JNIEnv
};

pub mod utils;
use utils::*;

pub mod exception;
use exception::*;

pub mod field_element;
pub mod merkle_tree;
pub mod poseidon_hash;
pub mod schnorr_signature;

ffi_export!(
    fn Java_com_horizen_librustsidechains_Library_nativePanickingFunction(
    _env: JNIEnv,
    _class: JClass,
) { panic!("Oh no ! A panic occured !") });

