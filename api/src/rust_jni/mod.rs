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

ffi_export!(
    fn Java_io_horizen_common_librustsidechains_Constants_nativeInitializeAllConstants(
        _env: JNIEnv,
        _class: JClass,
    ) {
        let class = _env
            .find_class("io/horizen/common/librustsidechains/Constants")
            .expect("Should be able to find Constants class");

        macro_rules! set_constant {
            ($name: expr, $value: expr) => {
                _env.set_static_field(
                    class,
                    _env.get_static_field_id(class, $name, "I")
                        .expect(format!("Should be able to get ID of {} field", $name).as_str()),
                    JValue::Int($value as jint),
                )
                .expect(format!("Should be able to set {} field", $name).as_str());
            };
        }

        // Supply the value for all constants
        set_constant!("FIELD_ELEMENT_LENGTH", FIELD_SIZE);
        set_constant!("SCHNORR_PK_LENGTH", SCHNORR_PK_SIZE);
        set_constant!("SCHNORR_SK_LENGTH", SCHNORR_SK_SIZE);
        set_constant!("SCHNORR_SIGNATURE_LENGTH", SCHNORR_SIG_SIZE);
        set_constant!("VRF_PK_LENGTH", VRF_PK_SIZE);
        set_constant!("VRF_SK_LENGTH", VRF_SK_SIZE);
        set_constant!("VRF_PROOF_LENGTH", VRF_PROOF_SIZE);
    }
);
