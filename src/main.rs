extern crate rand;

use rand::Rng;

fn main(){
    println!("Welcome to CryptoPlayground");
    println!("Start LFSR");
    lfsr();
}

fn lfsr(){
    // Create the 7-bit-register
    let mut register = [false; 7];

    // Initialize it with true of false
    for (i,j) in register.iter_mut().enumerate() {
        let val = rand::thread_rng().gen_range(0, 2);
        match val {
            0 => *j = false,
            1 => *j = true,
            _ => panic!("Unvalid value generated: {}", val),
        }
    };
    println!("Register: {:?}", register);

    for i in 1..20{
        // Get Important bits:
        let first   = register[0];
        let third   = register[2];
        let seventh = register[6];

        // Calculate the new input
        let result  = (first ^ third) ^ seventh;

        // Shift
        // Following code is not beautiful, but easy to understand:
        register[0] = register[1];
        register[1] = register[2];
        register[2] = register[3];
        register[3] = register[4];
        register[4] = register[5];
        register[5] = register[6];
        register[6] = result;
        println!("Output: {}", first);
        println!("Register after iteration {}: {:?}", i, register);
    }

}
