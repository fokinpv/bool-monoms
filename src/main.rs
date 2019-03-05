mod monoms {
    use std::convert::From;
    use std::fmt;
    use std::ops;

    pub struct MonomOne;
    // impl MonomOne {
    //     #[allow(dead_code)]
    //     fn is_one(&self) -> bool {
    //         return true;
    //     }
    // }
    impl fmt::Display for MonomOne {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "1")
        }
    }

    pub struct MonomZero;
    // impl MonomZero {
    //     #[allow(dead_code)]
    //     fn is_zero(&self) -> bool {
    //         return true;
    //     }
    // }
    impl fmt::Display for MonomZero {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "0")
        }
    }

    pub trait Monom {
        type Zero;
        type One;

        fn zero() -> Self::Zero;
        fn one() -> Self::One;
        fn is_zero(&self) -> bool;
        fn is_divisible(&self, other: &Self) -> bool;
        // fn is_one(&self) -> bool {
        //     false
        // }
    }
    #[derive(Debug)]
    pub struct MonomBits {
        bits: u32,
    }

    impl Monom for MonomBits {
        type Zero = MonomZero;
        type One = MonomOne;

        fn is_zero(&self) -> bool {
            return self.bits == 0
        }
        fn zero() -> MonomZero {
            MonomZero
        }
        fn one() -> MonomOne {
            MonomOne
        }
        fn is_divisible(&self, other: &Self) -> bool {
            self.bits == self.bits | other.bits
        }
    }
    impl MonomBits {
        fn new() -> MonomBits {
            MonomBits { bits: 0 }
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
                return MonomBits::new()
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

}

use monoms::Monom;
use monoms::MonomBits;

fn main() {
    let a1 = MonomBits::from(0);
    let b1 = MonomBits::from(1);
    let a2 = MonomBits::from(0);
    let b2 = MonomBits::from(1);
    let m3 = MonomBits::from(vec![0, 12, 31]);
    let m4 = MonomBits::zero();
    let m5 = MonomBits::one();
    let ab = a1*b1;
    let ab2 = a2/b2;
    println!("{}", ab);
    println!("{}", ab2);
    // println!("{}", m2);
    println!("{}", m3);
    println!("{}", m4);
    println!("{}", m5);
}
