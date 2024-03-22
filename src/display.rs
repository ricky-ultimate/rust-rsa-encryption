use crate::utils;
use std::io;
use std::io::Write;

pub fn selection_display() {
    println!("RSA Calculator");
    println!("----------------");
    println!();
    println!("1. Generate Keys");
    println!("2. Encryption");
    println!("3. Decryption");
    println!("4. Quit and exit");
    println!();
}

pub fn display() {
    clearscreen::clear().expect("FAILED TO CLEAR SCREEN");
    selection_display();

    loop {
        print!("Enter option (1/2/3/4): ");
        io::stdout().flush().unwrap();

        let choice = utils::get_user_choice();

        match choice {
            1 => {
                clearscreen::clear().expect("Failed to clear screen");
                crate::core::generate_keys::generate_keys();
                break;
            }
            2 => {
                clearscreen::clear().expect("Failed to clear screen");
                crate::core::encryption::encryption();
                break;
            }
            3 => {
                clearscreen::clear().expect("Failed to clear screen");
                crate::core::decryption::decryption();
                break;
            }
            4 => break,
            _ => println!("Invalid choice. Please enter 1, 2, 3 or 4."),
        }
    }
}
