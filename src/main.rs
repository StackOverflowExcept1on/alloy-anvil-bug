use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
};
use anyhow::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    #[allow(unused_mut)]
    let mut anvil = Anvil::new().block_time_f64(0.001).try_spawn()?;
    let provider = ProviderBuilder::new()
        .on_builtin(&anvil.ws_endpoint())
        .await?;

    // workaround:
    // drop(anvil.child_mut().stdout.take());

    let block_subscription = provider.subscribe_blocks().await?;
    let mut block_stream = block_subscription.into_stream();

    while let Some(block) = block_stream.next().await {
        let block_number = block.header.number.expect("failed to get block number");
        println!("block_number = {block_number}");
    }

    Ok(())
}
