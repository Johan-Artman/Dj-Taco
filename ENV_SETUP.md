# Environment Configuration

This project uses a **single `.env` file** at the root level that's shared between both the Discord bot and Lavalink.

## Setup

1. **Copy the example file:**
   ```bash
   cp .env.example .env
   ```

2. **Fill in your credentials:**
   ```bash
   # Discord Bot Configuration
   DISCORD_TOKEN=your_actual_discord_bot_token
   DISCORD_USER_ID=your_discord_user_id
   
   # Lavalink Configuration
   LAVALINK_HOSTNAME=localhost:2333
   LAVALINK_PASSWORD=your_lavalink_password
   
   # Spotify API Credentials
   SPOTIFY_CLIENT_ID=your_spotify_client_id
   SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
   ```

## Running the Project

### Start Lavalink (from root directory):
```bash
docker-compose up -d
```

### Start the Discord Bot (from dj directory):
```bash
cd dj
cargo run
```

## File Structure
```
Dj-Taco/
├── .env                    # ← Single shared environment file
├── .env.example           # ← Template
├── docker-compose.yml     # ← Lavalink service (reads root .env)
├── dj/                    # ← Discord bot (reads ../.env)
└── lavalink/
    └── application.yml    # ← Uses ${ENV_VAR} syntax
```