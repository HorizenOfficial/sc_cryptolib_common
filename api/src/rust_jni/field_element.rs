use super::*;
use crate::ginger_calls::{field_element::*, into_i8};

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
        _env,
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
        _env,
        _field_element_bytes,
        None,
        None,
        "com/horizen/librustsidechains/FieldElement",
        "com/horizen/librustsidechains/FieldElementException"
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

    return_field_element(_env, fe)
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

    return_field_element(_env, fe)
});

ffi_export!(
    fn Java_com_horizen_librustsidechains_FieldElement_nativeClone(
    _env: JNIEnv,
    _field_element: JObject,
) -> jobject
{
    let fe = parse_rust_struct_from_jobject::<FieldElement>(&_env, _field_element, "fieldElementPointer");
    return_field_element(_env, fe.clone())
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
    _fe: JObject,
)
{
    drop_rust_struct_from_jobject::<FieldElement>(_env, _fe, "fieldElementPointer")
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
    let field_1 = parse_rust_struct_from_jobject::<FieldElement>(&_env, _field_element_1, "fieldElementPointer");
    let field_2 = parse_rust_struct_from_jobject::<FieldElement>(&_env, _field_element_2, "fieldElementPointer");

    match field_1 == field_2 {
        true => JNI_TRUE,
        false => JNI_FALSE,
    }
});