use crate::utils::{get_key, get_message};

pub fn encryption() {
    println!("ENCRYPTION");
    println!("----------------");
    let e = get_key("e");
    let n = get_key("n");
    let m = get_message("Plain");

    //let m_split: Vec<char> = m.chars().collect();

    let m_ascii: Vec<u128> = m
        .chars()
        .map(|m| match m {
            '0' => 0,            ' ' => 0,
            //',' => 3,
            m if m.is_ascii_lowercase() => m as u128 - 'a' as u128 + 1,
            m if m.is_ascii_uppercase() => m as u128 - 'A' as u128 + 1,
            _ => panic!("Invalid character provided"),
        })
        .collect();

    let mut c: Vec<u128> = Vec::new();

    println!(" ");

    // println!(" Plain Text: {:?}", m_split);
    // println!(" Plain (ASCII) Text: {:?}", m_ascii);

    for i in m_ascii {
        let encrypted_ascii_msg = i.pow(e.try_into().unwrap()) % n;
        c.push(encrypted_ascii_msg);
    }

    println!("Cipher Text: {:?}", c);

    println!();
}
