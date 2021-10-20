use super::*;
use crate::ginger_calls::{
    field_element::read_field_element_from_buffer_with_padding, serialization::*,
};
use algebra::{serialize::*, SemanticallyValid};
use std::{any::type_name, fmt::Debug};

pub fn read_raw_pointer<'a, T>(env: &JNIEnv, input: *const T) -> &'a T {
    if input.is_null() {
        throw_and_exit!(
            env,
            "java/lang/NullPointerException",
            "Received null pointer"
        );
    }
    unsafe { &*input }
}

pub fn read_mut_raw_pointer<'a, T>(env: &JNIEnv, input: *mut T) -> &'a mut T {
    if input.is_null() {
        throw_and_exit!(
            env,
            "java/lang/NullPointerException",
            "Received null pointer"
        );
    }
    unsafe { &mut *input }
}

#[allow(dead_code)]
pub fn read_nullable_raw_pointer<'a, T>(input: *const T) -> Option<&'a T> {
    unsafe { input.as_ref() }
}

pub fn serialize_from_raw_pointer<T: CanonicalSerialize>(
    _env: &JNIEnv,
    to_write: *const T,
    compressed: Option<bool>,
) -> Vec<u8> {
    serialize_to_buffer(read_raw_pointer(&_env, to_write), compressed)
        .expect(format!("unable to write {} to buffer", type_name::<T>()).as_str())
}

#[allow(dead_code)]
pub fn parse_int_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> i32 {
    _env.get_field(obj, field_name, "I")
        .expect(format!("Should be able to get field {:?}", field_name).as_str())
        .i()
        .unwrap()
}

pub fn parse_long_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> i64 {
    _env.get_field(obj, field_name, "J")
        .expect(format!("Should be able to get field {:?}", field_name).as_str())
        .j()
        .unwrap()
}

fn parse_jbyte_array_from_jobject<'a>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
) -> jbyteArray {
    _env.get_field(obj, field_name, "[B")
        .expect(format!("Should be able to get field {:?}", field_name).as_str())
        .l()
        .unwrap()
        .cast()
}

pub fn parse_bytes_from_jobject<'a>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
) -> Vec<u8> {
    let t: jbyteArray = parse_jbyte_array_from_jobject(_env, obj, field_name);
    _env.convert_byte_array(t).expect(
        format!(
            "Should be able to convert {:?} to Rust byte array",
            field_name
        )
        .as_str(),
    )
}

#[allow(dead_code)]
pub fn parse_fixed_jbyte_array<'a>(
    _env: &'a JNIEnv,
    array: jbyteArray,
    length: usize,
) -> Result<Vec<u8>, Error> {
    let vec = _env
        .convert_byte_array(array)
        .expect("Should be able to convert to Rust byte array");

    if vec.len() != length {
        return Err(format!(
            "Retrieved array of size {} expected to be {}.",
            vec.len(),
            length
        ))?;
    }

    Ok(vec)
}

pub fn parse_rust_struct_from_jobject<'a, T: Sized>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
) -> &'a T {
    read_raw_pointer(
        &_env,
        parse_long_from_jobject(_env, obj, field_name) as *const T,
    )
}

#[macro_export]
macro_rules! parse_rust_struct_vec_from_jobject_array {
    ($_env: expr, $obj_array: expr, $rust_struct_array: expr, $array_name: expr, $field_name: expr) => {
        // Array can be empty
        for i in 0..$_env
            .get_array_length($obj_array)
            .expect(format!("Should be able to read {:?} array size", $array_name).as_str())
        {
            let obj = $_env.get_object_array_element($obj_array, i).expect(
                format!(
                    "Should be able to read elem {} of the {:?} array",
                    i, $array_name
                )
                .as_str(),
            );

            let rust_struct = parse_rust_struct_from_jobject(&$_env, obj, $field_name);

            $rust_struct_array.push(rust_struct);
        }
    };
}

pub fn parse_mut_rust_struct_from_jobject<'a, T: Sized>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
) -> &'a mut T {
    read_mut_raw_pointer(
        &_env,
        parse_long_from_jobject(_env, obj, field_name) as *mut T,
    )
}

pub fn drop_rust_struct_from_jobject<'a, T: Sized>(
    _env: JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
) {
    let ptr = parse_long_from_jobject(&_env, obj, field_name) as *mut T;

    if ptr.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(ptr) });
}

pub fn return_jobject<'a, T: Sized>(_env: &'a JNIEnv, obj: T, class_path: &str) -> JObject<'a> {
    //Return field element
    let obj_ptr: jlong = jlong::from(Box::into_raw(Box::new(obj)) as i64);

    let obj_class = _env
        .find_class(class_path)
        .expect("Should be able to find class");

    _env.new_object(obj_class, "(J)V", &[JValue::Long(obj_ptr)])
        .expect("Should be able to create new jobject")
}

