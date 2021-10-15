use super::*;
use algebra::{PrimeField, SerializationError, UniformRand};
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

pub fn read_field_element_from_u64(num: u64) -> FieldElement {
    FieldElement::from_repr(BigInteger::from(num))
}

//Will return error if buffer.len > FIELD_SIZE. If buffer.len < FIELD_SIZE, padding 0s will be added
pub fn read_field_element_from_buffer_with_padding<F: PrimeField>(
    buffer: &[u8],
) -> Result<F, SerializationError> {
    let buff_len = buffer.len();

    //Pad to reach field element size
    let mut new_buffer = Vec::new();
    new_buffer.extend_from_slice(buffer);

    for _ in buff_len..FIELD_SIZE {
        new_buffer.push(0u8)
    } //Add padding zeros to reach field size

    algebra::serialize::CanonicalDeserialize::deserialize(new_buffer.as_slice())
}

//*******************************Generic functions**********************************************

// NOTE: This function relies on a non-cryptographically safe RNG, therefore it
// must be used ONLY for testing purposes
pub fn get_random_field_element(seed: u64) -> FieldElement {
    let mut rng = XorShiftRng::seed_from_u64(seed);
    FieldElement::rand(&mut rng)
}
