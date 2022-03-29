#[cfg(test)]
mod run_tests {
    use log::info;

    #[tokio::test]
    async fn test_client_call() -> anyhow::Result<()> {
        // use tokio::time::Duration;

        // crate::test::prepare::init_log();

        // let client = crate::service::network::new_client(String::from("127.0.0.1:45555")).await?;

        // for i in 0..5 {
        //     let resp_message = client
        //         .call(
        //             crate::network::message::Message::HeartBeatReq(
        //                 crate::network::message::HeartBeatReq {
        //                     time_stamp: 1000 + i,
        //                 },
        //             ),
        //             Duration::from_secs(1),
        //         )
        //         .await?;

        //     if let crate::network::message::Message::HeartBeatResp(message) = resp_message {
        //         info!("test_client_call: receive resp: {:?}", message);
        //     } else {
        //         return Err(anyhow::anyhow!("test_client_call: mismatched message"));
        //     }
        // }

        Ok(())
    }

    // #[test]
    // fn device_id() -> anyhow::Result<()> {
    //     use rand::rngs::OsRng;
    //     use rand::RngCore;

    //     let mut key = [0u8; 32];
    //     OsRng.fill_bytes(&mut key);
    //     println!("{:02X?}", &key);

    //     let device_id = generate_device_id(&key)?;

    //     println!("{:?}", device_id);
    //     Ok(())
    // }
}
