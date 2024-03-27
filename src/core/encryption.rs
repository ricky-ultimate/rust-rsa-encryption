use crate::utils::{get_key, get_message};

pub fn encryption() {
    println!("ENCRYPTION");
    println!("------------");
    let e = get_key("e");
    let n = get_key("n");
    let msg = get_message("Plain");
    
    let msg_ascii: Vec<u128> = msg
        .chars()
        .map(|m| match m {
            '0' => 0,
            ' ' => 0,
            //',' => 3,
            m if m.is_ascii_lowercase() => m as u128 - 'a' as u128 + 1,
            m if m.is_ascii_uppercase() => m as u128 - 'A' as u128 + 1,
            _ => panic!("Invalid character provided"),
        })
        .collect();

    let mut c: Vec<u128> = Vec::new();

    println!(" ");

    for m in msg_ascii {
        let encrypted_ascii_msg = m.pow(e.try_into().unwrap()) % n;
        c.push(encrypted_ascii_msg);
    }

    println!("Cipher Text: {:?}", c);

    println!();
}
