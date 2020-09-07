mod key_gen;
mod utils;

fn main() {
    // println!("{}", key_gen::coprime(&BigUint::from(3000 as u32)));
    let (private_key, public_key) = key_gen::gen_key(72);
    println!("Private: {:?}", private_key);
    println!("Public: {:?}", public_key);
    let enc = key_gen::encrypt("test", &private_key);
    println!("Encrypt: {}", &enc);
    println!("Decrypt: {}", key_gen::decrypt(&enc, &public_key));
}