/// Map a Result<T, E> to a jobject if Ok(), otherwise throw exception and return default JNI_NULL.
/// To be used mainly as final instruction of a Rust implementation of a JNI function returning a
/// jobject.
pub fn map_to_jobject_or_throw_exc<'a, T: Sized, E: Debug>(
    env: JNIEnv,
    res: Result<T, E>,
    class_path: &str,
    exception_path: &str,
    description: &str,
) -> jobject {
    res.map_or_else(
        |e| {
            throw!(
                &env,
                exception_path,
                format!("{:?}: {:?}", description, e).as_str(),
                JNI_NULL
            )
        },
        |ret| return_jobject(&env, ret, class_path).into_inner(),
    )
}

/// Map a type Result<bool, E> to a jboolean set to JNI_TRUE if Ok(true), JNI_FALSE if Ok(false)
/// otherwise throw exception and return default JNI_FALSE.
/// To be used mainly as final instruction of a Rust implementation of a JNI function returning a
/// jboolean.
pub fn map_to_jboolean_or_throw_exc<'a, E: Debug>(
    env: JNIEnv,
    res: Result<bool, E>,
    exception_path: &str,
    description: &str,
) -> jboolean {
    res.map_or_else(
        |e| {
            throw!(
                &env,
                exception_path,
                format!("{:?}: {:?}", description, e).as_str(),
                JNI_FALSE
            )
        },
        |ret| if ret { JNI_TRUE } else { JNI_FALSE },
    )
}

/// To be used mainly as final instruction of a Rust implementation of a JNI function returning a
/// jbyteArray.
pub fn map_to_jbytearray_or_throw_exc<'a, T: CanonicalSerialize + SemanticallyValid, E: Debug>(
    env: JNIEnv,
    res: Result<T, E>,
    compressed: Option<bool>,
    result_name: &str,
    exception_path: &str,
    description: &str,
) -> jbyteArray {
    res.map_or_else(
        |e| {
            throw!(
                &env,
                exception_path,
                format!("{:?}: {:?}", description, e).as_str(),
                JNI_NULL
            )
        },
        |ret| {
            //Serialize ret
            let ret_bytes = serialize_to_buffer(&ret, compressed)
                .expect(format!("Should be able to write {:?} into bytes", result_name).as_str());

            //Return jbyteArray
            env.byte_array_from_slice(ret_bytes.as_ref())
                .expect("Should be able to convert Rust slice into jbytearray")
        },
    )
}

pub fn deserialize_to_jobject<T: CanonicalDeserialize + SemanticallyValid>(
    _env: JNIEnv,
    obj_bytes: jbyteArray,
    checked: Option<jboolean>, // Can be none for types with trivial checks or without themn
    compressed: Option<jboolean>, // Can be none for uncompressable types
    class_path: &str,
    exception_path: &str,
) -> jobject {
    let obj_bytes = _env
        .convert_byte_array(obj_bytes)
        .expect("Cannot read bytes.");

    map_to_jobject_or_throw_exc(
        _env,
        deserialize_from_buffer::<T>(
            obj_bytes.as_slice(),
            checked.map(|jni_bool| jni_bool == JNI_TRUE),
            compressed.map(|jni_bool| jni_bool == JNI_TRUE),
        ),
        class_path,
        exception_path,
        format!("Unable to deserialize {:?}", class_path).as_str(),
    )
}
pub fn serialize_from_jobject<T: CanonicalSerialize>(
    _env: JNIEnv,
    obj: JObject,
    ptr_name: &str,
    compressed: Option<jboolean>, // Can be none for uncompressable types
) -> jbyteArray {
    let obj_bytes = serialize_from_raw_pointer(
        &_env,
        parse_long_from_jobject(&_env, obj, ptr_name) as *const T,
        compressed.map(|jni_bool| jni_bool == JNI_TRUE),
    );

    _env.byte_array_from_slice(obj_bytes.as_slice())
        .expect("Cannot write object.")
}

pub fn parse_field_element_from_jbyte_array<'a>(
    _env: &'a JNIEnv,
    bytes: jbyteArray,
) -> Result<FieldElement, Error> {
    let fe_bytes = parse_fixed_jbyte_array(&_env, bytes, FIELD_SIZE)?;
    let fe = read_field_element_from_buffer_with_padding(fe_bytes.as_slice())?;
    Ok(fe)
}

pub fn return_field_element(_env: JNIEnv, fe: FieldElement) -> jobject {
    return_jobject(
        &_env,
        fe,
        "com/horizen/common/librustsidechains/FieldElement",
    )
    .into_inner()
}
