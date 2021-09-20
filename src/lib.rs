use std::ops::{Add, Div, Mul, Sub};

mod utils;

trait Fract<B, S, O> {
    fn to_float(&self) -> O;
    fn new(numerator: B, denominator: B) -> S;
    fn invert(&self) -> S;
    fn expand(&self, multiplicator: B) -> S;
    fn reduce(&self) -> S;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fract8 {
    pub numerator: u8,
    pub denominator: u8,
}

impl Fract<u8, Fract8, f32> for Fract8 {
    #[inline]
    fn to_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    #[inline]
    fn new(numerator: u8, denominator: u8) -> Fract8 {
        Fract8 {
            numerator: numerator,
            denominator: denominator,
        }
    }

    #[inline]
    fn invert(&self) -> Fract8 {
        Fract8 {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    #[inline]
    fn expand(&self, multiplicator: u8) -> Fract8 {
        Fract8 {
            numerator: self.numerator * multiplicator,
            denominator: self.denominator * multiplicator,
        }
    }

    #[inline]
    fn reduce(&self) -> Fract8 {
        let gcd: u8 = utils::gcd_u8(self.numerator, self.denominator);
        Fract8 {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl From<u8> for Fract8 {
    #[inline]
    fn from(input: u8) -> Self {
        Fract8 {
            numerator: input,
            denominator: 1,
        }
    }
}

impl Add for Fract8 {
    type Output = Fract8;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract8 = self;
        let mut nrhs: Fract8 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u8 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract8 {
            numerator: nlhs.numerator + nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Sub for Fract8 {
    type Output = Fract8;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract8 = self;
        let mut nrhs: Fract8 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u8 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract8 {
            numerator: nlhs.numerator - nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Mul for Fract8 {
    type Output = Fract8;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Fract8 {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Fract8 {
    type Output = Fract8;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.invert()
    }
}
#[cfg(test)]
mod tests_fract8 {
    use assert_approx_eq::assert_approx_eq;

    use crate::{Fract, Fract8};

    #[test]
    fn should_create() {
        let expected: Fract8 = Fract8 {
            numerator: 8,
            denominator: 10,
        };

        let actual: Fract8 = Fract8::new(8, 10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_invert() {
        let expected: Fract8 = Fract8 {
            numerator: 10,
            denominator: 8,
        };

        let actual: Fract8 = Fract8::new(8, 10).invert();

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_expand() {
        let expected: Fract8 = Fract8 {
            numerator: 80,
            denominator: 100,
        };

        let actual: Fract8 = Fract8::new(8, 10).expand(10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_convert() {
        let expected: f32 = 0.8;
        let actual: f32 = Fract8::new(8, 10).to_float();

        assert_approx_eq!(expected, actual)
    }

    #[test]
    fn should_add() {
        let expected: Fract8 = Fract8 {
            numerator: 28,
            denominator: 20,
        };

        let first: Fract8 = Fract8::new(1, 2);
        let second: Fract8 = Fract8::new(9, 10);
        let result: Fract8 = first + second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_sub() {
        let expected: Fract8 = Fract8 {
            numerator: 22,
            denominator: 20,
        };

        let first: Fract8 = Fract8::new(4, 2);
        let second: Fract8 = Fract8::new(9, 10);
        let result: Fract8 = first - second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_mul() {
        let expected: Fract8 = Fract8 {
            numerator: 8,
            denominator: 10,
        };

        let first: Fract8 = Fract8::new(2, 5);
        let second: Fract8 = Fract8::new(4, 2);
        let result: Fract8 = first * second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_div() {
        let expected: Fract8 = Fract8 {
            numerator: 10,
            denominator: 18,
        };

        let first: Fract8 = Fract8::new(1, 2);
        let second: Fract8 = Fract8::new(9, 10);
        let result: Fract8 = first / second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_reduce() {
        let expected: Fract8 = Fract8 {
            numerator: 5,
            denominator: 9,
        };

        let value: Fract8 = Fract8 {
            numerator: 10,
            denominator: 18,
        };

        assert_eq!(expected, value.reduce())
    }
}


// Fract16
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fract16 {
    pub numerator: u16,
    pub denominator: u16,
}

impl Fract<u16, Fract16, f32> for Fract16 {
    #[inline]
    fn to_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    #[inline]
    fn new(numerator: u16, denominator: u16) -> Fract16 {
        Fract16 {
            numerator: numerator,
            denominator: denominator,
        }
    }

    #[inline]
    fn invert(&self) -> Fract16 {
        Fract16 {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    #[inline]
    fn expand(&self, multiplicator: u16) -> Fract16 {
        Fract16 {
            numerator: self.numerator * multiplicator,
            denominator: self.denominator * multiplicator,
        }
    }

    #[inline]
    fn reduce(&self) -> Fract16 {
        let gcd: u16 = utils::gcd_u16(self.numerator, self.denominator);
        Fract16 {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl From<u16> for Fract16 {
    #[inline]
    fn from(input: u16) -> Self {
        Fract16 {
            numerator: input,
            denominator: 1,
        }
    }
}

impl Add for Fract16 {
    type Output = Fract16;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract16 = self;
        let mut nrhs: Fract16 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u16 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract16 {
            numerator: nlhs.numerator + nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Sub for Fract16 {
    type Output = Fract16;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract16 = self;
        let mut nrhs: Fract16 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u16 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract16 {
            numerator: nlhs.numerator - nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Mul for Fract16 {
    type Output = Fract16;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Fract16 {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Fract16 {
    type Output = Fract16;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.invert()
    }
}
#[cfg(test)]
mod tests_fract16 {
    use assert_approx_eq::assert_approx_eq;

    use crate::{Fract, Fract16};

    #[test]
    fn should_create() {
        let expected: Fract16 = Fract16 {
            numerator: 8,
            denominator: 10,
        };

        let actual: Fract16 = Fract16::new(8, 10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_invert() {
        let expected: Fract16 = Fract16 {
            numerator: 10,
            denominator: 8,
        };

        let actual: Fract16 = Fract16::new(8, 10).invert();

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_expand() {
        let expected: Fract16 = Fract16 {
            numerator: 80,
            denominator: 100,
        };

        let actual: Fract16 = Fract16::new(8, 10).expand(10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_convert() {
        let expected: f32 = 0.8;
        let actual: f32 = Fract16::new(8, 10).to_float();

        assert_approx_eq!(expected, actual)
    }

    #[test]
    fn should_add() {
        let expected: Fract16 = Fract16 {
            numerator: 28,
            denominator: 20,
        };

        let first: Fract16 = Fract16::new(1, 2);
        let second: Fract16 = Fract16::new(9, 10);
        let result: Fract16 = first + second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_sub() {
        let expected: Fract16 = Fract16 {
            numerator: 22,
            denominator: 20,
        };

        let first: Fract16 = Fract16::new(4, 2);
        let second: Fract16 = Fract16::new(9, 10);
        let result: Fract16 = first - second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_mul() {
        let expected: Fract16 = Fract16 {
            numerator: 8,
            denominator: 10,
        };

        let first: Fract16 = Fract16::new(2, 5);
        let second: Fract16 = Fract16::new(4, 2);
        let result: Fract16 = first * second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_div() {
        let expected: Fract16 = Fract16 {
            numerator: 10,
            denominator: 18,
        };

        let first: Fract16 = Fract16::new(1, 2);
        let second: Fract16 = Fract16::new(9, 10);
        let result: Fract16 = first / second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_reduce() {
        let expected: Fract16 = Fract16 {
            numerator: 5,
            denominator: 9,
        };

        let value: Fract16 = Fract16 {
            numerator: 10,
            denominator: 18,
        };

        assert_eq!(expected, value.reduce())
    }
}

// Fract32
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fract32 {
    pub numerator: u32,
    pub denominator: u32,
}

impl Fract<u32, Fract32, f32> for Fract32 {
    #[inline]
    fn to_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    #[inline]
    fn new(numerator: u32, denominator: u32) -> Fract32 {
        Fract32 {
            numerator: numerator,
            denominator: denominator,
        }
    }

    #[inline]
    fn invert(&self) -> Fract32 {
        Fract32 {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    #[inline]
    fn expand(&self, multiplicator: u32) -> Fract32 {
        Fract32 {
            numerator: self.numerator * multiplicator,
            denominator: self.denominator * multiplicator,
        }
    }

    #[inline]
    fn reduce(&self) -> Fract32 {
        let gcd: u32 = utils::gcd_u32(self.numerator, self.denominator);
        Fract32 {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl From<u32> for Fract32 {
    #[inline]
    fn from(input: u32) -> Self {
        Fract32 {
            numerator: input,
            denominator: 1,
        }
    }
}

impl Add for Fract32 {
    type Output = Fract32;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract32 = self;
        let mut nrhs: Fract32 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u32 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract32 {
            numerator: nlhs.numerator + nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Sub for Fract32 {
    type Output = Fract32;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract32 = self;
        let mut nrhs: Fract32 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u32 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract32 {
            numerator: nlhs.numerator - nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Mul for Fract32 {
    type Output = Fract32;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Fract32 {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Fract32 {
    type Output = Fract32;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.invert()
    }
}
#[cfg(test)]
mod tests_Fract32 {
    use assert_approx_eq::assert_approx_eq;

    use crate::{Fract, Fract32};

    #[test]
    fn should_create() {
        let expected: Fract32 = Fract32 {
            numerator: 8,
            denominator: 10,
        };

        let actual: Fract32 = Fract32::new(8, 10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_invert() {
        let expected: Fract32 = Fract32 {
            numerator: 10,
            denominator: 8,
        };

        let actual: Fract32 = Fract32::new(8, 10).invert();

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_expand() {
        let expected: Fract32 = Fract32 {
            numerator: 80,
            denominator: 100,
        };

        let actual: Fract32 = Fract32::new(8, 10).expand(10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_convert() {
        let expected: f32 = 0.8;
        let actual: f32 = Fract32::new(8, 10).to_float();

        assert_approx_eq!(expected, actual)
    }

    #[test]
    fn should_add() {
        let expected: Fract32 = Fract32 {
            numerator: 28,
            denominator: 20,
        };

        let first: Fract32 = Fract32::new(1, 2);
        let second: Fract32 = Fract32::new(9, 10);
        let result: Fract32 = first + second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_sub() {
        let expected: Fract32 = Fract32 {
            numerator: 22,
            denominator: 20,
        };

        let first: Fract32 = Fract32::new(4, 2);
        let second: Fract32 = Fract32::new(9, 10);
        let result: Fract32 = first - second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_mul() {
        let expected: Fract32 = Fract32 {
            numerator: 8,
            denominator: 10,
        };

        let first: Fract32 = Fract32::new(2, 5);
        let second: Fract32 = Fract32::new(4, 2);
        let result: Fract32 = first * second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_div() {
        let expected: Fract32 = Fract32 {
            numerator: 10,
            denominator: 18,
        };

        let first: Fract32 = Fract32::new(1, 2);
        let second: Fract32 = Fract32::new(9, 10);
        let result: Fract32 = first / second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_reduce() {
        let expected: Fract32 = Fract32 {
            numerator: 5,
            denominator: 9,
        };

        let value: Fract32 = Fract32 {
            numerator: 10,
            denominator: 18,
        };

        assert_eq!(expected, value.reduce())
    }
}

// Fract64
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fract64 {
    pub numerator: u64,
    pub denominator: u64,
}

impl Fract<u64, Fract64, f64> for Fract64 {
    #[inline]
    fn to_float(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    #[inline]
    fn new(numerator: u64, denominator: u64) -> Fract64 {
        Fract64 {
            numerator: numerator,
            denominator: denominator,
        }
    }

    #[inline]
    fn invert(&self) -> Fract64 {
        Fract64 {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    #[inline]
    fn expand(&self, multiplicator: u64) -> Fract64 {
        Fract64 {
            numerator: self.numerator * multiplicator,
            denominator: self.denominator * multiplicator,
        }
    }

    #[inline]
    fn reduce(&self) -> Fract64 {
        let gcd: u64 = utils::gcd_u64(self.numerator, self.denominator);
        Fract64 {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl From<u64> for Fract64 {
    #[inline]
    fn from(input: u64) -> Self {
        Fract64 {
            numerator: input,
            denominator: 1,
        }
    }
}

impl Add for Fract64 {
    type Output = Fract64;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract64 = self;
        let mut nrhs: Fract64 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u64 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract64 {
            numerator: nlhs.numerator + nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Sub for Fract64 {
    type Output = Fract64;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut nlhs: Fract64 = self;
        let mut nrhs: Fract64 = rhs;

        if self.denominator != rhs.denominator {
            let old_denom: u64 = nlhs.denominator;
            nlhs = nlhs.expand(nrhs.denominator);
            nrhs = nrhs.expand(old_denom);
        }

        Fract64 {
            numerator: nlhs.numerator - nrhs.numerator,
            denominator: nlhs.denominator,
        }
    }
}

impl Mul for Fract64 {
    type Output = Fract64;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Fract64 {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Fract64 {
    type Output = Fract64;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.invert()
    }
}
#[cfg(test)]
mod tests_Fract64 {
    use assert_approx_eq::assert_approx_eq;

    use crate::{Fract, Fract64};

    #[test]
    fn should_create() {
        let expected: Fract64 = Fract64 {
            numerator: 8,
            denominator: 10,
        };

        let actual: Fract64 = Fract64::new(8, 10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_invert() {
        let expected: Fract64 = Fract64 {
            numerator: 10,
            denominator: 8,
        };

        let actual: Fract64 = Fract64::new(8, 10).invert();

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_expand() {
        let expected: Fract64 = Fract64 {
            numerator: 80,
            denominator: 100,
        };

        let actual: Fract64 = Fract64::new(8, 10).expand(10);

        assert_eq!(expected, actual)
    }

    #[test]
    fn should_convert() {
        let expected: f64 = 0.8;
        let actual: f64 = Fract64::new(8, 10).to_float();

        assert_approx_eq!(expected, actual)
    }

    #[test]
    fn should_add() {
        let expected: Fract64 = Fract64 {
            numerator: 28,
            denominator: 20,
        };

        let first: Fract64 = Fract64::new(1, 2);
        let second: Fract64 = Fract64::new(9, 10);
        let result: Fract64 = first + second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_sub() {
        let expected: Fract64 = Fract64 {
            numerator: 22,
            denominator: 20,
        };

        let first: Fract64 = Fract64::new(4, 2);
        let second: Fract64 = Fract64::new(9, 10);
        let result: Fract64 = first - second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_mul() {
        let expected: Fract64 = Fract64 {
            numerator: 8,
            denominator: 10,
        };

        let first: Fract64 = Fract64::new(2, 5);
        let second: Fract64 = Fract64::new(4, 2);
        let result: Fract64 = first * second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_div() {
        let expected: Fract64 = Fract64 {
            numerator: 10,
            denominator: 18,
        };

        let first: Fract64 = Fract64::new(1, 2);
        let second: Fract64 = Fract64::new(9, 10);
        let result: Fract64 = first / second;

        assert_eq!(expected, result)
    }

    #[test]
    fn should_reduce() {
        let expected: Fract64 = Fract64 {
            numerator: 5,
            denominator: 9,
        };

        let value: Fract64 = Fract64 {
            numerator: 10,
            denominator: 18,
        };

        assert_eq!(expected, value.reduce())
    }
}