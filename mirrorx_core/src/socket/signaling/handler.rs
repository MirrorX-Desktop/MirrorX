pub async fn handle_connect(
    endpoint: Arc<EndPoint>,
    req: ConnectRequest,
) -> anyhow::Result<ConnectReply> {
    tracing::trace!(req = %req, "connect");

    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 4096)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_n = pub_key.n().to_bytes_le();
    let pub_key_e = pub_key.e().to_bytes_le();

    endpoint
        .cache()
        .set(CacheKey::PasswordVerifyPrivateKey, priv_key);

    Ok(ConnectReply {
        pub_key_n,
        pub_key_e,
    })
}

pub async fn handle_key_exchange_and_verify_password(
    endpoint: Arc<EndPoint>,
    req: KeyExchangeAndVerifyPasswordRequest,
) -> anyhow::Result<KeyExchangeAndVerifyPasswordReply> {
    tracing::trace!(req = %req, "key_exchange_and_verify_password");

    // todo: check white list

    let local_password = ConfigProvider::current()?
        .read_device_password()?
        .ok_or(anyhow!(
            "key_exchange_and_verify_password: local password not set, refuse request"
        ))?;

    let priv_key = endpoint
        .cache()
        .take::<RsaPrivateKey>(CacheKey::PasswordVerifyPrivateKey)
        .ok_or(anyhow::anyhow!(
            "key_exchange_and_verify_password: no private key found"
        ))?;

    let req_password = priv_key
        .decrypt(PaddingScheme::PKCS1v15Encrypt, &req.password_secret)
        .map_err(|err| {
            anyhow!(
                "key_exchange_and_verify_password: decrypt password secret failed: {}",
                err
            )
        })?;

    let req_password = String::from_utf8(req_password).map_err(|err| {
        anyhow!(
            "key_exchange_and_verify_password: parse local password bytes to utf8 failed: {}",
            err
        )
    })?;

    if req_password != local_password {
        return Ok(KeyExchangeAndVerifyPasswordReply {
            password_correct: false,
            exchange_pub_key: Vec::default(),
            exchange_salt: Vec::default(),
        });
    }

    // gen key exchange
    let ephemeral_rng = ring::rand::SystemRandom::new();
    let local_private_key =
        ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519, &ephemeral_rng)
            .map_err(|err| {
                anyhow!(
                    "key_exchange_and_verify_password: generate ephemeral private key failed: {}",
                    err
                )
            })?;

    let local_public_key = local_private_key.compute_public_key().map_err(|err| {
        anyhow::anyhow!(
            "key_exchange_and_verify_password: compute public key failed: {}",
            err
        )
    })?;

    let exchange_pub_key = local_public_key.as_ref().to_vec();

    let mut exchange_salt = Vec::<u8>::new();
    exchange_salt.resize(32, 0);
    ephemeral_rng.fill(&mut exchange_salt).map_err(|err| {
        anyhow::anyhow!(
            "key_exchange_and_verify_password: generate exchange salt failed: {}",
            err
        )
    })?;

    let remote_public_key =
        ring::agreement::UnparsedPublicKey::new(&ring::agreement::X25519, &req.exchange_pub_key);

    let (sealing_key, opening_key) = ring::agreement::agree_ephemeral(
        local_private_key,
        &remote_public_key,
        ring::error::Unspecified,
        |key_material| {
            let send_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::new();
                    key.resize(ring::aead::CHACHA20_POLY1305.key_len(), 0);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

            let recv_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &req.exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::new();
                    key.resize(ring::aead::CHACHA20_POLY1305.key_len(), 0);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

            Ok((send_key, recv_key))
        },
    )
    .map_err(|err| {
        anyhow!(
            "key_exchange_and_verify_password: agree ephemeral key failed: {:?}",
            err
        )
    })?;

    // initial endpoint opening(recv) key
    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &opening_key).map_err(
            |err| {
                anyhow::anyhow!(
                    "key_exchange_and_verify_password: create unbounded key for opening failed: {}",
                    err
                )
            },
        )?;

    let opening_initial_nonce =
        unsafe { u64::from_le_bytes(*(exchange_salt[..8].as_ptr() as *const [u8; 8])) };

    endpoint
        .set_opening_key(unbound_opening_key, opening_initial_nonce)
        .await;

    // initial endpoint sealing(send) key
    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &sealing_key).map_err(
            |err| {
                anyhow::anyhow!(
                    "key_exchange_and_verify_password: create unbounded key for sealing failed: {}",
                    err
                )
            },
        )?;

    let sealing_initial_nonce =
        unsafe { u64::from_le_bytes(*(req.exchange_salt[..8].as_ptr() as *const [u8; 8])) };

    endpoint
        .set_sealing_key(unbound_sealing_key, sealing_initial_nonce)
        .await;

    tracing::trace!("key_exchange_and_verify_password success");

    Ok(KeyExchangeAndVerifyPasswordReply {
        password_correct: true,
        exchange_pub_key,
        exchange_salt,
    })
}
