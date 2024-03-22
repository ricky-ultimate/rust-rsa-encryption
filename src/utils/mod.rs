use std::io;
use std::io::Write;

const PRIMES: [u128; 46] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199,
];
pub fn get_val(name: &str) -> u128 {
    print!("Enter {}: ", name);
    io::stdout().flush().unwrap();

    let mut val = String::new();

    io::stdin().read_line(&mut val).expect("Error");

    let val: u128 = match val.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!(
                "Its seems there is some kind of error! Lets try again with an integer shall we?"
            );
            get_val(name)
        }
    };

    if PRIMES.contains(&val) {
        return val;
    } else {
        println!("The provided number is not prime!");
        get_val(name)
    }
}

pub fn get_key(name: &str) -> u128 {
    print!("Enter key '{}' : ", name);
    io::stdout().flush().unwrap();

    loop {
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Error");

        let value = value.trim();

        match value.parse::<u128>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a number"),
        }
    }
}

pub fn get_user_choice() -> u8 {
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input");

        let choice = choice.trim();

        match choice.parse::<u8>() {
            Ok(choice) if (1..=4).contains(&choice) => return choice,
            Ok(_) => println!("Invalid choice. Please enter (1/2/3/4)"),
            Err(_) => println!("Invalid input. Please enter a number."),
        }
        io::stdout().flush().unwrap();
    }
}

pub fn get_message(name: &str) -> String {
    print!("Enter {} Text: ", name);
    io::stdout().flush().unwrap();

    let mut msg = String::new();
    io::stdin()
        .read_line(&mut msg)
        .expect("Error getting message");

    return msg.trim().parse().unwrap();
}
