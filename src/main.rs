mod lfsr;
mod feistel;

fn main(){
    println!("Welcome to CryptoPlayground");
    println!("Running LFSR");
    lfsr::lfsr();

    println!("Running Feistel");
    feistel::feistel();
}
