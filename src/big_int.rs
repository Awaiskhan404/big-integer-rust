use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct BigInt {
    pub digits: Vec<u32>,
    pub sign: i8,
}

impl BigInt {
    pub fn new(n: i64) -> BigInt {
        let mut sign = 1;
        let mut num = n;
        if num < 0 {
            sign = -1;
            num = -num;
        }
        BigInt {
            digits: num
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect(),
            sign,
        }
    }

    pub fn from_digits(digits: Vec<u32>, sign: i8) -> BigInt {
        BigInt { digits, sign }
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        // Implement addition of two big integers here
        unimplemented!()
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, other: BigInt) -> BigInt {
        // Implement subtraction of two big integers here
        unimplemented!()
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, other: BigInt) -> BigInt {
        // Implement multiplication of two big integers here
        unimplemented!()
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, other: BigInt) -> BigInt {
        // Implement division of two big integers here
        unimplemented!()
    }
}

impl FromStr for BigInt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = if s.starts_with('-') {
            -1
        } else {
            1
        };
        let digits = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        Ok(BigInt::from_digits(digits, sign))
    }
}
