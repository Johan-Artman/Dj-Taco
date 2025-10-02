use lavalink_rs::{hook, model::events, prelude::*};
use poise::serenity_prelude::{
    model::id::ChannelId, 
    Http, 
    CreateEmbed, 
    CreateMessage,
    Colour
};
use crate::spotify::{SpotifyClient, extract_spotify_track_id};
use std::env;


#[hook]
pub async fn raw_event(_: LavalinkClient, session_id: String, event: &serde_json::Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        info!("{:?} -> {:?}", session_id, event);
    }
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, std::sync::Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    let track = &event.track;
    let user_data = track.user_data.clone().unwrap();
    let requester_id = user_data["requester_id"].as_str().unwrap();
    
    // Format duration
    let duration = if track.info.length > 0 {
        let minutes = track.info.length / 60000;
        let seconds = (track.info.length % 60000) / 1000;
        format!("{}:{:02}", minutes, seconds)
    } else {
        "🔴 LIVE".to_string()
    };

    // Create embed
    let embed = CreateEmbed::new()
        .title("🎵 Now Playing")
        .description(format!("**[{}]({})**", track.info.title, track.info.uri.as_ref().unwrap_or(&"".to_string())))
        .field("Artist", &track.info.author, true)
        .field("Duration", duration, true)
        .field("Requested by", format!("<@{}>", requester_id), true)
        .color(Colour::from_rgb(255, 165, 0)) // Orange color
        .timestamp(chrono::Utc::now())
        .footer(poise::serenity_prelude::CreateEmbedFooter::new("DJ Taco 🌮"));

    // Add thumbnail if available
    let embed = if let Some(artwork_url) = get_artwork_url(&track.info.uri).await {
        embed.thumbnail(artwork_url)
    } else {
        embed.thumbnail("https://via.placeholder.com/320x320/ff6b35/ffffff?text=🎵")
    };

    let message = CreateMessage::new().embed(embed);
    let _ = channel_id.send_message(http, message).await;
}

// Helper function to extract artwork URL from different sources
async fn get_artwork_url(uri: &Option<String>) -> Option<String> {
    info!("get_artwork_url called with: {:?}", uri);
    if let Some(url) = uri {
        // YouTube
        if url.contains("youtube.com") || url.contains("youtu.be") {
            info!("Detected YouTube URL: {}", url);
            if let Some(video_id) = extract_youtube_id(url) {
                return Some(format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", video_id));
            }
        }
        // Spotify - now with real API integration!
        else if url.contains("spotify.com") || url.starts_with("spotify:") {
            info!("Detected Spotify URL: {}", url);
            if let Some(track_id) = extract_spotify_track_id(url) {
                info!("Extracted Spotify track ID: {}", track_id);
                match get_spotify_artwork(&track_id).await {
                    Ok(artwork_url) => {
                        info!("Successfully got Spotify artwork: {}", artwork_url);
                        return Some(artwork_url);
                    }
                    Err(e) => {
                        error!("Failed to get Spotify artwork: {}", e);
                    }
                }
            } else {
                error!("Failed to extract Spotify track ID from URL: {}", url);
            }
            // Fallback if API fails
            return Some("https://via.placeholder.com/320x320/1db954/ffffff?text=Spotify".to_string());
        }
        // SoundCloud
        else if url.contains("soundcloud.com") {
            // SoundCloud artwork would need API integration
            return Some("https://via.placeholder.com/320x320/ff8800/ffffff?text=SoundCloud".to_string());
        }
    }
    None
}

// Get Spotify track artwork using the Spotify Web API
async fn get_spotify_artwork(track_id: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("Getting Spotify artwork for track: {}", track_id);
    
    let client_id = env::var("SPOTIFY_CLIENT_ID")
        .map_err(|e| {
            error!("SPOTIFY_CLIENT_ID not found in environment: {}", e);
            e
        })?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")
        .map_err(|e| {
            error!("SPOTIFY_CLIENT_SECRET not found in environment: {}", e);
            e
        })?;
    
    info!("Using Spotify Client ID: {}...", &client_id[..8]);
    
    let mut spotify_client = SpotifyClient::new(client_id, client_secret);
    let track = spotify_client.get_track(track_id).await
        .map_err(|e| {
            error!("Failed to get track from Spotify API: {}", e);
            e
        })?;
    
    // Get the largest available image (usually the first one)
    if let Some(image) = track.album.images.first() {
        info!("Found album image: {}", image.url);
        Ok(image.url.clone())
    } else {
        error!("No album artwork found for track: {}", track_id);
        Err("No album artwork found".into())
    }
}

// Extract YouTube video ID from various URL formats
fn extract_youtube_id(url: &str) -> Option<String> {
    if let Ok(parsed_url) = url::Url::parse(url) {
        match parsed_url.host_str() {
            Some("www.youtube.com") | Some("youtube.com") => {
                if let Some(query) = parsed_url.query() {
                    for pair in query.split('&') {
                        if let Some((key, value)) = pair.split_once('=') {
                            if key == "v" {
                                return Some(value.to_string());
                            }
                        }
                    }
                }
            }
            Some("youtu.be") => {
                if let Some(mut path) = parsed_url.path_segments() {
                    return path.next().map(|s| s.to_string());
                }
            }
            _ => {}
        }
    }
    None
}