use std;

static PRIME: u64 = 1099511628211;

/// Creates a 64-Bit FNV1-a hash from the given string
pub fn fnv1a(string: String){
    let data_bytes = string.into_bytes();
    
    
    let mut hash: u64 = 14695981039346656037;    // Offset. Startvalue.
    
    for byte in data_bytes {
        // Get last byte of hash
        // Note: hash is mutable.
        step_xor(&mut hash, &byte);
        step_multiply(&mut hash);
        println!("{:?}", hash);
    }
    
    /// XOR for FNV1(-a) hash.
    /// XOR is calculated on the last byte of the hash.
    fn step_xor(current_hash: &mut u64, data: &u8){
        // Get last byte of hash
        let mut hashbyte: [u8; 8] = unsafe {std::mem::transmute(*current_hash)};
        println!("Hashbyte: {:?}", hashbyte);
        let last_byte = hashbyte[0];
        let xored: u8 = last_byte ^ data;
        println!("XORED: {:?}",xored );
        // Insert result into hash
        
        hashbyte[0] = xored;
        
        // Translate bytes back to integer and assign to mutable hash
        *current_hash = unsafe {std::mem::transmute(hashbyte)};
    }
    
    /// Muiplication for FNV1(-a) hash.
    fn step_multiply(hash: &mut u64){
        println!("Hash: {:?}", *hash);
        println!("Prime: {:?}", PRIME);
    }
}

