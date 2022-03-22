// use std::time::Duration;

// use crate::network::{
//     proto::{HeartBeatReq, HeartBeatResp, ProtoMessage},
//     Client,
// };
// use log::info;

// pub async fn handle_heart_beat_req(
//     client: &Client,
//     req: &HeartBeatReq,
// ) -> anyhow::Result<Option<Box<dyn ProtoMessage>>> {
//     info!("receive heart_beat_req: {:?}", req);
//     let resp: HeartBeatResp = client
//         .call(
//             &HeartBeatReq {
//                 time_stamp: req.time_stamp + 1,
//             },
//             Duration::from_secs(1),
//         )
//         .await?;

//     Ok(Some(Box::new(resp)))
// }
