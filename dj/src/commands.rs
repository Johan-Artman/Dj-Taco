
use std::time::Duration;
use crate::{Context, Error};
use futures::future;
use futures::stream::StreamExt;
use lavalink_rs::model::*;
use lavalink_rs::model::track::{TrackLoadType, TrackLoadData};

/// Play a song from YouTube or other sources
#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The URL or search query for the song"] query: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    // Get user's voice channel
    let channel_id = {
        let guild = ctx.guild().unwrap();
        guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id)
            .ok_or("You must be in a voice channel to use this command")?
    };

    let lava_client = ctx.data().lavalink.clone();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();

    // Join the voice channel
    let join_result = manager.join_gateway(guild_id, channel_id).await;
    
    let connection_info = match join_result {
        Ok((info, _call)) => info,
        Err(e) => {
            ctx.say(format!("Failed to join voice channel: {}", e)).await?;
            return Ok(());
        }
    };

    // Create or get player context
    let player = if let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) {
        player
    } else {
        lava_client.create_player_context(GuildId(guild_id.into()), connection_info).await?
    };

    // Load the track
    let loaded = lava_client.load_tracks(GuildId(guild_id.into()), &query).await?;
    
    match loaded.load_type {
        TrackLoadType::Track => {
            if let Some(TrackLoadData::Track(mut track)) = loaded.data {
                // Set user data for the track
                track.user_data = Some(serde_json::json!({
                    "requester_id": ctx.author().id.to_string()
                }));
                
                player.get_queue().push_to_back(track.clone())?;
                ctx.say(format!("Added to queue: {} - {}", 
                    track.info.author, 
                    track.info.title
                )).await?;
            } else {
                ctx.say("No track found").await?;
            }
        },
        TrackLoadType::Playlist => {
            if let Some(TrackLoadData::Playlist(playlist)) = loaded.data {
                let mut added_count = 0;
                for mut track in playlist.tracks {
                    track.user_data = Some(serde_json::json!({
                        "requester_id": ctx.author().id.to_string()
                    }));
                    
                    player.get_queue().push_to_back(track)?;
                    added_count += 1;
                }
                ctx.say(format!("Added {} tracks from playlist: {}", 
                    added_count, 
                    playlist.info.name
                )).await?;
            } else {
                ctx.say("Failed to load playlist").await?;
            }
        },
        TrackLoadType::Search => {
            if let Some(TrackLoadData::Search(tracks)) = loaded.data {
                if let Some(mut track) = tracks.into_iter().next() {
                    track.user_data = Some(serde_json::json!({
                        "requester_id": ctx.author().id.to_string()
                    }));
                    
                    player.get_queue().push_to_back(track.clone())?;
                    ctx.say(format!("Added to queue: {} - {}", 
                        track.info.author, 
                        track.info.title
                    )).await?;
                } else {
                    ctx.say("No tracks found for your search").await?;
                }
            } else {
                ctx.say("Search failed").await?;
            }
        },
        TrackLoadType::Empty => {
            ctx.say("No tracks found or unsupported source").await?;
        },
        TrackLoadType::Error => {
            if let Some(TrackLoadData::Error(error)) = loaded.data {
                ctx.say(format!("Failed to load track: {}", error.message)).await?;
            } else {
                ctx.say("Failed to load track").await?;
            }
        }
    }

    Ok(())
}

