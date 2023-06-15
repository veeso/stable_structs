use std::path::Path;

use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::Agent;

mod generic_identity;
use generic_identity::GenericIdentity;

/// Initialize an IC Agent
pub async fn init_agent(identity_path: &Path, url: &str) -> anyhow::Result<Agent> {
    let identity = GenericIdentity::try_from(identity_path)?;
    let transport = ReqwestHttpReplicaV2Transport::create(url)?;

    let agent = Agent::builder()
        .with_transport(transport)
        .with_identity(identity)
        .build()?;
    agent.fetch_root_key().await?;

    Ok(agent)
}
