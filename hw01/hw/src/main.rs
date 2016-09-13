extern crate hw;

use hw::problem3::sieve;
use hw::problem4::{hanoi, Peg};
use hw::problem5::{bloom, jenkins, fnv, djb2};

pub fn main() {
    println!("{:?}", sieve(12));
    hanoi(5, Peg::A, Peg::B, Peg::C);

    let data = vec!["apple", "blueberry", "carrot", "date", "eggplant",
        "fig", "grapefruit"];
    let hashes = [djb2, fnv, jenkins];
    bloom(&data, hashes, "carrot");
    bloom(&data, hashes, "milk");
    bloom(&data, hashes, "bread");
}
