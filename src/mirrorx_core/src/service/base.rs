use rand::{thread_rng, Rng};

// whithout I and l
static PASSWORD_CHARSET: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789";

pub fn generate_device_password() -> String {
    let mut password: String = String::new();

    for _ in 0..8 {
        let n = thread_rng().gen_range(0..PASSWORD_CHARSET.len());
        password.push(PASSWORD_CHARSET[n] as char);
    }

    password
}
