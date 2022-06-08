use rand::{thread_rng, Rng};

pub fn generate_device_password() -> String {
    static DEVICE_PASSWORD_ALPHABET: &[u8] =
        b"ABCDEFGHJKLMNOPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789";

    let mut password: String = String::new();

    for _ in 0..8 {
        let n = thread_rng().gen_range(0..DEVICE_PASSWORD_ALPHABET.len());
        password.push(DEVICE_PASSWORD_ALPHABET[n] as char);
    }

    password
}
