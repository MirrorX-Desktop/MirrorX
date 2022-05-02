use std::sync::Arc;

use log::info;

use super::message::{
    client_to_server::HeartBeatRequest, reply_error::ReplyError, server_to_client::HeartBeatReply,
};

// pub async fn heartbeat(
//     client: Arc<Streamer>,
//     req: HeartBeatRequest,
// ) -> Result<HeartBeatReply, ReplyError> {
//     info!("heartbeat: {:?}", req);

//     Ok(HeartBeatReply {
//         time_stamp: req.time_stamp,
//     })
// }
