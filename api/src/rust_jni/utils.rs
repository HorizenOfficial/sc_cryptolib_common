use super::*;
use std::{any::type_name, fmt::Debug, convert::TryInto};

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
        .unwrap_or_else(|_| panic!("unable to write {} to buffer", type_name::<T>()))
}

#[allow(dead_code)]
pub fn parse_int_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> i32 {
    _env.get_field(obj, field_name, "I")
        .unwrap_or_else(|_| panic!("Should be able to get field {:?}", field_name))
        .i()
        .unwrap()
}

pub fn parse_long_from_jobject<'a>(_env: &'a JNIEnv, obj: JObject<'a>, field_name: &'a str) -> i64 {
    _env.get_field(obj, field_name, "J")
        .unwrap_or_else(|_| panic!("Should be able to get field {:?}", field_name))
        .j()
        .unwrap()
}

fn parse_jbyte_array_from_jobject<'a>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
) -> jbyteArray {
    _env.get_field(obj, field_name, "[B")
        .unwrap_or_else(|_| panic!("Should be able to get field {:?}", field_name))
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
    _env.convert_byte_array(t).unwrap_or_else(|_| {
        panic!(
            "Should be able to convert {:?} to Rust byte array",
            field_name
        )
    })
}

#[allow(dead_code)]
pub fn parse_fixed_jbyte_array(
    _env: &JNIEnv,
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

pub fn parse_fixed_size_byte_array_from_jobject<const N: usize>(
    _env: &JNIEnv,
    obj: JObject,
    name: &str,
) -> Result<[u8; N], Error> {
    let j_bytes = parse_jbyte_array_from_jobject(_env, obj, name);
    Ok(
        parse_fixed_jbyte_array(_env, j_bytes, N)?
            .try_into()
            .unwrap()
    )
}

#[allow(unused)]
pub fn parse_fixed_size_bits_from_jbytearray_in_jobject<const N: usize>(
    _env: &JNIEnv,
    obj: JObject,
    name: &str,
) -> Result<[bool; N], Error> {
    let j_bytes = parse_jbyte_array_from_jobject(_env, obj, name);
    let len = (N as f32 / 8f32).ceil() as usize;
    let fixed_bytes = parse_fixed_jbyte_array(_env, j_bytes, len)?;
    let mut bits = Vec::with_capacity(fixed_bytes.len() * 8);
    for byte in fixed_bytes {
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            bits.push(bit == 1)
        }
    }
   Ok(bits[..N].try_into().unwrap())
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

pub fn parse_jobject_from_jobject<'a>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    field_name: &'a str,
    field_class_path: &'a str,
) -> JObject<'a> {
    _env
        .get_field(obj, field_name, format!("L{};", field_class_path))
        .expect(format!("Should be able to get {} field", field_class_path).as_str())
        .l()
        .unwrap()
}

/// Parse a Rust struct from a JObject 'outer_obj' containing another JObject
/// of type 'outer_obj_field_class_path', named 'outer_obj_field_name'.
/// We want to parse and convert the latter 'inner_field_name' into a Rust struct T.
pub fn parse_rust_struct_from_composite_jobject<'a, T: Sized>(
    _env: &'a JNIEnv,
    outer_obj: JObject<'a>,
    outer_obj_field_name: &'a str,
    outer_obj_field_class_path: &'a str,
    inner_obj_field_name: &'a str,
) -> &'a T {

    let inner_obj = parse_jobject_from_jobject(
        _env,
        outer_obj,
        outer_obj_field_name,
        outer_obj_field_class_path
    );

    parse_rust_struct_from_jobject(_env, inner_obj, inner_obj_field_name)
}

pub fn cast_joption_to_rust_option<'a>(
    _env: &'a JNIEnv,
    opt_object: JObject<'a>,
) -> Option<JObject<'a>> {
    if !_env
        .call_method(opt_object, "isPresent", "()Z", &[])
        .expect("Should be able to call isPresent method on Optional object")
        .z()
        .unwrap()
    {
        None
    } else {
        Some(
            _env.call_method(opt_object, "get", "()Ljava/lang/Object;", &[])
                .expect("Should be able to unwrap a non empty Optional")
                .l()
                .unwrap(),
        )
    }
}

pub fn parse_joption_from_jobject<'a>(
    _env: &'a JNIEnv,
    obj: JObject<'a>,
    opt_name: &'a str,
) -> Option<JObject<'a>> {
    // Parse Optional object
    let opt_object = parse_jobject_from_jobject(
        _env,
        obj,
        opt_name,
        "java/util/Optional"
    );

    // Cast it to Rust option
    cast_joption_to_rust_option(_env, opt_object)
}

pub fn parse_jobject_array_from_jobject(
    _env: &JNIEnv,
    obj: JObject,
    field_name: &str,
    list_obj_name: &str,
) -> jobjectArray {
    _env.get_field(obj, field_name, format!("[L{};", list_obj_name).as_str())
        .unwrap_or_else(|_| panic!("Should be able to get {}", field_name))
        .l()
        .unwrap()
        .cast()
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

#[allow(clippy::mut_from_ref)]
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
    let obj_ptr: jlong = Box::into_raw(Box::new(obj)) as i64;

    let obj_class = _env
        .find_class(class_path)
        .expect("Should be able to find class");

    _env.new_object(obj_class, "(J)V", &[JValue::Long(obj_ptr)])
        .expect("Should be able to create new jobject")
}

pub fn return_jobject_from_class<'a, T: Sized>(_env: &'a JNIEnv, obj: T, class: JClass<'a>) -> JObject<'a> {
    let obj_ptr: jlong = Box::into_raw(Box::new(obj)) as i64;

    _env.new_object(class, "(J)V", &[JValue::Long(obj_ptr)])
        .expect("Should be able to create new jobject")
}

/// Map a Result<T, E> to a jobject if Ok(), otherwise throw exception and return default JNI_NULL.
/// To be used mainly as final instruction of a Rust implementation of a JNI function returning a
/// jobject.
pub fn map_to_jobject_or_throw_exc<T: Sized, E: Debug>(
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
pub fn map_to_jboolean_or_throw_exc<E: Debug>(
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
pub fn map_to_jbytearray_or_throw_exc<T: CanonicalSerialize + SemanticallyValid, E: Debug>(
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
                .unwrap_or_else(|_| panic!("Should be able to write {:?} into bytes", result_name));

            //Return jbyteArray
            env.byte_array_from_slice(ret_bytes.as_ref())
                .unwrap_or_else(|_| panic!("Should be able to write {:?} into bytes", result_name))
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

pub fn parse_field_element_from_jbyte_array(
    _env: &JNIEnv,
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
        "io/horizen/common/librustsidechains/FieldElement",
    )
    .into_inner()
}
