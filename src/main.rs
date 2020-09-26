mod cripto;
mod utils;
use num_bigint::BigUint;
use std::env;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args.get(1) {
        Some(s) => s.as_str(),
        None => "",
    };
    match command {
        "encrypt" => {
            if args.len() != 5 {
                println!("Usage: encrypt [file path] [key exp] [key n]");
                return;
            }

            let exp = BigUint::from_str(&args[3]).expect("Error in key exp");
            let n = BigUint::from_str(&args[4]).expect("Error in key n");

            let path = &args[2];
            let now = Instant::now();
            cripto::encrypt_file(path, &cripto::Key { exp, n });
            println!("Elapsed time: {} us", now.elapsed().as_micros());
            println!("File \"{}\" was encrypted", path);
        }
        "genkey" => {
            let mut bits: usize = 128;
            if args.len() == 3 {
                bits = args[2].parse::<usize>().unwrap();
            }
            let now = Instant::now();
            println!("Creating a {} key ...", bits);
            let (private_key, public_key) = cripto::gen_key(bits);
            println!("Elapsed time: {} us", now.elapsed().as_micros());
            println!("Public: ({} {})", public_key.exp, public_key.n);
            println!("Private: ({} {})", private_key.exp, private_key.n);
        }
        "decrypt" => {
            if args.len() != 5 {
                println!("Usage: decrypt [file path] [key exp] [key n]");
                return;
            }

            let exp = BigUint::from_str(&args[3]).expect("Error in key exp");
            let n = BigUint::from_str(&args[4]).expect("Error in key n");

            let path = &args[2];
            let now = Instant::now();
            cripto::decrypt_file(path, &cripto::Key { exp, n });
            println!("Elapsed time: {} us", now.elapsed().as_micros());
            println!("File \"{}\" was decrypted", path);
        }
        a => {
            println!("Help: {} Command not found! \n", a);
            println!("Commands:");
            println!("\tgenkey");
            println!("\t\tGenerate the public and private key");
            println!("\tencrypt [file path] [key exp] [key n]");
            println!("\t\tEncrypt a file");
            println!("\tdecrypt [file path] [key exp] [key n]");
            println!("\t\tDecrypt a file");
            println!("\thelp");
        }
    }
}
