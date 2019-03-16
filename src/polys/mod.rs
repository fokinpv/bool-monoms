use crate::monoms::*;

#[derive(Debug)]
pub struct Poly {
    pub monoms: Vec<MonomBits>,
}
impl Poly {
    pub fn new() -> Self {
        Poly {
            monoms: vec![],
        }
    }
    pub fn from_monom(m: MonomBits) -> Self {
        Poly {
            monoms: vec![m],
        }
    }
    pub fn from_vec(monoms: Vec<MonomBits>) -> Self {
        Poly {
            monoms: monoms,
        }
    }
    pub fn is_one(&self) -> bool {
        self.monoms.len() == 1 && self.monoms[0].is_one()
    }
    pub fn is_zero(&self) -> bool {
        self.monoms.is_empty()
    }
    pub fn one() -> Self {
        Poly {
            monoms: vec![MonomBits::one()],
        }
    }
    pub fn zero() -> Self {
        Poly::new()
    }
}

#[cfg(test)]
#[test]
fn test_new() {
    let poly = Poly::new();
    assert!(poly.monoms.is_empty());
}
#[test]
fn test_one() {
    let poly = Poly::one();
    assert!(poly.is_one());
    assert!(!poly.is_zero());
}
#[test]
fn test_zero() {
    let poly = Poly::zero();
    assert!(poly.is_zero());
    assert!(!poly.is_one());
}
#[test]
fn test_from_monom() {
    let monom = MonomBits::from(0);
    let poly = Poly::from_monom(monom);
    println!("{:?}", poly.monoms[0]);
}
#[test]
fn test_from_vec() {
    let monoms = vec![MonomBits::from(0), MonomBits::from(2)];
    let poly = Poly::from_vec(monoms);
    println!("{:?}", poly);
}
