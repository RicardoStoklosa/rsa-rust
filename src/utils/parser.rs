use num_bigint::BigUint;
use num_traits::zero;
use num_traits::ToPrimitive;

pub fn parse_to_num(message: &String) -> Vec<BigUint> {
    let len = message.chars().count();
    let mut out = Vec::new();
    for i in (0..len).step_by(3) {
        let substr = &message[i..i + std::cmp::min(3, len - i)];
        let mut aux = 0u32;
        for (ci, c) in substr.chars().enumerate() {
            aux += c as u32 * (255u32).pow(2u32 - ci as u32);
        }

        out.push(BigUint::from(aux));
    }

    out
}

pub fn parse_to_str(message: &Vec<BigUint>) -> String {
    let mut out = String::new();
    for i in message {
        &out.push((i / 255u32.pow(2)).to_u8().expect("Cannot parse to char") as char);
        let res_1 = i % 255u32.pow(2);
        if &res_1 != &zero() {
            &out.push(
                (&res_1 / 255u32.pow(1))
                    .to_u8()
                    .expect("Cannot parse to char") as char,
            );
            let res_2 = &res_1 % 255u32.pow(1);
            if res_2 != zero() {
                &out.push(res_2.to_u8().expect("Cannot parse to char") as char);
            }
        }
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    #[test]
    fn parse_to_num_test() {
        let expect = vec![5488725u32, 7486140u32, 7486132u32, 2110481u32, 7507455u32]
            .iter()
            .map(|x| BigUint::from(*x))
            .collect::<Vec<BigUint>>();
        assert_eq!(parse_to_num(&String::from("This is a test")), expect);
    }
    #[test]
    fn parse_to_str_test() {
        let mut message = String::from("This is a test");
        let parsed = parse_to_num(&message);
        message.push('\n');
        assert_eq!(parse_to_str(&parsed), message);
    }
}
