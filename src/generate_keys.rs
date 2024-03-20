use gcd::Gcd;
use crate::{get_key, get_val};

pub fn generate_keys() {
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