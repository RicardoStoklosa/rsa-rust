use super::utils::euclidian::mod_inv;
use super::utils::parser::{parse_to_num, parse_to_str};
use super::utils::prime::{coprime, gen_prime};

use num_bigint::BigUint;
use std::fs::{remove_file, rename, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

#[derive(Debug)]
pub struct Key {
    pub exp: BigUint,
    pub n: BigUint,
}

fn get_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - 1 as u8) * (q - 1 as u8)
}

pub fn gen_key(bits: usize) -> (Key, Key) {
    let p = gen_prime(bits / 2);
    let q = gen_prime(bits - bits / 2);
    let n = &p * &q;
    let phi = get_phi(&p, &q);
    let e = coprime(&phi);
    let d = mod_inv(&e, &phi);
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

pub fn encrypt_file(path: &str, key: &Key) {
    let file = File::open(&path).expect("Unnable to read file");

    let out_file_path = format!("{}.temp", path);

    let mut out_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&out_file_path)
        .expect("Unnable to create file");

    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        let line = line.expect("Line error");
        let mut num_message = parse_to_num(&line)
            .iter()
            .map(|x| {
                let mut out = x.modpow(&key.exp, &key.n).to_string();
                out.push(' ');
                out
            })
            .collect::<String>();
        num_message.push('\n');
        out_file
            .write(num_message.as_bytes())
            .expect("Error to write the temp file");
    }

    remove_file(path).expect("Error to delete old file");
    rename(out_file_path, path).expect("Error to create new file");
}

pub fn decrypt_file(path: &str, key: &Key) {
    let file = File::open(&path).expect("Unnable to read file");

    let out_file_path = format!("{}.temp", path);

    let mut out_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&out_file_path)
        .expect("Unnable to create file");

    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        let line = line.expect("Line error");

        let line = line
            .split(' ')
            .filter(|&x| x != "")
            .map(|x| BigUint::from_str(x).unwrap().modpow(&key.exp, &key.n))
            .collect::<Vec<BigUint>>();

        let message = parse_to_str(&line);

        out_file
            .write(message.as_bytes())
            .expect("Error to write the temp file");
    }
    remove_file(path).expect("Error to delete old file");
    rename(out_file_path, path).expect("Error to create new file");
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
}
