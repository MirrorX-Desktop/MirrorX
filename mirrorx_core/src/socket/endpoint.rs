use super::{message::client_to_client::ClientToClientMessage, packet::Packet};
use crate::{
    media::video_decoder::VideoDecoder, provider::socket::SocketProvider,
    utility::serializer::BINCODE_SERIALIZER,
};
use anyhow::bail;
use bincode::Options;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use ring::aead::{BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use std::{any::Any, os::raw::c_void};
use tokio::sync::Mutex;
use tracing::error;

pub struct EndPoint {
    local_device_id: String,
    remote_device_id: String,
    opening_key: Mutex<Option<OpeningKey<NonceValue>>>,
    sealing_key: Mutex<Option<SealingKey<NonceValue>>>,
    cache: MemoryCache,
    // texture_id: OnceCell<i64>,
    // video_texture_ptr: OnceCell<i64>,
    // update_frame_callback: OnceCell<unsafe extern "C" fn(i64, *mut c_void, *mut c_void)>,
    video_decoder: OnceCell<VideoDecoder>,
}

impl EndPoint {
    pub fn new(local_device_id: String, remote_device_id: String) -> Self {
        Self {
            local_device_id,
            remote_device_id,
            opening_key: Mutex::new(None),
            sealing_key: Mutex::new(None),
            cache: MemoryCache::new(),
            // texture_id: OnceCell::new(),
            // video_texture_ptr: OnceCell::new(),
            // update_frame_callback: OnceCell::new(),
            video_decoder: OnceCell::new(),
        }
    }

    #[must_use]
    pub fn remote_device_id(&self) -> &str {
        self.remote_device_id.as_ref()
    }

    #[must_use]
    pub fn local_device_id(&self) -> &str {
        self.local_device_id.as_ref()
    }

    #[must_use]
    pub fn cache(&self) -> &MemoryCache {
        &self.cache
    }

    pub async fn set_opening_key(&self, key: UnboundKey, initial_nonce: u64) {
        let opening_key =
            ring::aead::OpeningKey::<NonceValue>::new(key, NonceValue::new(initial_nonce));
        let mut key = self.opening_key.lock().await;
        *key = Some(opening_key);
    }

    pub async fn set_sealing_key(&self, key: UnboundKey, initial_nonce: u64) {
        let sealing_key =
            ring::aead::SealingKey::<NonceValue>::new(key, NonceValue::new(initial_nonce));
        let mut key = self.sealing_key.lock().await;
        *key = Some(sealing_key);
    }

    pub async fn secure_open(&self, buf: &mut [u8]) -> anyhow::Result<()> {
        match self
            .opening_key
            .lock()
            .await
            .as_mut()
            .and_then(|key| Some(key.open_in_place(ring::aead::Aad::empty(), buf)))
        {
            Some(res) => match res {
                Ok(_) => Ok(()),
                Err(err) => bail!("secure_open: opening message failed: {}", err),
            },
            None => bail!("secure_open: opening key is not set"),
        }
    }

    pub async fn secure_seal(&self, message: ClientToClientMessage) -> anyhow::Result<()> {
        let mut buf = BINCODE_SERIALIZER.serialize(&message)?;
        let mut sealing_key = self.sealing_key.lock().await;
        match sealing_key
            .as_mut()
            .and_then(|key| Some(key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buf)))
        {
            Some(res) => {
                if let Err(err) = res {
                    bail!("secure_send: sealing message failed: {}", err);
                }
            }
            None => bail!("secure_send: sealing key is not set"),
        };

        SocketProvider::current()?
            .send(Packet::ClientToClient(
                0,
                self.local_device_id.clone(),
                self.remote_device_id.clone(),
                true,
                buf,
            ))
            .await
    }

    pub fn start_desktop_render_thread(
        &self,
        texture_id: i64,
        video_texture_ptr: i64,
        update_frame_callback_ptr: i64,
    ) -> anyhow::Result<()> {
        unsafe {
            let update_frame_callback = std::mem::transmute::<
                *mut c_void,
                unsafe extern "C" fn(
                    texture_id: i64,
                    video_texture_ptr: *mut c_void,
                    new_frame_ptr: *mut c_void,
                ),
            >(update_frame_callback_ptr as *mut c_void);

            let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264")?;

            let frame_rx = decoder.open()?;

            std::thread::spawn(move || loop {
                match frame_rx.recv() {
                    Ok(video_frame) => update_frame_callback(
                        texture_id,
                        video_texture_ptr as *mut c_void,
                        video_frame.0,
                    ),
                    Err(err) => {
                        error!(err= ?err,"desktop render thread error");
                        break;
                    }
                }
            });

            let _ = self.video_decoder.set(decoder);

            Ok(())
        }
    }

    pub fn transfer_desktop_video_frame(&self, frame: Vec<u8>) {
        if let Some(decoder) = self.video_decoder.get() {
            decoder.decode(frame.as_ptr(), frame.len() as i32, 0, 0);
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum CacheKey {
    PasswordVerifyPublicKey,
    PasswordVerifyPrivateKey,
}

pub struct MemoryCache {
    values: DashMap<CacheKey, Box<dyn Any + Send + Sync>>,
}

impl MemoryCache {
    fn new() -> Self {
        Self {
            values: DashMap::new(),
        }
    }

    pub fn set<T>(&self, key: CacheKey, value: T)
    where
        T: Any + Send + Sync,
    {
        self.values.insert(key, Box::new(value));
    }

    pub fn take<T>(&self, key: CacheKey) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        match self.values.remove(&key) {
            Some(entry) => match entry.1.downcast::<T>() {
                Ok(v) => Some(*v),
                Err(_) => None,
            },
            None => None,
        }
    }
}

struct NonceValue {
    n: u128,
}

impl NonceValue {
    fn new(n: u64) -> Self {
        Self { n: n as u128 }
    }
}

impl NonceSequence for NonceValue {
    fn advance(&mut self) -> Result<ring::aead::Nonce, ring::error::Unspecified> {
        self.n += 1;
        let m = self.n & 0xFFFFFFFFFFFF;
        Nonce::try_assume_unique_for_key(&m.to_le_bytes()[..12])
    }
}
