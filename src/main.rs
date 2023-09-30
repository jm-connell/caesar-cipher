use std::io;
use std::io::{Write};

fn main() {
    println!("\n ----------------------------------\n| Caesar Cipher Encryption Program |\n ----------------------------------\n");

    // Display option menu
    loop {
        println!("Choose an option:");
        println!("1. Encrypt message");
        println!("2. Decrypt message");
        println!("3. Quit");

        let mut menu_choice: u32 = get_choice();

        match menu_choice {
            1 => { encrypt(); }
            2 => { decrypt(); }
            3 => {
                println!(" --------\n| Quitting... |\n --------");
                break
            }
            _ => println!("Invalid choice. Please choose 1, 2, or 3."),
        }
    }
}


fn encrypt() {

    // Get message to encrypt
    let mut message = String::new();
    print!("Enter a secret message: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    io::stdin().read_line(&mut message).expect("Error reading input");
    println!("");

    // Get encryption key
    let mut key = get_key();

    // Encrypt message
    let mut output = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let is_upper = c.is_uppercase();
            let offset = if is_upper { b'A' } else { b'a' };
            let shifted_char = (((c as u8 - offset) + key as u8) % 26 + offset) as char;
            output.push(shifted_char);
        } else {
            output.push(c);
        }
    }

    // Display encrypted message
    println!(" -------------------");
    println!("| Encrypted Message |");
    println!(" -------------------");
    println!("{}", output);
}

fn decrypt() {

    // Get message to decrypt
    let mut message = String::new();
    print!("Enter a message to decrypt: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    io::stdin().read_line(&mut message).expect("Error reading input");
    println!("");

    // Get encryption key
    let mut key = get_key();

    let mut output = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let is_upper = c.is_uppercase();
            let offset = if is_upper { b'A' } else { b'a' };
            let shifted_char = (((c as u8 - offset) + 26 - key as u8) % 26 + offset) as char;
            output.push(shifted_char);
        } else {
            output.push(c);
        }
    }

    // Display decrypted message
    println!(" -------------------");
    println!("| Decrypted Message |");
    println!(" -------------------");
    println!("{}", output);
}

fn get_choice() -> u32 {
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input");
        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter 1, 2, or 3."),
        }
    }
}

fn get_key() -> u32 {

    let mut key_string = String::new();
    print!("Enter the encryption key (positive integer): ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    io::stdin().read_line(&mut key_string).expect("Error reading input");

    // Parse key as u32 int, if error set to default of 12
    let key_int: u32 = match key_string.trim().parse() {
        Ok(key_int) => key_int,
        Err(_) => {
            println!("Key input is invalid. Default of 12 will be used.");
            12
        }
    };
    key_int
}