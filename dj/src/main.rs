#[macro_use]
extern crate tracing;


pub mod commands;
pub mod music_events;

use lavalink_rs::{model::events::{self, Events}, prelude::*};
use poise::serenity_prelude as serenity;
use songbird::SerenityInit;





pub struct Data {
    pub lavalink: LavalinkClient,
    pub guild_id: u64,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info,lavalink_rs=trace");
    tracing_subscriber::fmt::init();

    // Get the Discord token from environment variables
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not set");

    // Setup Lavalink framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::queue(),
                commands::skip(),
                commands::pause(),
                commands::resume(),
                commands::stop(),
                commands::seek(),
                commands::clear(),
                commands::remove(),
                commands::swap(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(",".to_string().into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands globally (if needed)
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Event handler setup
                let events = events::Events {
                    raw: Some(music_events::raw_event),
                    ready: Some(music_events::ready_event),
                    track_start: Some(music_events::track_start),
                    ..Default::default()
                };

                // Setup Lavalink node configuration
                let node_builder = NodeBuilder {
                    hostname: "localhost:2333".to_string(),
                    password: "PASSWORD HERE".to_string(), // maybe also put lavalink in env?
                    user_id: UserId("USERID HERE"), //TODO, fetch userid with api instead of manual code
                    ..Default::default()
                };


            

                // Initialize Lavalink client with the node and events
                let client = LavalinkClient::new(
                    events,
                    vec![node_builder],
                    NodeDistributionStrategy::round_robin(),
                )
                .await;

                // Rust is unable to recognize the guild id when in the dotenv????
                let guild_id: u64 = "GUILD ID HERE";

                // Return the data for Lavalink usage in the bot
                Ok(Data { lavalink: client, guild_id })
            })
        })
        .build();

    // Setup Sereity client with songbird for voice support
    let mut client = serenity::ClientBuilder::new(
        discord_token,
        serenity::GatewayIntents::all(),
    )
    .register_songbird()  // Register songbird for voice functionality
    .framework(framework) // Add the poise framework
    .await?;

    // Start the bot client
    client.start().await?;

    Ok(())
}
