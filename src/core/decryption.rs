use crate::utils::{get_key, get_message};

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