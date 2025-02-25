//! Various coding schemes to be used in conjunction with the core Paillier encryption scheme.

use std::marker::PhantomData;

use arithimpl::traits::ConvertFrom;
use serde::de::Deserialize;
use serde::ser::Serialize;
use BigInt;

pub mod integral;

/// Encrypted message with type information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedCiphertext<T> {
    #[serde(with = "::serialize::bigint")]
    pub raw: BigInt,

    pub components: usize,

    _phantom: PhantomData<T>,
}

impl<T> EncodedCiphertext<T> {
    pub fn new(raw: BigInt, components: usize) -> Self {
        Self { raw, components, _phantom: PhantomData }
    }
}

pub fn pack<T>(components: &[T], component_bitsize: usize) -> BigInt
where
    BigInt: From<T>,
    T: Copy,
{
    let mut packed = BigInt::from(components[0]);
    for component in &components[1..] {
        packed = packed << component_bitsize;
        packed = packed + BigInt::from(*component);
    }
    packed
}

pub fn unpack<T>(
    mut packed_components: BigInt,
    component_bitsize: usize,
    component_count: usize,
) -> Vec<T>
where
    T: ConvertFrom<BigInt>,
{
    let mask = BigInt::one() << component_bitsize;
    let mut components: Vec<T> = vec![];
    for _ in 0..component_count {
        let raw_component = &packed_components % &mask; // TODO replace with bitwise AND
        let component = T::_from(&raw_component);
        components.push(component);
        packed_components = &packed_components >> component_bitsize;
    }
    components.reverse();
    components
}

#[test]
fn test_pack() {
    let v: Vec<u64> = vec![1, 2, 3];

    let component_bitsize = 64;

    let packed = pack(&*v, component_bitsize);
    assert_eq!(
        packed,
        BigInt::from(1) * (BigInt::from(1) << 2 * component_bitsize)
            + BigInt::from(2) * (BigInt::from(1) << 1 * component_bitsize)
            + BigInt::from(3) * (BigInt::from(1) << 0 * component_bitsize)
    );

    let unpacked: Vec<u64> = unpack(packed, component_bitsize, 3);
    assert_eq!(unpacked, v);
}
