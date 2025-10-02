use reqwest::Client;
use serde::Deserialize;
use base64::{Engine, prelude::BASE64_STANDARD};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SpotifyTrack {
    pub id: String,
    pub name: String,
    pub artists: Vec<SpotifyArtist>,
    pub album: SpotifyAlbum,
    pub external_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyArtist {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyAlbum {
    pub name: String,
    pub images: Vec<SpotifyImage>,
}

#[derive(Debug, Deserialize)]
pub struct SpotifyImage {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

pub struct SpotifyClient {
    client: Client,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
}

impl SpotifyClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            access_token: None,
        }
    }

    async fn get_access_token(&mut self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let auth_string = format!("{}:{}", self.client_id, self.client_secret);
        let auth_header = format!("Basic {}", BASE64_STANDARD.encode(auth_string));

        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");

        let response = self
            .client
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", auth_header)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let token_response: TokenResponse = response.json().await?;
        self.access_token = Some(token_response.access_token.clone());
        Ok(token_response.access_token)
    }

    pub async fn get_track(&mut self, track_id: &str) -> Result<SpotifyTrack, Box<dyn std::error::Error + Send + Sync>> {
        // Get access token if we don't have one
        if self.access_token.is_none() {
            self.get_access_token().await?;
        }

        let token = self.access_token.as_ref().unwrap();
        let url = format!("https://api.spotify.com/v1/tracks/{}", track_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if response.status() == 401 {
            // Token might be expired, get a new one
            self.get_access_token().await?;
            let token = self.access_token.as_ref().unwrap();
            
            let response = self
                .client
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;
            
            let track: SpotifyTrack = response.json().await?;
            Ok(track)
        } else {
            let track: SpotifyTrack = response.json().await?;
            Ok(track)
        }
    }
}

// Extract Spotify track ID from various URL formats
pub fn extract_spotify_track_id(url: &str) -> Option<String> {
    if let Ok(parsed_url) = url::Url::parse(url) {
        match parsed_url.host_str() {
            Some("open.spotify.com") => {
                let path = parsed_url.path();
                if path.starts_with("/track/") {
                    let track_id = path.strip_prefix("/track/")?;
                    // Remove any query parameters
                    let track_id = track_id.split('?').next()?;
                    return Some(track_id.to_string());
                }
            }
            Some("spotify.link") => {
                // Handle spotify.link shortened URLs - these would need to be resolved
                // For now, return None as they require HTTP resolution
                return None;
            }
            _ => {}
        }
    }
    
    // Handle spotify: URIs like spotify:track:4iV5W9uYEdYUVa79Axb7Rh
    if url.starts_with("spotify:track:") {
        return Some(url.strip_prefix("spotify:track:")?.to_string());
    }
    
    None
}