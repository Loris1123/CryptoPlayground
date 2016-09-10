extern crate rand;

use rand::Rng;

fn main(){
    println!("Welcome to CryptoPlayground");
    println!("Start LFSR");
    lfsr();
}

fn lfsr(){
    // Create the 7-bit-register
    let mut register = [0; 7];

    // Initialize it with values between 0 and 1
    for (i,j) in register.iter_mut().enumerate() {
        *j = rand::thread_rng().gen_range(0, 2);
    }
    println!("Register: {:?}", register);

    
}
