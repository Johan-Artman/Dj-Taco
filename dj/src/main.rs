#[macro_use]
extern crate tracing;


pub mod commands;
pub mod music_events;

use lavalink_rs::{model::events::{self}, prelude::*};
use poise::serenity_prelude as serenity;
use songbird::SerenityInit;





pub struct Data {
    pub lavalink: LavalinkClient,
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
    
    // Get Lavalink configuration from environment variables
    let lavalink_hostname = std::env::var("LAVALINK_HOSTNAME").unwrap_or_else(|_| "localhost:2333".to_string());
    let lavalink_password = std::env::var("LAVALINK_PASSWORD").expect("LAVALINK_PASSWORD is not set");
    let discord_user_id = std::env::var("DISCORD_USER_ID")
        .expect("DISCORD_USER_ID is not set")
        .parse::<u64>()
        .expect("DISCORD_USER_ID must be a valid u64");

    // Setup Lavalink framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::play(),
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
        .setup(move |ctx, _ready, framework| {
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
                    hostname: lavalink_hostname,
                    password: lavalink_password,
                    user_id: UserId(discord_user_id),
                    ..Default::default()
                };


            

                // Initialize Lavalink client with the node and events
                let client = LavalinkClient::new(
                    events,
                    vec![node_builder],
                    NodeDistributionStrategy::round_robin(),
                )
                .await;

                // Return the data for Lavalink usage in the bot
                Ok(Data { lavalink: client })
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
