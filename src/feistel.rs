pub fn encrypt(){
    // Some testdata to encrypt.
    //let plaintext = "Hello Feistel, please encrypt this text";

    //println!("{:?}", plaintext.chars());

    let mut left = [125u8, 34u8, 124u8, 168u8, 86u8, 24u8, 201u8, 134u8];
    let mut right = [12u8, 14u8, 174u8, 128u8, 46u8, 24u8, 121u8, 184u8];

    for _ in 1..16 {
        println!("Call round");
        println!("Current Left:  {:?}", left);
        println!("Current Right: {:?}", right);

        let result = round_function(&left, &right);
        left = result.0;
        right = result.1;

        println!("");
    }
    println!("Final:", );
    println!("Left:  {:?}", left);
    println!("Right: {:?}", right);

}

/// Round function for feistel structure.
fn round_function(left: &[u8; 8], right: &[u8; 8]) -> ([u8;8],[u8;8]){
    let mut tmp = [0u8; 8];   // Will be result of XOR. So becoming the new right
    let crypt_right = crypto_function(right);
    for i in 0..8 {
        // XOR each byte
        tmp[i] = left[i] ^ crypt_right[1];
    }
    println!("{:?}", tmp );

    (right.clone(),tmp)

}

// "Encrypts" the given data byte by XOR with the key.
fn crypto_function(data: &[u8]) -> [u8; 8]{
    //println!("CryptoFunction");
    //println!("Data: {:?}", data);
    let key = [54u8, 128u8, 37u8, 219u8, 44u8, 51u8, 116u8, 197u8];
    let mut result = [0u8; 8];

    for i in 0..8 {
        result[i] = key[i] ^ data[i];
    }
    //println!("Enc: {:?}", result);
    result

}
