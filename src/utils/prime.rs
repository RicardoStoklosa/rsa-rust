use num_bigint::{BigUint, RandBigInt};
use num_integer::gcd;
use num_traits::{one, zero};

fn miller_test(mut d: BigUint, n: &BigUint) -> bool {
    let mut rng = rand::thread_rng();
    let mut random_num = BigUint::from(1u8);
    if n != &BigUint::from(5u8) {
        random_num = rng.gen_biguint_range(&one(), &(n - 4u8));
    }
    let a = BigUint::from(2u8) + random_num;

    let mut x = BigUint::modpow(&a, &d, &n);

    if x == one() || x == n - 1u8 {
        return true;
    }

    while d != n - 1u8 {
        x = (&x * &x) % n;
        d *= 2u8;

        if x == one() {
            return false;
        }
        if x == n - 1u8 {
            return true;
        }
    }

    false
}

pub fn is_prime(num: &BigUint) -> bool {
    if num <= &one() || num == &BigUint::from(4u8) {
        return false;
    }
    if num <= &BigUint::from(3u8) {
        return true;
    }

    let mut d = num - 1u8;
    while &d % 2u8 == zero() {
        d /= 2u8
    }

    for _ in 0..10 {
        if miller_test(d.clone(), num) == false {
            return false;
        }
    }
    true
}

pub fn gen_prime(n: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate = rng.gen_biguint(n as u64);
        candidate |= BigUint::from(1u8) << n - 1 | BigUint::from(1u8);
        if is_prime(&candidate) == true {
            return candidate;
        }
    }
}

pub fn coprime(phi: &BigUint) -> BigUint {
    let two = BigUint::from(2u8);

    let mut rng = rand::thread_rng();
    let mut e: BigUint = rng.gen_biguint_range(&two, &phi);

    while gcd(e.clone(), phi.clone()) != one() {
        e = rng.gen_biguint_range(&two, &phi);
    }
    e.clone()
}

#[cfg(test)]
mod is_prime_tests {
    use super::*;
    #[test]
    fn should_be_prime_3() {
        assert_eq!(is_prime(&BigUint::from(3u8)), true);
    }
    #[test]
    fn shouldnt_be_prime_4() {
        assert_eq!(is_prime(&BigUint::from(4u8)), false);
    }
    #[test]
    fn should_be_prime_5() {
        assert_eq!(is_prime(&BigUint::from(5u8)), true);
    }
    #[test]
    fn should_be_prime_6() {
        assert_eq!(is_prime(&BigUint::from(6u8)), false);
    }
    #[test]
    fn should_be_prime_7873() {
        assert_eq!(is_prime(&BigUint::from(7873u32)), true);
    }
    #[test]
    fn should_be_prime_7919() {
        assert_eq!(is_prime(&BigUint::from(7919u32)), true);
    }
}

#[cfg(test)]
mod gen_prime_tests {
    use super::*;
    #[test]
    fn should_work() {
        let prime = gen_prime(8);
        assert_eq!(is_prime(&prime), true);
    }
}

#[cfg(test)]
mod coprime_tests {
    use super::*;
    #[test]
    fn should_work() {
        let phi = &BigUint::from(1000u32);
        let cp = coprime(phi);
        assert!(&cp < phi);
        assert!(&cp > &zero());
    }
}
