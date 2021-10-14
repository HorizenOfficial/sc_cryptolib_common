use super::*;
use algebra::{
    serialize::*, SemanticallyValid
};
use crate::ginger_calls::serialization::*;
use std::any::type_name;

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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn parse_int_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> i32 {
    _env
        .get_field(obj, field_name, "I")
        .expect(format!("Should be able to get field {:?}", field_name).as_str())
        .i()
        .unwrap()
}

pub fn parse_long_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> i64 {
    _env
        .get_field(obj, field_name, "J")
        .expect(format!("Should be able to get field {:?}", field_name).as_str())
        .j()
        .unwrap()
}

fn parse_jbyte_array_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> jbyteArray {
    _env
        .get_field(obj, field_name, "[B")
        .expect(format!("Should be able to get field {:?}", field_name).as_str())
        .l()
        .unwrap()
        .cast()
}

pub fn parse_bytes_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> Vec<u8> {
    let t: jbyteArray = parse_jbyte_array_from_jobject(_env, obj, field_name);
    _env
        .convert_byte_array(t)
        .expect(format!("Should be able to convert {:?} to Rust byte array", field_name).as_str())
}

#[allow(dead_code)]
pub fn parse_fixed_jbyte_array<'a>(_env: &'a JNIEnv, array: jbyteArray, length: usize) -> Result<Vec<u8>, Error>
{
    let vec = _env
        .convert_byte_array(array)
        .expect("Should be able to convert to Rust byte array");

    if vec.len() != length {
       return Err(format!("Retrieved array of size {} expected to be {}.", vec.len(), length))?;
    }
    
    Ok(vec)
}

pub fn parse_rust_struct_from_jobject<'a, T: Sized>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> &'a T {
    read_raw_pointer(&_env, parse_long_from_jobject(_env, obj, field_name) as *const T)
}

pub fn parse_mut_rust_struct_from_jobject<'a, T: Sized>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> &'a mut T {
    read_mut_raw_pointer(&_env, parse_long_from_jobject(_env, obj, field_name) as *mut T)
}

pub fn drop_rust_struct_from_jobject<'a, T: Sized>(_env: JNIEnv, obj: JObject<'a>, field_name: &'a str) {
    let ptr = parse_long_from_jobject(&_env, obj, field_name) as *mut T;

    if ptr.is_null()  { return }
    drop(unsafe { Box::from_raw(ptr) });
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
    exception_path: &str,
) -> jobject
{
    let obj_bytes = _env.convert_byte_array(obj_bytes)
        .expect("Cannot read bytes.");
    
    map_or_throw!(
        _env,
        deserialize_from_buffer::<T>(
            obj_bytes.as_slice(),
            checked.map(|jni_bool| jni_bool == JNI_TRUE),
            compressed.map(|jni_bool| jni_bool == JNI_TRUE)
        ),
        class_path,
        exception_path,
        format!("Unable to deserialize {:?}", class_path).as_str()
    )
}
pub fn serialize_from_jobject<T: CanonicalSerialize>(
    _env: &JNIEnv,
    obj: JObject,
    ptr_name: &str,
    compressed: Option<jboolean>, // Can be none for uncompressable types
) -> jbyteArray
{
    let obj_bytes = serialize_from_raw_pointer(
        _env,
        parse_long_from_jobject(_env, obj, ptr_name) as *const T,
        compressed.map(|jni_bool| jni_bool == JNI_TRUE)
    );

    _env.byte_array_from_slice(obj_bytes.as_slice())
        .expect("Cannot write object.")
}

pub fn return_field_element(_env: &JNIEnv, fe: FieldElement) -> jobject
{
    return_jobject(_env, fe, "com/horizen/librustsidechains/FieldElement")
        .into_inner()
}