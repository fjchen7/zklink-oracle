use num_bigint::BigUint;
use pairing::{
    ff::{PrimeField, ScalarEngine},
    Engine,
};
use std::str::FromStr;
use sync_vm::{
    circuit_structures::byte::Byte,
    franklin_crypto::{
        bellman::{plonk::better_better_cs::cs::ConstraintSystem, SynthesisError},
        plonk::circuit::allocated_num::Num,
    },
    vm::primitives::uint256::UInt256,
};

pub fn bytes_be_to_num<CS: ConstraintSystem<E>, E: Engine>(
    cs: &mut CS,
    hash: &[Byte<E>],
) -> Result<Num<E>, SynthesisError> {
    let mut bytes = [Byte::zero(); 32];
    let len = hash.len();
    assert!(len <= <<E as ScalarEngine>::Fr as PrimeField>::CAPACITY as usize);
    bytes[(32 - len)..].copy_from_slice(hash);
    let uint = UInt256::from_be_bytes_fixed(cs, &bytes)?;
    uint.to_num_unchecked(cs)
}

pub fn bytes_to_hex<E: Engine>(bytes: &[Byte<E>]) -> String {
    let bbs = bytes
        .iter()
        .map(|b| b.get_byte_value().unwrap())
        .collect::<Vec<_>>();
    hex::encode(bbs)
}

pub fn uint256_and_u8_chunks_from_str<E: Engine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    str: &str,
) -> Result<(UInt256<E>, [Num<E>; 32]), SynthesisError> {
    let biguint = BigUint::from_str(str).map_err(|e| {
        let err = std::io::Error::new(std::io::ErrorKind::Other, e);
        SynthesisError::from(err)
    })?;
    UInt256::alloc_from_biguint_and_return_u8_chunks(cs, Some(biguint))
}

pub fn uint256_from_str<E: Engine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    str: &str,
) -> Result<UInt256<E>, SynthesisError> {
    Ok(uint256_and_u8_chunks_from_str(cs, str)?.0)
}

pub fn uint256_from_be_hex_str<E: Engine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    str: &str,
) -> Result<UInt256<E>, SynthesisError> {
    let bytes = hex::decode(str)
        .unwrap()
        .into_iter()
        .map(|b| Byte::constant(b))
        .collect::<Vec<_>>();
    let bytes: [Byte<E>; 32] = bytes.try_into().unwrap();
    UInt256::from_be_bytes_fixed(cs, &bytes)
}

pub fn new_synthesis_error<T: AsRef<str>>(msg: T) -> SynthesisError {
    let err = std::io::Error::new(std::io::ErrorKind::Other, msg.as_ref());
    SynthesisError::from(err)
}
