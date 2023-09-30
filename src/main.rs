use std::io;
use std::io::{stdout, Write};

fn main() {
    println!("\n ----------------------------------\n| Caesar Cipher Encryption Program |\n ----------------------------------\n");

    // Create variables
    let mut message = String::new();
    let mut key_string = String::new();
    let mut key_int: u32 = 12;

    // Get message to encrypt
    print!("Enter a secret message: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    io::stdin().read_line(&mut message).expect("Error reading input");
    println!("");

    // Get encryption key
    print!("Enter the encryption key (positive integer): ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    io::stdin().read_line(&mut key_string).expect("Error reading input");

    // Parse key as u32 int, if error set to default of 12
    let key_string: u32 = match key_string.trim().parse() {
        Ok(key_int) => key_int,
        Err(_) => {
            println!("Key input is invalid. Default of 12 will be used.");
            12
        }
    };

    // Encrypt message
    let encrypted_message = encrypt(&message, key_int);
    println!("\nOriginal Message:\n{}\nEncrypted Message:\n{}", message, encrypted_message);

    let decrypted_message = decrypt(&encrypted_message, key_int);
    println!("\nDecrypted Message:\n{}", decrypted_message);

}


fn encrypt(input: &str, key: u32) -> String {
    let mut output = String::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            let is_upper = c.is_uppercase();
            let offset = if is_upper { b'A' } else { b'a' };
            let shifted_char = (((c as u8 - offset) + key as u8) % 26 + offset) as char;
            output.push(shifted_char);
        } else {
            output.push(c);
        }
    }

    output
}

fn decrypt(input: &str, key: u32) -> String {
    let mut output = String::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            let is_upper = c.is_uppercase();
            let offset = if is_upper { b'A' } else { b'a' };
            let shifted_char = (((c as u8 - offset) + 26 - key as u8) % 26 + offset) as char;
            output.push(shifted_char);
        } else {
            output.push(c);
        }
    }
    output
}