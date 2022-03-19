#[cfg(test)]
mod run_tests {

    use crate::test::prepare::init_log;

    #[tokio::test]
    async fn test_transporter() -> anyhow::Result<()> {
        use crate::{network::proto, network::Transporter};
        use tokio::time::Duration;

        init_log();

        let cs = Transporter::new().await?;
        let _ = cs.send(&proto::DesktopConnectOfferReq {
            device_id: String::from("test_device_id"),
        });

        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok(())
    }
}
