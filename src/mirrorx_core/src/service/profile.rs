use lazy_static::lazy_static;
use num::{BigUint, FromPrimitive, Integer, ToPrimitive, Zero};
use rand::{thread_rng, Rng};
use std::{ops::Rem, str::FromStr};

// without 0, I, O
static DEVICE_ID_ALPHABET: &[char] = &[
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

lazy_static! {
static ref DEVICE_ID_ALPHABET_LEN: BigUint =
    BigUint::from_usize(DEVICE_ID_ALPHABET.len()).unwrap();

static ref DEVICE_ID_MIN: BigUint = BigUint::from_u64(808334348993).unwrap();

// range = max - min (max: 2818806960592 ("zyxwvuts"), min: 808334348993 ("abcdefgh"))
static ref DEVICE_ID_RANGE: BigUint =
    BigUint::from_str("2010472611599").unwrap();
}

// whithout I and l
static DEVICE_PASSWORD_ALPHABET: &[u8] =
    b"ABCDEFGHJKLMNOPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789";

pub fn generate_device_id(pub_key_buf: &[u8]) -> anyhow::Result<Option<String>> {
    if pub_key_buf.is_empty() {
        return Err(anyhow::anyhow!("generate_device_id: pub_key_buf is empty"));
    }

    let pub_key_num = BigUint::from_bytes_le(pub_key_buf);
    let mut compute_result = pub_key_num.rem(DEVICE_ID_RANGE.clone());
    let mut device_id_str = String::new();

    while !compute_result.is_zero() {
        let (quotient, remainder) = compute_result.div_rem(&DEVICE_ID_ALPHABET_LEN);
        let remainder = match remainder.to_usize() {
            Some(res) => res,
            None => {
                return Err(anyhow::anyhow!("generate_device_id: compute error"));
            }
        };
        device_id_str.push(DEVICE_ID_ALPHABET[remainder]);
        compute_result = quotient;
    }

    Ok(Some(device_id_str))
}

pub fn generate_device_password() -> String {
    let mut password: String = String::new();

    for _ in 0..8 {
        let n = thread_rng().gen_range(0..DEVICE_PASSWORD_ALPHABET.len());
        password.push(DEVICE_PASSWORD_ALPHABET[n] as char);
    }

    password
}
