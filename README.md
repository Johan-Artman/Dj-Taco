# Dj-Taco 🎵🤖

A powerful Discord music bot built in Rust that integrates with Lavalink for high-quality audio streaming. Dj-Taco provides a comprehensive set of music playback controls and queue management features for your Discord server.

## Features

- 🎵 High-quality music playback via Lavalink
- 📋 Advanced queue management (add, remove, swap, clear)
- ⏯️ Full playback controls (play, pause, resume, stop, skip)
- ⏭️ Seek to specific timestamps in tracks
- 🎯 Both slash commands and prefix commands (`,` prefix)
- 📱 Real-time track information and queue display
- 🔄 Automatic event handling for seamless playback

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Java 17+ (for running Lavalink)
- A Discord Application with Bot Token (create one at [Discord Developer Portal](https://discord.com/developers/applications))

## Installation

### 1. Discord Bot Setup
1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a "New Application" and give it a name
3. Go to the "Bot" section and create a bot
4. Copy the bot token (you'll need this for configuration)
5. Under "Privileged Gateway Intents", enable "Message Content Intent"
6. Go to "OAuth2" → "URL Generator", select "bot" scope and "Connect", "Speak" permissions
7. Use the generated URL to invite the bot to your Discord server

### 2. Install the Bot
```bash
git clone https://github.com/Johan-Artman/Dj-Taco.git
cd Dj-Taco/dj
cargo build --release
```

## Configuration

### Environment Variables

Create a `.env` file in the `dj/` directory with your Discord bot credentials:

```env
DISCORD_TOKEN=your_discord_bot_token_here
DISCORD_USER_ID=your_bot_user_id_here
LAVALINK_HOSTNAME=localhost:2333
LAVALINK_PASSWORD=123
```

**Note**: Get your bot user ID by enabling Developer Mode in Discord, right-clicking your bot, and selecting "Copy User ID".

### Lavalink Setup

1. **Download Lavalink v4+** from [GitHub releases](https://github.com/lavalink-devs/Lavalink/releases)
2. **Use the provided configuration**: The repository includes a ready-to-use `lavalink/application.yml`
3. **Run Lavalink**:
   ```bash
   cd lavalink
   java -jar Lavalink.jar
   ```

The included configuration supports YouTube (via plugin), SoundCloud, Bandcamp, Twitch, and other sources.

## Usage

### Starting the Bot

1. **Start Lavalink** (in the `lavalink/` directory):
   ```bash
   java -jar Lavalink.jar
   ```

2. **Start the bot** (in the `dj/` directory):
   ```bash
   cargo run --release
   ```

3. **Join a voice channel** in Discord and start using commands!

### Commands

All commands work with both slash commands (`/play`) and text commands (`,play`):

**Music Playback:**
- `,play <query/URL>` - Play music from YouTube, SoundCloud, etc.
- `,pause` / `,resume` - Control playback
- `,skip` - Skip current track
- `,stop` - Stop and disconnect
- `,seek <seconds>` - Jump to timestamp

**Queue Management:**
- `,queue` - Show current queue (up to 9 tracks)
- `,remove <position>` - Remove track at position
- `,swap <pos1> <pos2>` - Swap two tracks
- `,clear` - Clear entire queue

### Quick Start Examples

```bash
,play never gonna give you up    # Search and play
,play https://youtu.be/dQw4w9WgXcQ  # Play from URL
,queue                           # Show what's playing
,skip                           # Skip current song
,seek 90                        # Jump to 1:30
```

## Technical Details

**Built with:**
- **Rust** with [poise](https://crates.io/crates/poise) (Discord framework)
- **[Lavalink](https://github.com/lavalink-devs/Lavalink)** for high-quality audio streaming
- **[Songbird](https://crates.io/crates/songbird)** for Discord voice integration

**Architecture:**
- `main.rs` - Bot setup and configuration
- `commands.rs` - All music commands
- `music_events.rs` - Audio event handling


## Roadmap
- [ ] MAKE YOUTUBE WORK
- [ ] Multi-server support
- [ ] Playlist import/export
- [ ] Loop/repeat modes
- [ ] Web dashboard for queue management
- [ ] Audio filters and effects
