use ring::aead::NonceSequence;

const NONCE_MAX: u128 = (1 << 96) - 1;

pub struct NonceValue(u128);

impl NonceValue {
    pub fn new(initial_nonce: [u8; ring::aead::NONCE_LEN]) -> Self {
        let mut u128_bytes = [0u8; 16];

        for i in 0..ring::aead::NONCE_LEN {
            u128_bytes[i] = initial_nonce[i];
        }

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
            let v: &[u8] = std::slice::from_raw_parts(&self.0 as *const _ as *const u8, 16);
            let arr: &[u8; 12] = match v.try_into() {
                Ok(res) => res,
                Err(err) => return Err(ring::error::Unspecified::from(err)),
            };

            Ok(ring::aead::Nonce::assume_unique_for_key(*arr))
        }
    }
}
