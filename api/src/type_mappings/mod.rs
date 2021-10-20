#[macro_use]
pub mod macros;
pub use macros::*;

#[cfg(all(not(feature = "bn_382"), feature = "tweedle"))]
pub mod tweedle;
#[cfg(all(not(feature = "bn_382"), feature = "tweedle"))]
pub use tweedle::*;

#[cfg(all(not(feature = "tweedle"), feature = "bn_382"))]
pub mod bn_382;
#[cfg(all(not(feature = "tweedle"), feature = "bn_382"))]
pub use bn_382::*;


use algebra::{PrimeField, FpParameters, FromCompressedBits, AffineCurve};
use blake2s_simd::{
    Hash, Params
};
use bit_vec::BitVec;

const GH_FIRST_BLOCK: &'static [u8; 64]
= b"53756e4d65726375727956656e757345617274684d6172734a75706974657253";

pub fn hash_to_curve<F: PrimeField, G: AffineCurve + FromCompressedBits>(
    tag: &[u8],
    personalization: &[u8]
) -> Option<G> {

    let compute_chunk =
        |input: &[u8], personalization: &[u8]| -> Hash {
            Params::new()
                .hash_length(32)
                .personal(personalization)
                .to_state()
                .update(GH_FIRST_BLOCK)
                .update(input)
                .finalize()
        };

    // Append counter byte to tag
    let tag_len = tag.len();
    let mut tag = tag.to_vec();
    tag.push(0u8);

    // Compute number of hashes to be concatenated in order to obtain a field element
    let field_size = F::size_in_bits();
    let bigint_size = (field_size + F::Params::REPR_SHAVE_BITS as usize)/8;
    let chunk_num = if bigint_size % 32 == 0 { bigint_size/32 } else { (bigint_size/32) + 1};
    let max_value = u8::max_value();
    let mut g = None;

    while tag[tag_len] <= max_value {

        let mut chunks = vec![];

        //chunk_0 = H(tag), chunk_1 = H(chunk_0) = H(H(tag)), ..., chunk_i = H(chunk_i-1)
        let mut prev_hash = tag.clone();
        for _ in 0..chunk_num {
            let hash = compute_chunk(prev_hash.as_slice(), personalization);
            chunks.extend_from_slice(hash.as_ref());
            prev_hash = hash.as_ref().to_vec();
        }

        tag[tag_len] += 1u8;

        //Mask away REPR_SHAVE_BITS
        let mut chunk_bits = BitVec::from_bytes(chunks.as_slice());
        for i in field_size..(bigint_size * 8) {
            chunk_bits.set(i, false);
        }

        //Get field element from `chunks`
        let chunk_bytes = chunk_bits.to_bytes();
        let fe = match F::from_random_bytes(&chunk_bytes[..bigint_size]) {
            Some(fe) => fe,
            None => continue
        };

        //Get point from chunks
        let mut fe_bits = fe.write_bits();
        fe_bits.push(false); //We don't want an infinity point
        fe_bits.push(false); //We decide to choose the even y coordinate
        match G::decompress(fe_bits) {
            Ok(point) => {
                g = Some(point);
                break;
            },
            Err(_) => continue
        };
    };
    g
}