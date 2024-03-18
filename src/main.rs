use gcd::Gcd;
use std::io;
use std::io::Write;

const PRIMES: [u128;46] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
    191, 193, 197, 199,
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

pub fn get_p() -> u128 {

    print!("Enter p: ");
    io::stdout().flush().unwrap();

    let mut p = String::new();

    io::stdin().read_line(&mut p).expect("Error");

    let p: u128 = match p.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!(
                "Its seems there is some kind of error! Lets try again with an integer shall we?"
            );
            get_p()
        }
    };

    if PRIMES.contains(&p) {
        return p;
    } else {
        println!("The entered number is not a prime number.");
        get_p()
    }
}

pub fn get_q() -> u128 {
    
    print!("Enter q: ");
    io::stdout().flush().unwrap();

    let mut q = String::new();

    io::stdin().read_line(&mut q).expect("Error");

    let q: u128 = match q.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!(
                "Its seems there is some kind of error! Lets try again with an integer shall we?"
            );
            get_q()
        }
    };

    if PRIMES.contains(&q) {
        return q;
    } else {
        println!("The entered number is not a prime number.");
        get_q()
    }
}

pub fn get_e() -> u128 {
    print!("Enter public key 'e' : ");
    io::stdout().flush().unwrap();
    loop {
        let mut e = String::new();
        io::stdin().read_line(&mut e).expect("Error");

        let e = e.trim();

        match e.parse::<u128>() {
            Ok(e) => return e,
            Err(_) => println!("Invalid input. Please enter a number"),
        }
    }
}

pub fn get_n() -> u128 {
    print!("Enter public key 'n' : ");
    io::stdout().flush().unwrap();
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error");

        let n = n.trim();

        match n.parse::<u128>() {
            Ok(n) => return n,
            Err(_) => println!("Invalid input. Please enter a number"),
        }
    }
}

pub fn get_d() -> u128 {
    print!("Enter public key 'd' : ");
    io::stdout().flush().unwrap();
    loop {
        let mut d = String::new();
        io::stdin().read_line(&mut d).expect("Error");

        let d = d.trim();

        match d.parse::<u128>() {
            Ok(d) => return d,
            Err(_) => println!("Invalid input. Please enter a number"),
        }
    }
}

pub fn compute_keys() {
    let p: u128 = get_p();
    let q: u128 = get_q();
    let n: u128 = p * q;
    let phi_n = (p - 1) * (q - 1);

    let mut e: u128;
    loop {
        e = get_e();
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

    println!("");
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
    let e = get_e();
    let n = get_n();
    let m = get_plain_text();

    let m_split: Vec<char> = m.chars().collect();

    let m_ascii: Vec<u128> = m
        .chars()
        .map(|m| match m {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '0' => 0,
            ' ' => 0,
            ',' => 3,
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

    println!("");
}

pub fn decryption() {
    println!("DECRYPTION");
    println!("----------------");

    let d = get_d();
    let n = get_n();
    let c = get_encrypted_text();

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

    println!("");
    println!("Cipher text: {:?}", c_split);

    for i in c_split {
        let decrypted_msg_ascii = i.pow(d.try_into().unwrap()) % n;
        m_ascii.push(decrypted_msg_ascii);
    }

    println!("Plain (ASCII) text: {:?}", m_ascii);

    let m_pl_txt: Vec<char> = m_ascii
        .into_iter()
        .map(|m| match m {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            0 => ' ',
            3 => ',',
            m if m >= 1 && m <= 26 => (m as u8 - 1 + b'a') as char,
            m if m >= 27 && m <= 27 => (m as u8 - 27 + b'A') as char,
            _ => panic!("Invalid number"),
        })
        .collect();

    println!("Plain Text : {:?}", m_pl_txt);

    println!("");
}

pub fn display() {
    println!("RSA Calculator");
    println!("----------------");
    println!("");
    println!("1. Generate Keys");
    println!("2. Encryption");
    println!("3. Decryption");
    println!("4. Quit and exit");
    println!("");
}

pub fn get_plain_text() -> String {
    print!("Enter message to be encrypted: ");
    io::stdout().flush().unwrap();

    let mut m = String::new();
    io::stdin()
        .read_line(&mut m)
        .expect("Error handling plain text!");

    return m.trim().parse().unwrap();
}

pub fn get_encrypted_text() -> String {
    print!("Enter message to be decrypted: ");
    io::stdout().flush().unwrap();

    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Error handling plain text!");

    return c.trim().parse().unwrap();
}
