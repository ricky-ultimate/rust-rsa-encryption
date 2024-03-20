use gcd::Gcd;
use std::io;
use std::io::Write;

const PRIMES: [u128; 46] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199,
];

fn main() {
    clearscreen::clear().expect("FAILED TO CLEAR SCREEN");
    display();

    loop {
        print!("Enter option (1/2/3/4): ");
        io::stdout().flush().unwrap();

        let choice = get_user_choice();

        match choice {
            1 => {
                clearscreen::clear().expect("Failed to clear screen");
                compute_keys();
                break;
            }
            2 => {
                clearscreen::clear().expect("Failed to clear screen");
                encryption();
                break;
            }
            3 => {
                clearscreen::clear().expect("Failed to clear screen");
                decryption();
                break;
            }
            4 => break,
            _ => println!("Invalid choice. Please enter 1, 2, 3 or 4."),
        }
    }
}

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
        println!("The entered number is not a prime number.");
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

pub fn compute_keys() {
    let p: u128 = get_val("p");
    let q: u128 = get_val("q");
    let n: u128 = p * q;
    let phi_n = (p - 1) * (q - 1);

    let mut e: u128;
    loop {
        e = get_key("e");
        if e.gcd(phi_n) == 1 {
            break;
        }
    }

    let mut d: u128 = 0;

    while ((d * e) % phi_n) != 1 {
        d += 1
    }

    let mut pub_key: Vec<u128> = Vec::with_capacity(2);
    pub_key.push(e);
    pub_key.push(n);

    let mut priv_key: Vec<u128> = Vec::with_capacity(2);
    priv_key.push(d);
    priv_key.push(n);

    println!("φ(n) is: {phi_n}");

    clearscreen::clear().expect("FAILED TO CLEAR SCREEN");

    println!("Keys Generated successfully!");
    println!("----------------------------");

    println!("Public key is (e,n) => ({},{})", pub_key[0], pub_key[1]);
    println!("Private key is (d,n) => ({},{})", priv_key[0], priv_key[1]);

    println!();
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

pub fn encryption() {
    println!("ENCRYPTION");
    println!("----------------");
    let e = get_key("e");
    let n = get_key("n");
    let m = get_message("Plain");

    let m_split: Vec<char> = m.chars().collect();

    let m_ascii: Vec<u128> = m
        .chars()
        .map(|m| match m {
            '0' => 0,
            ' ' => 0,
            //',' => 3,
            m if m.is_ascii_lowercase() => m as u128 - 'a' as u128 + 1,
            m if m.is_ascii_uppercase() => m as u128 - 'A' as u128 + 1,
            _ => panic!("no"),
        })
        .collect();

    let mut c: Vec<u128> = Vec::new();

    println!(" ");

    println!(" Plain Text: {:?}", m_split);
    println!(" Plain (ASCII) Text: {:?}", m_ascii);

    for i in m_ascii {
        let encrypted_ascii_msg = i.pow(e.try_into().unwrap()) % n;
        c.push(encrypted_ascii_msg);
    }

    println!(" Cipher Text: {:?}", c);

    println!();
}

pub fn decryption() {
    println!("DECRYPTION");
    println!("----------------");

    let d = get_key("d");
    let n = get_key("n");
    let c = get_message("Cipher");

    let mut c_split: Vec<u128> = Vec::new();
    let mut m_ascii: Vec<u128> = Vec::new();

    for i in c.split_whitespace() {
        let j: u128 = match i.parse() {
            Ok(j) => j,
            Err(_) => {
                println!("Invalid number: {}", i);
                continue;
            }
        };
        c_split.push(j);
    }

    println!();
    println!("Cipher text: {:?}", c_split);

    for i in c_split {
        let decrypted_msg_ascii = i.pow(d.try_into().unwrap()) % n;
        m_ascii.push(decrypted_msg_ascii);
    }

    println!("Plain (ASCII) text: {:?}", m_ascii);

    let m_pl_txt: Vec<char> = m_ascii
        .into_iter()
        .map(|m| match m {
            0 => ' ',
            //3 => ',',
            m if m >= 1 && m <= 26 => (m as u8 - 1 + b'a') as char,
            m if m >= 27 && m <= 50 => (m as u8 - 27 + b'A') as char,
            _ => panic!("Invalid number"),
        })
        .collect();

    println!("Plain Text : {:?}", m_pl_txt);

    println!();
}

pub fn display() {
    println!("RSA Calculator");
    println!("----------------");
    println!();
    println!("1. Generate Keys");
    println!("2. Encryption");
    println!("3. Decryption");
    println!("4. Quit and exit");
    println!();
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
