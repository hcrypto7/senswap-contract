use num_bigint::BigUint;

pub struct Math {}

impl Math {
  ///
  /// Babylonian method (with a selectively initial guest)
  /// Complexity is O(log(log(n)))
  ///
  pub fn sqrt(m: BigUint) -> BigUint {
    if m < BigUint::from(2u128) {
      return m;
    }
    let bits: u64 = (m.bits() + 1) / 2;
    let mut start = BigUint::from(1u128) << (bits - 1);
    let mut end = BigUint::from(1u128) << (bits + 1);
    while start < end {
      end = (start.clone() + end.clone()) >> 1;
      start = m.clone() / end.clone();
    }
    end
  }
}