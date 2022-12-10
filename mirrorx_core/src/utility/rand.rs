use rand::Rng;

#[inline]
pub fn generate_device_finger_print() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[inline]
pub fn generate_random_password() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(18)
        .map(char::from)
        .collect()
}

#[inline]
pub fn generate_random_ping_value() -> i32 {
    rand::thread_rng().gen()
}
