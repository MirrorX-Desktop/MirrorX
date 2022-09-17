use ring::aead::NonceSequence;
use tracing::error;

const NONCE_MAX: u128 = (1 << 96) - 1;

pub struct NonceValue(u128);

impl NonceValue {
    pub fn new(initial_nonce: [u8; ring::aead::NONCE_LEN]) -> Self {
        let mut u128_bytes = [0u8; 16];

        u128_bytes[0..ring::aead::NONCE_LEN]
            .copy_from_slice(&initial_nonce[0..ring::aead::NONCE_LEN]);

        Self(u128::from_le_bytes(u128_bytes))
    }
}

impl NonceSequence for NonceValue {
    fn advance(&mut self) -> Result<ring::aead::Nonce, ring::error::Unspecified> {
        self.0 += 1;
        if self.0 > NONCE_MAX {
            self.0 = 1;
        }

        unsafe {
            let nonce_bytes = self.0.to_le_bytes();
            let nonce_bytes_ref: &[u8] = nonce_bytes.as_ref(); //std::slice::from_raw_parts(&self.0 as *const _ as *const u8, 16);
            let nonce_array: [u8; ring::aead::NONCE_LEN] =
                match std::slice::from_raw_parts(nonce_bytes_ref.as_ptr(), 12).try_into() {
                    Ok(v) => v,
                    Err(err) => {
                        error!("parse nonce from slice failed ({})", err);
                        return Err(ring::error::Unspecified::from(err));
                    }
                };

            Ok(ring::aead::Nonce::assume_unique_for_key(nonce_array))
        }
    }
}
