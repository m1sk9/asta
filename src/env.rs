#[derive(serde::Deserialize)]
pub struct AstaEnv {
    pub discord_api_token: String,
    pub guild_id: u64,
}

pub fn asta_envs() -> &'static AstaEnv {
    static ASTA_ENV: std::sync::OnceLock<AstaEnv> = std::sync::OnceLock::new();
    ASTA_ENV.get_or_init(|| {
        envy::from_env()
            .expect("Failed to read environment variables. Do you set the environment variables?")
    })
}
