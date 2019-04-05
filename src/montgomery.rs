use crate::field::MODULUS;
use crate::u256::U256;
use crate::u256h;
use crate::utils::{adc, mac};
use hex_literal::*;
use std::num::Wrapping;

// M64 = -MODULUS^(-1) mod 2^64
pub const M64: u64 = 0xffff_ffff_ffff_ffff; // = -1

// R2 = 2^512 mod MODULUS
pub const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
pub const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
pub const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");

// TODO: Make const fn
#[inline(always)]
pub fn redc(lo: &U256, hi: &U256) -> U256 {
    // Algorithm 14.32 from Handbook of Applied Cryptography.
    // TODO: Optimize for the specific values of M64 and MODULUS.
    let ui = lo.c0.wrapping_mul(M64);
    let (_a0, carry) = mac(lo.c0, ui, MODULUS.c0, 0);
    let (a1, carry) = mac(lo.c1, ui, MODULUS.c1, carry);
    let (a2, carry) = mac(lo.c2, ui, MODULUS.c2, carry);
    let (a3, carry) = mac(lo.c3, ui, MODULUS.c3, carry);
    let (a4, carry) = adc(hi.c0, 0, carry);
    let (a5, carry) = adc(hi.c1, 0, carry);
    let (a6, carry) = adc(hi.c2, 0, carry);
    let (a7, _carry) = adc(hi.c3, 0, carry);
    let ui = a1.wrapping_mul(M64);
    let (_a1, carry) = mac(a1, ui, MODULUS.c0, 0);
    let (a2, carry) = mac(a2, ui, MODULUS.c1, carry);
    let (a3, carry) = mac(a3, ui, MODULUS.c2, carry);
    let (a4, carry) = mac(a4, ui, MODULUS.c3, carry);
    let (a5, carry) = adc(a5, 0, carry);
    let (a6, carry) = adc(a6, 0, carry);
    let (a7, _carry) = adc(a7, 0, carry);
    let ui = a2.wrapping_mul(M64);
    let (_a2, carry) = mac(a2, ui, MODULUS.c0, 0);
    let (a3, carry) = mac(a3, ui, MODULUS.c1, carry);
    let (a4, carry) = mac(a4, ui, MODULUS.c2, carry);
    let (a5, carry) = mac(a5, ui, MODULUS.c3, carry);
    let (a6, carry) = adc(a6, 0, carry);
    let (a7, _carry) = adc(a7, 0, carry);
    let ui = a3.wrapping_mul(M64);
    let (_a3, carry) = mac(a3, ui, MODULUS.c0, carry);
    let (a4, carry) = mac(a4, ui, MODULUS.c1, carry);
    let (a5, carry) = mac(a5, ui, MODULUS.c2, carry);
    let (a6, carry) = mac(a6, ui, MODULUS.c3, carry);
    let (a7, _carry) = adc(a7, 0, carry);

    // Final reduction
    let mut r = U256::new(a4, a5, a6, a7);
    if r >= MODULUS {
        r -= &MODULUS;
    }
    r
}

pub fn mul_redc(x: &U256, y: &U256) -> U256 {
    // Algorithm 14.36 from Handbook of Applied Cryptography
    let ui = ((Wrapping(x.c0) * Wrapping(y.c0)) * Wrapping(M64)).0;
    let (a0, carry) = mac(0, x.c0, y.c0, 0);
    let (a1, carry) = mac(0, x.c0, y.c1, carry);
    let (a2, carry) = mac(0, x.c0, y.c2, carry);
    let (a3, carry) = mac(0, x.c0, y.c3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, ui, MODULUS.c0, 0);
    let (a0, carry) = mac(a1, ui, MODULUS.c1, carry);
    let (a1, carry) = mac(a2, ui, MODULUS.c2, carry);
    let (a2, carry) = mac(a3, ui, MODULUS.c3, carry);
    let a3 = a4 + carry;
    let ui = ((Wrapping(a0) + Wrapping(x.c1) * Wrapping(y.c0)) * Wrapping(M64)).0;
    let (a0, carry) = mac(a0, x.c1, y.c0, 0);
    let (a1, carry) = mac(a1, x.c1, y.c1, carry);
    let (a2, carry) = mac(a2, x.c1, y.c2, carry);
    let (a3, carry) = mac(a3, x.c1, y.c3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, ui, MODULUS.c0, 0);
    let (a0, carry) = mac(a1, ui, MODULUS.c1, carry);
    let (a1, carry) = mac(a2, ui, MODULUS.c2, carry);
    let (a2, carry) = mac(a3, ui, MODULUS.c3, carry);
    let a3 = a4 + carry;
    let ui = ((Wrapping(a0) + Wrapping(x.c2) * Wrapping(y.c0)) * Wrapping(M64)).0;
    let (a0, carry) = mac(a0, x.c2, y.c0, 0);
    let (a1, carry) = mac(a1, x.c2, y.c1, carry);
    let (a2, carry) = mac(a2, x.c2, y.c2, carry);
    let (a3, carry) = mac(a3, x.c2, y.c3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, ui, MODULUS.c0, 0);
    let (a0, carry) = mac(a1, ui, MODULUS.c1, carry);
    let (a1, carry) = mac(a2, ui, MODULUS.c2, carry);
    let (a2, carry) = mac(a3, ui, MODULUS.c3, carry);
    let a3 = a4 + carry;
    let ui = ((Wrapping(a0) + Wrapping(x.c3) * Wrapping(y.c0)) * Wrapping(M64)).0;
    let (a0, carry) = mac(a0, x.c3, y.c0, 0);
    let (a1, carry) = mac(a1, x.c3, y.c1, carry);
    let (a2, carry) = mac(a2, x.c3, y.c2, carry);
    let (a3, carry) = mac(a3, x.c3, y.c3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, ui, MODULUS.c0, 0);
    let (a0, carry) = mac(a1, ui, MODULUS.c1, carry);
    let (a1, carry) = mac(a2, ui, MODULUS.c2, carry);
    let (a2, carry) = mac(a3, ui, MODULUS.c3, carry);
    let a3 = a4 + carry;

    // Final reduction
    let mut r = U256::new(a0, a1, a2, a3);
    if r >= MODULUS {
        r -= &MODULUS;
    }
    r
}

pub fn sqr_redc(a: &U256) -> U256 {
    // TODO: special case
    mul_redc(&a, &a)
}

pub fn inv_redc(n: &U256) -> Option<U256> {
    n.invmod(&MODULUS).map(|ni| mul_redc(&ni, &R3))
}

pub fn to_montgomery(n: &U256) -> U256 {
    mul_redc(n, &R2)
}

pub fn from_montgomery(n: &U256) -> U256 {
    redc(n, &U256::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012e440f0965e7029c218b64f1010006b5c4ba8b1497c4174a32fec025c197bc");
        assert_eq!(redc(&a, &b), c);
    }

    #[test]
    fn test_mul_redc() {
        let a = u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        let b = u256h!("024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b");
        let c = u256h!("012b854fc6321976d374ad069cfdec8bb7b2bd184259dae8f530cbb28f0805b4");
        assert_eq!(mul_redc(&a, &b), c);
    }

    #[quickcheck]
    #[test]
    fn test_to_from(mut n: U256) -> bool {
        n %= MODULUS;
        from_montgomery(&to_montgomery(&n)) == n
    }
}
