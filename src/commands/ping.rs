#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: crate::Context<'_>) -> anyhow::Result<(), anyhow::Error> {
    poise::say_reply(ctx, format!("🏓 Pong!")).await?;
    Ok(())
}
