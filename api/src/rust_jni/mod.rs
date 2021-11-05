use super::*;
use jni::{
    objects::{JClass, JObject, JValue},
    sys::{jboolean, jbyteArray, jint, jlong, jobject, jobjectArray, JNI_FALSE, JNI_TRUE},
    JNIEnv,
};
use type_mappings::macros::*;

#[cfg(feature = "bn_382")]
use type_mappings::instantiated::bn_382::*;

#[cfg(feature = "tweedle")]
use type_mappings::instantiated::tweedle::*;

pub mod utils;
use utils::*;

pub mod exception;
use exception::*;

pub mod field_element;
pub mod merkle_tree;
pub mod poseidon_hash;
pub mod schnorr_signature;
pub mod vrf;

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_Library_nativePanickingFunction(
        _env: JNIEnv,
        _class: JClass,
    ) {
        panic!("Oh no ! A panic occured !")
    }
);
