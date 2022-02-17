use ark_ff::{Zero};
use ark_nonnative_field::NonNativeFieldVar;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::{ToBitsGadget};
use ark_relations::r1cs::ConstraintSystem;

#[test]
fn to_bits_test() {
    type F = ark_pallas::Fr;
    type CF = ark_pallas::Fq;

    let cs = ConstraintSystem::<CF>::new_ref();
    let f = F::zero();

    let f_var = NonNativeFieldVar::<F, CF>::new_input(cs.clone(), || Ok(f)).unwrap();
    f_var.to_bits_le().unwrap();
}
