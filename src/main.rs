#![warn(clippy::pedantic)]
use rand::distr::{Alphanumeric, SampleString};
use std::env;

pub fn main() {
    let mut times: u32 = set_number_of_outputs(1);
    let code_length: usize = set_code_length(2);
    while times > 0 {
        //Use the rand crate's random string generator
        let code = Alphanumeric.sample_string(&mut rand::rng(), code_length);
        //Print the string
        println!("^{}", code.to_lowercase());
        times -= 1;
    }
}

fn set_number_of_outputs(cli_arg: usize) -> u32 {
    match env::args().nth(cli_arg) {
        Some(number) => number.trim().parse().expect("Could not parse NUMBER OF CODES"),
        None => 1,
    }
}

fn set_code_length(cli_arg: usize) -> usize {
    match env::args().nth(cli_arg) {
        Some(number) => number.trim().parse().expect("Could not parse CODE LENGTH"),
        None => 6,
    }
}
