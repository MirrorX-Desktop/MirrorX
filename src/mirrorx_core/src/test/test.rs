#[cfg(test)]
mod run_tests {

    use std::time::Duration;

    use log::info;
    use tokio::{net::TcpStream, time::timeout};

    use crate::test::prepare::init_log;

    #[tokio::test]
    async fn test_client_call() -> anyhow::Result<()> {
        use crate::{network::proto, network::Client};
        use tokio::time::Duration;

        init_log();

        let stream = timeout(
            Duration::from_secs(1),
            TcpStream::connect("127.0.0.1:45555"),
        )
        .await??;

        let cs = Client::new(stream).await?;

        for i in 0..5 {
            let resp: proto::HeartBeatResp = cs
                .call(
                    &proto::HeartBeatReq {
                        time_stamp: i as u32,
                    },
                    Duration::from_secs(5),
                )
                .await?;

            info!("test_client_call: receive resp: {:?}", resp);
        }
        Ok(())
    }
}
