use std::convert::From;
use std::fmt;
use std::ops;
use std::cmp;

pub trait Monom {
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    fn is_divisible(&self, other: &Self) -> bool;
}

#[derive(Debug)]
pub struct MonomBits {
    is_zero: bool,
    is_one: bool,
    bits: u32,
}

impl Monom for MonomBits {
    fn is_zero(&self) -> bool {
        return self.is_zero
    }
    fn is_one(&self) -> bool {
        return self.is_one
    }
    fn zero() -> Self {
        MonomBits {
            is_zero: true,
            is_one: false,
            bits: 0
        }
    }
    fn one() -> Self {
        MonomBits {
            is_zero: false,
            is_one: true,
            bits: 0
        }
    }
    fn is_divisible(&self, other: &Self) -> bool {
        self.bits == self.bits | other.bits
    }
}
impl MonomBits {
    fn new() -> MonomBits {
        MonomBits {
            is_zero: false,
            is_one: false,
            bits: 0
        }
    }

    fn set_var(&mut self, var: usize) {
        self.bits |= 1 << var;
    }
}
impl From<usize> for MonomBits {
    fn from(var: usize) -> Self {
        let mut monom = MonomBits::new();
        monom.set_var(var);
        return monom
    }
}
impl From<Vec<usize>> for MonomBits {
    fn from(vars: Vec<usize>) -> Self {
        let mut monom = MonomBits::new();
        for var in vars {
            monom.set_var(var);
        }
        return monom
    }
}
impl cmp::PartialEq for MonomBits {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}
impl ops::Mul for MonomBits {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut res = MonomBits::new();
        res.bits = self.bits | rhs.bits;
        return res
    }
}
impl ops::Div for MonomBits {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if !self.is_divisible(&rhs) {
            return MonomBits::zero()
        }
        let mut res = MonomBits::new();
        res.bits = self.bits ^ rhs.bits;
        return res
    }
}
impl fmt::Display for MonomBits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0")
        }
        if self.is_one() {
            return write!(f, "1")
        }
        let vars: Vec<usize> = format!("{:032b}", self.bits)
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c != '0')
            .map(|(i, _)| i)
            .collect();

        write!(f, "{:?}", vars)
    }
}

#[cfg(test)]
mod tests;
