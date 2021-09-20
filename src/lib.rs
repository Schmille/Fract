use std::ops::{Add, Div, Mul, Sub};

mod utils;

trait Fract<B, S, O> {
    fn to_float(&self) -> O;
    fn new(numerator: B, denominator: B) -> S;
    fn invert(&self) -> S;
    fn expand(&self, multiplicator: u8) -> S;
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
