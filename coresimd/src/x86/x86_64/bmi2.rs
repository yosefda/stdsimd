#[cfg(test)]
use stdsimd_test::assert_instr;

/// Unsigned multiply without affecting flags.
///
/// Unsigned multiplication of `a` with `b` returning a pair `(lo, hi)` with
/// the low half and the high half of the result.
#[inline(always)]
#[cfg_attr(test, assert_instr(mulx))]
#[target_feature(enable = "bmi2")]
#[cfg(not(target_arch = "x86"))] // calls an intrinsic
pub unsafe fn _mulx_u64(a: u64, b: u64, hi: &mut u64) -> u64 {
    let result: u128 = (a as u128) * (b as u128);
    *hi = (result >> 64) as u64;
    result as u64
}

/// Zero higher bits of `a` >= `index`.
#[inline(always)]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(bzhi))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _bzhi_u64(a: u64, index: u32) -> u64 {
    x86_bmi2_bzhi_64(a, index as u64)
}

/// Scatter contiguous low order bits of `a` to the result at the positions
/// specified by the `mask`.
#[inline(always)]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(pdep))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _pdep_u64(a: u64, mask: u64) -> u64 {
    x86_bmi2_pdep_64(a, mask)
}

/// Gathers the bits of `x` specified by the `mask` into the contiguous low
/// order bit positions of the result.
#[inline(always)]
#[target_feature(enable = "bmi2")]
#[cfg_attr(test, assert_instr(pext))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _pext_u64(a: u64, mask: u64) -> u64 {
    x86_bmi2_pext_64(a, mask)
}

extern "C" {
    #[link_name = "llvm.x86.bmi.bzhi.64"]
    fn x86_bmi2_bzhi_64(x: u64, y: u64) -> u64;
    #[link_name = "llvm.x86.bmi.pdep.64"]
    fn x86_bmi2_pdep_64(x: u64, y: u64) -> u64;
    #[link_name = "llvm.x86.bmi.pext.64"]
    fn x86_bmi2_pext_64(x: u64, y: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;

    use x86::*;

    #[simd_test = "bmi2"]
    unsafe fn test_pext_u64() {
        let n = 0b1011_1110_1001_0011u64;

        let m0 = 0b0110_0011_1000_0101u64;
        let s0 = 0b0000_0000_0011_0101u64;

        let m1 = 0b1110_1011_1110_1111u64;
        let s1 = 0b0001_0111_0100_0011u64;

        assert_eq!(_pext_u64(n, m0), s0);
        assert_eq!(_pext_u64(n, m1), s1);
    }

    #[simd_test = "bmi2"]
    unsafe fn test_pdep_u64() {
        let n = 0b1011_1110_1001_0011u64;

        let m0 = 0b0110_0011_1000_0101u64;
        let s0 = 0b0000_0010_0000_0101u64;

        let m1 = 0b1110_1011_1110_1111u64;
        let s1 = 0b1110_1001_0010_0011u64;

        assert_eq!(_pdep_u64(n, m0), s0);
        assert_eq!(_pdep_u64(n, m1), s1);
    }

    #[simd_test = "bmi2"]
    unsafe fn test_bzhi_u64() {
        let n = 0b1111_0010u64;
        let s = 0b0001_0010u64;
        assert_eq!(_bzhi_u64(n, 5), s);
    }

    #[simd_test = "bmi2"]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    unsafe fn test_mulx_u64() {
        let a: u64 = 9_223_372_036_854_775_800;
        let b: u64 = 100;
        let mut hi = 0;
        let lo = _mulx_u64(a, b, &mut hi);
        /*
result = 922337203685477580000 =
0b00110001_1111111111111111_1111111111111111_1111111111111111_1111110011100000
  ^~hi~~~~ ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        */
        assert_eq!(
            lo,
            0b11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u64
        );
        assert_eq!(hi, 0b00110001u64);
    }
}