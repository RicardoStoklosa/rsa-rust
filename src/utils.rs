use num_bigint::{BigUint, RandBigInt};
use num_traits::{one, zero};

pub fn is_prime(num: &BigUint) -> bool {
    let mut d = BigUint::from(2 as u8);

    while &d * &d <= *num {
        if num % &d == zero() {
            return false;
        }
        d = d + 1 as u8;
    }
    true
}

pub fn gen_prime(n: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate = rng.gen_biguint(n as u64);
        candidate |= BigUint::from(1 as u8) << n - 1 | BigUint::from(1 as u8);
        if is_prime(&candidate) == true {
            return candidate;
        }
    }
}

pub fn mod_inv(u: &BigUint, v: &BigUint) -> BigUint {
    let mut q: BigUint;
    let mut t1: BigUint;
    let mut t3: BigUint;

    let (mut u1, mut u3, mut v1, mut v3) = (
        BigUint::from(1 as u8),
        u.clone(),
        BigUint::from(0 as u8),
        v.clone(),
    );
    let mut iter: i8 = 1;

    while &v3 != &zero() {
        q = &u3 / &v3;
        t3 = &u3 % &v3;
        t1 = &u1 + &q * &v1;

        u1 = v1;
        v1 = t1;
        u3 = v3;
        v3 = t3;

        iter = -iter;
    }

    if &u3 != &one() {
        return zero();
    }

    if iter < 0 {
        return v - u1;
    }

    u1
}

#[cfg(test)]
mod prime_tests {
    use super::*;
    #[test]
    fn test_prime_3() {
        assert_eq!(is_prime(&BigUint::from(3 as u32)), true);
    }
    #[test]
    fn test_prime_4() {
        assert_eq!(is_prime(&BigUint::from(4 as u32)), false);
    }
    #[test]
    fn test_prime_5() {
        assert_eq!(is_prime(&BigUint::from(5 as u32)), true);
    }
    #[test]
    fn test_prime_7872() {
        assert_eq!(is_prime(&BigUint::from(6 as u32)), false);
    }
    #[test]
    fn test_prime_7873() {
        assert_eq!(is_prime(&BigUint::from(7873 as u32)), true);
    }
    #[test]
    fn test_prime_7979() {
        assert_eq!(is_prime(&BigUint::from(7919 as u32)), true);
    }
}

#[cfg(test)]
mod mod_inv {
    use super::*;
    #[test]
    fn test_should_be_work() {
        assert_eq!(
            mod_inv(&BigUint::from(42 as u32), &BigUint::from(2017 as u32)),
            BigUint::from(1969 as u32)
        );
    }
    #[test]
    fn test_should_not_work() {
        assert_eq!(
            mod_inv(&BigUint::from(4 as u32), &BigUint::from(2 as u32)),
            BigUint::from(0 as u32)
        );
    }
}
