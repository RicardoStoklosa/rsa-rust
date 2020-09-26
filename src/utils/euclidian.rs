use num_bigint::BigUint;
use num_traits::{one, zero};

// Algoritimo de euclides extendido modificado
pub fn mod_inv(u: &BigUint, v: &BigUint) -> BigUint {
    let mut q: BigUint;
    let mut t1: BigUint;
    let mut t3: BigUint;

    let (mut u1, mut u3, mut v1, mut v3) =
        (BigUint::from(1u8), u.clone(), BigUint::from(0u8), v.clone());
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
mod mod_inv {
    use super::*;
    #[test]
    fn should_work() {
        assert_eq!(
            mod_inv(&BigUint::from(42u32), &BigUint::from(2017u32)),
            BigUint::from(1969u32)
        );
    }
    #[test]
    fn should_not_work() {
        assert_eq!(
            mod_inv(&BigUint::from(4u32), &BigUint::from(2u32)),
            BigUint::from(0u32)
        );
    }
}
