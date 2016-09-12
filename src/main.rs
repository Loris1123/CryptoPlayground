use std::env;

mod lfsr;
mod feistel;

fn main(){
    println!("Welcome to CryptoPlayground");
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        print_helptext();
        return;
    }

    for arg in &args[1..]{
        match arg.as_ref() {
            "--lfsr" => lfsr::lfsr(),
            "--feistel" => feistel::feistel(),
            "--help" => print_helptext(),
            _ => print_helptext(),
        }
    }
}

fn print_helptext(){
    println!("CryptoPlayground");
    println!("");
    println!("Specify what you want to do by choosing one of the followng arguments");
    println!("The following arguments are available");
    println!("--help\tPrint this helptext");
    println!("--lfsr\tStart the Linear-Feedback-Shift-Register");
    println!("--feistel\tStart the Feistelcrypt");
}
