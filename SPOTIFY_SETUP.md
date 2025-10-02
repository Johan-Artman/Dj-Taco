# Spotify Integration Setup

## Getting Spotify API Credentials

1. **Create a Spotify Developer Account**
   - Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
   - Log in with your Spotify account (or create one if needed)

2. **Create a New App**
   - Click "Create App"
   - Fill in the required information:
     - App name: "DJ Taco Bot" (or any name you prefer)
     - App description: "Discord music bot"
     - Website: You can use your GitHub repo URL
     - Redirect URI: Not needed for this bot (leave empty)
   - Accept the terms and create the app

3. **Get Your Credentials**
   - Once created, click on your app
   - You'll see your **Client ID** and **Client Secret** (click "Show client secret")
   - Copy these values

4. **Add to Environment Variables**
   - Copy `.env.example` to `.env`: `cp .env.example .env`
   - Edit `.env` and add your credentials:
     ```bash
     SPOTIFY_CLIENT_ID=your_actual_client_id_here
     SPOTIFY_CLIENT_SECRET=your_actual_client_secret_here
     ```

## Features

Once configured, your bot will automatically:

- ✅ **Fetch real album artwork** from Spotify tracks
- ✅ **Support all Spotify URL formats**:
  - `https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh`
  - `spotify:track:4iV5W9uYEdYUVa79Axb7Rh`
- ✅ **Automatic token management** (handles authentication behind the scenes)
- ✅ **Fallback to placeholder** if API fails

## Supported URL Formats

The bot now supports artwork extraction for:

- **YouTube**: `https://www.youtube.com/watch?v=dQw4w9WgXcQ` or `https://youtu.be/dQw4w9WgXcQ`
- **Spotify**: `https://open.spotify.com/track/ID` or `spotify:track:ID`
- **SoundCloud**: Placeholder image (API integration possible)

## Troubleshooting

- **No artwork showing**: Check that your Spotify credentials are correct in `.env`
- **API rate limits**: The bot uses Client Credentials flow which has generous rate limits
- **Invalid credentials**: Check the bot logs for authentication errors

## Notes

- The bot only needs **read access** to Spotify's public catalog
- No user authentication required (uses Client Credentials flow)
- Album artwork is fetched in real-time when songs are played
- The integration gracefully falls back to placeholder images if anything fails