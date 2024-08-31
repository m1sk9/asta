use poise::serenity_prelude::{self, ActivityData, Context, EventHandler, Ready};

pub struct Handler;

#[serenity_prelude::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, client: Ready) {
        tracing::info!(
            "Connected as {}, id: {}",
            &client.user.name,
            &client.user.id
        );
        ctx.set_activity(Some(ActivityData::playing(format!(
            "Asta v{}",
            env!("CARGO_PKG_VERSION")
        ))));
        tracing::debug!("client: {:?}", client);
        tracing::info!("Asta is ready!");
    }
}
