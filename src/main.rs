mod monoms;

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