/// Show the current queue
#[poise::command(slash_command, prefix_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let queue = player.get_queue();
    let player_data = player.get_player().await?;
    let max = queue.get_count().await?.min(9);

    let queue_message = queue
        .enumerate()
        .take_while(|(idx, _)| future::ready(*idx < max))
        .map(|(idx, x)| {
            if let Some(uri) = &x.track.info.uri {
                format!(
                    "{} -> [{} - {}](<{}>) | Requested by <@!{}>",
                    idx + 1,
                    x.track.info.author,
                    x.track.info.title,
                    uri,
                    x.track.user_data.unwrap()["requester_id"]
                )
            } else {
                format!(
                    "{} -> {} - {} | Requested by <@!{}>",
                    idx + 1,
                    x.track.info.author,
                    x.track.info.title,
                    x.track.user_data.unwrap()["requester_id"]
                )
            }
        })
        .collect::<Vec<_>>()
        .await
        .join("\n");

    let now_playing_message = if let Some(track) = player_data.track {
        let time_s = player_data.state.position / 1000 % 60;
        let time_m = player_data.state.position / 1000 / 60;
        let time = format!("{:02}:{:02}", time_m, time_s);

        if let Some(uri) = &track.info.uri {
            format!(
                "Now playing: [{} - {}](<{}>) | {}, Requested by <@!{}>",
                track.info.author,
                track.info.title,
                uri,
                time,
                track.user_data.unwrap()["requester_id"]
            )
        } else {
            format!(
                "Now playing: {} - {} | {}, Requested by <@!{}>",
                track.info.author,
                track.info.title,
                time,
                track.user_data.unwrap()["requester_id"]
            )
        }
    } else {
        "Now playing: nothing".to_string()
    };

    ctx.say(format!("{}\n\n{}", now_playing_message, queue_message)).await?;

    Ok(())
}

/// Skip the current song.
#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.skip()?;
        ctx.say(format!("Skipped {}", np.info.title)).await?;
    } else {
        ctx.say("Nothing to skip").await?;
    }

    Ok(())
}

/// Pause the current song.
#[poise::command(slash_command, prefix_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    player.set_pause(true).await?;

    ctx.say("Paused").await?;

    Ok(())
}

/// Resume playing the current song.
#[poise::command(slash_command, prefix_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    player.set_pause(false).await?;

    ctx.say("Resumed playback").await?;

    Ok(())
}

/// Stops the playback of the current song.
#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.stop_now().await?;
        ctx.say(format!("Stopped {}", np.info.title)).await?;
    } else {
        ctx.say("Nothing to stop").await?;
    }

    Ok(())
}

/// Jump to a specific time in the song, in seconds.
#[poise::command(slash_command, prefix_command)]
pub async fn seek(
    ctx: Context<'_>,
    #[description = "Time to jump to (in seconds)"] time: u64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if now_playing.is_some() {
        player.set_position(Duration::from_secs(time)).await?;
        ctx.say(format!("Jumped to {}s", time)).await?;
    } else {
        ctx.say("Nothing is playing").await?;
    }

    Ok(())
}

/// Remove a specific song from the queue.
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Queue item index to remove"] index: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    player.get_queue().remove(index)?;

    ctx.say("Removed successfully").await?;

    Ok(())
}

/// Clear the current queue.
#[poise::command(slash_command, prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    player.get_queue().clear()?;

    ctx.say("Queue cleared successfully").await?;

    Ok(())
}

/// Swap between 2 songs in the queue.
#[poise::command(slash_command, prefix_command)]
pub async fn swap(
    ctx: Context<'_>,
    #[description = "Queue item index to swap"] index1: usize,
    #[description = "The other queue item index to swap"] index2: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command can only be used in a guild")?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(GuildId(guild_id.into())) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let queue = player.get_queue();
    let queue_len = queue.get_count().await?;

    if index1 > queue_len || index2 > queue_len {
        ctx.say(format!("Maximum allowed index: {}", queue_len))
            .await?;
        return Ok(());
    } else if index1 == index2 {
        ctx.say("Can't swap between the same indexes").await?;
        return Ok(());
    }

    let track1 = queue.get_track(index1 - 1).await?.unwrap();
    let track2 = queue.get_track(index1 - 2).await?.unwrap();

    queue.swap(index1 - 1, track2)?;
    queue.swap(index2 - 1, track1)?;

    ctx.say("Swapped successfully").await?;

    Ok(())
}