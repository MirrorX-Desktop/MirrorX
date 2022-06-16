use anyhow::bail;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

use crate::socket::endpoint::endpoint::EndPoint;

static ENDPOINTS: DashMap<String, Arc<EndPoint>> = DashMap::new();

pub async fn connect<A>(addr: A) -> anyhow::Result<()>
where
    A: ToSocketAddrs,
{
}
