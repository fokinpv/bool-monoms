mod monoms;

use monoms::MonomBits;
use monoms::Monom;
// use std::io::prelude::*;
// use std::io;
use std::fs;

fn main() {
    println!("Symbolic Boolean Expressions");
    println!("{}", std::u32::MAX);

    // let f = fs::File::open("data/monoms-42mln.txt").unwrap();
    // let f = io::BufReader::new(f);

    let contents = fs::read_to_string("data/monoms-42mln.txt")
        .expect("Something went wrong reading the file");

    let mut monoms: Vec<MonomBits> = Vec::new();

    // for line in f.lines() {
    for line in contents.lines() {
        // println!("{}", line);
        // let num = line.unwrap().parse::<u32>().unwrap();
        let num = line.parse::<u32>().unwrap();
        let monom = MonomBits::from_int(num);
        monoms.push(monom);
    }

    let mut res = MonomBits::one();
    // for monom in monoms {
    //     res = res * monom;
    // }
    for monom in monoms {
        res = res / monom;
    }
    println!("{}", res);
}
