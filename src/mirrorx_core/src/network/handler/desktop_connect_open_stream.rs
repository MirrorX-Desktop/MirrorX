use std::{ffi::CString, sync::Arc, time::Duration};

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    duplicator,
    network::{
        message::{Message, MessageError},
        Client,
    },
    video_encoder,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOpenStreamReq {
    pub offer_device_id: String,
    pub ask_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOpenStreamResp {}

impl DesktopConnectOpenStreamReq {
    pub async fn handle(self, _: Arc<Client>) -> anyhow::Result<Message, MessageError> {
        info!("handle desktop connect open stream: {:?}", self);

        tokio::spawn(async move {
            info!("spawn duplicator task");

            let dup = match duplicator::duplicator::Duplicator::new() {
                Ok(dup) => dup,
                Err(err) => {
                    error!("duplicator error: {:?}", err);
                    return;
                }
            };

            let encoder = match video_encoder::video_encoder::VideoEncoder::new(
                CString::new("h264_videotoolbox").unwrap().as_ptr(),
            ) {
                Ok(encoder) => encoder,
                Err(err) => {
                    error!("video encoder error: {:?}", err);
                    return;
                }
            };

            dup.start_capture();

            let frame_ticker = tokio::time::interval(Duration::from_millis(16));
            loop {
                frame_ticker.tick().await;
                let frame = match dup.get_frame() {
                    Some(frame) => frame,
                    None => {
                        break;
                    }
                };

                encoder.send_frame(Box::new(frame))
            }

            dup.stop_capture();
            info!("end duplicator task");
        });

        Ok(Message::DesktopConnectOpenStreamResp(
            DesktopConnectOpenStreamResp {},
        ))
    }
}
