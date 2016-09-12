extern crate rand;

use self::rand::Rng;


/// A Linear-Feedback-Shift-Register.
///
/// Start with a random Initialization Vector.
/// Then 1st bit will be taken as output.
/// XOR with 3rd and 7th bit.
/// The result will be used as a new input and the register is shifted 1 bit to the left
pub fn lfsr(){
    // Create the 7-bit-register
    let mut register = [0; 7];

    // Initialize it with true of false
    for (i,j) in register.iter_mut().enumerate() {
        *j = rand::thread_rng().gen_range(0, 2);
    };
    println!("Register: {:?}", register);

    for i in 1..20{
        // Calculate the new input-bit
        let result  = (register[0] ^ register[2]) ^ register[6];

        // Shift
        // Following code is not beautiful, but easy to understand:
        println!("Output: {}", register[0]);
        register[0] = register[1];
        register[1] = register[2];
        register[2] = register[3];
        register[3] = register[4];
        register[4] = register[5];
        register[5] = register[6];
        register[6] = result;
        println!("Register after iteration {}: {:?}", i, register);
    }
}
