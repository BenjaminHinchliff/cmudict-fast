#![feature(alloc_system)]
extern crate alloc_system;

extern crate cmudict;
use cmudict::Cmudict;

fn main() {
    let dict = Cmudict::download().expect("Couldn't get dict");
}

