use crate::{
    api::endpoint::{create_passive_endpoint_client, EndPointStream},
    error::CoreResult,
};
use std::net::IpAddr;

pub struct Server {
    exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl Server {
    pub async fn new(local_lan_ip: IpAddr) -> CoreResult<Self> {
        let listener = tokio::net::TcpListener::bind((local_lan_ip, 30000)).await?;
        let local_addr = listener.local_addr()?;
        let (exit_tx, mut exit_rx) = tokio::sync::oneshot::channel();
        tracing::info!(?local_addr, "local lan server listen");

        tokio::spawn(async move {
            loop {
                let (stream, addr) = tokio::select! {
                    _ = &mut exit_rx => {
                        tracing::info!("local lan server exit");
                        return;
                    },
                    res = listener.accept() => match res {
                        Ok(stream) => stream,
                        Err(err) => {
                            tracing::error!(?err, "local lan server accept stream failed");
                            continue;
                        }
                    }
                };

                if let Err(err) = create_passive_endpoint_client(
                    crate::api::endpoint::id::EndPointID::LANID {
                        local_ip: local_lan_ip,
                        remote_ip: addr.ip(),
                    },
                    None,
                    EndPointStream::PassiveTCP(stream),
                    None,
                )
                .await
                {
                    tracing::error!(?err, "create passive endpoint client from lan failed");
                }

                tracing::info!(?addr, "local lan server accept stream");
            }
        });

        Ok(Self {
            exit_tx: Some(exit_tx),
        })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(exit_ex) = self.exit_tx.take() {
            let _ = exit_ex.send(());
        }
    }
}
