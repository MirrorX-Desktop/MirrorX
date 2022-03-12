use rand::{distributions, thread_rng, Rng};
use std::error::Error;

pub fn generate_device_password() -> String {
    let password: String = thread_rng()
        .sample_iter(distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    password
}
