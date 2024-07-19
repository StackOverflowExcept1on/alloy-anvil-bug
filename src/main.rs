use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
};
use anyhow::Result;
use futures::StreamExt;
use time::{
    macros,
    util::local_offset::{self, Soundness},
    UtcOffset,
};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt::time::OffsetTime,
};

#[tokio::main]
async fn main() -> Result<()> {
    unsafe { local_offset::set_soundness(Soundness::Unsound) };

    tracing_subscriber::fmt()
        .with_timer(OffsetTime::new(
            UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
            macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        ))
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let anvil = Anvil::new().block_time(1).try_spawn()?;
    let provider = ProviderBuilder::new()
        .on_builtin(&anvil.ws_endpoint())
        .await?;

    let block_subscription = provider.subscribe_blocks().await?;
    let mut block_stream = block_subscription.into_stream();

    while let Some(block) = block_stream.next().await {
        let block_number = block.header.number.expect("failed to get block number");
        tracing::info!("block_number = {block_number}");
    }

    Ok(())
}
