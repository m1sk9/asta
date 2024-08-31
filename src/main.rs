use anyhow::Ok;
use handler::Handler;
use poise::{
    samples::create_application_commands,
    serenity_prelude::{self, FullEvent, GatewayIntents, GuildId},
    FrameworkError,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub mod commands;
pub mod env;
pub mod handler;

struct Data {} // TODO: Add your own data here
type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::info!("Starting asta at v{}", env!("CARGO_PKG_VERSION"));

    if let Err(why) = dotenvy::dotenv() {
        Err(anyhow::anyhow!("Failed to load .env file: {}", why))?;
    }

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("asta=debug"));
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing_subscriber as global default.");

    let intents = GatewayIntents::GUILDS;
    let framework_options = poise::FrameworkOptions {
        commands: vec![commands::ping::ping()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("a!".to_string()),
            ..Default::default()
        },
        pre_command: |ctx| {
            Box::pin(async move {
                tracing::info!(
                    "Running command (/{}), called by {}",
                    ctx.command().qualified_name,
                    ctx.author().name,
                );
            })
        },
        on_error: |err| {
            Box::pin(async move {
                match err {
                    FrameworkError::Command { error, ctx, .. } => {
                        let message = format!(
                            "Error while processing command (/{}): {:?}",
                            ctx.command().qualified_name,
                            error
                        );

                        tracing::error!("{}", message);
                        let _ = ctx.say(message).await;
                    }
                    _ => {}
                }
            })
        },
        event_handler: |ctx, event, framework, _| {
            Box::pin(async move {
                if let FullEvent::Ready { .. } = event {
                    let commands = create_application_commands(&framework.options().commands);
                    GuildId::new(env::asta_envs().guild_id)
                        .set_commands(&ctx.http, commands)
                        .await?;
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(move |_, _, _| Box::pin(async move { Ok(Data {}) }))
        .build();

    let mut client =
        serenity_prelude::ClientBuilder::new(&env::asta_envs().discord_api_token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await
            .expect("Failed to create a new client.");

    if let Err(why) = client.start().await {
        Err(anyhow::anyhow!("Failed to start the client: {}", why))?;
    }
    Ok(())
}
