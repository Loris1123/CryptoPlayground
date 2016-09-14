extern crate getopts;

use std::env;
use getopts::Options;

mod lfsr;
mod feistel;
mod gcd;

fn main(){
    println!("Welcome to CryptoPlayground");
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this helptext");
    opts.optflag("", "feistel", "Start Feistel encryption");
    opts.optflag("", "lfsr", "Start LFSR");
    opts.optflag("", "gcd", "Calculate the greatest common divisor of two numbers");

    let matches = match opts.parse(&args[1..]){
        Ok(m) => m,
        Err(f) => panic!("Error: {}",f),
    };

    if matches.opt_present("h"){
        print!("{}", opts.usage(""));
    }
    if matches.opt_present("feistel"){
        feistel::encrypt();
    }
    if matches.opt_present("lfsr"){
        lfsr::lfsr(15);
    }
    if matches.opt_present("gcd"){
        gcd::calculate_gcd(146,12);
    }
}
