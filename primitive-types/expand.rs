#![feature(prelude_import)]
//! Primitive types shared by Substrate and Parity Ethereum.
//!
//! Those are uint types `U128`, `U256` and `U512`, and fixed hash types `H160`,
//! `H256` and `H512`, with optional serde serialization, parity-scale-codec and
//! rlp encoding.
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use core::convert::TryFrom;
use fixed_hash::{construct_fixed_hash, impl_fixed_hash_conversions};
use uint::{construct_uint, uint_full_mul_reg};
/// Error type for conversion.
pub enum Error {
    /// Overflow encountered.
    Overflow,
}
#[automatically_derived]
impl ::core::fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Overflow")
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Error {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Error) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Error {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
/// Little-endian large integer type
#[repr(C)]
/// 128-bit unsigned integer.
pub struct U128(pub [u64; 2]);
#[automatically_derived]
impl ::core::marker::Copy for U128 {}
#[automatically_derived]
impl ::core::clone::Clone for U128 {
    #[inline]
    fn clone(&self) -> U128 {
        let _: ::core::clone::AssertParamIsClone<[u64; 2]>;
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for U128 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u64; 2]>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for U128 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for U128 {
    #[inline]
    fn eq(&self, other: &U128) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::hash::Hash for U128 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
/// Get a reference to the underlying little-endian words.
impl AsRef<[u64]> for U128 {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}
impl<'a> From<&'a U128> for U128 {
    fn from(x: &'a U128) -> U128 {
        *x
    }
}
impl U128 {
    const WORD_BITS: usize = 64;
    /// Maximum value.
    pub const MAX: U128 = U128([u64::max_value(); 2]);
    /// Converts a string slice in a given base to an integer. Only supports radixes of 10
    /// and 16.
    pub fn from_str_radix(
        txt: &str,
        radix: u32,
    ) -> Result<Self, ::uint::FromStrRadixErr> {
        let parsed = match radix {
            10 => Self::from_dec_str(txt)?,
            16 => core::str::FromStr::from_str(txt)?,
            _ => return Err(::uint::FromStrRadixErr::unsupported()),
        };
        Ok(parsed)
    }
    /// Convert from a decimal string.
    pub fn from_dec_str(
        value: &str,
    ) -> ::uint::core_::result::Result<Self, ::uint::FromDecStrErr> {
        let mut res = Self::default();
        for b in value.bytes().map(|b| b.wrapping_sub(b'0')) {
            if b > 9 {
                return Err(::uint::FromDecStrErr::InvalidCharacter);
            }
            let (r, overflow) = res.overflowing_mul_u64(10);
            if overflow > 0 {
                return Err(::uint::FromDecStrErr::InvalidLength);
            }
            let (r, overflow) = r.overflowing_add(b.into());
            if overflow {
                return Err(::uint::FromDecStrErr::InvalidLength);
            }
            res = r;
        }
        Ok(res)
    }
    /// Conversion to u32
    #[inline]
    pub const fn low_u32(&self) -> u32 {
        let &U128(ref arr) = self;
        arr[0] as u32
    }
    /// Low word (u64)
    #[inline]
    pub const fn low_u64(&self) -> u64 {
        let &U128(ref arr) = self;
        arr[0]
    }
    /// Conversion to u32 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than 2^32.
    #[inline]
    pub fn as_u32(&self) -> u32 {
        let &U128(ref arr) = self;
        if !self.fits_word() || arr[0] > u32::max_value() as u64 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to u32"),
                );
            }
        }
        self.as_u64() as u32
    }
    /// Conversion to u64 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than u64::max_value().
    #[inline]
    pub fn as_u64(&self) -> u64 {
        let &U128(ref arr) = self;
        if !self.fits_word() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to u64"),
                );
            }
        }
        arr[0]
    }
    /// Conversion to usize with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than usize::max_value().
    #[inline]
    pub fn as_usize(&self) -> usize {
        let &U128(ref arr) = self;
        if !self.fits_word() || arr[0] > usize::max_value() as u64 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to usize"),
                );
            }
        }
        arr[0] as usize
    }
    /// Whether this is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        let &U128(ref arr) = self;
        let mut i = 0;
        while i < 2 {
            if arr[i] != 0 {
                return false;
            } else {
                i += 1;
            }
        }
        return true;
    }
    #[inline]
    fn fits_word(&self) -> bool {
        let &U128(ref arr) = self;
        for i in 1..2 {
            if arr[i] != 0 {
                return false;
            }
        }
        return true;
    }
    /// Return the least number of bits needed to represent the number
    #[inline]
    pub fn bits(&self) -> usize {
        let &U128(ref arr) = self;
        for i in 1..2 {
            if arr[2 - i] > 0 {
                return (0x40 * (2 - i + 1)) - arr[2 - i].leading_zeros() as usize;
            }
        }
        0x40 - arr[0].leading_zeros() as usize
    }
    /// Return if specific bit is set.
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the bit width of the number.
    #[inline]
    pub const fn bit(&self, index: usize) -> bool {
        let &U128(ref arr) = self;
        arr[index / 64] & (1 << (index % 64)) != 0
    }
    /// Returns the number of leading zeros in the binary representation of self.
    pub fn leading_zeros(&self) -> u32 {
        let mut r = 0;
        for i in 0..2 {
            let w = self.0[2 - i - 1];
            if w == 0 {
                r += 64;
            } else {
                r += w.leading_zeros();
                break;
            }
        }
        r
    }
    /// Returns the number of trailing zeros in the binary representation of self.
    pub fn trailing_zeros(&self) -> u32 {
        let mut r = 0;
        for i in 0..2 {
            let w = self.0[i];
            if w == 0 {
                r += 64;
            } else {
                r += w.trailing_zeros();
                break;
            }
        }
        r
    }
    /// Return specific byte. Byte 0 is the least significant value (ie~ little endian).
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the byte width of the number.
    #[inline]
    pub const fn byte(&self, index: usize) -> u8 {
        let &U128(ref arr) = self;
        (arr[index / 8] >> (((index % 8)) * 8)) as u8
    }
    /// Write to the slice in big-endian format.
    #[inline]
    pub fn to_big_endian(&self, bytes: &mut [u8]) {
        use ::uint::byteorder::{ByteOrder, BigEndian};
        if true {
            if !(2 * 8 == bytes.len()) {
                ::core::panicking::panic("assertion failed: 2 * 8 == bytes.len()")
            }
        }
        for i in 0..2 {
            BigEndian::write_u64(&mut bytes[8 * i..], self.0[2 - i - 1]);
        }
    }
    /// Write to the slice in little-endian format.
    #[inline]
    pub fn to_little_endian(&self, bytes: &mut [u8]) {
        use ::uint::byteorder::{ByteOrder, LittleEndian};
        if true {
            if !(2 * 8 == bytes.len()) {
                ::core::panicking::panic("assertion failed: 2 * 8 == bytes.len()")
            }
        }
        for i in 0..2 {
            LittleEndian::write_u64(&mut bytes[8 * i..], self.0[i]);
        }
    }
    /// Create `10**n` as this type.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type.
    #[inline]
    pub fn exp10(n: usize) -> Self {
        match n {
            0 => Self::from(1u64),
            _ => Self::exp10(n - 1) * 10u32,
        }
    }
    /// Zero (additive identity) of this type.
    #[inline]
    pub const fn zero() -> Self {
        Self([0; 2])
    }
    /// One (multiplicative identity) of this type.
    #[inline]
    pub const fn one() -> Self {
        let mut words = [0; 2];
        words[0] = 1u64;
        Self(words)
    }
    /// The maximum value which can be inhabited by this type.
    #[inline]
    pub const fn max_value() -> Self {
        Self::MAX
    }
    fn full_shl(self, shift: u32) -> [u64; 2 + 1] {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic(
                    "assertion failed: shift < Self::WORD_BITS as u32",
                )
            }
        }
        let mut u = [0u64; 2 + 1];
        let u_lo = self.0[0] << shift;
        let u_hi = self >> (Self::WORD_BITS as u32 - shift);
        u[0] = u_lo;
        u[1..].copy_from_slice(&u_hi.0[..]);
        u
    }
    fn full_shr(u: [u64; 2 + 1], shift: u32) -> Self {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic(
                    "assertion failed: shift < Self::WORD_BITS as u32",
                )
            }
        }
        let mut res = Self::zero();
        for i in 0..2 {
            res.0[i] = u[i] >> shift;
        }
        if shift > 0 {
            for i in 1..=2 {
                res.0[i - 1] |= u[i] << (Self::WORD_BITS as u32 - shift);
            }
        }
        res
    }
    fn full_mul_u64(self, by: u64) -> [u64; 2 + 1] {
        let (prod, carry) = self.overflowing_mul_u64(by);
        let mut res = [0u64; 2 + 1];
        res[..2].copy_from_slice(&prod.0[..]);
        res[2] = carry;
        res
    }
    fn div_mod_small(mut self, other: u64) -> (Self, Self) {
        let mut rem = 0u64;
        self.0
            .iter_mut()
            .rev()
            .for_each(|d| {
                let (q, r) = Self::div_mod_word(rem, *d, other);
                *d = q;
                rem = r;
            });
        (self, rem.into())
    }
    fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
        if true {
            if !(self.bits() >= v.bits() && !v.fits_word()) {
                ::core::panicking::panic(
                    "assertion failed: self.bits() >= v.bits() && !v.fits_word()",
                )
            }
        }
        if true {
            if !(n + m <= 2) {
                ::core::panicking::panic("assertion failed: n + m <= 2")
            }
        }
        let shift = v.0[n - 1].leading_zeros();
        v <<= shift;
        let mut u = self.full_shl(shift);
        let mut q = Self::zero();
        let v_n_1 = v.0[n - 1];
        let v_n_2 = v.0[n - 2];
        for j in (0..=m).rev() {
            let u_jn = u[j + n];
            let mut q_hat = if u_jn < v_n_1 {
                let (mut q_hat, mut r_hat) = Self::div_mod_word(
                    u_jn,
                    u[j + n - 1],
                    v_n_1,
                );
                loop {
                    let (hi, lo) = Self::split_u128(
                        u128::from(q_hat) * u128::from(v_n_2),
                    );
                    if (hi, lo) <= (r_hat, u[j + n - 2]) {
                        break;
                    }
                    q_hat -= 1;
                    let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                    r_hat = new_r_hat;
                    if overflow {
                        break;
                    }
                }
                q_hat
            } else {
                u64::max_value()
            };
            let q_hat_v = v.full_mul_u64(q_hat);
            let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);
            if c {
                q_hat -= 1;
                let c = Self::add_slice(&mut u[j..], &v.0[..n]);
                u[j + n] = u[j + n].wrapping_add(u64::from(c));
            }
            q.0[j] = q_hat;
        }
        let remainder = Self::full_shr(u, shift);
        (q, remainder)
    }
    fn words(bits: usize) -> usize {
        if true {
            if !(bits > 0) {
                ::core::panicking::panic("assertion failed: bits > 0")
            }
        }
        1 + (bits - 1) / Self::WORD_BITS
    }
    /// Returns a pair `(self / other, self % other)`.
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero.
    pub fn div_mod(mut self, mut other: Self) -> (Self, Self) {
        use ::uint::core_::cmp::Ordering;
        let my_bits = self.bits();
        let your_bits = other.bits();
        if !(your_bits != 0) {
            {
                ::core::panicking::panic_fmt(format_args!("division by zero"));
            }
        }
        if my_bits < your_bits {
            return (Self::zero(), self);
        }
        if your_bits <= Self::WORD_BITS {
            return self.div_mod_small(other.low_u64());
        }
        let (n, m) = {
            let my_words = Self::words(my_bits);
            let your_words = Self::words(your_bits);
            (your_words, my_words - your_words)
        };
        self.div_mod_knuth(other, n, m)
    }
    /// Compute the highest `n` such that `n * n <= self`.
    pub fn integer_sqrt(&self) -> Self {
        let one = Self::one();
        if self <= &one {
            return *self;
        }
        let shift: u32 = (self.bits() as u32 + 1) / 2;
        let mut x_prev = one << shift;
        loop {
            let x = (x_prev + self / x_prev) >> 1;
            if x >= x_prev {
                return x_prev;
            }
            x_prev = x;
        }
    }
    /// Fast exponentiation by squaring
    /// https://en.wikipedia.org/wiki/Exponentiation_by_squaring
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type.
    pub fn pow(self, expon: Self) -> Self {
        if expon.is_zero() {
            return Self::one();
        }
        let is_even = |x: &Self| x.low_u64() & 1 == 0;
        let u_one = Self::one();
        let mut y = u_one;
        let mut n = expon;
        let mut x = self;
        while n > u_one {
            if is_even(&n) {
                x = x * x;
                n >>= 1usize;
            } else {
                y = x * y;
                x = x * x;
                n.0[2 - 1] &= (!0u64) >> 1;
                n >>= 1usize;
            }
        }
        x * y
    }
    /// Fast exponentiation by squaring. Returns result and overflow flag.
    pub fn overflowing_pow(self, expon: Self) -> (Self, bool) {
        if expon.is_zero() {
            return (Self::one(), false);
        }
        let is_even = |x: &Self| x.low_u64() & 1 == 0;
        let u_one = Self::one();
        let mut y = u_one;
        let mut n = expon;
        let mut x = self;
        let mut overflow = false;
        while n > u_one {
            if is_even(&n) {
                x = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(x);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                n >>= 1usize;
            } else {
                y = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(y);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                x = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(x);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                n = (n - u_one) >> 1usize;
            }
        }
        let res = {
            let (overflow_x, overflow_overflow) = x.overflowing_mul(y);
            overflow |= overflow_overflow;
            overflow_x
        };
        (res, overflow)
    }
    /// Checked exponentiation. Returns `None` if overflow occurred.
    pub fn checked_pow(self, expon: U128) -> Option<U128> {
        match self.overflowing_pow(expon) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Addition which overflows and returns a flag if it does.
    #[inline(always)]
    pub fn overflowing_add(self, other: U128) -> (U128, bool) {
        {
            use ::uint::core_ as core;
            let U128(ref me) = self;
            let U128(ref you) = other;
            let mut ret = [0u64; 2];
            let mut carry = 0u64;
            #[allow(unknown_lints, eq_op)]
            const _: [(); 0
                - !{
                    const ASSERT: bool = core::isize::MAX as usize
                        / core::mem::size_of::<u64>() > 2;
                    ASSERT
                } as usize] = [];
            use ::uint::unroll;
            #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U128(ret), carry > 0)
        }
    }
    /// Addition which saturates at the maximum value (Self::MAX).
    pub fn saturating_add(self, other: U128) -> U128 {
        match self.overflowing_add(other) {
            (_, true) => U128::MAX,
            (val, false) => val,
        }
    }
    /// Checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(self, other: U128) -> Option<U128> {
        match self.overflowing_add(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Subtraction which underflows and returns a flag if it does.
    #[inline(always)]
    pub fn overflowing_sub(self, other: U128) -> (U128, bool) {
        {
            use ::uint::core_ as core;
            let U128(ref me) = self;
            let U128(ref you) = other;
            let mut ret = [0u64; 2];
            let mut carry = 0u64;
            #[allow(unknown_lints, eq_op)]
            const _: [(); 0
                - !{
                    const ASSERT: bool = core::isize::MAX as usize
                        / core::mem::size_of::<u64>() > 2;
                    ASSERT
                } as usize] = [];
            use ::uint::unroll;
            #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U128(ret), carry > 0)
        }
    }
    /// Subtraction which saturates at zero.
    pub fn saturating_sub(self, other: U128) -> U128 {
        match self.overflowing_sub(other) {
            (_, true) => U128::zero(),
            (val, false) => val,
        }
    }
    /// Checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(self, other: U128) -> Option<U128> {
        match self.overflowing_sub(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Computes the absolute difference between self and other.
    pub fn abs_diff(self, other: U128) -> U128 {
        if self > other {
            self.overflowing_sub(other).0
        } else {
            other.overflowing_sub(self).0
        }
    }
    /// Multiply with overflow, returning a flag if it does.
    #[inline(always)]
    pub fn overflowing_mul(self, other: U128) -> (U128, bool) {
        {
            let ret: [u64; 2 * 2] = {
                {
                    #![allow(unused_assignments)]
                    let U128(ref me) = self;
                    let U128(ref you) = other;
                    let mut ret = [0u64; 2 * 2];
                    use ::uint::unroll;
                    #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                    {
                        {
                            const i: usize = 0;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 1;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        };
                    }
                    ret
                }
            };
            let ret: [[u64; 2]; 2] = unsafe { ::uint::core_::mem::transmute(ret) };
            #[inline(always)]
            fn any_nonzero(arr: &[u64; 2]) -> bool {
                use ::uint::unroll;
                #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                {
                    {
                        const i: usize = 0;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 1;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    };
                }
                false
            }
            (U128(ret[0]), any_nonzero(&ret[1]))
        }
    }
    /// Multiplication which saturates at the maximum value..
    pub fn saturating_mul(self, other: U128) -> U128 {
        match self.overflowing_mul(other) {
            (_, true) => U128::MAX,
            (val, false) => val,
        }
    }
    /// Checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(self, other: U128) -> Option<U128> {
        match self.overflowing_mul(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Checked division. Returns `None` if `other == 0`.
    pub fn checked_div(self, other: U128) -> Option<U128> {
        if other.is_zero() { None } else { Some(self / other) }
    }
    /// Checked modulus. Returns `None` if `other == 0`.
    pub fn checked_rem(self, other: U128) -> Option<U128> {
        if other.is_zero() { None } else { Some(self % other) }
    }
    /// Negation with overflow.
    pub fn overflowing_neg(self) -> (U128, bool) {
        if self.is_zero() { (self, false) } else { (!self + 1, true) }
    }
    /// Checked negation. Returns `None` unless `self == 0`.
    pub fn checked_neg(self) -> Option<U128> {
        match self.overflowing_neg() {
            (_, true) => None,
            (zero, false) => Some(zero),
        }
    }
    #[inline(always)]
    fn div_mod_word(hi: u64, lo: u64, y: u64) -> (u64, u64) {
        if true {
            if !(hi < y) {
                ::core::panicking::panic("assertion failed: hi < y")
            }
        }
        let x = (u128::from(hi) << 64) + u128::from(lo);
        let y = u128::from(y);
        ((x / y) as u64, (x % y) as u64)
    }
    #[inline(always)]
    fn add_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_add)
    }
    #[inline(always)]
    fn sub_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_sub)
    }
    #[inline(always)]
    fn binop_slice(
        a: &mut [u64],
        b: &[u64],
        binop: impl Fn(u64, u64) -> (u64, bool) + Copy,
    ) -> bool {
        let mut c = false;
        a.iter_mut()
            .zip(b.iter())
            .for_each(|(x, y)| {
                let (res, carry) = Self::binop_carry(*x, *y, c, binop);
                *x = res;
                c = carry;
            });
        c
    }
    #[inline(always)]
    fn binop_carry(
        a: u64,
        b: u64,
        c: bool,
        binop: impl Fn(u64, u64) -> (u64, bool),
    ) -> (u64, bool) {
        let (res1, overflow1) = b.overflowing_add(u64::from(c));
        let (res2, overflow2) = binop(a, res1);
        (res2, overflow1 || overflow2)
    }
    #[inline(always)]
    const fn mul_u64(a: u64, b: u64, carry: u64) -> (u64, u64) {
        let (hi, lo) = Self::split_u128(a as u128 * b as u128 + carry as u128);
        (lo, hi)
    }
    #[inline(always)]
    const fn split(a: u64) -> (u64, u64) {
        (a >> 32, a & 0xFFFF_FFFF)
    }
    #[inline(always)]
    const fn split_u128(a: u128) -> (u64, u64) {
        ((a >> 64) as _, (a & 0xFFFFFFFFFFFFFFFF) as _)
    }
    /// Overflowing multiplication by u64.
    /// Returns the result and carry.
    fn overflowing_mul_u64(mut self, other: u64) -> (Self, u64) {
        let mut carry = 0u64;
        for d in self.0.iter_mut() {
            let (res, c) = Self::mul_u64(*d, other, carry);
            *d = res;
            carry = c;
        }
        (self, carry)
    }
    /// Converts from big endian representation bytes in memory.
    pub fn from_big_endian(slice: &[u8]) -> Self {
        use ::uint::byteorder::{ByteOrder, BigEndian};
        if !(2 * 8 >= slice.len()) {
            ::core::panicking::panic("assertion failed: 2 * 8 >= slice.len()")
        }
        let mut padded = [0u8; 2 * 8];
        padded[2 * 8 - slice.len()..2 * 8].copy_from_slice(&slice);
        let mut ret = [0; 2];
        for i in 0..2 {
            ret[2 - i - 1] = BigEndian::read_u64(&padded[8 * i..]);
        }
        U128(ret)
    }
    /// Converts from little endian representation bytes in memory.
    pub fn from_little_endian(slice: &[u8]) -> Self {
        use ::uint::byteorder::{ByteOrder, LittleEndian};
        if !(2 * 8 >= slice.len()) {
            ::core::panicking::panic("assertion failed: 2 * 8 >= slice.len()")
        }
        let mut padded = [0u8; 2 * 8];
        padded[0..slice.len()].copy_from_slice(&slice);
        let mut ret = [0; 2];
        for i in 0..2 {
            ret[i] = LittleEndian::read_u64(&padded[8 * i..]);
        }
        U128(ret)
    }
    fn fmt_hex(
        &self,
        f: &mut ::uint::core_::fmt::Formatter,
        is_lower: bool,
    ) -> ::uint::core_::fmt::Result {
        let &U128(ref data) = self;
        if self.is_zero() {
            return f.pad_integral(true, "0x", "0");
        }
        let mut latch = false;
        let mut buf = [0_u8; 2 * 16];
        let mut i = 0;
        for ch in data.iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64))
                    >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0;
                }
                if latch {
                    let nibble = match nibble {
                        0..=9 => nibble as u8 + b'0',
                        _ if is_lower => nibble as u8 - 10 + b'a',
                        _ => nibble as u8 - 10 + b'A',
                    };
                    buf[i] = nibble;
                    i += 1;
                }
            }
        }
        let s = unsafe { ::uint::core_::str::from_utf8_unchecked(&buf[0..i]) };
        f.pad_integral(true, "0x", s)
    }
}
impl ::uint::core_::convert::From<U128> for [u8; 2 * 8] {
    fn from(number: U128) -> Self {
        let mut arr = [0u8; 2 * 8];
        number.to_big_endian(&mut arr);
        arr
    }
}
impl ::uint::core_::convert::From<[u8; 2 * 8]> for U128 {
    fn from(bytes: [u8; 2 * 8]) -> Self {
        Self::from(&bytes)
    }
}
impl<'a> ::uint::core_::convert::From<&'a [u8; 2 * 8]> for U128 {
    fn from(bytes: &[u8; 2 * 8]) -> Self {
        Self::from(&bytes[..])
    }
}
impl ::uint::core_::default::Default for U128 {
    fn default() -> Self {
        U128::zero()
    }
}
impl ::uint::core_::convert::From<u64> for U128 {
    fn from(value: u64) -> U128 {
        let mut ret = [0; 2];
        ret[0] = value;
        U128(ret)
    }
}
impl From<u8> for U128 {
    fn from(value: u8) -> U128 {
        From::from(value as u64)
    }
}
impl From<u16> for U128 {
    fn from(value: u16) -> U128 {
        From::from(value as u64)
    }
}
impl From<u32> for U128 {
    fn from(value: u32) -> U128 {
        From::from(value as u64)
    }
}
impl From<usize> for U128 {
    fn from(value: usize) -> U128 {
        From::from(value as u64)
    }
}
impl ::uint::core_::convert::From<i64> for U128 {
    fn from(value: i64) -> U128 {
        match value >= 0 {
            true => From::from(value as u64),
            false => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Unsigned integer can\'t be created from negative value",
                        ),
                    );
                };
            }
        }
    }
}
impl From<i8> for U128 {
    fn from(value: i8) -> U128 {
        From::from(value as i64)
    }
}
impl From<i16> for U128 {
    fn from(value: i16) -> U128 {
        From::from(value as i64)
    }
}
impl From<i32> for U128 {
    fn from(value: i32) -> U128 {
        From::from(value as i64)
    }
}
impl From<isize> for U128 {
    fn from(value: isize) -> U128 {
        From::from(value as i64)
    }
}
impl<'a> ::uint::core_::convert::From<&'a [u8]> for U128 {
    fn from(bytes: &[u8]) -> U128 {
        Self::from_big_endian(bytes)
    }
}
impl ::uint::core_::convert::TryFrom<U128> for u8 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<u8, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <u8>::max_value() as u64 {
            Err("integer overflow when casting to u8")
        } else {
            Ok(arr[0] as u8)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for u16 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<u16, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <u16>::max_value() as u64 {
            Err("integer overflow when casting to u16")
        } else {
            Ok(arr[0] as u16)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for u32 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<u32, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <u32>::max_value() as u64 {
            Err("integer overflow when casting to u32")
        } else {
            Ok(arr[0] as u32)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for usize {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<usize, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <usize>::max_value() as u64 {
            Err("integer overflow when casting to usize")
        } else {
            Ok(arr[0] as usize)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for u64 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<u64, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <u64>::max_value() as u64 {
            Err("integer overflow when casting to u64")
        } else {
            Ok(arr[0] as u64)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for i8 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<i8, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <i8>::max_value() as u64 {
            Err("integer overflow when casting to i8")
        } else {
            Ok(arr[0] as i8)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for i16 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<i16, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <i16>::max_value() as u64 {
            Err("integer overflow when casting to i16")
        } else {
            Ok(arr[0] as i16)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for i32 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<i32, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <i32>::max_value() as u64 {
            Err("integer overflow when casting to i32")
        } else {
            Ok(arr[0] as i32)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for isize {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<isize, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <isize>::max_value() as u64 {
            Err("integer overflow when casting to isize")
        } else {
            Ok(arr[0] as isize)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U128> for i64 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<i64, &'static str> {
        let U128(arr) = u;
        if !u.fits_word() || arr[0] > <i64>::max_value() as u64 {
            Err("integer overflow when casting to i64")
        } else {
            Ok(arr[0] as i64)
        }
    }
}
impl<T> ::uint::core_::ops::Add<T> for U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn add(self, other: T) -> U128 {
        let (result, overflow) = self.overflowing_add(other.into());
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a, T> ::uint::core_::ops::Add<T> for &'a U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn add(self, other: T) -> U128 {
        *self + other
    }
}
impl ::uint::core_::ops::AddAssign<U128> for U128 {
    fn add_assign(&mut self, other: U128) {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        *self = result;
    }
}
impl<T> ::uint::core_::ops::Sub<T> for U128
where
    T: Into<U128>,
{
    type Output = U128;
    #[inline]
    fn sub(self, other: T) -> U128 {
        let (result, overflow) = self.overflowing_sub(other.into());
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a, T> ::uint::core_::ops::Sub<T> for &'a U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn sub(self, other: T) -> U128 {
        *self - other
    }
}
impl ::uint::core_::ops::SubAssign<U128> for U128 {
    fn sub_assign(&mut self, other: U128) {
        let (result, overflow) = self.overflowing_sub(other);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<U128> for U128 {
    type Output = U128;
    fn mul(self, other: U128) -> U128 {
        let bignum: U128 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a U128> for U128 {
    type Output = U128;
    fn mul(self, other: &'a U128) -> U128 {
        let bignum: U128 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a U128> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a U128) -> U128 {
        let bignum: U128 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<U128> for &'a U128 {
    type Output = U128;
    fn mul(self, other: U128) -> U128 {
        let bignum: U128 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<U128> for U128 {
    fn mul_assign(&mut self, other: U128) {
        let result = *self * other;
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u8> for U128 {
    type Output = U128;
    fn mul(self, other: u8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u8> for U128 {
    type Output = U128;
    fn mul(self, other: &'a u8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u8> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a u8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u8> for &'a U128 {
    type Output = U128;
    fn mul(self, other: u8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u8> for U128 {
    fn mul_assign(&mut self, other: u8) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u16> for U128 {
    type Output = U128;
    fn mul(self, other: u16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u16> for U128 {
    type Output = U128;
    fn mul(self, other: &'a u16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u16> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a u16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u16> for &'a U128 {
    type Output = U128;
    fn mul(self, other: u16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u16> for U128 {
    fn mul_assign(&mut self, other: u16) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u32> for U128 {
    type Output = U128;
    fn mul(self, other: u32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u32> for U128 {
    type Output = U128;
    fn mul(self, other: &'a u32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u32> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a u32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u32> for &'a U128 {
    type Output = U128;
    fn mul(self, other: u32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u32> for U128 {
    fn mul_assign(&mut self, other: u32) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u64> for U128 {
    type Output = U128;
    fn mul(self, other: u64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u64> for U128 {
    type Output = U128;
    fn mul(self, other: &'a u64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u64> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a u64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u64> for &'a U128 {
    type Output = U128;
    fn mul(self, other: u64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u64> for U128 {
    fn mul_assign(&mut self, other: u64) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<usize> for U128 {
    type Output = U128;
    fn mul(self, other: usize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a usize> for U128 {
    type Output = U128;
    fn mul(self, other: &'a usize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a usize> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a usize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<usize> for &'a U128 {
    type Output = U128;
    fn mul(self, other: usize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<usize> for U128 {
    fn mul_assign(&mut self, other: usize) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i8> for U128 {
    type Output = U128;
    fn mul(self, other: i8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i8> for U128 {
    type Output = U128;
    fn mul(self, other: &'a i8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i8> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a i8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i8> for &'a U128 {
    type Output = U128;
    fn mul(self, other: i8) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i8> for U128 {
    fn mul_assign(&mut self, other: i8) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i16> for U128 {
    type Output = U128;
    fn mul(self, other: i16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i16> for U128 {
    type Output = U128;
    fn mul(self, other: &'a i16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i16> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a i16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i16> for &'a U128 {
    type Output = U128;
    fn mul(self, other: i16) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i16> for U128 {
    fn mul_assign(&mut self, other: i16) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i32> for U128 {
    type Output = U128;
    fn mul(self, other: i32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i32> for U128 {
    type Output = U128;
    fn mul(self, other: &'a i32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i32> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a i32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i32> for &'a U128 {
    type Output = U128;
    fn mul(self, other: i32) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i32> for U128 {
    fn mul_assign(&mut self, other: i32) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i64> for U128 {
    type Output = U128;
    fn mul(self, other: i64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i64> for U128 {
    type Output = U128;
    fn mul(self, other: &'a i64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i64> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a i64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i64> for &'a U128 {
    type Output = U128;
    fn mul(self, other: i64) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i64> for U128 {
    fn mul_assign(&mut self, other: i64) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<isize> for U128 {
    type Output = U128;
    fn mul(self, other: isize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a isize> for U128 {
    type Output = U128;
    fn mul(self, other: &'a isize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a isize> for &'a U128 {
    type Output = U128;
    fn mul(self, other: &'a isize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<isize> for &'a U128 {
    type Output = U128;
    fn mul(self, other: isize) -> U128 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<isize> for U128 {
    fn mul_assign(&mut self, other: isize) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl<T> ::uint::core_::ops::Div<T> for U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn div(self, other: T) -> U128 {
        let other: Self = other.into();
        self.div_mod(other).0
    }
}
impl<'a, T> ::uint::core_::ops::Div<T> for &'a U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn div(self, other: T) -> U128 {
        *self / other
    }
}
impl<T> ::uint::core_::ops::DivAssign<T> for U128
where
    T: Into<U128>,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}
impl<T> ::uint::core_::ops::Rem<T> for U128
where
    T: Into<U128> + Copy,
{
    type Output = U128;
    fn rem(self, other: T) -> U128 {
        let mut sub_copy = self;
        sub_copy %= other;
        sub_copy
    }
}
impl<'a, T> ::uint::core_::ops::Rem<T> for &'a U128
where
    T: Into<U128> + Copy,
{
    type Output = U128;
    fn rem(self, other: T) -> U128 {
        *self % other
    }
}
impl<T> ::uint::core_::ops::RemAssign<T> for U128
where
    T: Into<U128> + Copy,
{
    fn rem_assign(&mut self, other: T) {
        let other: Self = other.into();
        let rem = self.div_mod(other).1;
        *self = rem;
    }
}
impl ::uint::core_::ops::BitAnd<U128> for U128 {
    type Output = U128;
    #[inline]
    fn bitand(self, other: U128) -> U128 {
        let U128(ref arr1) = self;
        let U128(ref arr2) = other;
        let mut ret = [0u64; 2];
        for i in 0..2 {
            ret[i] = arr1[i] & arr2[i];
        }
        U128(ret)
    }
}
impl ::uint::core_::ops::BitAndAssign<U128> for U128 {
    fn bitand_assign(&mut self, rhs: U128) {
        *self = *self & rhs;
    }
}
impl ::uint::core_::ops::BitXor<U128> for U128 {
    type Output = U128;
    #[inline]
    fn bitxor(self, other: U128) -> U128 {
        let U128(ref arr1) = self;
        let U128(ref arr2) = other;
        let mut ret = [0u64; 2];
        for i in 0..2 {
            ret[i] = arr1[i] ^ arr2[i];
        }
        U128(ret)
    }
}
impl ::uint::core_::ops::BitXorAssign<U128> for U128 {
    fn bitxor_assign(&mut self, rhs: U128) {
        *self = *self ^ rhs;
    }
}
impl ::uint::core_::ops::BitOr<U128> for U128 {
    type Output = U128;
    #[inline]
    fn bitor(self, other: U128) -> U128 {
        let U128(ref arr1) = self;
        let U128(ref arr2) = other;
        let mut ret = [0u64; 2];
        for i in 0..2 {
            ret[i] = arr1[i] | arr2[i];
        }
        U128(ret)
    }
}
impl ::uint::core_::ops::BitOrAssign<U128> for U128 {
    fn bitor_assign(&mut self, rhs: U128) {
        *self = *self | rhs;
    }
}
impl ::uint::core_::ops::Not for U128 {
    type Output = U128;
    #[inline]
    fn not(self) -> U128 {
        let U128(ref arr) = self;
        let mut ret = [0u64; 2];
        for i in 0..2 {
            ret[i] = !arr[i];
        }
        U128(ret)
    }
}
impl<T> ::uint::core_::ops::Shl<T> for U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn shl(self, shift: T) -> U128 {
        let shift = shift.into().as_usize();
        let U128(ref original) = self;
        let mut ret = [0u64; 2];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..2 {
            ret[i] = original[i - word_shift] << bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..2 {
                ret[i] += original[i - 1 - word_shift] >> (64 - bit_shift);
            }
        }
        U128(ret)
    }
}
impl<'a, T> ::uint::core_::ops::Shl<T> for &'a U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn shl(self, shift: T) -> U128 {
        *self << shift
    }
}
impl<T> ::uint::core_::ops::ShlAssign<T> for U128
where
    T: Into<U128>,
{
    fn shl_assign(&mut self, shift: T) {
        *self = *self << shift;
    }
}
impl<T> ::uint::core_::ops::Shr<T> for U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn shr(self, shift: T) -> U128 {
        let shift = shift.into().as_usize();
        let U128(ref original) = self;
        let mut ret = [0u64; 2];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..2 {
            ret[i - word_shift] = original[i] >> bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..2 {
                ret[i - word_shift - 1] += original[i] << (64 - bit_shift);
            }
        }
        U128(ret)
    }
}
impl<'a, T> ::uint::core_::ops::Shr<T> for &'a U128
where
    T: Into<U128>,
{
    type Output = U128;
    fn shr(self, shift: T) -> U128 {
        *self >> shift
    }
}
impl<T> ::uint::core_::ops::ShrAssign<T> for U128
where
    T: Into<U128>,
{
    fn shr_assign(&mut self, shift: T) {
        *self = *self >> shift;
    }
}
impl ::uint::core_::cmp::Ord for U128 {
    fn cmp(&self, other: &U128) -> ::uint::core_::cmp::Ordering {
        self.as_ref().iter().rev().cmp(other.as_ref().iter().rev())
    }
}
impl ::uint::core_::cmp::PartialOrd for U128 {
    fn partial_cmp(&self, other: &U128) -> Option<::uint::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::uint::core_::fmt::Debug for U128 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        ::uint::core_::fmt::Display::fmt(self, f)
    }
}
impl ::uint::core_::fmt::Display for U128 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        if self.is_zero() {
            return f.write_fmt(format_args!("0"));
        }
        let mut buf = [0_u8; 2 * 20];
        let mut i = buf.len() - 1;
        let mut current = *self;
        let ten = U128::from(10);
        loop {
            let digit = (current % ten).low_u64() as u8;
            buf[i] = digit + b'0';
            current /= ten;
            if current.is_zero() {
                break;
            }
            i -= 1;
        }
        let s = unsafe { ::uint::core_::str::from_utf8_unchecked(&buf[i..]) };
        f.pad_integral(true, "", s)
    }
}
impl ::uint::core_::fmt::LowerHex for U128 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        self.fmt_hex(f, true)
    }
}
impl ::uint::core_::fmt::UpperHex for U128 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        self.fmt_hex(f, false)
    }
}
impl ::uint::core_::str::FromStr for U128 {
    type Err = ::uint::FromHexError;
    fn from_str(value: &str) -> ::uint::core_::result::Result<U128, Self::Err> {
        let value = value.strip_prefix("0x").unwrap_or(value);
        const BYTES_LEN: usize = 2 * 8;
        const MAX_ENCODED_LEN: usize = BYTES_LEN * 2;
        let mut bytes = [0_u8; BYTES_LEN];
        let encoded = value.as_bytes();
        if encoded.len() > MAX_ENCODED_LEN {
            return Err(::uint::hex::FromHexError::InvalidStringLength.into());
        }
        if encoded.len() % 2 == 0 {
            let out = &mut bytes[BYTES_LEN - encoded.len() / 2..];
            ::uint::hex::decode_to_slice(encoded, out).map_err(Self::Err::from)?;
        } else {
            let mut s = [b'0'; MAX_ENCODED_LEN];
            s[MAX_ENCODED_LEN - encoded.len()..].copy_from_slice(encoded);
            let encoded = &s[MAX_ENCODED_LEN - encoded.len() - 1..];
            let out = &mut bytes[BYTES_LEN - encoded.len() / 2..];
            ::uint::hex::decode_to_slice(encoded, out).map_err(Self::Err::from)?;
        }
        let bytes_ref: &[u8] = &bytes;
        Ok(From::from(bytes_ref))
    }
}
impl ::uint::core_::convert::From<&'static str> for U128 {
    fn from(s: &'static str) -> Self {
        s.parse().unwrap()
    }
}
impl ::uint::core_::convert::From<u128> for U128 {
    fn from(value: u128) -> U128 {
        let mut ret = [0; 2];
        ret[0] = value as u64;
        ret[1] = (value >> 64) as u64;
        U128(ret)
    }
}
impl ::uint::core_::convert::From<i128> for U128 {
    fn from(value: i128) -> U128 {
        match value >= 0 {
            true => From::from(value as u128),
            false => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Unsigned integer can\'t be created from negative value",
                        ),
                    );
                };
            }
        }
    }
}
impl U128 {
    /// Low 2 words (u128)
    #[inline]
    pub const fn low_u128(&self) -> u128 {
        let &U128(ref arr) = self;
        ((arr[1] as u128) << 64) + arr[0] as u128
    }
    /// Conversion to u128 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than 2^128.
    #[inline]
    pub fn as_u128(&self) -> u128 {
        let &U128(ref arr) = self;
        for i in 2..2 {
            if arr[i] != 0 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Integer overflow when casting to u128"),
                    );
                }
            }
        }
        self.low_u128()
    }
}
impl ::uint::core_::convert::TryFrom<U128> for u128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<u128, &'static str> {
        let U128(arr) = u;
        for i in 2..2 {
            if arr[i] != 0 {
                return Err("integer overflow when casting to u128");
            }
        }
        Ok(((arr[1] as u128) << 64) + arr[0] as u128)
    }
}
impl ::uint::core_::convert::TryFrom<U128> for i128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U128) -> ::uint::core_::result::Result<i128, &'static str> {
        let err_str = "integer overflow when casting to i128";
        let i = u128::try_from(u).map_err(|_| err_str)?;
        if i > i128::max_value() as u128 { Err(err_str) } else { Ok(i as i128) }
    }
}
/// Little-endian large integer type
#[repr(C)]
/// 256-bit unsigned integer.
pub struct U256(pub [u64; 4]);
#[automatically_derived]
impl ::core::marker::Copy for U256 {}
#[automatically_derived]
impl ::core::clone::Clone for U256 {
    #[inline]
    fn clone(&self) -> U256 {
        let _: ::core::clone::AssertParamIsClone<[u64; 4]>;
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for U256 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u64; 4]>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for U256 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for U256 {
    #[inline]
    fn eq(&self, other: &U256) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::hash::Hash for U256 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
/// Get a reference to the underlying little-endian words.
impl AsRef<[u64]> for U256 {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}
impl<'a> From<&'a U256> for U256 {
    fn from(x: &'a U256) -> U256 {
        *x
    }
}
impl U256 {
    const WORD_BITS: usize = 64;
    /// Maximum value.
    pub const MAX: U256 = U256([u64::max_value(); 4]);
    /// Converts a string slice in a given base to an integer. Only supports radixes of 10
    /// and 16.
    pub fn from_str_radix(
        txt: &str,
        radix: u32,
    ) -> Result<Self, ::uint::FromStrRadixErr> {
        let parsed = match radix {
            10 => Self::from_dec_str(txt)?,
            16 => core::str::FromStr::from_str(txt)?,
            _ => return Err(::uint::FromStrRadixErr::unsupported()),
        };
        Ok(parsed)
    }
    /// Convert from a decimal string.
    pub fn from_dec_str(
        value: &str,
    ) -> ::uint::core_::result::Result<Self, ::uint::FromDecStrErr> {
        let mut res = Self::default();
        for b in value.bytes().map(|b| b.wrapping_sub(b'0')) {
            if b > 9 {
                return Err(::uint::FromDecStrErr::InvalidCharacter);
            }
            let (r, overflow) = res.overflowing_mul_u64(10);
            if overflow > 0 {
                return Err(::uint::FromDecStrErr::InvalidLength);
            }
            let (r, overflow) = r.overflowing_add(b.into());
            if overflow {
                return Err(::uint::FromDecStrErr::InvalidLength);
            }
            res = r;
        }
        Ok(res)
    }
    /// Conversion to u32
    #[inline]
    pub const fn low_u32(&self) -> u32 {
        let &U256(ref arr) = self;
        arr[0] as u32
    }
    /// Low word (u64)
    #[inline]
    pub const fn low_u64(&self) -> u64 {
        let &U256(ref arr) = self;
        arr[0]
    }
    /// Conversion to u32 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than 2^32.
    #[inline]
    pub fn as_u32(&self) -> u32 {
        let &U256(ref arr) = self;
        if !self.fits_word() || arr[0] > u32::max_value() as u64 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to u32"),
                );
            }
        }
        self.as_u64() as u32
    }
    /// Conversion to u64 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than u64::max_value().
    #[inline]
    pub fn as_u64(&self) -> u64 {
        let &U256(ref arr) = self;
        if !self.fits_word() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to u64"),
                );
            }
        }
        arr[0]
    }
    /// Conversion to usize with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than usize::max_value().
    #[inline]
    pub fn as_usize(&self) -> usize {
        let &U256(ref arr) = self;
        if !self.fits_word() || arr[0] > usize::max_value() as u64 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to usize"),
                );
            }
        }
        arr[0] as usize
    }
    /// Whether this is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        let &U256(ref arr) = self;
        let mut i = 0;
        while i < 4 {
            if arr[i] != 0 {
                return false;
            } else {
                i += 1;
            }
        }
        return true;
    }
    #[inline]
    fn fits_word(&self) -> bool {
        let &U256(ref arr) = self;
        for i in 1..4 {
            if arr[i] != 0 {
                return false;
            }
        }
        return true;
    }
    /// Return the least number of bits needed to represent the number
    #[inline]
    pub fn bits(&self) -> usize {
        let &U256(ref arr) = self;
        for i in 1..4 {
            if arr[4 - i] > 0 {
                return (0x40 * (4 - i + 1)) - arr[4 - i].leading_zeros() as usize;
            }
        }
        0x40 - arr[0].leading_zeros() as usize
    }
    /// Return if specific bit is set.
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the bit width of the number.
    #[inline]
    pub const fn bit(&self, index: usize) -> bool {
        let &U256(ref arr) = self;
        arr[index / 64] & (1 << (index % 64)) != 0
    }
    /// Returns the number of leading zeros in the binary representation of self.
    pub fn leading_zeros(&self) -> u32 {
        let mut r = 0;
        for i in 0..4 {
            let w = self.0[4 - i - 1];
            if w == 0 {
                r += 64;
            } else {
                r += w.leading_zeros();
                break;
            }
        }
        r
    }
    /// Returns the number of trailing zeros in the binary representation of self.
    pub fn trailing_zeros(&self) -> u32 {
        let mut r = 0;
        for i in 0..4 {
            let w = self.0[i];
            if w == 0 {
                r += 64;
            } else {
                r += w.trailing_zeros();
                break;
            }
        }
        r
    }
    /// Return specific byte. Byte 0 is the least significant value (ie~ little endian).
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the byte width of the number.
    #[inline]
    pub const fn byte(&self, index: usize) -> u8 {
        let &U256(ref arr) = self;
        (arr[index / 8] >> (((index % 8)) * 8)) as u8
    }
    /// Write to the slice in big-endian format.
    #[inline]
    pub fn to_big_endian(&self, bytes: &mut [u8]) {
        use ::uint::byteorder::{ByteOrder, BigEndian};
        if true {
            if !(4 * 8 == bytes.len()) {
                ::core::panicking::panic("assertion failed: 4 * 8 == bytes.len()")
            }
        }
        for i in 0..4 {
            BigEndian::write_u64(&mut bytes[8 * i..], self.0[4 - i - 1]);
        }
    }
    /// Write to the slice in little-endian format.
    #[inline]
    pub fn to_little_endian(&self, bytes: &mut [u8]) {
        use ::uint::byteorder::{ByteOrder, LittleEndian};
        if true {
            if !(4 * 8 == bytes.len()) {
                ::core::panicking::panic("assertion failed: 4 * 8 == bytes.len()")
            }
        }
        for i in 0..4 {
            LittleEndian::write_u64(&mut bytes[8 * i..], self.0[i]);
        }
    }
    /// Create `10**n` as this type.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type.
    #[inline]
    pub fn exp10(n: usize) -> Self {
        match n {
            0 => Self::from(1u64),
            _ => Self::exp10(n - 1) * 10u32,
        }
    }
    /// Zero (additive identity) of this type.
    #[inline]
    pub const fn zero() -> Self {
        Self([0; 4])
    }
    /// One (multiplicative identity) of this type.
    #[inline]
    pub const fn one() -> Self {
        let mut words = [0; 4];
        words[0] = 1u64;
        Self(words)
    }
    /// The maximum value which can be inhabited by this type.
    #[inline]
    pub const fn max_value() -> Self {
        Self::MAX
    }
    fn full_shl(self, shift: u32) -> [u64; 4 + 1] {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic(
                    "assertion failed: shift < Self::WORD_BITS as u32",
                )
            }
        }
        let mut u = [0u64; 4 + 1];
        let u_lo = self.0[0] << shift;
        let u_hi = self >> (Self::WORD_BITS as u32 - shift);
        u[0] = u_lo;
        u[1..].copy_from_slice(&u_hi.0[..]);
        u
    }
    fn full_shr(u: [u64; 4 + 1], shift: u32) -> Self {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic(
                    "assertion failed: shift < Self::WORD_BITS as u32",
                )
            }
        }
        let mut res = Self::zero();
        for i in 0..4 {
            res.0[i] = u[i] >> shift;
        }
        if shift > 0 {
            for i in 1..=4 {
                res.0[i - 1] |= u[i] << (Self::WORD_BITS as u32 - shift);
            }
        }
        res
    }
    fn full_mul_u64(self, by: u64) -> [u64; 4 + 1] {
        let (prod, carry) = self.overflowing_mul_u64(by);
        let mut res = [0u64; 4 + 1];
        res[..4].copy_from_slice(&prod.0[..]);
        res[4] = carry;
        res
    }
    fn div_mod_small(mut self, other: u64) -> (Self, Self) {
        let mut rem = 0u64;
        self.0
            .iter_mut()
            .rev()
            .for_each(|d| {
                let (q, r) = Self::div_mod_word(rem, *d, other);
                *d = q;
                rem = r;
            });
        (self, rem.into())
    }
    fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
        if true {
            if !(self.bits() >= v.bits() && !v.fits_word()) {
                ::core::panicking::panic(
                    "assertion failed: self.bits() >= v.bits() && !v.fits_word()",
                )
            }
        }
        if true {
            if !(n + m <= 4) {
                ::core::panicking::panic("assertion failed: n + m <= 4")
            }
        }
        let shift = v.0[n - 1].leading_zeros();
        v <<= shift;
        let mut u = self.full_shl(shift);
        let mut q = Self::zero();
        let v_n_1 = v.0[n - 1];
        let v_n_2 = v.0[n - 2];
        for j in (0..=m).rev() {
            let u_jn = u[j + n];
            let mut q_hat = if u_jn < v_n_1 {
                let (mut q_hat, mut r_hat) = Self::div_mod_word(
                    u_jn,
                    u[j + n - 1],
                    v_n_1,
                );
                loop {
                    let (hi, lo) = Self::split_u128(
                        u128::from(q_hat) * u128::from(v_n_2),
                    );
                    if (hi, lo) <= (r_hat, u[j + n - 2]) {
                        break;
                    }
                    q_hat -= 1;
                    let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                    r_hat = new_r_hat;
                    if overflow {
                        break;
                    }
                }
                q_hat
            } else {
                u64::max_value()
            };
            let q_hat_v = v.full_mul_u64(q_hat);
            let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);
            if c {
                q_hat -= 1;
                let c = Self::add_slice(&mut u[j..], &v.0[..n]);
                u[j + n] = u[j + n].wrapping_add(u64::from(c));
            }
            q.0[j] = q_hat;
        }
        let remainder = Self::full_shr(u, shift);
        (q, remainder)
    }
    fn words(bits: usize) -> usize {
        if true {
            if !(bits > 0) {
                ::core::panicking::panic("assertion failed: bits > 0")
            }
        }
        1 + (bits - 1) / Self::WORD_BITS
    }
    /// Returns a pair `(self / other, self % other)`.
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero.
    pub fn div_mod(mut self, mut other: Self) -> (Self, Self) {
        use ::uint::core_::cmp::Ordering;
        let my_bits = self.bits();
        let your_bits = other.bits();
        if !(your_bits != 0) {
            {
                ::core::panicking::panic_fmt(format_args!("division by zero"));
            }
        }
        if my_bits < your_bits {
            return (Self::zero(), self);
        }
        if your_bits <= Self::WORD_BITS {
            return self.div_mod_small(other.low_u64());
        }
        let (n, m) = {
            let my_words = Self::words(my_bits);
            let your_words = Self::words(your_bits);
            (your_words, my_words - your_words)
        };
        self.div_mod_knuth(other, n, m)
    }
    /// Compute the highest `n` such that `n * n <= self`.
    pub fn integer_sqrt(&self) -> Self {
        let one = Self::one();
        if self <= &one {
            return *self;
        }
        let shift: u32 = (self.bits() as u32 + 1) / 2;
        let mut x_prev = one << shift;
        loop {
            let x = (x_prev + self / x_prev) >> 1;
            if x >= x_prev {
                return x_prev;
            }
            x_prev = x;
        }
    }
    /// Fast exponentiation by squaring
    /// https://en.wikipedia.org/wiki/Exponentiation_by_squaring
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type.
    pub fn pow(self, expon: Self) -> Self {
        if expon.is_zero() {
            return Self::one();
        }
        let is_even = |x: &Self| x.low_u64() & 1 == 0;
        let u_one = Self::one();
        let mut y = u_one;
        let mut n = expon;
        let mut x = self;
        while n > u_one {
            if is_even(&n) {
                x = x * x;
                n >>= 1usize;
            } else {
                y = x * y;
                x = x * x;
                n.0[4 - 1] &= (!0u64) >> 1;
                n >>= 1usize;
            }
        }
        x * y
    }
    /// Fast exponentiation by squaring. Returns result and overflow flag.
    pub fn overflowing_pow(self, expon: Self) -> (Self, bool) {
        if expon.is_zero() {
            return (Self::one(), false);
        }
        let is_even = |x: &Self| x.low_u64() & 1 == 0;
        let u_one = Self::one();
        let mut y = u_one;
        let mut n = expon;
        let mut x = self;
        let mut overflow = false;
        while n > u_one {
            if is_even(&n) {
                x = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(x);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                n >>= 1usize;
            } else {
                y = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(y);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                x = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(x);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                n = (n - u_one) >> 1usize;
            }
        }
        let res = {
            let (overflow_x, overflow_overflow) = x.overflowing_mul(y);
            overflow |= overflow_overflow;
            overflow_x
        };
        (res, overflow)
    }
    /// Checked exponentiation. Returns `None` if overflow occurred.
    pub fn checked_pow(self, expon: U256) -> Option<U256> {
        match self.overflowing_pow(expon) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Addition which overflows and returns a flag if it does.
    #[inline(always)]
    pub fn overflowing_add(self, other: U256) -> (U256, bool) {
        {
            use ::uint::core_ as core;
            let U256(ref me) = self;
            let U256(ref you) = other;
            let mut ret = [0u64; 4];
            let mut carry = 0u64;
            #[allow(unknown_lints, eq_op)]
            const _: [(); 0
                - !{
                    const ASSERT: bool = core::isize::MAX as usize
                        / core::mem::size_of::<u64>() > 4;
                    ASSERT
                } as usize] = [];
            use ::uint::unroll;
            #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 2;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 3;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U256(ret), carry > 0)
        }
    }
    /// Addition which saturates at the maximum value (Self::MAX).
    pub fn saturating_add(self, other: U256) -> U256 {
        match self.overflowing_add(other) {
            (_, true) => U256::MAX,
            (val, false) => val,
        }
    }
    /// Checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(self, other: U256) -> Option<U256> {
        match self.overflowing_add(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Subtraction which underflows and returns a flag if it does.
    #[inline(always)]
    pub fn overflowing_sub(self, other: U256) -> (U256, bool) {
        {
            use ::uint::core_ as core;
            let U256(ref me) = self;
            let U256(ref you) = other;
            let mut ret = [0u64; 4];
            let mut carry = 0u64;
            #[allow(unknown_lints, eq_op)]
            const _: [(); 0
                - !{
                    const ASSERT: bool = core::isize::MAX as usize
                        / core::mem::size_of::<u64>() > 4;
                    ASSERT
                } as usize] = [];
            use ::uint::unroll;
            #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 2;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 3;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U256(ret), carry > 0)
        }
    }
    /// Subtraction which saturates at zero.
    pub fn saturating_sub(self, other: U256) -> U256 {
        match self.overflowing_sub(other) {
            (_, true) => U256::zero(),
            (val, false) => val,
        }
    }
    /// Checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(self, other: U256) -> Option<U256> {
        match self.overflowing_sub(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Computes the absolute difference between self and other.
    pub fn abs_diff(self, other: U256) -> U256 {
        if self > other {
            self.overflowing_sub(other).0
        } else {
            other.overflowing_sub(self).0
        }
    }
    /// Multiply with overflow, returning a flag if it does.
    #[inline(always)]
    pub fn overflowing_mul(self, other: U256) -> (U256, bool) {
        {
            let ret: [u64; 4 * 2] = {
                {
                    #![allow(unused_assignments)]
                    let U256(ref me) = self;
                    let U256(ref you) = other;
                    let mut ret = [0u64; 4 * 2];
                    use ::uint::unroll;
                    #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                    {
                        {
                            const i: usize = 0;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 1;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 2;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 3;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|_, _| true)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        };
                    }
                    ret
                }
            };
            let ret: [[u64; 4]; 2] = unsafe { ::uint::core_::mem::transmute(ret) };
            #[inline(always)]
            fn any_nonzero(arr: &[u64; 4]) -> bool {
                use ::uint::unroll;
                #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                {
                    {
                        const i: usize = 0;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 1;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 2;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 3;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    };
                }
                false
            }
            (U256(ret[0]), any_nonzero(&ret[1]))
        }
    }
    /// Multiplication which saturates at the maximum value..
    pub fn saturating_mul(self, other: U256) -> U256 {
        match self.overflowing_mul(other) {
            (_, true) => U256::MAX,
            (val, false) => val,
        }
    }
    /// Checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(self, other: U256) -> Option<U256> {
        match self.overflowing_mul(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Checked division. Returns `None` if `other == 0`.
    pub fn checked_div(self, other: U256) -> Option<U256> {
        if other.is_zero() { None } else { Some(self / other) }
    }
    /// Checked modulus. Returns `None` if `other == 0`.
    pub fn checked_rem(self, other: U256) -> Option<U256> {
        if other.is_zero() { None } else { Some(self % other) }
    }
    /// Negation with overflow.
    pub fn overflowing_neg(self) -> (U256, bool) {
        if self.is_zero() { (self, false) } else { (!self + 1, true) }
    }
    /// Checked negation. Returns `None` unless `self == 0`.
    pub fn checked_neg(self) -> Option<U256> {
        match self.overflowing_neg() {
            (_, true) => None,
            (zero, false) => Some(zero),
        }
    }
    #[inline(always)]
    fn div_mod_word(hi: u64, lo: u64, y: u64) -> (u64, u64) {
        if true {
            if !(hi < y) {
                ::core::panicking::panic("assertion failed: hi < y")
            }
        }
        let x = (u128::from(hi) << 64) + u128::from(lo);
        let y = u128::from(y);
        ((x / y) as u64, (x % y) as u64)
    }
    #[inline(always)]
    fn add_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_add)
    }
    #[inline(always)]
    fn sub_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_sub)
    }
    #[inline(always)]
    fn binop_slice(
        a: &mut [u64],
        b: &[u64],
        binop: impl Fn(u64, u64) -> (u64, bool) + Copy,
    ) -> bool {
        let mut c = false;
        a.iter_mut()
            .zip(b.iter())
            .for_each(|(x, y)| {
                let (res, carry) = Self::binop_carry(*x, *y, c, binop);
                *x = res;
                c = carry;
            });
        c
    }
    #[inline(always)]
    fn binop_carry(
        a: u64,
        b: u64,
        c: bool,
        binop: impl Fn(u64, u64) -> (u64, bool),
    ) -> (u64, bool) {
        let (res1, overflow1) = b.overflowing_add(u64::from(c));
        let (res2, overflow2) = binop(a, res1);
        (res2, overflow1 || overflow2)
    }
    #[inline(always)]
    const fn mul_u64(a: u64, b: u64, carry: u64) -> (u64, u64) {
        let (hi, lo) = Self::split_u128(a as u128 * b as u128 + carry as u128);
        (lo, hi)
    }
    #[inline(always)]
    const fn split(a: u64) -> (u64, u64) {
        (a >> 32, a & 0xFFFF_FFFF)
    }
    #[inline(always)]
    const fn split_u128(a: u128) -> (u64, u64) {
        ((a >> 64) as _, (a & 0xFFFFFFFFFFFFFFFF) as _)
    }
    /// Overflowing multiplication by u64.
    /// Returns the result and carry.
    fn overflowing_mul_u64(mut self, other: u64) -> (Self, u64) {
        let mut carry = 0u64;
        for d in self.0.iter_mut() {
            let (res, c) = Self::mul_u64(*d, other, carry);
            *d = res;
            carry = c;
        }
        (self, carry)
    }
    /// Converts from big endian representation bytes in memory.
    pub fn from_big_endian(slice: &[u8]) -> Self {
        use ::uint::byteorder::{ByteOrder, BigEndian};
        if !(4 * 8 >= slice.len()) {
            ::core::panicking::panic("assertion failed: 4 * 8 >= slice.len()")
        }
        let mut padded = [0u8; 4 * 8];
        padded[4 * 8 - slice.len()..4 * 8].copy_from_slice(&slice);
        let mut ret = [0; 4];
        for i in 0..4 {
            ret[4 - i - 1] = BigEndian::read_u64(&padded[8 * i..]);
        }
        U256(ret)
    }
    /// Converts from little endian representation bytes in memory.
    pub fn from_little_endian(slice: &[u8]) -> Self {
        use ::uint::byteorder::{ByteOrder, LittleEndian};
        if !(4 * 8 >= slice.len()) {
            ::core::panicking::panic("assertion failed: 4 * 8 >= slice.len()")
        }
        let mut padded = [0u8; 4 * 8];
        padded[0..slice.len()].copy_from_slice(&slice);
        let mut ret = [0; 4];
        for i in 0..4 {
            ret[i] = LittleEndian::read_u64(&padded[8 * i..]);
        }
        U256(ret)
    }
    fn fmt_hex(
        &self,
        f: &mut ::uint::core_::fmt::Formatter,
        is_lower: bool,
    ) -> ::uint::core_::fmt::Result {
        let &U256(ref data) = self;
        if self.is_zero() {
            return f.pad_integral(true, "0x", "0");
        }
        let mut latch = false;
        let mut buf = [0_u8; 4 * 16];
        let mut i = 0;
        for ch in data.iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64))
                    >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0;
                }
                if latch {
                    let nibble = match nibble {
                        0..=9 => nibble as u8 + b'0',
                        _ if is_lower => nibble as u8 - 10 + b'a',
                        _ => nibble as u8 - 10 + b'A',
                    };
                    buf[i] = nibble;
                    i += 1;
                }
            }
        }
        let s = unsafe { ::uint::core_::str::from_utf8_unchecked(&buf[0..i]) };
        f.pad_integral(true, "0x", s)
    }
}
impl ::uint::core_::convert::From<U256> for [u8; 4 * 8] {
    fn from(number: U256) -> Self {
        let mut arr = [0u8; 4 * 8];
        number.to_big_endian(&mut arr);
        arr
    }
}
impl ::uint::core_::convert::From<[u8; 4 * 8]> for U256 {
    fn from(bytes: [u8; 4 * 8]) -> Self {
        Self::from(&bytes)
    }
}
impl<'a> ::uint::core_::convert::From<&'a [u8; 4 * 8]> for U256 {
    fn from(bytes: &[u8; 4 * 8]) -> Self {
        Self::from(&bytes[..])
    }
}
impl ::uint::core_::default::Default for U256 {
    fn default() -> Self {
        U256::zero()
    }
}
impl ::uint::core_::convert::From<u64> for U256 {
    fn from(value: u64) -> U256 {
        let mut ret = [0; 4];
        ret[0] = value;
        U256(ret)
    }
}
impl From<u8> for U256 {
    fn from(value: u8) -> U256 {
        From::from(value as u64)
    }
}
impl From<u16> for U256 {
    fn from(value: u16) -> U256 {
        From::from(value as u64)
    }
}
impl From<u32> for U256 {
    fn from(value: u32) -> U256 {
        From::from(value as u64)
    }
}
impl From<usize> for U256 {
    fn from(value: usize) -> U256 {
        From::from(value as u64)
    }
}
impl ::uint::core_::convert::From<i64> for U256 {
    fn from(value: i64) -> U256 {
        match value >= 0 {
            true => From::from(value as u64),
            false => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Unsigned integer can\'t be created from negative value",
                        ),
                    );
                };
            }
        }
    }
}
impl From<i8> for U256 {
    fn from(value: i8) -> U256 {
        From::from(value as i64)
    }
}
impl From<i16> for U256 {
    fn from(value: i16) -> U256 {
        From::from(value as i64)
    }
}
impl From<i32> for U256 {
    fn from(value: i32) -> U256 {
        From::from(value as i64)
    }
}
impl From<isize> for U256 {
    fn from(value: isize) -> U256 {
        From::from(value as i64)
    }
}
impl<'a> ::uint::core_::convert::From<&'a [u8]> for U256 {
    fn from(bytes: &[u8]) -> U256 {
        Self::from_big_endian(bytes)
    }
}
impl ::uint::core_::convert::TryFrom<U256> for u8 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<u8, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <u8>::max_value() as u64 {
            Err("integer overflow when casting to u8")
        } else {
            Ok(arr[0] as u8)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for u16 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<u16, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <u16>::max_value() as u64 {
            Err("integer overflow when casting to u16")
        } else {
            Ok(arr[0] as u16)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for u32 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<u32, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <u32>::max_value() as u64 {
            Err("integer overflow when casting to u32")
        } else {
            Ok(arr[0] as u32)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for usize {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<usize, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <usize>::max_value() as u64 {
            Err("integer overflow when casting to usize")
        } else {
            Ok(arr[0] as usize)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for u64 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<u64, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <u64>::max_value() as u64 {
            Err("integer overflow when casting to u64")
        } else {
            Ok(arr[0] as u64)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for i8 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<i8, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <i8>::max_value() as u64 {
            Err("integer overflow when casting to i8")
        } else {
            Ok(arr[0] as i8)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for i16 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<i16, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <i16>::max_value() as u64 {
            Err("integer overflow when casting to i16")
        } else {
            Ok(arr[0] as i16)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for i32 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<i32, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <i32>::max_value() as u64 {
            Err("integer overflow when casting to i32")
        } else {
            Ok(arr[0] as i32)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for isize {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<isize, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <isize>::max_value() as u64 {
            Err("integer overflow when casting to isize")
        } else {
            Ok(arr[0] as isize)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U256> for i64 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<i64, &'static str> {
        let U256(arr) = u;
        if !u.fits_word() || arr[0] > <i64>::max_value() as u64 {
            Err("integer overflow when casting to i64")
        } else {
            Ok(arr[0] as i64)
        }
    }
}
impl<T> ::uint::core_::ops::Add<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn add(self, other: T) -> U256 {
        let (result, overflow) = self.overflowing_add(other.into());
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a, T> ::uint::core_::ops::Add<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn add(self, other: T) -> U256 {
        *self + other
    }
}
impl ::uint::core_::ops::AddAssign<U256> for U256 {
    fn add_assign(&mut self, other: U256) {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        *self = result;
    }
}
impl<T> ::uint::core_::ops::Sub<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    #[inline]
    fn sub(self, other: T) -> U256 {
        let (result, overflow) = self.overflowing_sub(other.into());
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a, T> ::uint::core_::ops::Sub<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn sub(self, other: T) -> U256 {
        *self - other
    }
}
impl ::uint::core_::ops::SubAssign<U256> for U256 {
    fn sub_assign(&mut self, other: U256) {
        let (result, overflow) = self.overflowing_sub(other);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<U256> for U256 {
    type Output = U256;
    fn mul(self, other: U256) -> U256 {
        let bignum: U256 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a U256> for U256 {
    type Output = U256;
    fn mul(self, other: &'a U256) -> U256 {
        let bignum: U256 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a U256> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a U256) -> U256 {
        let bignum: U256 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<U256> for &'a U256 {
    type Output = U256;
    fn mul(self, other: U256) -> U256 {
        let bignum: U256 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<U256> for U256 {
    fn mul_assign(&mut self, other: U256) {
        let result = *self * other;
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u8> for U256 {
    type Output = U256;
    fn mul(self, other: u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u8> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u8> for U256 {
    fn mul_assign(&mut self, other: u8) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u16> for U256 {
    type Output = U256;
    fn mul(self, other: u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u16> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u16> for U256 {
    fn mul_assign(&mut self, other: u16) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u32> for U256 {
    type Output = U256;
    fn mul(self, other: u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u32> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u32> for U256 {
    fn mul_assign(&mut self, other: u32) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u64> for U256 {
    type Output = U256;
    fn mul(self, other: u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u64> for U256 {
    type Output = U256;
    fn mul(self, other: &'a u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: u64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u64> for U256 {
    fn mul_assign(&mut self, other: u64) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<usize> for U256 {
    type Output = U256;
    fn mul(self, other: usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a usize> for U256 {
    type Output = U256;
    fn mul(self, other: &'a usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a usize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<usize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: usize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<usize> for U256 {
    fn mul_assign(&mut self, other: usize) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i8> for U256 {
    type Output = U256;
    fn mul(self, other: i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i8> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i8> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i8) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i8> for U256 {
    fn mul_assign(&mut self, other: i8) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i16> for U256 {
    type Output = U256;
    fn mul(self, other: i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i16> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i16> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i16) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i16> for U256 {
    fn mul_assign(&mut self, other: i16) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i32> for U256 {
    type Output = U256;
    fn mul(self, other: i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i32> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i32> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i32) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i32> for U256 {
    fn mul_assign(&mut self, other: i32) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i64> for U256 {
    type Output = U256;
    fn mul(self, other: i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i64> for U256 {
    type Output = U256;
    fn mul(self, other: &'a i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i64> for &'a U256 {
    type Output = U256;
    fn mul(self, other: i64) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i64> for U256 {
    fn mul_assign(&mut self, other: i64) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<isize> for U256 {
    type Output = U256;
    fn mul(self, other: isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a isize> for U256 {
    type Output = U256;
    fn mul(self, other: &'a isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a isize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: &'a isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<isize> for &'a U256 {
    type Output = U256;
    fn mul(self, other: isize) -> U256 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<isize> for U256 {
    fn mul_assign(&mut self, other: isize) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl<T> ::uint::core_::ops::Div<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn div(self, other: T) -> U256 {
        let other: Self = other.into();
        self.div_mod(other).0
    }
}
impl<'a, T> ::uint::core_::ops::Div<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn div(self, other: T) -> U256 {
        *self / other
    }
}
impl<T> ::uint::core_::ops::DivAssign<T> for U256
where
    T: Into<U256>,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}
impl<T> ::uint::core_::ops::Rem<T> for U256
where
    T: Into<U256> + Copy,
{
    type Output = U256;
    fn rem(self, other: T) -> U256 {
        let mut sub_copy = self;
        sub_copy %= other;
        sub_copy
    }
}
impl<'a, T> ::uint::core_::ops::Rem<T> for &'a U256
where
    T: Into<U256> + Copy,
{
    type Output = U256;
    fn rem(self, other: T) -> U256 {
        *self % other
    }
}
impl<T> ::uint::core_::ops::RemAssign<T> for U256
where
    T: Into<U256> + Copy,
{
    fn rem_assign(&mut self, other: T) {
        let other: Self = other.into();
        let rem = self.div_mod(other).1;
        *self = rem;
    }
}
impl ::uint::core_::ops::BitAnd<U256> for U256 {
    type Output = U256;
    #[inline]
    fn bitand(self, other: U256) -> U256 {
        let U256(ref arr1) = self;
        let U256(ref arr2) = other;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = arr1[i] & arr2[i];
        }
        U256(ret)
    }
}
impl ::uint::core_::ops::BitAndAssign<U256> for U256 {
    fn bitand_assign(&mut self, rhs: U256) {
        *self = *self & rhs;
    }
}
impl ::uint::core_::ops::BitXor<U256> for U256 {
    type Output = U256;
    #[inline]
    fn bitxor(self, other: U256) -> U256 {
        let U256(ref arr1) = self;
        let U256(ref arr2) = other;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = arr1[i] ^ arr2[i];
        }
        U256(ret)
    }
}
impl ::uint::core_::ops::BitXorAssign<U256> for U256 {
    fn bitxor_assign(&mut self, rhs: U256) {
        *self = *self ^ rhs;
    }
}
impl ::uint::core_::ops::BitOr<U256> for U256 {
    type Output = U256;
    #[inline]
    fn bitor(self, other: U256) -> U256 {
        let U256(ref arr1) = self;
        let U256(ref arr2) = other;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = arr1[i] | arr2[i];
        }
        U256(ret)
    }
}
impl ::uint::core_::ops::BitOrAssign<U256> for U256 {
    fn bitor_assign(&mut self, rhs: U256) {
        *self = *self | rhs;
    }
}
impl ::uint::core_::ops::Not for U256 {
    type Output = U256;
    #[inline]
    fn not(self) -> U256 {
        let U256(ref arr) = self;
        let mut ret = [0u64; 4];
        for i in 0..4 {
            ret[i] = !arr[i];
        }
        U256(ret)
    }
}
impl<T> ::uint::core_::ops::Shl<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shl(self, shift: T) -> U256 {
        let shift = shift.into().as_usize();
        let U256(ref original) = self;
        let mut ret = [0u64; 4];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..4 {
            ret[i] = original[i - word_shift] << bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..4 {
                ret[i] += original[i - 1 - word_shift] >> (64 - bit_shift);
            }
        }
        U256(ret)
    }
}
impl<'a, T> ::uint::core_::ops::Shl<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shl(self, shift: T) -> U256 {
        *self << shift
    }
}
impl<T> ::uint::core_::ops::ShlAssign<T> for U256
where
    T: Into<U256>,
{
    fn shl_assign(&mut self, shift: T) {
        *self = *self << shift;
    }
}
impl<T> ::uint::core_::ops::Shr<T> for U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shr(self, shift: T) -> U256 {
        let shift = shift.into().as_usize();
        let U256(ref original) = self;
        let mut ret = [0u64; 4];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..4 {
            ret[i - word_shift] = original[i] >> bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..4 {
                ret[i - word_shift - 1] += original[i] << (64 - bit_shift);
            }
        }
        U256(ret)
    }
}
impl<'a, T> ::uint::core_::ops::Shr<T> for &'a U256
where
    T: Into<U256>,
{
    type Output = U256;
    fn shr(self, shift: T) -> U256 {
        *self >> shift
    }
}
impl<T> ::uint::core_::ops::ShrAssign<T> for U256
where
    T: Into<U256>,
{
    fn shr_assign(&mut self, shift: T) {
        *self = *self >> shift;
    }
}
impl ::uint::core_::cmp::Ord for U256 {
    fn cmp(&self, other: &U256) -> ::uint::core_::cmp::Ordering {
        self.as_ref().iter().rev().cmp(other.as_ref().iter().rev())
    }
}
impl ::uint::core_::cmp::PartialOrd for U256 {
    fn partial_cmp(&self, other: &U256) -> Option<::uint::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::uint::core_::fmt::Debug for U256 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        ::uint::core_::fmt::Display::fmt(self, f)
    }
}
impl ::uint::core_::fmt::Display for U256 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        if self.is_zero() {
            return f.write_fmt(format_args!("0"));
        }
        let mut buf = [0_u8; 4 * 20];
        let mut i = buf.len() - 1;
        let mut current = *self;
        let ten = U256::from(10);
        loop {
            let digit = (current % ten).low_u64() as u8;
            buf[i] = digit + b'0';
            current /= ten;
            if current.is_zero() {
                break;
            }
            i -= 1;
        }
        let s = unsafe { ::uint::core_::str::from_utf8_unchecked(&buf[i..]) };
        f.pad_integral(true, "", s)
    }
}
impl ::uint::core_::fmt::LowerHex for U256 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        self.fmt_hex(f, true)
    }
}
impl ::uint::core_::fmt::UpperHex for U256 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        self.fmt_hex(f, false)
    }
}
impl ::uint::core_::str::FromStr for U256 {
    type Err = ::uint::FromHexError;
    fn from_str(value: &str) -> ::uint::core_::result::Result<U256, Self::Err> {
        let value = value.strip_prefix("0x").unwrap_or(value);
        const BYTES_LEN: usize = 4 * 8;
        const MAX_ENCODED_LEN: usize = BYTES_LEN * 2;
        let mut bytes = [0_u8; BYTES_LEN];
        let encoded = value.as_bytes();
        if encoded.len() > MAX_ENCODED_LEN {
            return Err(::uint::hex::FromHexError::InvalidStringLength.into());
        }
        if encoded.len() % 2 == 0 {
            let out = &mut bytes[BYTES_LEN - encoded.len() / 2..];
            ::uint::hex::decode_to_slice(encoded, out).map_err(Self::Err::from)?;
        } else {
            let mut s = [b'0'; MAX_ENCODED_LEN];
            s[MAX_ENCODED_LEN - encoded.len()..].copy_from_slice(encoded);
            let encoded = &s[MAX_ENCODED_LEN - encoded.len() - 1..];
            let out = &mut bytes[BYTES_LEN - encoded.len() / 2..];
            ::uint::hex::decode_to_slice(encoded, out).map_err(Self::Err::from)?;
        }
        let bytes_ref: &[u8] = &bytes;
        Ok(From::from(bytes_ref))
    }
}
impl ::uint::core_::convert::From<&'static str> for U256 {
    fn from(s: &'static str) -> Self {
        s.parse().unwrap()
    }
}
impl ::uint::core_::convert::From<u128> for U256 {
    fn from(value: u128) -> U256 {
        let mut ret = [0; 4];
        ret[0] = value as u64;
        ret[1] = (value >> 64) as u64;
        U256(ret)
    }
}
impl ::uint::core_::convert::From<i128> for U256 {
    fn from(value: i128) -> U256 {
        match value >= 0 {
            true => From::from(value as u128),
            false => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Unsigned integer can\'t be created from negative value",
                        ),
                    );
                };
            }
        }
    }
}
impl U256 {
    /// Low 2 words (u128)
    #[inline]
    pub const fn low_u128(&self) -> u128 {
        let &U256(ref arr) = self;
        ((arr[1] as u128) << 64) + arr[0] as u128
    }
    /// Conversion to u128 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than 2^128.
    #[inline]
    pub fn as_u128(&self) -> u128 {
        let &U256(ref arr) = self;
        for i in 2..4 {
            if arr[i] != 0 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Integer overflow when casting to u128"),
                    );
                }
            }
        }
        self.low_u128()
    }
}
impl ::uint::core_::convert::TryFrom<U256> for u128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<u128, &'static str> {
        let U256(arr) = u;
        for i in 2..4 {
            if arr[i] != 0 {
                return Err("integer overflow when casting to u128");
            }
        }
        Ok(((arr[1] as u128) << 64) + arr[0] as u128)
    }
}
impl ::uint::core_::convert::TryFrom<U256> for i128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U256) -> ::uint::core_::result::Result<i128, &'static str> {
        let err_str = "integer overflow when casting to i128";
        let i = u128::try_from(u).map_err(|_| err_str)?;
        if i > i128::max_value() as u128 { Err(err_str) } else { Ok(i as i128) }
    }
}
/// Little-endian large integer type
#[repr(C)]
/// 512-bits unsigned integer.
pub struct U512(pub [u64; 8]);
#[automatically_derived]
impl ::core::marker::Copy for U512 {}
#[automatically_derived]
impl ::core::clone::Clone for U512 {
    #[inline]
    fn clone(&self) -> U512 {
        let _: ::core::clone::AssertParamIsClone<[u64; 8]>;
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for U512 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u64; 8]>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for U512 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for U512 {
    #[inline]
    fn eq(&self, other: &U512) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::hash::Hash for U512 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
/// Get a reference to the underlying little-endian words.
impl AsRef<[u64]> for U512 {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}
impl<'a> From<&'a U512> for U512 {
    fn from(x: &'a U512) -> U512 {
        *x
    }
}
impl U512 {
    const WORD_BITS: usize = 64;
    /// Maximum value.
    pub const MAX: U512 = U512([u64::max_value(); 8]);
    /// Converts a string slice in a given base to an integer. Only supports radixes of 10
    /// and 16.
    pub fn from_str_radix(
        txt: &str,
        radix: u32,
    ) -> Result<Self, ::uint::FromStrRadixErr> {
        let parsed = match radix {
            10 => Self::from_dec_str(txt)?,
            16 => core::str::FromStr::from_str(txt)?,
            _ => return Err(::uint::FromStrRadixErr::unsupported()),
        };
        Ok(parsed)
    }
    /// Convert from a decimal string.
    pub fn from_dec_str(
        value: &str,
    ) -> ::uint::core_::result::Result<Self, ::uint::FromDecStrErr> {
        let mut res = Self::default();
        for b in value.bytes().map(|b| b.wrapping_sub(b'0')) {
            if b > 9 {
                return Err(::uint::FromDecStrErr::InvalidCharacter);
            }
            let (r, overflow) = res.overflowing_mul_u64(10);
            if overflow > 0 {
                return Err(::uint::FromDecStrErr::InvalidLength);
            }
            let (r, overflow) = r.overflowing_add(b.into());
            if overflow {
                return Err(::uint::FromDecStrErr::InvalidLength);
            }
            res = r;
        }
        Ok(res)
    }
    /// Conversion to u32
    #[inline]
    pub const fn low_u32(&self) -> u32 {
        let &U512(ref arr) = self;
        arr[0] as u32
    }
    /// Low word (u64)
    #[inline]
    pub const fn low_u64(&self) -> u64 {
        let &U512(ref arr) = self;
        arr[0]
    }
    /// Conversion to u32 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than 2^32.
    #[inline]
    pub fn as_u32(&self) -> u32 {
        let &U512(ref arr) = self;
        if !self.fits_word() || arr[0] > u32::max_value() as u64 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to u32"),
                );
            }
        }
        self.as_u64() as u32
    }
    /// Conversion to u64 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than u64::max_value().
    #[inline]
    pub fn as_u64(&self) -> u64 {
        let &U512(ref arr) = self;
        if !self.fits_word() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to u64"),
                );
            }
        }
        arr[0]
    }
    /// Conversion to usize with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than usize::max_value().
    #[inline]
    pub fn as_usize(&self) -> usize {
        let &U512(ref arr) = self;
        if !self.fits_word() || arr[0] > usize::max_value() as u64 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("Integer overflow when casting to usize"),
                );
            }
        }
        arr[0] as usize
    }
    /// Whether this is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        let &U512(ref arr) = self;
        let mut i = 0;
        while i < 8 {
            if arr[i] != 0 {
                return false;
            } else {
                i += 1;
            }
        }
        return true;
    }
    #[inline]
    fn fits_word(&self) -> bool {
        let &U512(ref arr) = self;
        for i in 1..8 {
            if arr[i] != 0 {
                return false;
            }
        }
        return true;
    }
    /// Return the least number of bits needed to represent the number
    #[inline]
    pub fn bits(&self) -> usize {
        let &U512(ref arr) = self;
        for i in 1..8 {
            if arr[8 - i] > 0 {
                return (0x40 * (8 - i + 1)) - arr[8 - i].leading_zeros() as usize;
            }
        }
        0x40 - arr[0].leading_zeros() as usize
    }
    /// Return if specific bit is set.
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the bit width of the number.
    #[inline]
    pub const fn bit(&self, index: usize) -> bool {
        let &U512(ref arr) = self;
        arr[index / 64] & (1 << (index % 64)) != 0
    }
    /// Returns the number of leading zeros in the binary representation of self.
    pub fn leading_zeros(&self) -> u32 {
        let mut r = 0;
        for i in 0..8 {
            let w = self.0[8 - i - 1];
            if w == 0 {
                r += 64;
            } else {
                r += w.leading_zeros();
                break;
            }
        }
        r
    }
    /// Returns the number of trailing zeros in the binary representation of self.
    pub fn trailing_zeros(&self) -> u32 {
        let mut r = 0;
        for i in 0..8 {
            let w = self.0[i];
            if w == 0 {
                r += 64;
            } else {
                r += w.trailing_zeros();
                break;
            }
        }
        r
    }
    /// Return specific byte. Byte 0 is the least significant value (ie~ little endian).
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the byte width of the number.
    #[inline]
    pub const fn byte(&self, index: usize) -> u8 {
        let &U512(ref arr) = self;
        (arr[index / 8] >> (((index % 8)) * 8)) as u8
    }
    /// Write to the slice in big-endian format.
    #[inline]
    pub fn to_big_endian(&self, bytes: &mut [u8]) {
        use ::uint::byteorder::{ByteOrder, BigEndian};
        if true {
            if !(8 * 8 == bytes.len()) {
                ::core::panicking::panic("assertion failed: 8 * 8 == bytes.len()")
            }
        }
        for i in 0..8 {
            BigEndian::write_u64(&mut bytes[8 * i..], self.0[8 - i - 1]);
        }
    }
    /// Write to the slice in little-endian format.
    #[inline]
    pub fn to_little_endian(&self, bytes: &mut [u8]) {
        use ::uint::byteorder::{ByteOrder, LittleEndian};
        if true {
            if !(8 * 8 == bytes.len()) {
                ::core::panicking::panic("assertion failed: 8 * 8 == bytes.len()")
            }
        }
        for i in 0..8 {
            LittleEndian::write_u64(&mut bytes[8 * i..], self.0[i]);
        }
    }
    /// Create `10**n` as this type.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type.
    #[inline]
    pub fn exp10(n: usize) -> Self {
        match n {
            0 => Self::from(1u64),
            _ => Self::exp10(n - 1) * 10u32,
        }
    }
    /// Zero (additive identity) of this type.
    #[inline]
    pub const fn zero() -> Self {
        Self([0; 8])
    }
    /// One (multiplicative identity) of this type.
    #[inline]
    pub const fn one() -> Self {
        let mut words = [0; 8];
        words[0] = 1u64;
        Self(words)
    }
    /// The maximum value which can be inhabited by this type.
    #[inline]
    pub const fn max_value() -> Self {
        Self::MAX
    }
    fn full_shl(self, shift: u32) -> [u64; 8 + 1] {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic(
                    "assertion failed: shift < Self::WORD_BITS as u32",
                )
            }
        }
        let mut u = [0u64; 8 + 1];
        let u_lo = self.0[0] << shift;
        let u_hi = self >> (Self::WORD_BITS as u32 - shift);
        u[0] = u_lo;
        u[1..].copy_from_slice(&u_hi.0[..]);
        u
    }
    fn full_shr(u: [u64; 8 + 1], shift: u32) -> Self {
        if true {
            if !(shift < Self::WORD_BITS as u32) {
                ::core::panicking::panic(
                    "assertion failed: shift < Self::WORD_BITS as u32",
                )
            }
        }
        let mut res = Self::zero();
        for i in 0..8 {
            res.0[i] = u[i] >> shift;
        }
        if shift > 0 {
            for i in 1..=8 {
                res.0[i - 1] |= u[i] << (Self::WORD_BITS as u32 - shift);
            }
        }
        res
    }
    fn full_mul_u64(self, by: u64) -> [u64; 8 + 1] {
        let (prod, carry) = self.overflowing_mul_u64(by);
        let mut res = [0u64; 8 + 1];
        res[..8].copy_from_slice(&prod.0[..]);
        res[8] = carry;
        res
    }
    fn div_mod_small(mut self, other: u64) -> (Self, Self) {
        let mut rem = 0u64;
        self.0
            .iter_mut()
            .rev()
            .for_each(|d| {
                let (q, r) = Self::div_mod_word(rem, *d, other);
                *d = q;
                rem = r;
            });
        (self, rem.into())
    }
    fn div_mod_knuth(self, mut v: Self, n: usize, m: usize) -> (Self, Self) {
        if true {
            if !(self.bits() >= v.bits() && !v.fits_word()) {
                ::core::panicking::panic(
                    "assertion failed: self.bits() >= v.bits() && !v.fits_word()",
                )
            }
        }
        if true {
            if !(n + m <= 8) {
                ::core::panicking::panic("assertion failed: n + m <= 8")
            }
        }
        let shift = v.0[n - 1].leading_zeros();
        v <<= shift;
        let mut u = self.full_shl(shift);
        let mut q = Self::zero();
        let v_n_1 = v.0[n - 1];
        let v_n_2 = v.0[n - 2];
        for j in (0..=m).rev() {
            let u_jn = u[j + n];
            let mut q_hat = if u_jn < v_n_1 {
                let (mut q_hat, mut r_hat) = Self::div_mod_word(
                    u_jn,
                    u[j + n - 1],
                    v_n_1,
                );
                loop {
                    let (hi, lo) = Self::split_u128(
                        u128::from(q_hat) * u128::from(v_n_2),
                    );
                    if (hi, lo) <= (r_hat, u[j + n - 2]) {
                        break;
                    }
                    q_hat -= 1;
                    let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                    r_hat = new_r_hat;
                    if overflow {
                        break;
                    }
                }
                q_hat
            } else {
                u64::max_value()
            };
            let q_hat_v = v.full_mul_u64(q_hat);
            let c = Self::sub_slice(&mut u[j..], &q_hat_v[..n + 1]);
            if c {
                q_hat -= 1;
                let c = Self::add_slice(&mut u[j..], &v.0[..n]);
                u[j + n] = u[j + n].wrapping_add(u64::from(c));
            }
            q.0[j] = q_hat;
        }
        let remainder = Self::full_shr(u, shift);
        (q, remainder)
    }
    fn words(bits: usize) -> usize {
        if true {
            if !(bits > 0) {
                ::core::panicking::panic("assertion failed: bits > 0")
            }
        }
        1 + (bits - 1) / Self::WORD_BITS
    }
    /// Returns a pair `(self / other, self % other)`.
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero.
    pub fn div_mod(mut self, mut other: Self) -> (Self, Self) {
        use ::uint::core_::cmp::Ordering;
        let my_bits = self.bits();
        let your_bits = other.bits();
        if !(your_bits != 0) {
            {
                ::core::panicking::panic_fmt(format_args!("division by zero"));
            }
        }
        if my_bits < your_bits {
            return (Self::zero(), self);
        }
        if your_bits <= Self::WORD_BITS {
            return self.div_mod_small(other.low_u64());
        }
        let (n, m) = {
            let my_words = Self::words(my_bits);
            let your_words = Self::words(your_bits);
            (your_words, my_words - your_words)
        };
        self.div_mod_knuth(other, n, m)
    }
    /// Compute the highest `n` such that `n * n <= self`.
    pub fn integer_sqrt(&self) -> Self {
        let one = Self::one();
        if self <= &one {
            return *self;
        }
        let shift: u32 = (self.bits() as u32 + 1) / 2;
        let mut x_prev = one << shift;
        loop {
            let x = (x_prev + self / x_prev) >> 1;
            if x >= x_prev {
                return x_prev;
            }
            x_prev = x;
        }
    }
    /// Fast exponentiation by squaring
    /// https://en.wikipedia.org/wiki/Exponentiation_by_squaring
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type.
    pub fn pow(self, expon: Self) -> Self {
        if expon.is_zero() {
            return Self::one();
        }
        let is_even = |x: &Self| x.low_u64() & 1 == 0;
        let u_one = Self::one();
        let mut y = u_one;
        let mut n = expon;
        let mut x = self;
        while n > u_one {
            if is_even(&n) {
                x = x * x;
                n >>= 1usize;
            } else {
                y = x * y;
                x = x * x;
                n.0[8 - 1] &= (!0u64) >> 1;
                n >>= 1usize;
            }
        }
        x * y
    }
    /// Fast exponentiation by squaring. Returns result and overflow flag.
    pub fn overflowing_pow(self, expon: Self) -> (Self, bool) {
        if expon.is_zero() {
            return (Self::one(), false);
        }
        let is_even = |x: &Self| x.low_u64() & 1 == 0;
        let u_one = Self::one();
        let mut y = u_one;
        let mut n = expon;
        let mut x = self;
        let mut overflow = false;
        while n > u_one {
            if is_even(&n) {
                x = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(x);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                n >>= 1usize;
            } else {
                y = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(y);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                x = {
                    let (overflow_x, overflow_overflow) = x.overflowing_mul(x);
                    overflow |= overflow_overflow;
                    overflow_x
                };
                n = (n - u_one) >> 1usize;
            }
        }
        let res = {
            let (overflow_x, overflow_overflow) = x.overflowing_mul(y);
            overflow |= overflow_overflow;
            overflow_x
        };
        (res, overflow)
    }
    /// Checked exponentiation. Returns `None` if overflow occurred.
    pub fn checked_pow(self, expon: U512) -> Option<U512> {
        match self.overflowing_pow(expon) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Addition which overflows and returns a flag if it does.
    #[inline(always)]
    pub fn overflowing_add(self, other: U512) -> (U512, bool) {
        {
            use ::uint::core_ as core;
            let U512(ref me) = self;
            let U512(ref you) = other;
            let mut ret = [0u64; 8];
            let mut carry = 0u64;
            #[allow(unknown_lints, eq_op)]
            const _: [(); 0
                - !{
                    const ASSERT: bool = core::isize::MAX as usize
                        / core::mem::size_of::<u64>() > 8;
                    ASSERT
                } as usize] = [];
            use ::uint::unroll;
            #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 2;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 3;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 4;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 5;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 6;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 7;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_add)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_add)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_add)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U512(ret), carry > 0)
        }
    }
    /// Addition which saturates at the maximum value (Self::MAX).
    pub fn saturating_add(self, other: U512) -> U512 {
        match self.overflowing_add(other) {
            (_, true) => U512::MAX,
            (val, false) => val,
        }
    }
    /// Checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(self, other: U512) -> Option<U512> {
        match self.overflowing_add(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Subtraction which underflows and returns a flag if it does.
    #[inline(always)]
    pub fn overflowing_sub(self, other: U512) -> (U512, bool) {
        {
            use ::uint::core_ as core;
            let U512(ref me) = self;
            let U512(ref you) = other;
            let mut ret = [0u64; 8];
            let mut carry = 0u64;
            #[allow(unknown_lints, eq_op)]
            const _: [(); 0
                - !{
                    const ASSERT: bool = core::isize::MAX as usize
                        / core::mem::size_of::<u64>() > 8;
                    ASSERT
                } as usize] = [];
            use ::uint::unroll;
            #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
            {
                {
                    const i: usize = 0;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 1;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 2;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 3;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 4;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 5;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 6;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                }
                {
                    const i: usize = 0 + 7;
                    {
                        if i >= 0 {
                            use core::ptr;
                            if carry != 0 {
                                let (res1, overflow1) = (u64::overflowing_sub)(
                                    me[i],
                                    you[i],
                                );
                                let (res2, overflow2) = (u64::overflowing_sub)(res1, carry);
                                ret[i] = res2;
                                carry = (overflow1 as u8 + overflow2 as u8) as u64;
                            } else {
                                let (res, overflow) = (u64::overflowing_sub)(me[i], you[i]);
                                ret[i] = res;
                                carry = overflow as u64;
                            }
                        }
                    }
                };
            }
            (U512(ret), carry > 0)
        }
    }
    /// Subtraction which saturates at zero.
    pub fn saturating_sub(self, other: U512) -> U512 {
        match self.overflowing_sub(other) {
            (_, true) => U512::zero(),
            (val, false) => val,
        }
    }
    /// Checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(self, other: U512) -> Option<U512> {
        match self.overflowing_sub(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Computes the absolute difference between self and other.
    pub fn abs_diff(self, other: U512) -> U512 {
        if self > other {
            self.overflowing_sub(other).0
        } else {
            other.overflowing_sub(self).0
        }
    }
    /// Multiply with overflow, returning a flag if it does.
    #[inline(always)]
    pub fn overflowing_mul(self, other: U512) -> (U512, bool) {
        {
            let ret: [u64; 8 * 2] = {
                {
                    #![allow(unused_assignments)]
                    let U512(ref me) = self;
                    let U512(ref you) = other;
                    let mut ret = [0u64; 8 * 2];
                    use ::uint::unroll;
                    #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                    {
                        {
                            const i: usize = 0;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 1;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 2;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 3;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 4;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 5;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 6;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        {
                            const i: usize = 0 + 7;
                            {
                                if i >= 0 {
                                    let mut carry = 0u64;
                                    let b = you[i];
                                    #[allow(non_upper_case_globals)]
                                    #[allow(unused_comparisons)]
                                    {
                                        {
                                            const j: usize = 0;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 1;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 2;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 3;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 4;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 5;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 6;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            const j: usize = 0 + 7;
                                            {
                                                if j >= 0 {
                                                    if (|a, b| a != 0 || b != 0)(me[j], carry) {
                                                        let a = me[j];
                                                        let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                        let overflow = {
                                                            let existing_low = &mut ret[i + j];
                                                            let (low, o) = low.overflowing_add(*existing_low);
                                                            *existing_low = low;
                                                            o
                                                        };
                                                        carry = {
                                                            let existing_hi = &mut ret[i + j + 1];
                                                            let hi = hi + overflow as u64;
                                                            let (hi, o0) = hi.overflowing_add(carry);
                                                            let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                            *existing_hi = hi;
                                                            (o0 | o1) as u64
                                                        };
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                            }
                        };
                    }
                    ret
                }
            };
            let ret: [[u64; 8]; 2] = unsafe { ::uint::core_::mem::transmute(ret) };
            #[inline(always)]
            fn any_nonzero(arr: &[u64; 8]) -> bool {
                use ::uint::unroll;
                #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                {
                    {
                        const i: usize = 0;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 1;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 2;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 3;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 4;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 5;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 6;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 7;
                        {
                            if i >= 0 {
                                if arr[i] != 0 {
                                    return true;
                                }
                            }
                        }
                    };
                }
                false
            }
            (U512(ret[0]), any_nonzero(&ret[1]))
        }
    }
    /// Multiplication which saturates at the maximum value..
    pub fn saturating_mul(self, other: U512) -> U512 {
        match self.overflowing_mul(other) {
            (_, true) => U512::MAX,
            (val, false) => val,
        }
    }
    /// Checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(self, other: U512) -> Option<U512> {
        match self.overflowing_mul(other) {
            (_, true) => None,
            (val, _) => Some(val),
        }
    }
    /// Checked division. Returns `None` if `other == 0`.
    pub fn checked_div(self, other: U512) -> Option<U512> {
        if other.is_zero() { None } else { Some(self / other) }
    }
    /// Checked modulus. Returns `None` if `other == 0`.
    pub fn checked_rem(self, other: U512) -> Option<U512> {
        if other.is_zero() { None } else { Some(self % other) }
    }
    /// Negation with overflow.
    pub fn overflowing_neg(self) -> (U512, bool) {
        if self.is_zero() { (self, false) } else { (!self + 1, true) }
    }
    /// Checked negation. Returns `None` unless `self == 0`.
    pub fn checked_neg(self) -> Option<U512> {
        match self.overflowing_neg() {
            (_, true) => None,
            (zero, false) => Some(zero),
        }
    }
    #[inline(always)]
    fn div_mod_word(hi: u64, lo: u64, y: u64) -> (u64, u64) {
        if true {
            if !(hi < y) {
                ::core::panicking::panic("assertion failed: hi < y")
            }
        }
        let x = (u128::from(hi) << 64) + u128::from(lo);
        let y = u128::from(y);
        ((x / y) as u64, (x % y) as u64)
    }
    #[inline(always)]
    fn add_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_add)
    }
    #[inline(always)]
    fn sub_slice(a: &mut [u64], b: &[u64]) -> bool {
        Self::binop_slice(a, b, u64::overflowing_sub)
    }
    #[inline(always)]
    fn binop_slice(
        a: &mut [u64],
        b: &[u64],
        binop: impl Fn(u64, u64) -> (u64, bool) + Copy,
    ) -> bool {
        let mut c = false;
        a.iter_mut()
            .zip(b.iter())
            .for_each(|(x, y)| {
                let (res, carry) = Self::binop_carry(*x, *y, c, binop);
                *x = res;
                c = carry;
            });
        c
    }
    #[inline(always)]
    fn binop_carry(
        a: u64,
        b: u64,
        c: bool,
        binop: impl Fn(u64, u64) -> (u64, bool),
    ) -> (u64, bool) {
        let (res1, overflow1) = b.overflowing_add(u64::from(c));
        let (res2, overflow2) = binop(a, res1);
        (res2, overflow1 || overflow2)
    }
    #[inline(always)]
    const fn mul_u64(a: u64, b: u64, carry: u64) -> (u64, u64) {
        let (hi, lo) = Self::split_u128(a as u128 * b as u128 + carry as u128);
        (lo, hi)
    }
    #[inline(always)]
    const fn split(a: u64) -> (u64, u64) {
        (a >> 32, a & 0xFFFF_FFFF)
    }
    #[inline(always)]
    const fn split_u128(a: u128) -> (u64, u64) {
        ((a >> 64) as _, (a & 0xFFFFFFFFFFFFFFFF) as _)
    }
    /// Overflowing multiplication by u64.
    /// Returns the result and carry.
    fn overflowing_mul_u64(mut self, other: u64) -> (Self, u64) {
        let mut carry = 0u64;
        for d in self.0.iter_mut() {
            let (res, c) = Self::mul_u64(*d, other, carry);
            *d = res;
            carry = c;
        }
        (self, carry)
    }
    /// Converts from big endian representation bytes in memory.
    pub fn from_big_endian(slice: &[u8]) -> Self {
        use ::uint::byteorder::{ByteOrder, BigEndian};
        if !(8 * 8 >= slice.len()) {
            ::core::panicking::panic("assertion failed: 8 * 8 >= slice.len()")
        }
        let mut padded = [0u8; 8 * 8];
        padded[8 * 8 - slice.len()..8 * 8].copy_from_slice(&slice);
        let mut ret = [0; 8];
        for i in 0..8 {
            ret[8 - i - 1] = BigEndian::read_u64(&padded[8 * i..]);
        }
        U512(ret)
    }
    /// Converts from little endian representation bytes in memory.
    pub fn from_little_endian(slice: &[u8]) -> Self {
        use ::uint::byteorder::{ByteOrder, LittleEndian};
        if !(8 * 8 >= slice.len()) {
            ::core::panicking::panic("assertion failed: 8 * 8 >= slice.len()")
        }
        let mut padded = [0u8; 8 * 8];
        padded[0..slice.len()].copy_from_slice(&slice);
        let mut ret = [0; 8];
        for i in 0..8 {
            ret[i] = LittleEndian::read_u64(&padded[8 * i..]);
        }
        U512(ret)
    }
    fn fmt_hex(
        &self,
        f: &mut ::uint::core_::fmt::Formatter,
        is_lower: bool,
    ) -> ::uint::core_::fmt::Result {
        let &U512(ref data) = self;
        if self.is_zero() {
            return f.pad_integral(true, "0x", "0");
        }
        let mut latch = false;
        let mut buf = [0_u8; 8 * 16];
        let mut i = 0;
        for ch in data.iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64))
                    >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0;
                }
                if latch {
                    let nibble = match nibble {
                        0..=9 => nibble as u8 + b'0',
                        _ if is_lower => nibble as u8 - 10 + b'a',
                        _ => nibble as u8 - 10 + b'A',
                    };
                    buf[i] = nibble;
                    i += 1;
                }
            }
        }
        let s = unsafe { ::uint::core_::str::from_utf8_unchecked(&buf[0..i]) };
        f.pad_integral(true, "0x", s)
    }
}
impl ::uint::core_::convert::From<U512> for [u8; 8 * 8] {
    fn from(number: U512) -> Self {
        let mut arr = [0u8; 8 * 8];
        number.to_big_endian(&mut arr);
        arr
    }
}
impl ::uint::core_::convert::From<[u8; 8 * 8]> for U512 {
    fn from(bytes: [u8; 8 * 8]) -> Self {
        Self::from(&bytes)
    }
}
impl<'a> ::uint::core_::convert::From<&'a [u8; 8 * 8]> for U512 {
    fn from(bytes: &[u8; 8 * 8]) -> Self {
        Self::from(&bytes[..])
    }
}
impl ::uint::core_::default::Default for U512 {
    fn default() -> Self {
        U512::zero()
    }
}
impl ::uint::core_::convert::From<u64> for U512 {
    fn from(value: u64) -> U512 {
        let mut ret = [0; 8];
        ret[0] = value;
        U512(ret)
    }
}
impl From<u8> for U512 {
    fn from(value: u8) -> U512 {
        From::from(value as u64)
    }
}
impl From<u16> for U512 {
    fn from(value: u16) -> U512 {
        From::from(value as u64)
    }
}
impl From<u32> for U512 {
    fn from(value: u32) -> U512 {
        From::from(value as u64)
    }
}
impl From<usize> for U512 {
    fn from(value: usize) -> U512 {
        From::from(value as u64)
    }
}
impl ::uint::core_::convert::From<i64> for U512 {
    fn from(value: i64) -> U512 {
        match value >= 0 {
            true => From::from(value as u64),
            false => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Unsigned integer can\'t be created from negative value",
                        ),
                    );
                };
            }
        }
    }
}
impl From<i8> for U512 {
    fn from(value: i8) -> U512 {
        From::from(value as i64)
    }
}
impl From<i16> for U512 {
    fn from(value: i16) -> U512 {
        From::from(value as i64)
    }
}
impl From<i32> for U512 {
    fn from(value: i32) -> U512 {
        From::from(value as i64)
    }
}
impl From<isize> for U512 {
    fn from(value: isize) -> U512 {
        From::from(value as i64)
    }
}
impl<'a> ::uint::core_::convert::From<&'a [u8]> for U512 {
    fn from(bytes: &[u8]) -> U512 {
        Self::from_big_endian(bytes)
    }
}
impl ::uint::core_::convert::TryFrom<U512> for u8 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<u8, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <u8>::max_value() as u64 {
            Err("integer overflow when casting to u8")
        } else {
            Ok(arr[0] as u8)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for u16 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<u16, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <u16>::max_value() as u64 {
            Err("integer overflow when casting to u16")
        } else {
            Ok(arr[0] as u16)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for u32 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<u32, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <u32>::max_value() as u64 {
            Err("integer overflow when casting to u32")
        } else {
            Ok(arr[0] as u32)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for usize {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<usize, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <usize>::max_value() as u64 {
            Err("integer overflow when casting to usize")
        } else {
            Ok(arr[0] as usize)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for u64 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<u64, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <u64>::max_value() as u64 {
            Err("integer overflow when casting to u64")
        } else {
            Ok(arr[0] as u64)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for i8 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<i8, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <i8>::max_value() as u64 {
            Err("integer overflow when casting to i8")
        } else {
            Ok(arr[0] as i8)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for i16 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<i16, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <i16>::max_value() as u64 {
            Err("integer overflow when casting to i16")
        } else {
            Ok(arr[0] as i16)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for i32 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<i32, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <i32>::max_value() as u64 {
            Err("integer overflow when casting to i32")
        } else {
            Ok(arr[0] as i32)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for isize {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<isize, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <isize>::max_value() as u64 {
            Err("integer overflow when casting to isize")
        } else {
            Ok(arr[0] as isize)
        }
    }
}
impl ::uint::core_::convert::TryFrom<U512> for i64 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<i64, &'static str> {
        let U512(arr) = u;
        if !u.fits_word() || arr[0] > <i64>::max_value() as u64 {
            Err("integer overflow when casting to i64")
        } else {
            Ok(arr[0] as i64)
        }
    }
}
impl<T> ::uint::core_::ops::Add<T> for U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn add(self, other: T) -> U512 {
        let (result, overflow) = self.overflowing_add(other.into());
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a, T> ::uint::core_::ops::Add<T> for &'a U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn add(self, other: T) -> U512 {
        *self + other
    }
}
impl ::uint::core_::ops::AddAssign<U512> for U512 {
    fn add_assign(&mut self, other: U512) {
        let (result, overflow) = self.overflowing_add(other);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        *self = result;
    }
}
impl<T> ::uint::core_::ops::Sub<T> for U512
where
    T: Into<U512>,
{
    type Output = U512;
    #[inline]
    fn sub(self, other: T) -> U512 {
        let (result, overflow) = self.overflowing_sub(other.into());
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a, T> ::uint::core_::ops::Sub<T> for &'a U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn sub(self, other: T) -> U512 {
        *self - other
    }
}
impl ::uint::core_::ops::SubAssign<U512> for U512 {
    fn sub_assign(&mut self, other: U512) {
        let (result, overflow) = self.overflowing_sub(other);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<U512> for U512 {
    type Output = U512;
    fn mul(self, other: U512) -> U512 {
        let bignum: U512 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a U512> for U512 {
    type Output = U512;
    fn mul(self, other: &'a U512) -> U512 {
        let bignum: U512 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a U512> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a U512) -> U512 {
        let bignum: U512 = (*other).into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<U512> for &'a U512 {
    type Output = U512;
    fn mul(self, other: U512) -> U512 {
        let bignum: U512 = other.into();
        let (result, overflow) = self.overflowing_mul(bignum);
        if overflow {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<U512> for U512 {
    fn mul_assign(&mut self, other: U512) {
        let result = *self * other;
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u8> for U512 {
    type Output = U512;
    fn mul(self, other: u8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u8> for U512 {
    type Output = U512;
    fn mul(self, other: &'a u8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u8> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a u8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u8> for &'a U512 {
    type Output = U512;
    fn mul(self, other: u8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u8> for U512 {
    fn mul_assign(&mut self, other: u8) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u16> for U512 {
    type Output = U512;
    fn mul(self, other: u16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u16> for U512 {
    type Output = U512;
    fn mul(self, other: &'a u16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u16> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a u16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u16> for &'a U512 {
    type Output = U512;
    fn mul(self, other: u16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u16> for U512 {
    fn mul_assign(&mut self, other: u16) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u32> for U512 {
    type Output = U512;
    fn mul(self, other: u32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u32> for U512 {
    type Output = U512;
    fn mul(self, other: &'a u32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u32> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a u32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u32> for &'a U512 {
    type Output = U512;
    fn mul(self, other: u32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u32> for U512 {
    fn mul_assign(&mut self, other: u32) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<u64> for U512 {
    type Output = U512;
    fn mul(self, other: u64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u64> for U512 {
    type Output = U512;
    fn mul(self, other: &'a u64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a u64> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a u64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<u64> for &'a U512 {
    type Output = U512;
    fn mul(self, other: u64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<u64> for U512 {
    fn mul_assign(&mut self, other: u64) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<usize> for U512 {
    type Output = U512;
    fn mul(self, other: usize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a usize> for U512 {
    type Output = U512;
    fn mul(self, other: &'a usize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a usize> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a usize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<usize> for &'a U512 {
    type Output = U512;
    fn mul(self, other: usize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<usize> for U512 {
    fn mul_assign(&mut self, other: usize) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i8> for U512 {
    type Output = U512;
    fn mul(self, other: i8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i8> for U512 {
    type Output = U512;
    fn mul(self, other: &'a i8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i8> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a i8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i8> for &'a U512 {
    type Output = U512;
    fn mul(self, other: i8) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i8> for U512 {
    fn mul_assign(&mut self, other: i8) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i16> for U512 {
    type Output = U512;
    fn mul(self, other: i16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i16> for U512 {
    type Output = U512;
    fn mul(self, other: &'a i16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i16> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a i16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i16> for &'a U512 {
    type Output = U512;
    fn mul(self, other: i16) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i16> for U512 {
    fn mul_assign(&mut self, other: i16) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i32> for U512 {
    type Output = U512;
    fn mul(self, other: i32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i32> for U512 {
    type Output = U512;
    fn mul(self, other: &'a i32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i32> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a i32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i32> for &'a U512 {
    type Output = U512;
    fn mul(self, other: i32) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i32> for U512 {
    fn mul_assign(&mut self, other: i32) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<i64> for U512 {
    type Output = U512;
    fn mul(self, other: i64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i64> for U512 {
    type Output = U512;
    fn mul(self, other: &'a i64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a i64> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a i64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<i64> for &'a U512 {
    type Output = U512;
    fn mul(self, other: i64) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<i64> for U512 {
    fn mul_assign(&mut self, other: i64) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl ::uint::core_::ops::Mul<isize> for U512 {
    type Output = U512;
    fn mul(self, other: isize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a isize> for U512 {
    type Output = U512;
    fn mul(self, other: &'a isize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<&'a isize> for &'a U512 {
    type Output = U512;
    fn mul(self, other: &'a isize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(*other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl<'a> ::uint::core_::ops::Mul<isize> for &'a U512 {
    type Output = U512;
    fn mul(self, other: isize) -> U512 {
        let (result, carry) = self.overflowing_mul_u64(other as u64);
        if carry > 0 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("arithmetic operation overflow"),
                );
            }
        }
        result
    }
}
impl ::uint::core_::ops::MulAssign<isize> for U512 {
    fn mul_assign(&mut self, other: isize) {
        let result = *self * (other as u64);
        *self = result;
    }
}
impl<T> ::uint::core_::ops::Div<T> for U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn div(self, other: T) -> U512 {
        let other: Self = other.into();
        self.div_mod(other).0
    }
}
impl<'a, T> ::uint::core_::ops::Div<T> for &'a U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn div(self, other: T) -> U512 {
        *self / other
    }
}
impl<T> ::uint::core_::ops::DivAssign<T> for U512
where
    T: Into<U512>,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other.into();
    }
}
impl<T> ::uint::core_::ops::Rem<T> for U512
where
    T: Into<U512> + Copy,
{
    type Output = U512;
    fn rem(self, other: T) -> U512 {
        let mut sub_copy = self;
        sub_copy %= other;
        sub_copy
    }
}
impl<'a, T> ::uint::core_::ops::Rem<T> for &'a U512
where
    T: Into<U512> + Copy,
{
    type Output = U512;
    fn rem(self, other: T) -> U512 {
        *self % other
    }
}
impl<T> ::uint::core_::ops::RemAssign<T> for U512
where
    T: Into<U512> + Copy,
{
    fn rem_assign(&mut self, other: T) {
        let other: Self = other.into();
        let rem = self.div_mod(other).1;
        *self = rem;
    }
}
impl ::uint::core_::ops::BitAnd<U512> for U512 {
    type Output = U512;
    #[inline]
    fn bitand(self, other: U512) -> U512 {
        let U512(ref arr1) = self;
        let U512(ref arr2) = other;
        let mut ret = [0u64; 8];
        for i in 0..8 {
            ret[i] = arr1[i] & arr2[i];
        }
        U512(ret)
    }
}
impl ::uint::core_::ops::BitAndAssign<U512> for U512 {
    fn bitand_assign(&mut self, rhs: U512) {
        *self = *self & rhs;
    }
}
impl ::uint::core_::ops::BitXor<U512> for U512 {
    type Output = U512;
    #[inline]
    fn bitxor(self, other: U512) -> U512 {
        let U512(ref arr1) = self;
        let U512(ref arr2) = other;
        let mut ret = [0u64; 8];
        for i in 0..8 {
            ret[i] = arr1[i] ^ arr2[i];
        }
        U512(ret)
    }
}
impl ::uint::core_::ops::BitXorAssign<U512> for U512 {
    fn bitxor_assign(&mut self, rhs: U512) {
        *self = *self ^ rhs;
    }
}
impl ::uint::core_::ops::BitOr<U512> for U512 {
    type Output = U512;
    #[inline]
    fn bitor(self, other: U512) -> U512 {
        let U512(ref arr1) = self;
        let U512(ref arr2) = other;
        let mut ret = [0u64; 8];
        for i in 0..8 {
            ret[i] = arr1[i] | arr2[i];
        }
        U512(ret)
    }
}
impl ::uint::core_::ops::BitOrAssign<U512> for U512 {
    fn bitor_assign(&mut self, rhs: U512) {
        *self = *self | rhs;
    }
}
impl ::uint::core_::ops::Not for U512 {
    type Output = U512;
    #[inline]
    fn not(self) -> U512 {
        let U512(ref arr) = self;
        let mut ret = [0u64; 8];
        for i in 0..8 {
            ret[i] = !arr[i];
        }
        U512(ret)
    }
}
impl<T> ::uint::core_::ops::Shl<T> for U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn shl(self, shift: T) -> U512 {
        let shift = shift.into().as_usize();
        let U512(ref original) = self;
        let mut ret = [0u64; 8];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..8 {
            ret[i] = original[i - word_shift] << bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..8 {
                ret[i] += original[i - 1 - word_shift] >> (64 - bit_shift);
            }
        }
        U512(ret)
    }
}
impl<'a, T> ::uint::core_::ops::Shl<T> for &'a U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn shl(self, shift: T) -> U512 {
        *self << shift
    }
}
impl<T> ::uint::core_::ops::ShlAssign<T> for U512
where
    T: Into<U512>,
{
    fn shl_assign(&mut self, shift: T) {
        *self = *self << shift;
    }
}
impl<T> ::uint::core_::ops::Shr<T> for U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn shr(self, shift: T) -> U512 {
        let shift = shift.into().as_usize();
        let U512(ref original) = self;
        let mut ret = [0u64; 8];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in word_shift..8 {
            ret[i - word_shift] = original[i] >> bit_shift;
        }
        if bit_shift > 0 {
            for i in word_shift + 1..8 {
                ret[i - word_shift - 1] += original[i] << (64 - bit_shift);
            }
        }
        U512(ret)
    }
}
impl<'a, T> ::uint::core_::ops::Shr<T> for &'a U512
where
    T: Into<U512>,
{
    type Output = U512;
    fn shr(self, shift: T) -> U512 {
        *self >> shift
    }
}
impl<T> ::uint::core_::ops::ShrAssign<T> for U512
where
    T: Into<U512>,
{
    fn shr_assign(&mut self, shift: T) {
        *self = *self >> shift;
    }
}
impl ::uint::core_::cmp::Ord for U512 {
    fn cmp(&self, other: &U512) -> ::uint::core_::cmp::Ordering {
        self.as_ref().iter().rev().cmp(other.as_ref().iter().rev())
    }
}
impl ::uint::core_::cmp::PartialOrd for U512 {
    fn partial_cmp(&self, other: &U512) -> Option<::uint::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::uint::core_::fmt::Debug for U512 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        ::uint::core_::fmt::Display::fmt(self, f)
    }
}
impl ::uint::core_::fmt::Display for U512 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        if self.is_zero() {
            return f.write_fmt(format_args!("0"));
        }
        let mut buf = [0_u8; 8 * 20];
        let mut i = buf.len() - 1;
        let mut current = *self;
        let ten = U512::from(10);
        loop {
            let digit = (current % ten).low_u64() as u8;
            buf[i] = digit + b'0';
            current /= ten;
            if current.is_zero() {
                break;
            }
            i -= 1;
        }
        let s = unsafe { ::uint::core_::str::from_utf8_unchecked(&buf[i..]) };
        f.pad_integral(true, "", s)
    }
}
impl ::uint::core_::fmt::LowerHex for U512 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        self.fmt_hex(f, true)
    }
}
impl ::uint::core_::fmt::UpperHex for U512 {
    fn fmt(&self, f: &mut ::uint::core_::fmt::Formatter) -> ::uint::core_::fmt::Result {
        self.fmt_hex(f, false)
    }
}
impl ::uint::core_::str::FromStr for U512 {
    type Err = ::uint::FromHexError;
    fn from_str(value: &str) -> ::uint::core_::result::Result<U512, Self::Err> {
        let value = value.strip_prefix("0x").unwrap_or(value);
        const BYTES_LEN: usize = 8 * 8;
        const MAX_ENCODED_LEN: usize = BYTES_LEN * 2;
        let mut bytes = [0_u8; BYTES_LEN];
        let encoded = value.as_bytes();
        if encoded.len() > MAX_ENCODED_LEN {
            return Err(::uint::hex::FromHexError::InvalidStringLength.into());
        }
        if encoded.len() % 2 == 0 {
            let out = &mut bytes[BYTES_LEN - encoded.len() / 2..];
            ::uint::hex::decode_to_slice(encoded, out).map_err(Self::Err::from)?;
        } else {
            let mut s = [b'0'; MAX_ENCODED_LEN];
            s[MAX_ENCODED_LEN - encoded.len()..].copy_from_slice(encoded);
            let encoded = &s[MAX_ENCODED_LEN - encoded.len() - 1..];
            let out = &mut bytes[BYTES_LEN - encoded.len() / 2..];
            ::uint::hex::decode_to_slice(encoded, out).map_err(Self::Err::from)?;
        }
        let bytes_ref: &[u8] = &bytes;
        Ok(From::from(bytes_ref))
    }
}
impl ::uint::core_::convert::From<&'static str> for U512 {
    fn from(s: &'static str) -> Self {
        s.parse().unwrap()
    }
}
impl ::uint::core_::convert::From<u128> for U512 {
    fn from(value: u128) -> U512 {
        let mut ret = [0; 8];
        ret[0] = value as u64;
        ret[1] = (value >> 64) as u64;
        U512(ret)
    }
}
impl ::uint::core_::convert::From<i128> for U512 {
    fn from(value: i128) -> U512 {
        match value >= 0 {
            true => From::from(value as u128),
            false => {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Unsigned integer can\'t be created from negative value",
                        ),
                    );
                };
            }
        }
    }
}
impl U512 {
    /// Low 2 words (u128)
    #[inline]
    pub const fn low_u128(&self) -> u128 {
        let &U512(ref arr) = self;
        ((arr[1] as u128) << 64) + arr[0] as u128
    }
    /// Conversion to u128 with overflow checking
    ///
    /// # Panics
    ///
    /// Panics if the number is larger than 2^128.
    #[inline]
    pub fn as_u128(&self) -> u128 {
        let &U512(ref arr) = self;
        for i in 2..8 {
            if arr[i] != 0 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Integer overflow when casting to u128"),
                    );
                }
            }
        }
        self.low_u128()
    }
}
impl ::uint::core_::convert::TryFrom<U512> for u128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<u128, &'static str> {
        let U512(arr) = u;
        for i in 2..8 {
            if arr[i] != 0 {
                return Err("integer overflow when casting to u128");
            }
        }
        Ok(((arr[1] as u128) << 64) + arr[0] as u128)
    }
}
impl ::uint::core_::convert::TryFrom<U512> for i128 {
    type Error = &'static str;
    #[inline]
    fn try_from(u: U512) -> ::uint::core_::result::Result<i128, &'static str> {
        let err_str = "integer overflow when casting to i128";
        let i = u128::try_from(u).map_err(|_| err_str)?;
        if i > i128::max_value() as u128 { Err(err_str) } else { Ok(i as i128) }
    }
}
#[repr(C)]
/// Fixed-size uninterpreted hash type with 16 bytes (128 bits) size.
pub struct H128(pub [u8; 16]);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for H128 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for H128 {
    #[inline]
    fn eq(&self, other: &H128) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for H128 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 16]>;
    }
}
impl From<[u8; 16]> for H128 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 16]) -> Self {
        H128(bytes)
    }
}
impl<'a> From<&'a [u8; 16]> for H128 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 16]) -> Self {
        H128(*bytes)
    }
}
impl<'a> From<&'a mut [u8; 16]> for H128 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 16]) -> Self {
        H128(*bytes)
    }
}
impl From<H128> for [u8; 16] {
    #[inline]
    fn from(s: H128) -> Self {
        s.0
    }
}
impl AsRef<[u8]> for H128 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<[u8]> for H128 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl H128 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> H128 {
        H128([byte; 16])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub const fn zero() -> H128 {
        H128::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        16
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> &[u8; 16] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; 16] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    pub const fn to_fixed_bytes(self) -> [u8; 16] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        match (&src.len(), &16) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        self.as_bytes_mut().copy_from_slice(src);
    }
    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    pub fn from_slice(src: &[u8]) -> Self {
        match (&src.len(), &16) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(b & self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}
impl ::fixed_hash::core_::fmt::Debug for H128 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("{0:#x}", self))
    }
}
impl ::fixed_hash::core_::fmt::Display for H128 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("0x"))?;
        for i in &self.0[0..2] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        f.write_fmt(format_args!("…"))?;
        for i in &self.0[16 - 2..16] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::LowerHex for H128 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0x"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::UpperHex for H128 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0X"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02X}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::marker::Copy for H128 {}
impl ::fixed_hash::core_::clone::Clone for H128 {
    fn clone(&self) -> H128 {
        *self
    }
}
impl ::fixed_hash::core_::cmp::PartialOrd for H128 {
    fn partial_cmp(&self, other: &Self) -> Option<::fixed_hash::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::fixed_hash::core_::hash::Hash for H128 {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::fixed_hash::core_::hash::Hasher,
    {
        state.write(&self.0);
    }
}
impl<I> ::fixed_hash::core_::ops::Index<I> for H128
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.as_bytes()[index]
    }
}
impl<I> ::fixed_hash::core_::ops::IndexMut<I> for H128
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8], Output = [u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.as_bytes_mut()[index]
    }
}
impl ::fixed_hash::core_::default::Default for H128 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}
impl<'r> ::fixed_hash::core_::ops::BitOrAssign<&'r H128> for H128 {
    fn bitor_assign(&mut self, rhs: &'r H128) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs |= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitOrAssign<H128> for H128 {
    #[inline]
    fn bitor_assign(&mut self, rhs: H128) {
        *self |= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitOr<&'r H128> for &'l H128 {
    type Output = H128;
    fn bitor(self, rhs: &'r H128) -> Self::Output {
        let mut ret = self.clone();
        ret |= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitOr<H128> for H128 {
    type Output = H128;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitAndAssign<&'r H128> for H128 {
    fn bitand_assign(&mut self, rhs: &'r H128) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs &= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitAndAssign<H128> for H128 {
    #[inline]
    fn bitand_assign(&mut self, rhs: H128) {
        *self &= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitAnd<&'r H128> for &'l H128 {
    type Output = H128;
    fn bitand(self, rhs: &'r H128) -> Self::Output {
        let mut ret = self.clone();
        ret &= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitAnd<H128> for H128 {
    type Output = H128;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitXorAssign<&'r H128> for H128 {
    fn bitxor_assign(&mut self, rhs: &'r H128) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs ^= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitXorAssign<H128> for H128 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: H128) {
        *self ^= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitXor<&'r H128> for &'l H128 {
    type Output = H128;
    fn bitxor(self, rhs: &'r H128) -> Self::Output {
        let mut ret = self.clone();
        ret ^= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitXor<H128> for H128 {
    type Output = H128;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
/// Utilities using the `byteorder` crate.
impl H128 {
    /// Returns the least significant `n` bytes as slice.
    ///
    /// # Panics
    ///
    /// If `n` is greater than the number of bytes in `self`.
    #[inline]
    fn least_significant_bytes(&self, n: usize) -> &[u8] {
        match (&true, &(n <= Self::len_bytes())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        &self[(Self::len_bytes() - n)..]
    }
    fn to_low_u64_with_byteorder<B>(&self) -> u64
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        buf[(8 - capped)..].copy_from_slice(self.least_significant_bytes(capped));
        B::read_u64(&buf)
    }
    /// Returns the lowest 8 bytes interpreted as big-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_be(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as little-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_le(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as native-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_ne(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>()
    }
    fn from_low_u64_with_byteorder<B>(val: u64) -> Self
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        B::write_u64(&mut buf, val);
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        let mut bytes = [0x0; ::fixed_hash::core_::mem::size_of::<Self>()];
        bytes[(Self::len_bytes() - capped)..].copy_from_slice(&buf[..capped]);
        Self::from_slice(&bytes)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as big endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_be(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as little endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_le(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as native endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_ne(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>(val)
    }
}
impl ::fixed_hash::rand::distributions::Distribution<H128>
for ::fixed_hash::rand::distributions::Standard {
    fn sample<R: ::fixed_hash::rand::Rng + ?Sized>(&self, rng: &mut R) -> H128 {
        let mut ret = H128::zero();
        for byte in ret.as_bytes_mut().iter_mut() {
            *byte = rng.gen();
        }
        ret
    }
}
/// Utilities using the `rand` crate.
impl H128 {
    /// Assign `self` to a cryptographically random value using the
    /// given random number generator.
    pub fn randomize_using<R>(&mut self, rng: &mut R)
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        use ::fixed_hash::rand::distributions::Distribution;
        *self = ::fixed_hash::rand::distributions::Standard.sample(rng);
    }
    /// Assign `self` to a cryptographically random value.
    pub fn randomize(&mut self) {
        let mut rng = ::fixed_hash::rand::rngs::OsRng;
        self.randomize_using(&mut rng);
    }
    /// Create a new hash with cryptographically random content using the
    /// given random number generator.
    pub fn random_using<R>(rng: &mut R) -> Self
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        let mut ret = Self::zero();
        ret.randomize_using(rng);
        ret
    }
    /// Create a new hash with cryptographically random content.
    pub fn random() -> Self {
        let mut hash = Self::zero();
        hash.randomize();
        hash
    }
}
impl ::fixed_hash::core_::cmp::Ord for H128 {
    #[inline]
    fn cmp(&self, other: &Self) -> ::fixed_hash::core_::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl ::fixed_hash::core_::str::FromStr for H128 {
    type Err = ::fixed_hash::rustc_hex::FromHexError;
    /// Creates a hash type instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn from_str(
        input: &str,
    ) -> ::fixed_hash::core_::result::Result<
        H128,
        ::fixed_hash::rustc_hex::FromHexError,
    > {
        let input = input.strip_prefix("0x").unwrap_or(input);
        let mut iter = ::fixed_hash::rustc_hex::FromHexIter::new(input);
        let mut result = Self::zero();
        for byte in result.as_mut() {
            *byte = iter.next().ok_or(Self::Err::InvalidHexLength)??;
        }
        if iter.next().is_some() {
            return Err(Self::Err::InvalidHexLength);
        }
        Ok(result)
    }
}
#[repr(C)]
/// Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
pub struct H160(pub [u8; 20]);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for H160 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for H160 {
    #[inline]
    fn eq(&self, other: &H160) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for H160 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 20]>;
    }
}
impl From<[u8; 20]> for H160 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 20]) -> Self {
        H160(bytes)
    }
}
impl<'a> From<&'a [u8; 20]> for H160 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 20]) -> Self {
        H160(*bytes)
    }
}
impl<'a> From<&'a mut [u8; 20]> for H160 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 20]) -> Self {
        H160(*bytes)
    }
}
impl From<H160> for [u8; 20] {
    #[inline]
    fn from(s: H160) -> Self {
        s.0
    }
}
impl AsRef<[u8]> for H160 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<[u8]> for H160 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl H160 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> H160 {
        H160([byte; 20])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub const fn zero() -> H160 {
        H160::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        20
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> &[u8; 20] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; 20] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    pub const fn to_fixed_bytes(self) -> [u8; 20] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        match (&src.len(), &20) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        self.as_bytes_mut().copy_from_slice(src);
    }
    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    pub fn from_slice(src: &[u8]) -> Self {
        match (&src.len(), &20) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(b & self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}
impl ::fixed_hash::core_::fmt::Debug for H160 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("{0:#x}", self))
    }
}
impl ::fixed_hash::core_::fmt::Display for H160 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("0x"))?;
        for i in &self.0[0..2] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        f.write_fmt(format_args!("…"))?;
        for i in &self.0[20 - 2..20] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::LowerHex for H160 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0x"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::UpperHex for H160 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0X"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02X}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::marker::Copy for H160 {}
impl ::fixed_hash::core_::clone::Clone for H160 {
    fn clone(&self) -> H160 {
        *self
    }
}
impl ::fixed_hash::core_::cmp::PartialOrd for H160 {
    fn partial_cmp(&self, other: &Self) -> Option<::fixed_hash::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::fixed_hash::core_::hash::Hash for H160 {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::fixed_hash::core_::hash::Hasher,
    {
        state.write(&self.0);
    }
}
impl<I> ::fixed_hash::core_::ops::Index<I> for H160
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.as_bytes()[index]
    }
}
impl<I> ::fixed_hash::core_::ops::IndexMut<I> for H160
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8], Output = [u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.as_bytes_mut()[index]
    }
}
impl ::fixed_hash::core_::default::Default for H160 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}
impl<'r> ::fixed_hash::core_::ops::BitOrAssign<&'r H160> for H160 {
    fn bitor_assign(&mut self, rhs: &'r H160) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs |= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitOrAssign<H160> for H160 {
    #[inline]
    fn bitor_assign(&mut self, rhs: H160) {
        *self |= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitOr<&'r H160> for &'l H160 {
    type Output = H160;
    fn bitor(self, rhs: &'r H160) -> Self::Output {
        let mut ret = self.clone();
        ret |= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitOr<H160> for H160 {
    type Output = H160;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitAndAssign<&'r H160> for H160 {
    fn bitand_assign(&mut self, rhs: &'r H160) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs &= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitAndAssign<H160> for H160 {
    #[inline]
    fn bitand_assign(&mut self, rhs: H160) {
        *self &= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitAnd<&'r H160> for &'l H160 {
    type Output = H160;
    fn bitand(self, rhs: &'r H160) -> Self::Output {
        let mut ret = self.clone();
        ret &= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitAnd<H160> for H160 {
    type Output = H160;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitXorAssign<&'r H160> for H160 {
    fn bitxor_assign(&mut self, rhs: &'r H160) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs ^= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitXorAssign<H160> for H160 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: H160) {
        *self ^= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitXor<&'r H160> for &'l H160 {
    type Output = H160;
    fn bitxor(self, rhs: &'r H160) -> Self::Output {
        let mut ret = self.clone();
        ret ^= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitXor<H160> for H160 {
    type Output = H160;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
/// Utilities using the `byteorder` crate.
impl H160 {
    /// Returns the least significant `n` bytes as slice.
    ///
    /// # Panics
    ///
    /// If `n` is greater than the number of bytes in `self`.
    #[inline]
    fn least_significant_bytes(&self, n: usize) -> &[u8] {
        match (&true, &(n <= Self::len_bytes())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        &self[(Self::len_bytes() - n)..]
    }
    fn to_low_u64_with_byteorder<B>(&self) -> u64
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        buf[(8 - capped)..].copy_from_slice(self.least_significant_bytes(capped));
        B::read_u64(&buf)
    }
    /// Returns the lowest 8 bytes interpreted as big-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_be(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as little-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_le(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as native-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_ne(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>()
    }
    fn from_low_u64_with_byteorder<B>(val: u64) -> Self
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        B::write_u64(&mut buf, val);
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        let mut bytes = [0x0; ::fixed_hash::core_::mem::size_of::<Self>()];
        bytes[(Self::len_bytes() - capped)..].copy_from_slice(&buf[..capped]);
        Self::from_slice(&bytes)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as big endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_be(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as little endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_le(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as native endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_ne(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>(val)
    }
}
impl ::fixed_hash::rand::distributions::Distribution<H160>
for ::fixed_hash::rand::distributions::Standard {
    fn sample<R: ::fixed_hash::rand::Rng + ?Sized>(&self, rng: &mut R) -> H160 {
        let mut ret = H160::zero();
        for byte in ret.as_bytes_mut().iter_mut() {
            *byte = rng.gen();
        }
        ret
    }
}
/// Utilities using the `rand` crate.
impl H160 {
    /// Assign `self` to a cryptographically random value using the
    /// given random number generator.
    pub fn randomize_using<R>(&mut self, rng: &mut R)
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        use ::fixed_hash::rand::distributions::Distribution;
        *self = ::fixed_hash::rand::distributions::Standard.sample(rng);
    }
    /// Assign `self` to a cryptographically random value.
    pub fn randomize(&mut self) {
        let mut rng = ::fixed_hash::rand::rngs::OsRng;
        self.randomize_using(&mut rng);
    }
    /// Create a new hash with cryptographically random content using the
    /// given random number generator.
    pub fn random_using<R>(rng: &mut R) -> Self
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        let mut ret = Self::zero();
        ret.randomize_using(rng);
        ret
    }
    /// Create a new hash with cryptographically random content.
    pub fn random() -> Self {
        let mut hash = Self::zero();
        hash.randomize();
        hash
    }
}
impl ::fixed_hash::core_::cmp::Ord for H160 {
    #[inline]
    fn cmp(&self, other: &Self) -> ::fixed_hash::core_::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl ::fixed_hash::core_::str::FromStr for H160 {
    type Err = ::fixed_hash::rustc_hex::FromHexError;
    /// Creates a hash type instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn from_str(
        input: &str,
    ) -> ::fixed_hash::core_::result::Result<
        H160,
        ::fixed_hash::rustc_hex::FromHexError,
    > {
        let input = input.strip_prefix("0x").unwrap_or(input);
        let mut iter = ::fixed_hash::rustc_hex::FromHexIter::new(input);
        let mut result = Self::zero();
        for byte in result.as_mut() {
            *byte = iter.next().ok_or(Self::Err::InvalidHexLength)??;
        }
        if iter.next().is_some() {
            return Err(Self::Err::InvalidHexLength);
        }
        Ok(result)
    }
}
#[repr(C)]
/// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
pub struct H256(pub [u8; 32]);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for H256 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for H256 {
    #[inline]
    fn eq(&self, other: &H256) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for H256 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
    }
}
impl From<[u8; 32]> for H256 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}
impl<'a> From<&'a [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        H256(*bytes)
    }
}
impl<'a> From<&'a mut [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        H256(*bytes)
    }
}
impl From<H256> for [u8; 32] {
    #[inline]
    fn from(s: H256) -> Self {
        s.0
    }
}
impl AsRef<[u8]> for H256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<[u8]> for H256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl H256 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> H256 {
        H256([byte; 32])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub const fn zero() -> H256 {
        H256::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        32
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    pub const fn to_fixed_bytes(self) -> [u8; 32] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        match (&src.len(), &32) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        self.as_bytes_mut().copy_from_slice(src);
    }
    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    pub fn from_slice(src: &[u8]) -> Self {
        match (&src.len(), &32) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(b & self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}
impl ::fixed_hash::core_::fmt::Debug for H256 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("{0:#x}", self))
    }
}
impl ::fixed_hash::core_::fmt::Display for H256 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("0x"))?;
        for i in &self.0[0..2] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        f.write_fmt(format_args!("…"))?;
        for i in &self.0[32 - 2..32] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::LowerHex for H256 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0x"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::UpperHex for H256 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0X"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02X}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::marker::Copy for H256 {}
impl ::fixed_hash::core_::clone::Clone for H256 {
    fn clone(&self) -> H256 {
        *self
    }
}
impl ::fixed_hash::core_::cmp::PartialOrd for H256 {
    fn partial_cmp(&self, other: &Self) -> Option<::fixed_hash::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::fixed_hash::core_::hash::Hash for H256 {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::fixed_hash::core_::hash::Hasher,
    {
        state.write(&self.0);
    }
}
impl<I> ::fixed_hash::core_::ops::Index<I> for H256
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.as_bytes()[index]
    }
}
impl<I> ::fixed_hash::core_::ops::IndexMut<I> for H256
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8], Output = [u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.as_bytes_mut()[index]
    }
}
impl ::fixed_hash::core_::default::Default for H256 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}
impl<'r> ::fixed_hash::core_::ops::BitOrAssign<&'r H256> for H256 {
    fn bitor_assign(&mut self, rhs: &'r H256) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs |= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitOrAssign<H256> for H256 {
    #[inline]
    fn bitor_assign(&mut self, rhs: H256) {
        *self |= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitOr<&'r H256> for &'l H256 {
    type Output = H256;
    fn bitor(self, rhs: &'r H256) -> Self::Output {
        let mut ret = self.clone();
        ret |= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitOr<H256> for H256 {
    type Output = H256;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitAndAssign<&'r H256> for H256 {
    fn bitand_assign(&mut self, rhs: &'r H256) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs &= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitAndAssign<H256> for H256 {
    #[inline]
    fn bitand_assign(&mut self, rhs: H256) {
        *self &= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitAnd<&'r H256> for &'l H256 {
    type Output = H256;
    fn bitand(self, rhs: &'r H256) -> Self::Output {
        let mut ret = self.clone();
        ret &= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitAnd<H256> for H256 {
    type Output = H256;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitXorAssign<&'r H256> for H256 {
    fn bitxor_assign(&mut self, rhs: &'r H256) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs ^= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitXorAssign<H256> for H256 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: H256) {
        *self ^= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitXor<&'r H256> for &'l H256 {
    type Output = H256;
    fn bitxor(self, rhs: &'r H256) -> Self::Output {
        let mut ret = self.clone();
        ret ^= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitXor<H256> for H256 {
    type Output = H256;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
/// Utilities using the `byteorder` crate.
impl H256 {
    /// Returns the least significant `n` bytes as slice.
    ///
    /// # Panics
    ///
    /// If `n` is greater than the number of bytes in `self`.
    #[inline]
    fn least_significant_bytes(&self, n: usize) -> &[u8] {
        match (&true, &(n <= Self::len_bytes())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        &self[(Self::len_bytes() - n)..]
    }
    fn to_low_u64_with_byteorder<B>(&self) -> u64
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        buf[(8 - capped)..].copy_from_slice(self.least_significant_bytes(capped));
        B::read_u64(&buf)
    }
    /// Returns the lowest 8 bytes interpreted as big-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_be(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as little-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_le(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as native-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_ne(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>()
    }
    fn from_low_u64_with_byteorder<B>(val: u64) -> Self
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        B::write_u64(&mut buf, val);
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        let mut bytes = [0x0; ::fixed_hash::core_::mem::size_of::<Self>()];
        bytes[(Self::len_bytes() - capped)..].copy_from_slice(&buf[..capped]);
        Self::from_slice(&bytes)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as big endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_be(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as little endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_le(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as native endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_ne(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>(val)
    }
}
impl ::fixed_hash::rand::distributions::Distribution<H256>
for ::fixed_hash::rand::distributions::Standard {
    fn sample<R: ::fixed_hash::rand::Rng + ?Sized>(&self, rng: &mut R) -> H256 {
        let mut ret = H256::zero();
        for byte in ret.as_bytes_mut().iter_mut() {
            *byte = rng.gen();
        }
        ret
    }
}
/// Utilities using the `rand` crate.
impl H256 {
    /// Assign `self` to a cryptographically random value using the
    /// given random number generator.
    pub fn randomize_using<R>(&mut self, rng: &mut R)
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        use ::fixed_hash::rand::distributions::Distribution;
        *self = ::fixed_hash::rand::distributions::Standard.sample(rng);
    }
    /// Assign `self` to a cryptographically random value.
    pub fn randomize(&mut self) {
        let mut rng = ::fixed_hash::rand::rngs::OsRng;
        self.randomize_using(&mut rng);
    }
    /// Create a new hash with cryptographically random content using the
    /// given random number generator.
    pub fn random_using<R>(rng: &mut R) -> Self
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        let mut ret = Self::zero();
        ret.randomize_using(rng);
        ret
    }
    /// Create a new hash with cryptographically random content.
    pub fn random() -> Self {
        let mut hash = Self::zero();
        hash.randomize();
        hash
    }
}
impl ::fixed_hash::core_::cmp::Ord for H256 {
    #[inline]
    fn cmp(&self, other: &Self) -> ::fixed_hash::core_::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl ::fixed_hash::core_::str::FromStr for H256 {
    type Err = ::fixed_hash::rustc_hex::FromHexError;
    /// Creates a hash type instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn from_str(
        input: &str,
    ) -> ::fixed_hash::core_::result::Result<
        H256,
        ::fixed_hash::rustc_hex::FromHexError,
    > {
        let input = input.strip_prefix("0x").unwrap_or(input);
        let mut iter = ::fixed_hash::rustc_hex::FromHexIter::new(input);
        let mut result = Self::zero();
        for byte in result.as_mut() {
            *byte = iter.next().ok_or(Self::Err::InvalidHexLength)??;
        }
        if iter.next().is_some() {
            return Err(Self::Err::InvalidHexLength);
        }
        Ok(result)
    }
}
#[repr(C)]
/// Fixed-size uninterpreted hash type with 48 bytes (384 bits) size.
pub struct H384(pub [u8; 48]);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for H384 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for H384 {
    #[inline]
    fn eq(&self, other: &H384) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for H384 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 48]>;
    }
}
impl From<[u8; 48]> for H384 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 48]) -> Self {
        H384(bytes)
    }
}
impl<'a> From<&'a [u8; 48]> for H384 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 48]) -> Self {
        H384(*bytes)
    }
}
impl<'a> From<&'a mut [u8; 48]> for H384 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 48]) -> Self {
        H384(*bytes)
    }
}
impl From<H384> for [u8; 48] {
    #[inline]
    fn from(s: H384) -> Self {
        s.0
    }
}
impl AsRef<[u8]> for H384 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<[u8]> for H384 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl H384 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> H384 {
        H384([byte; 48])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub const fn zero() -> H384 {
        H384::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        48
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> &[u8; 48] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; 48] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    pub const fn to_fixed_bytes(self) -> [u8; 48] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        match (&src.len(), &48) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        self.as_bytes_mut().copy_from_slice(src);
    }
    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    pub fn from_slice(src: &[u8]) -> Self {
        match (&src.len(), &48) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(b & self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}
impl ::fixed_hash::core_::fmt::Debug for H384 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("{0:#x}", self))
    }
}
impl ::fixed_hash::core_::fmt::Display for H384 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("0x"))?;
        for i in &self.0[0..2] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        f.write_fmt(format_args!("…"))?;
        for i in &self.0[48 - 2..48] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::LowerHex for H384 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0x"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::UpperHex for H384 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0X"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02X}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::marker::Copy for H384 {}
impl ::fixed_hash::core_::clone::Clone for H384 {
    fn clone(&self) -> H384 {
        *self
    }
}
impl ::fixed_hash::core_::cmp::PartialOrd for H384 {
    fn partial_cmp(&self, other: &Self) -> Option<::fixed_hash::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::fixed_hash::core_::hash::Hash for H384 {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::fixed_hash::core_::hash::Hasher,
    {
        state.write(&self.0);
    }
}
impl<I> ::fixed_hash::core_::ops::Index<I> for H384
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.as_bytes()[index]
    }
}
impl<I> ::fixed_hash::core_::ops::IndexMut<I> for H384
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8], Output = [u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.as_bytes_mut()[index]
    }
}
impl ::fixed_hash::core_::default::Default for H384 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}
impl<'r> ::fixed_hash::core_::ops::BitOrAssign<&'r H384> for H384 {
    fn bitor_assign(&mut self, rhs: &'r H384) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs |= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitOrAssign<H384> for H384 {
    #[inline]
    fn bitor_assign(&mut self, rhs: H384) {
        *self |= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitOr<&'r H384> for &'l H384 {
    type Output = H384;
    fn bitor(self, rhs: &'r H384) -> Self::Output {
        let mut ret = self.clone();
        ret |= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitOr<H384> for H384 {
    type Output = H384;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitAndAssign<&'r H384> for H384 {
    fn bitand_assign(&mut self, rhs: &'r H384) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs &= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitAndAssign<H384> for H384 {
    #[inline]
    fn bitand_assign(&mut self, rhs: H384) {
        *self &= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitAnd<&'r H384> for &'l H384 {
    type Output = H384;
    fn bitand(self, rhs: &'r H384) -> Self::Output {
        let mut ret = self.clone();
        ret &= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitAnd<H384> for H384 {
    type Output = H384;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitXorAssign<&'r H384> for H384 {
    fn bitxor_assign(&mut self, rhs: &'r H384) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs ^= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitXorAssign<H384> for H384 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: H384) {
        *self ^= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitXor<&'r H384> for &'l H384 {
    type Output = H384;
    fn bitxor(self, rhs: &'r H384) -> Self::Output {
        let mut ret = self.clone();
        ret ^= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitXor<H384> for H384 {
    type Output = H384;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
/// Utilities using the `byteorder` crate.
impl H384 {
    /// Returns the least significant `n` bytes as slice.
    ///
    /// # Panics
    ///
    /// If `n` is greater than the number of bytes in `self`.
    #[inline]
    fn least_significant_bytes(&self, n: usize) -> &[u8] {
        match (&true, &(n <= Self::len_bytes())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        &self[(Self::len_bytes() - n)..]
    }
    fn to_low_u64_with_byteorder<B>(&self) -> u64
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        buf[(8 - capped)..].copy_from_slice(self.least_significant_bytes(capped));
        B::read_u64(&buf)
    }
    /// Returns the lowest 8 bytes interpreted as big-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_be(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as little-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_le(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as native-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_ne(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>()
    }
    fn from_low_u64_with_byteorder<B>(val: u64) -> Self
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        B::write_u64(&mut buf, val);
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        let mut bytes = [0x0; ::fixed_hash::core_::mem::size_of::<Self>()];
        bytes[(Self::len_bytes() - capped)..].copy_from_slice(&buf[..capped]);
        Self::from_slice(&bytes)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as big endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_be(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as little endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_le(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as native endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_ne(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>(val)
    }
}
impl ::fixed_hash::rand::distributions::Distribution<H384>
for ::fixed_hash::rand::distributions::Standard {
    fn sample<R: ::fixed_hash::rand::Rng + ?Sized>(&self, rng: &mut R) -> H384 {
        let mut ret = H384::zero();
        for byte in ret.as_bytes_mut().iter_mut() {
            *byte = rng.gen();
        }
        ret
    }
}
/// Utilities using the `rand` crate.
impl H384 {
    /// Assign `self` to a cryptographically random value using the
    /// given random number generator.
    pub fn randomize_using<R>(&mut self, rng: &mut R)
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        use ::fixed_hash::rand::distributions::Distribution;
        *self = ::fixed_hash::rand::distributions::Standard.sample(rng);
    }
    /// Assign `self` to a cryptographically random value.
    pub fn randomize(&mut self) {
        let mut rng = ::fixed_hash::rand::rngs::OsRng;
        self.randomize_using(&mut rng);
    }
    /// Create a new hash with cryptographically random content using the
    /// given random number generator.
    pub fn random_using<R>(rng: &mut R) -> Self
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        let mut ret = Self::zero();
        ret.randomize_using(rng);
        ret
    }
    /// Create a new hash with cryptographically random content.
    pub fn random() -> Self {
        let mut hash = Self::zero();
        hash.randomize();
        hash
    }
}
impl ::fixed_hash::core_::cmp::Ord for H384 {
    #[inline]
    fn cmp(&self, other: &Self) -> ::fixed_hash::core_::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl ::fixed_hash::core_::str::FromStr for H384 {
    type Err = ::fixed_hash::rustc_hex::FromHexError;
    /// Creates a hash type instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn from_str(
        input: &str,
    ) -> ::fixed_hash::core_::result::Result<
        H384,
        ::fixed_hash::rustc_hex::FromHexError,
    > {
        let input = input.strip_prefix("0x").unwrap_or(input);
        let mut iter = ::fixed_hash::rustc_hex::FromHexIter::new(input);
        let mut result = Self::zero();
        for byte in result.as_mut() {
            *byte = iter.next().ok_or(Self::Err::InvalidHexLength)??;
        }
        if iter.next().is_some() {
            return Err(Self::Err::InvalidHexLength);
        }
        Ok(result)
    }
}
#[repr(C)]
/// Fixed-size uninterpreted hash type with 64 bytes (512 bits) size.
pub struct H512(pub [u8; 64]);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for H512 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for H512 {
    #[inline]
    fn eq(&self, other: &H512) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for H512 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 64]>;
    }
}
impl From<[u8; 64]> for H512 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 64]) -> Self {
        H512(bytes)
    }
}
impl<'a> From<&'a [u8; 64]> for H512 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 64]) -> Self {
        H512(*bytes)
    }
}
impl<'a> From<&'a mut [u8; 64]> for H512 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 64]) -> Self {
        H512(*bytes)
    }
}
impl From<H512> for [u8; 64] {
    #[inline]
    fn from(s: H512) -> Self {
        s.0
    }
}
impl AsRef<[u8]> for H512 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<[u8]> for H512 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl H512 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> H512 {
        H512([byte; 64])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub const fn zero() -> H512 {
        H512::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        64
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> &[u8; 64] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; 64] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    pub const fn to_fixed_bytes(self) -> [u8; 64] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        match (&src.len(), &64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        self.as_bytes_mut().copy_from_slice(src);
    }
    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    pub fn from_slice(src: &[u8]) -> Self {
        match (&src.len(), &64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(b & self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}
impl ::fixed_hash::core_::fmt::Debug for H512 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("{0:#x}", self))
    }
}
impl ::fixed_hash::core_::fmt::Display for H512 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("0x"))?;
        for i in &self.0[0..2] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        f.write_fmt(format_args!("…"))?;
        for i in &self.0[64 - 2..64] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::LowerHex for H512 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0x"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::UpperHex for H512 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0X"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02X}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::marker::Copy for H512 {}
impl ::fixed_hash::core_::clone::Clone for H512 {
    fn clone(&self) -> H512 {
        *self
    }
}
impl ::fixed_hash::core_::cmp::PartialOrd for H512 {
    fn partial_cmp(&self, other: &Self) -> Option<::fixed_hash::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::fixed_hash::core_::hash::Hash for H512 {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::fixed_hash::core_::hash::Hasher,
    {
        state.write(&self.0);
    }
}
impl<I> ::fixed_hash::core_::ops::Index<I> for H512
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.as_bytes()[index]
    }
}
impl<I> ::fixed_hash::core_::ops::IndexMut<I> for H512
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8], Output = [u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.as_bytes_mut()[index]
    }
}
impl ::fixed_hash::core_::default::Default for H512 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}
impl<'r> ::fixed_hash::core_::ops::BitOrAssign<&'r H512> for H512 {
    fn bitor_assign(&mut self, rhs: &'r H512) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs |= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitOrAssign<H512> for H512 {
    #[inline]
    fn bitor_assign(&mut self, rhs: H512) {
        *self |= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitOr<&'r H512> for &'l H512 {
    type Output = H512;
    fn bitor(self, rhs: &'r H512) -> Self::Output {
        let mut ret = self.clone();
        ret |= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitOr<H512> for H512 {
    type Output = H512;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitAndAssign<&'r H512> for H512 {
    fn bitand_assign(&mut self, rhs: &'r H512) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs &= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitAndAssign<H512> for H512 {
    #[inline]
    fn bitand_assign(&mut self, rhs: H512) {
        *self &= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitAnd<&'r H512> for &'l H512 {
    type Output = H512;
    fn bitand(self, rhs: &'r H512) -> Self::Output {
        let mut ret = self.clone();
        ret &= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitAnd<H512> for H512 {
    type Output = H512;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitXorAssign<&'r H512> for H512 {
    fn bitxor_assign(&mut self, rhs: &'r H512) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs ^= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitXorAssign<H512> for H512 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: H512) {
        *self ^= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitXor<&'r H512> for &'l H512 {
    type Output = H512;
    fn bitxor(self, rhs: &'r H512) -> Self::Output {
        let mut ret = self.clone();
        ret ^= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitXor<H512> for H512 {
    type Output = H512;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
/// Utilities using the `byteorder` crate.
impl H512 {
    /// Returns the least significant `n` bytes as slice.
    ///
    /// # Panics
    ///
    /// If `n` is greater than the number of bytes in `self`.
    #[inline]
    fn least_significant_bytes(&self, n: usize) -> &[u8] {
        match (&true, &(n <= Self::len_bytes())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        &self[(Self::len_bytes() - n)..]
    }
    fn to_low_u64_with_byteorder<B>(&self) -> u64
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        buf[(8 - capped)..].copy_from_slice(self.least_significant_bytes(capped));
        B::read_u64(&buf)
    }
    /// Returns the lowest 8 bytes interpreted as big-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_be(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as little-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_le(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as native-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_ne(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>()
    }
    fn from_low_u64_with_byteorder<B>(val: u64) -> Self
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        B::write_u64(&mut buf, val);
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        let mut bytes = [0x0; ::fixed_hash::core_::mem::size_of::<Self>()];
        bytes[(Self::len_bytes() - capped)..].copy_from_slice(&buf[..capped]);
        Self::from_slice(&bytes)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as big endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_be(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as little endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_le(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as native endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_ne(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>(val)
    }
}
impl ::fixed_hash::rand::distributions::Distribution<H512>
for ::fixed_hash::rand::distributions::Standard {
    fn sample<R: ::fixed_hash::rand::Rng + ?Sized>(&self, rng: &mut R) -> H512 {
        let mut ret = H512::zero();
        for byte in ret.as_bytes_mut().iter_mut() {
            *byte = rng.gen();
        }
        ret
    }
}
/// Utilities using the `rand` crate.
impl H512 {
    /// Assign `self` to a cryptographically random value using the
    /// given random number generator.
    pub fn randomize_using<R>(&mut self, rng: &mut R)
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        use ::fixed_hash::rand::distributions::Distribution;
        *self = ::fixed_hash::rand::distributions::Standard.sample(rng);
    }
    /// Assign `self` to a cryptographically random value.
    pub fn randomize(&mut self) {
        let mut rng = ::fixed_hash::rand::rngs::OsRng;
        self.randomize_using(&mut rng);
    }
    /// Create a new hash with cryptographically random content using the
    /// given random number generator.
    pub fn random_using<R>(rng: &mut R) -> Self
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        let mut ret = Self::zero();
        ret.randomize_using(rng);
        ret
    }
    /// Create a new hash with cryptographically random content.
    pub fn random() -> Self {
        let mut hash = Self::zero();
        hash.randomize();
        hash
    }
}
impl ::fixed_hash::core_::cmp::Ord for H512 {
    #[inline]
    fn cmp(&self, other: &Self) -> ::fixed_hash::core_::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl ::fixed_hash::core_::str::FromStr for H512 {
    type Err = ::fixed_hash::rustc_hex::FromHexError;
    /// Creates a hash type instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn from_str(
        input: &str,
    ) -> ::fixed_hash::core_::result::Result<
        H512,
        ::fixed_hash::rustc_hex::FromHexError,
    > {
        let input = input.strip_prefix("0x").unwrap_or(input);
        let mut iter = ::fixed_hash::rustc_hex::FromHexIter::new(input);
        let mut result = Self::zero();
        for byte in result.as_mut() {
            *byte = iter.next().ok_or(Self::Err::InvalidHexLength)??;
        }
        if iter.next().is_some() {
            return Err(Self::Err::InvalidHexLength);
        }
        Ok(result)
    }
}
#[repr(C)]
/// Fixed-size uninterpreted hash type with 96 bytes (768 bits) size.
pub struct H768(pub [u8; 96]);
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for H768 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for H768 {
    #[inline]
    fn eq(&self, other: &H768) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for H768 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u8; 96]>;
    }
}
impl From<[u8; 96]> for H768 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 96]) -> Self {
        H768(bytes)
    }
}
impl<'a> From<&'a [u8; 96]> for H768 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 96]) -> Self {
        H768(*bytes)
    }
}
impl<'a> From<&'a mut [u8; 96]> for H768 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 96]) -> Self {
        H768(*bytes)
    }
}
impl From<H768> for [u8; 96] {
    #[inline]
    fn from(s: H768) -> Self {
        s.0
    }
}
impl AsRef<[u8]> for H768 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<[u8]> for H768 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl H768 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub const fn repeat_byte(byte: u8) -> H768 {
        H768([byte; 96])
    }
    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub const fn zero() -> H768 {
        H768::repeat_byte(0u8)
    }
    /// Returns the size of this hash in bytes.
    #[inline]
    pub const fn len_bytes() -> usize {
        96
    }
    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub const fn as_fixed_bytes(&self) -> &[u8; 96] {
        &self.0
    }
    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes_mut(&mut self) -> &mut [u8; 96] {
        &mut self.0
    }
    /// Returns the inner bytes array.
    #[inline]
    pub const fn to_fixed_bytes(self) -> [u8; 96] {
        self.0
    }
    /// Returns a constant raw pointer to the value.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
    /// Returns a mutable raw pointer to the value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_bytes_mut().as_mut_ptr()
    }
    /// Assign the bytes from the byte slice `src` to `self`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `self` do not match.
    pub fn assign_from_slice(&mut self, src: &[u8]) {
        match (&src.len(), &96) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        self.as_bytes_mut().copy_from_slice(src);
    }
    /// Create a new fixed-hash from the given slice `src`.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    ///
    /// # Panics
    ///
    /// If the length of `src` and the number of bytes in `Self` do not match.
    pub fn from_slice(src: &[u8]) -> Self {
        match (&src.len(), &96) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let mut ret = Self::zero();
        ret.assign_from_slice(src);
        ret
    }
    /// Returns `true` if all bits set in `b` are also set in `self`.
    #[inline]
    pub fn covers(&self, b: &Self) -> bool {
        &(b & self) == b
    }
    /// Returns `true` if no bits are set.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|&byte| byte == 0u8)
    }
}
impl ::fixed_hash::core_::fmt::Debug for H768 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("{0:#x}", self))
    }
}
impl ::fixed_hash::core_::fmt::Display for H768 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        f.write_fmt(format_args!("0x"))?;
        for i in &self.0[0..2] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        f.write_fmt(format_args!("…"))?;
        for i in &self.0[96 - 2..96] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::LowerHex for H768 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0x"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02x}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::fmt::UpperHex for H768 {
    fn fmt(
        &self,
        f: &mut ::fixed_hash::core_::fmt::Formatter,
    ) -> ::fixed_hash::core_::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("0X"))?;
        }
        for i in &self.0[..] {
            f.write_fmt(format_args!("{0:02X}", i))?;
        }
        Ok(())
    }
}
impl ::fixed_hash::core_::marker::Copy for H768 {}
impl ::fixed_hash::core_::clone::Clone for H768 {
    fn clone(&self) -> H768 {
        *self
    }
}
impl ::fixed_hash::core_::cmp::PartialOrd for H768 {
    fn partial_cmp(&self, other: &Self) -> Option<::fixed_hash::core_::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::fixed_hash::core_::hash::Hash for H768 {
    fn hash<H>(&self, state: &mut H)
    where
        H: ::fixed_hash::core_::hash::Hasher,
    {
        state.write(&self.0);
    }
}
impl<I> ::fixed_hash::core_::ops::Index<I> for H768
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;
    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.as_bytes()[index]
    }
}
impl<I> ::fixed_hash::core_::ops::IndexMut<I> for H768
where
    I: ::fixed_hash::core_::slice::SliceIndex<[u8], Output = [u8]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.as_bytes_mut()[index]
    }
}
impl ::fixed_hash::core_::default::Default for H768 {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}
impl<'r> ::fixed_hash::core_::ops::BitOrAssign<&'r H768> for H768 {
    fn bitor_assign(&mut self, rhs: &'r H768) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs |= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitOrAssign<H768> for H768 {
    #[inline]
    fn bitor_assign(&mut self, rhs: H768) {
        *self |= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitOr<&'r H768> for &'l H768 {
    type Output = H768;
    fn bitor(self, rhs: &'r H768) -> Self::Output {
        let mut ret = self.clone();
        ret |= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitOr<H768> for H768 {
    type Output = H768;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitAndAssign<&'r H768> for H768 {
    fn bitand_assign(&mut self, rhs: &'r H768) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs &= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitAndAssign<H768> for H768 {
    #[inline]
    fn bitand_assign(&mut self, rhs: H768) {
        *self &= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitAnd<&'r H768> for &'l H768 {
    type Output = H768;
    fn bitand(self, rhs: &'r H768) -> Self::Output {
        let mut ret = self.clone();
        ret &= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitAnd<H768> for H768 {
    type Output = H768;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<'r> ::fixed_hash::core_::ops::BitXorAssign<&'r H768> for H768 {
    fn bitxor_assign(&mut self, rhs: &'r H768) {
        for (lhs, rhs) in self.as_bytes_mut().iter_mut().zip(rhs.as_bytes()) {
            *lhs ^= rhs;
        }
    }
}
impl ::fixed_hash::core_::ops::BitXorAssign<H768> for H768 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: H768) {
        *self ^= &rhs;
    }
}
impl<'l, 'r> ::fixed_hash::core_::ops::BitXor<&'r H768> for &'l H768 {
    type Output = H768;
    fn bitxor(self, rhs: &'r H768) -> Self::Output {
        let mut ret = self.clone();
        ret ^= rhs;
        ret
    }
}
impl ::fixed_hash::core_::ops::BitXor<H768> for H768 {
    type Output = H768;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
/// Utilities using the `byteorder` crate.
impl H768 {
    /// Returns the least significant `n` bytes as slice.
    ///
    /// # Panics
    ///
    /// If `n` is greater than the number of bytes in `self`.
    #[inline]
    fn least_significant_bytes(&self, n: usize) -> &[u8] {
        match (&true, &(n <= Self::len_bytes())) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        &self[(Self::len_bytes() - n)..]
    }
    fn to_low_u64_with_byteorder<B>(&self) -> u64
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        buf[(8 - capped)..].copy_from_slice(self.least_significant_bytes(capped));
        B::read_u64(&buf)
    }
    /// Returns the lowest 8 bytes interpreted as big-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_be(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as little-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_le(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>()
    }
    /// Returns the lowest 8 bytes interpreted as native-endian.
    ///
    /// # Note
    ///
    /// For hash type with less than 8 bytes the missing bytes
    /// are interpreted as being zero.
    #[inline]
    pub fn to_low_u64_ne(&self) -> u64 {
        self.to_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>()
    }
    fn from_low_u64_with_byteorder<B>(val: u64) -> Self
    where
        B: ::fixed_hash::byteorder::ByteOrder,
    {
        let mut buf = [0x0; 8];
        B::write_u64(&mut buf, val);
        let capped = ::fixed_hash::core_::cmp::min(Self::len_bytes(), 8);
        let mut bytes = [0x0; ::fixed_hash::core_::mem::size_of::<Self>()];
        bytes[(Self::len_bytes() - capped)..].copy_from_slice(&buf[..capped]);
        Self::from_slice(&bytes)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as big endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_be(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::BigEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as little endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_le(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::LittleEndian>(val)
    }
    /// Creates a new hash type from the given `u64` value.
    ///
    /// # Note
    ///
    /// - The given `u64` value is interpreted as native endian.
    /// - Ignores the most significant bits of the given value
    ///   if the hash type has less than 8 bytes.
    #[inline]
    pub fn from_low_u64_ne(val: u64) -> Self {
        Self::from_low_u64_with_byteorder::<::fixed_hash::byteorder::NativeEndian>(val)
    }
}
impl ::fixed_hash::rand::distributions::Distribution<H768>
for ::fixed_hash::rand::distributions::Standard {
    fn sample<R: ::fixed_hash::rand::Rng + ?Sized>(&self, rng: &mut R) -> H768 {
        let mut ret = H768::zero();
        for byte in ret.as_bytes_mut().iter_mut() {
            *byte = rng.gen();
        }
        ret
    }
}
/// Utilities using the `rand` crate.
impl H768 {
    /// Assign `self` to a cryptographically random value using the
    /// given random number generator.
    pub fn randomize_using<R>(&mut self, rng: &mut R)
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        use ::fixed_hash::rand::distributions::Distribution;
        *self = ::fixed_hash::rand::distributions::Standard.sample(rng);
    }
    /// Assign `self` to a cryptographically random value.
    pub fn randomize(&mut self) {
        let mut rng = ::fixed_hash::rand::rngs::OsRng;
        self.randomize_using(&mut rng);
    }
    /// Create a new hash with cryptographically random content using the
    /// given random number generator.
    pub fn random_using<R>(rng: &mut R) -> Self
    where
        R: ::fixed_hash::rand::Rng + ?Sized,
    {
        let mut ret = Self::zero();
        ret.randomize_using(rng);
        ret
    }
    /// Create a new hash with cryptographically random content.
    pub fn random() -> Self {
        let mut hash = Self::zero();
        hash.randomize();
        hash
    }
}
impl ::fixed_hash::core_::cmp::Ord for H768 {
    #[inline]
    fn cmp(&self, other: &Self) -> ::fixed_hash::core_::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}
impl ::fixed_hash::core_::str::FromStr for H768 {
    type Err = ::fixed_hash::rustc_hex::FromHexError;
    /// Creates a hash type instance from the given string.
    ///
    /// # Note
    ///
    /// The given input string is interpreted in big endian.
    ///
    /// # Errors
    ///
    /// - When encountering invalid non hex-digits
    /// - Upon empty string input or invalid input length in general
    fn from_str(
        input: &str,
    ) -> ::fixed_hash::core_::result::Result<
        H768,
        ::fixed_hash::rustc_hex::FromHexError,
    > {
        let input = input.strip_prefix("0x").unwrap_or(input);
        let mut iter = ::fixed_hash::rustc_hex::FromHexIter::new(input);
        let mut result = Self::zero();
        for byte in result.as_mut() {
            *byte = iter.next().ok_or(Self::Err::InvalidHexLength)??;
        }
        if iter.next().is_some() {
            return Err(Self::Err::InvalidHexLength);
        }
        Ok(result)
    }
}
mod borsh {
    use super::*;
    use fixed_hash::alloc_::string::ToString;
    use impl_borsh::{impl_fixed_hash_borsh, impl_uint_borsh};
}
#[allow(unknown_lints, eq_op)]
const _: [(); 0
    - !{
        const ASSERT: bool = ::fixed_hash::core_::mem::size_of::<H160>()
            < ::fixed_hash::core_::mem::size_of::<H256>();
        ASSERT
    } as usize] = [];
impl From<H160> for H256 {
    fn from(value: H160) -> H256 {
        let large_ty_size = H256::len_bytes();
        let small_ty_size = H160::len_bytes();
        if true {
            if !(large_ty_size > small_ty_size && large_ty_size % 2 == 0
                && small_ty_size % 2 == 0)
            {
                ::core::panicking::panic(
                    "assertion failed: large_ty_size > small_ty_size && large_ty_size % 2 == 0 &&\n    small_ty_size % 2 == 0",
                )
            }
        }
        let mut ret = H256::zero();
        ret.as_bytes_mut()[(large_ty_size - small_ty_size)..large_ty_size]
            .copy_from_slice(value.as_bytes());
        ret
    }
}
impl From<H256> for H160 {
    fn from(value: H256) -> H160 {
        let large_ty_size = H256::len_bytes();
        let small_ty_size = H160::len_bytes();
        if true {
            if !(large_ty_size > small_ty_size && large_ty_size % 2 == 0
                && small_ty_size % 2 == 0)
            {
                ::core::panicking::panic(
                    "assertion failed: large_ty_size > small_ty_size && large_ty_size % 2 == 0 &&\n    small_ty_size % 2 == 0",
                )
            }
        }
        let mut ret = H160::zero();
        ret.as_bytes_mut()
            .copy_from_slice(&value[(large_ty_size - small_ty_size)..large_ty_size]);
        ret
    }
}
impl U128 {
    /// Multiplies two 128-bit integers to produce full 256-bit integer.
    /// Overflow is not possible.
    #[inline(always)]
    pub fn full_mul(self, other: U128) -> U256 {
        U256({
            {
                #![allow(unused_assignments)]
                let U128(ref me) = self;
                let U128(ref you) = other;
                let mut ret = [0u64; 2 * 2];
                use ::uint::unroll;
                #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                {
                    {
                        const i: usize = 0;
                        {
                            if i >= 0 {
                                let mut carry = 0u64;
                                let b = you[i];
                                #[allow(non_upper_case_globals)]
                                #[allow(unused_comparisons)]
                                {
                                    {
                                        const j: usize = 0;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 1;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 1;
                        {
                            if i >= 0 {
                                let mut carry = 0u64;
                                let b = you[i];
                                #[allow(non_upper_case_globals)]
                                #[allow(unused_comparisons)]
                                {
                                    {
                                        const j: usize = 0;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 1;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    };
                }
                ret
            }
        })
    }
}
impl U256 {
    /// Multiplies two 256-bit integers to produce full 512-bit integer.
    /// Overflow is not possible.
    #[inline(always)]
    pub fn full_mul(self, other: U256) -> U512 {
        U512({
            {
                #![allow(unused_assignments)]
                let U256(ref me) = self;
                let U256(ref you) = other;
                let mut ret = [0u64; 4 * 2];
                use ::uint::unroll;
                #[allow(non_upper_case_globals)] #[allow(unused_comparisons)]
                {
                    {
                        const i: usize = 0;
                        {
                            if i >= 0 {
                                let mut carry = 0u64;
                                let b = you[i];
                                #[allow(non_upper_case_globals)]
                                #[allow(unused_comparisons)]
                                {
                                    {
                                        const j: usize = 0;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 1;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 2;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 3;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 1;
                        {
                            if i >= 0 {
                                let mut carry = 0u64;
                                let b = you[i];
                                #[allow(non_upper_case_globals)]
                                #[allow(unused_comparisons)]
                                {
                                    {
                                        const j: usize = 0;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 1;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 2;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 3;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 2;
                        {
                            if i >= 0 {
                                let mut carry = 0u64;
                                let b = you[i];
                                #[allow(non_upper_case_globals)]
                                #[allow(unused_comparisons)]
                                {
                                    {
                                        const j: usize = 0;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 1;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 2;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 3;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    }
                    {
                        const i: usize = 0 + 3;
                        {
                            if i >= 0 {
                                let mut carry = 0u64;
                                let b = you[i];
                                #[allow(non_upper_case_globals)]
                                #[allow(unused_comparisons)]
                                {
                                    {
                                        const j: usize = 0;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 1;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 2;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    {
                                        const j: usize = 0 + 3;
                                        {
                                            if j >= 0 {
                                                if (|_, _| true)(me[j], carry) {
                                                    let a = me[j];
                                                    let (hi, low) = Self::split_u128(a as u128 * b as u128);
                                                    let overflow = {
                                                        let existing_low = &mut ret[i + j];
                                                        let (low, o) = low.overflowing_add(*existing_low);
                                                        *existing_low = low;
                                                        o
                                                    };
                                                    carry = {
                                                        let existing_hi = &mut ret[i + j + 1];
                                                        let hi = hi + overflow as u64;
                                                        let (hi, o0) = hi.overflowing_add(carry);
                                                        let (hi, o1) = hi.overflowing_add(*existing_hi);
                                                        *existing_hi = hi;
                                                        (o0 | o1) as u64
                                                    };
                                                }
                                            }
                                        }
                                    };
                                }
                            }
                        }
                    };
                }
                ret
            }
        })
    }
}
impl From<U256> for U512 {
    fn from(value: U256) -> U512 {
        let U256(ref arr) = value;
        let mut ret = [0; 8];
        ret[0] = arr[0];
        ret[1] = arr[1];
        ret[2] = arr[2];
        ret[3] = arr[3];
        U512(ret)
    }
}
impl TryFrom<U256> for U128 {
    type Error = Error;
    fn try_from(value: U256) -> Result<U128, Error> {
        let U256(ref arr) = value;
        if arr[2] | arr[3] != 0 {
            return Err(Error::Overflow);
        }
        let mut ret = [0; 2];
        ret[0] = arr[0];
        ret[1] = arr[1];
        Ok(U128(ret))
    }
}
impl TryFrom<U512> for U256 {
    type Error = Error;
    fn try_from(value: U512) -> Result<U256, Error> {
        let U512(ref arr) = value;
        if arr[4] | arr[5] | arr[6] | arr[7] != 0 {
            return Err(Error::Overflow);
        }
        let mut ret = [0; 4];
        ret[0] = arr[0];
        ret[1] = arr[1];
        ret[2] = arr[2];
        ret[3] = arr[3];
        Ok(U256(ret))
    }
}
impl TryFrom<U512> for U128 {
    type Error = Error;
    fn try_from(value: U512) -> Result<U128, Error> {
        let U512(ref arr) = value;
        if arr[2] | arr[3] | arr[4] | arr[5] | arr[6] | arr[7] != 0 {
            return Err(Error::Overflow);
        }
        let mut ret = [0; 2];
        ret[0] = arr[0];
        ret[1] = arr[1];
        Ok(U128(ret))
    }
}
impl From<U128> for U512 {
    fn from(value: U128) -> U512 {
        let U128(ref arr) = value;
        let mut ret = [0; 8];
        ret[0] = arr[0];
        ret[1] = arr[1];
        U512(ret)
    }
}
impl From<U128> for U256 {
    fn from(value: U128) -> U256 {
        let U128(ref arr) = value;
        let mut ret = [0; 4];
        ret[0] = arr[0];
        ret[1] = arr[1];
        U256(ret)
    }
}
impl<'a> From<&'a U256> for U512 {
    fn from(value: &'a U256) -> U512 {
        let U256(ref arr) = *value;
        let mut ret = [0; 8];
        ret[0] = arr[0];
        ret[1] = arr[1];
        ret[2] = arr[2];
        ret[3] = arr[3];
        U512(ret)
    }
}
impl<'a> TryFrom<&'a U512> for U256 {
    type Error = Error;
    fn try_from(value: &'a U512) -> Result<U256, Error> {
        let U512(ref arr) = *value;
        if arr[4] | arr[5] | arr[6] | arr[7] != 0 {
            return Err(Error::Overflow);
        }
        let mut ret = [0; 4];
        ret[0] = arr[0];
        ret[1] = arr[1];
        ret[2] = arr[2];
        ret[3] = arr[3];
        Ok(U256(ret))
    }
}
