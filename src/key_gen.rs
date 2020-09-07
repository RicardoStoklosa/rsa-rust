use super::utils::{gen_prime, mod_inv};
use num_bigint::{BigUint, RandBigInt};
use num_integer::gcd;
use num_traits::{one, ToPrimitive};
use std::str::FromStr;

#[derive(Debug)]
pub struct Key {
    exp: BigUint,
    n: BigUint,
}

fn get_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - 1 as u8) * (q - 1 as u8)
}

pub fn coprime(phi: &BigUint) -> BigUint {
    let two = BigUint::from(2 as u8);

    let mut rng = rand::thread_rng();
    let mut e: BigUint = rng.gen_biguint_range(&two, &phi);

    while gcd(e.clone(), phi.clone()) != one() {
        e = rng.gen_biguint_range(&two, &phi);
    }
    e.clone()
}

pub fn gen_key(bits: usize) -> (Key, Key) {
    let p = gen_prime(bits / 2);
    let q = gen_prime(bits - bits / 2);
    let n = &p * &q;
    let phi = get_phi(&p, &q);
    let e = coprime(&phi);
    // verificar
    let d = mod_inv(&e, &phi);
    println!("===================");
    println!("p:{}, q:{}, n:{}", p, q, n);
    println!("phi:{}, e:{}, d:{}", &phi, e, d);
    println!("public key ({}, {})", n, e);
    println!("Private key ({}, {})", n, d);
    let m = BigUint::from('A' as u32);
    println!("Message {}", m);
    let c = m.modpow(&e, &n);
    println!("Encryption : {}", &c);
    let dec = c.modpow(&d, &n);
    println!("Decryption : {}, ", dec);
    match dec.to_u8() {
        None => println!("Cant convert"),
        Some(value) => println!("Dec message: {}", value as char),
    }
    println!("Eq : {}", m == dec);
    (
        Key {
            exp: e,
            n: n.clone(),
        },
        Key {
            exp: d,
            n: n.clone(),
        },
    )
}

fn parse_str(message: &str) -> BigUint {
    let mut out = String::new();

    for letter in message.chars() {
        let letter_value = letter as i32;
        let mut value_str = letter_value.to_string();

        while value_str.chars().count() < 3 {
            value_str.insert(0, '0');
        }

        out.push_str(&value_str);
    }
    BigUint::from_str(&out).unwrap()
}
pub fn encrypt(message: &str, key: &Key) -> BigUint {
    let message = parse_str(&message);
    println!("message: {}", &message);
    message.modpow(&key.exp, &key.n)
}

pub fn decrypt(message: &BigUint, key: &Key) -> String {
    let dec = message.modpow(&key.exp, &key.n).to_string();

    // for i in dec.chars().rev(){

    // }
    dec
}

#[cfg(test)]
mod key_gen_test {
    use super::*;

    #[test]
    fn test_phi() {
        assert_eq!(
            get_phi(&BigUint::from(2 as u8), &BigUint::from(7 as u8)),
            BigUint::from(6 as u8)
        );
    }

    #[test]
    fn test_coprime() {
        assert_eq!(coprime(&BigUint::from(6 as u8)), BigUint::from(5 as u8));
    }

    #[test]
    fn test_char_parse() {
        assert_eq!(parse_str("HH"), BigUint::from(72072 as u32));
    }
}
