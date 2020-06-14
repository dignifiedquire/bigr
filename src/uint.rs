use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::algorithms::*;

pub type Uint256 = Uint<4>;

pub type Digit = u64;
pub const DIGIT_BITS: usize = 64;

#[derive(Clone, Copy)]
pub struct Uint<const N: usize> {
    data: [Digit; N],
}

impl<const N: usize> Default for Uint<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> PartialEq for Uint<N> {
    fn eq(&self, other: &Self) -> bool {
        self.is_eq(other)
    }
}

impl<const N: usize> Eq for Uint<N> {}

impl<const N: usize> core::fmt::Debug for Uint<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x")?;
        for &b in self.data.iter().rev() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl<const N: usize> From<Digit> for Uint<N> {
    fn from(val: Digit) -> Self {
        Self::from_digit(val)
    }
}

impl<const N: usize> Add for Uint<N> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        (&mut self).add_assign(&other);
        self
    }
}

impl<const N: usize> AddAssign for Uint<N> {
    fn add_assign(&mut self, other: Self) {
        self.add_assign(&other);
    }
}

impl<const N: usize> Sub for Uint<N> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        (&mut self).sub_assign(&other);
        self
    }
}

impl<const N: usize> SubAssign for Uint<N> {
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other);
    }
}

impl<const N: usize> Mul for Uint<N> {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        (&mut self).mul_assign(&other);
        self
    }
}

impl<const N: usize> MulAssign for Uint<N> {
    fn mul_assign(&mut self, other: Self) {
        self.mul_assign(&other);
    }
}
impl<const N: usize> Uint<N> {
    pub const fn zero() -> Self {
        Self { data: [0; N] }
    }

    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < N {
            if self.data[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    pub const fn one() -> Self {
        Self::from_digit(1)
    }

    pub const fn from_digit(digit: Digit) -> Self {
        let mut data = [0; N];
        data[0] = digit;
        Self { data }
    }

    /// Adds `other` from this number, storing the result in `self`.
    ///
    /// # Example
    /// ```rust
    /// use bigr::Uint;
    ///
    /// let mut x: Uint<1> = 12.into();
    /// x.add_assign(&14.into());
    ///
    /// assert_eq!(x, 26.into());
    /// ```
    pub const fn add_assign(&mut self, other: &Self) {
        let mut carry = 0;

        let mut i = 0;
        while i < N {
            let (res, c) = adc(self.data[i] as u64, other.data[i] as u64, carry);
            self.data[i] = res;
            carry = c;
            i += 1;
        }

        // debug_assert_eq!(carry, 0);
    }

    /// Subtracts `other` from this number, storing the result in `self`.
    ///
    /// # Example
    /// ```rust
    /// use bigr::Uint;
    ///
    /// let mut x: Uint<1> = 14.into();
    /// x.sub_assign(&12.into());
    ///
    /// assert_eq!(x, 2.into());
    /// ```
    pub const fn sub_assign(&mut self, other: &Self) {
        let mut borrow = 0;

        let mut i = 0;
        while i < N {
            let (res, b) = sbb(self.data[i], other.data[i], borrow);
            self.data[i] = res;
            borrow = b;
            i += 1;
        }

        // debug_assert_eq!(borrow, 0);
    }

    /// Multiplies this number with `other`, storing the result in `self`.
    ///
    /// # Example
    /// ```rust
    /// use bigr::Uint;
    ///
    /// let mut x: Uint<1> = 12.into();
    /// x.mul_assign(&14.into());
    ///
    /// assert_eq!(x, 168.into());
    /// ```
    pub const fn mul_assign(&mut self, other: &Self) {
        let mut carry = 0;

        let mut i = 0;
        while i < N {
            let (res, c) = mac(0, self.data[i], other.data[i], carry);
            carry = c;
            self.data[i] = res;
            i += 1;
        }
    }

    /// Returns `true` if this number is equal to `other`, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use bigr::Uint;
    ///
    /// let mut x: Uint<1> = 12.into();
    ///
    /// assert!(!x.is_eq(&26.into()));
    /// ```
    pub const fn is_eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < N {
            if self.data[i] != other.data[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns `true` if this number is even, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use bigr::Uint;
    ///
    /// let mut x: Uint<1> = 12.into();
    ///
    /// assert!(x.is_even());
    /// ```
    pub const fn is_even(&self) -> bool {
        !self.is_odd()
    }

    /// Returns `true` if this number is odd, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use bigr::Uint;
    ///
    /// let mut x: Uint<1> = 13.into();
    ///
    /// assert!(x.is_odd());
    /// ```
    pub const fn is_odd(&self) -> bool {
        self.data[0] & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uint256() {
        let one: Uint256 = 1.into();
        let two: Uint256 = 2.into();
        let three: Uint256 = 3.into();
        let four: Uint256 = 4.into();

        assert_eq!(one + two, three);

        assert!(one.is_odd());
        assert!(two.is_even());
        assert!(three.is_odd());

        assert_eq!(two - one, one);
        assert_eq!(three - one, two);

        assert_eq!(two * two, four);

        assert!(two.is_eq(&two));
    }
}
