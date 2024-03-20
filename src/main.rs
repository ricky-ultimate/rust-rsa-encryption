mod core;
mod display;
mod utils;

use std::io;
use std::io::Write;

const PRIMES: [u128; 46] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199,
];

fn main() {
    clearscreen::clear().expect("FAILED TO CLEAR SCREEN");
    display::display();

    loop {
        print!("Enter option (1/2/3/4): ");
        io::stdout().flush().unwrap();

        let choice = get_user_choice();

        match choice {
            1 => {
                clearscreen::clear().expect("Failed to clear screen");
                core::generate_keys::generate_keys();
                break;
            }
            2 => {
                clearscreen::clear().expect("Failed to clear screen");
                core::encryption::encryption();
                break;
            }
            3 => {
                clearscreen::clear().expect("Failed to clear screen");
                core::decryption::decryption();
                break;
            }
            4 => break,
            _ => println!("Invalid choice. Please enter 1, 2, 3 or 4."),
        }
    }
}


