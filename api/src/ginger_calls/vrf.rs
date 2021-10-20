use super::*;
use algebra::{AffineCurve, ProjectiveCurve, ToConstraintField};
use primitives::{
    crh::FieldBasedHash,
    vrf::{ecvrf::*, FieldBasedVrf},
};
use rand::rngs::OsRng;

pub fn vrf_generate_key() -> (VRFPk, VRFSk) {
    let mut rng = OsRng;
    let (pk, sk) = VRFScheme::keygen(&mut rng);
    (pk.0.into_affine(), sk)
}

pub fn vrf_get_public_key(sk: &VRFSk) -> VRFPk {
    VRFScheme::get_public_key(sk).0.into_affine()
}

pub fn vrf_verify_public_key(pk: &VRFPk) -> bool {
    VRFScheme::keyverify(&FieldBasedEcVrfPk(pk.into_projective()))
}

pub fn vrf_prove(
    msg: &FieldElement,
    sk: &VRFSk,
    pk: &VRFPk,
) -> Result<(VRFProof, FieldElement), Error> {
    let mut rng = OsRng;

    //Compute proof
    let proof = VRFScheme::prove(
        &mut rng,
        &VRF_GH_PARAMS,
        &FieldBasedEcVrfPk(pk.into_projective()),
        sk,
        msg.clone(),
    )?;

    //Convert gamma from proof to field elements
    let gamma_coords = proof.gamma.to_field_elements().unwrap();

    //Compute VRF output
    let output = {
        let mut h = FieldHash::init_constant_length(3, None);
        h.update(msg.clone());
        gamma_coords.into_iter().for_each(|c| {
            h.update(c);
        });
        h.finalize()
    }?;

    Ok((proof, output))
}

pub fn vrf_proof_to_hash(
    msg: &FieldElement,
    pk: &VRFPk,
    proof: &VRFProof,
) -> Result<FieldElement, Error> {
    VRFScheme::proof_to_hash(
        &VRF_GH_PARAMS,
        &FieldBasedEcVrfPk(pk.into_projective()),
        msg.clone(),
        proof,
    )
}
