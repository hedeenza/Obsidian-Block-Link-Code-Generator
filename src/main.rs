#![warn(clippy::pedantic)]
use rand::distr::{Alphanumeric, SampleString};
use std::env;

pub fn main() {
    let mut times: u32 = match env::args().nth(1) {
        Some(number) => number.trim().parse().expect("Could not parse number"),
        None => 1,
    };
    let code_length = 6;
    while times > 0 {
        //Use the rand crate's random string generator
        let code = Alphanumeric.sample_string(&mut rand::rng(), code_length);
        //Print the string
        println!("^{}", code.to_lowercase());
        times -= 1;
    }
}
