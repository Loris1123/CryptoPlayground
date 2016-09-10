pub fn feistel(){
    let plaintext = "Hello Feistel. Please encrypt this text!";
    let ciphertext = encrypt(&plaintext);
    println!("Cipher: {}", ciphertext)
}

fn encrypt(plaintext: &str) -> String {
    println!("{}",plaintext);
    let mut cipher = "".to_string();
    for c in plaintext.chars(){
        cipher = cipher + &c.to_string();
    }
    cipher
}
