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

Before setting up Dj-Taco, ensure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable version)
- [Lavalink Server](https://github.com/freyacodes/Lavalink) (for audio streaming)
- Discord Bot Token (from [Discord Developer Portal](https://discord.com/developers/applications))

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Johan-Artman/Dj-Taco.git
   cd Dj-Taco/dj
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

## Configuration

### Environment Variables

Create a `.env` file in the `dj/` directory with the following variables:

```env
DISCORD_TOKEN=your_discord_bot_token_here
```

### Code Configuration

Currently, some configuration values need to be set directly in the code (this will be improved in future versions):

1. **Lavalink Configuration** (`src/main.rs`, lines 69-73):
   ```rust
   let node_builder = NodeBuilder {
       hostname: "localhost:2333".to_string(),
       password: "your_lavalink_password".to_string(),
       user_id: UserId("your_bot_user_id"),
       ..Default::default()
   };
   ```

2. **Guild ID** (`src/main.rs`, line 87):
   ```rust
   let guild_id: u64 = 123456789012345678; // Replace with your Discord server ID
   ```

### Lavalink Setup

1. Download Lavalink from the [official releases](https://github.com/freyacodes/Lavalink/releases)
2. Create an `application.yml` configuration file:
   ```yaml
   server:
     port: 2333
     address: 0.0.0.0
   lavalink:
     server:
       password: "youshallnotpass"
       sources:
         youtube: true
         bandcamp: true
         soundcloud: true
         twitch: true
         vimeo: true
         http: true
         local: false
       bufferDurationMs: 400
       frameBufferDurationMs: 5000
       youtubePlaylistLoadLimit: 6
       playerUpdateInterval: 5
       youtubeSearchEnabled: true
       soundcloudSearchEnabled: true
       gc-warnings: true
   ```
3. Run Lavalink: `java -jar Lavalink.jar`

## Usage

### Starting the Bot

```bash
cargo run --release
```

### Commands

All commands support both slash commands (`/command`) and prefix commands (`,command`):

#### Queue Management
- `,queue` - Display the current queue (shows up to 9 tracks)
- `,clear` - Clear the entire queue
- `,remove <index>` - Remove a specific song from the queue
- `,swap <index1> <index2>` - Swap two songs in the queue

#### Playback Controls
- `,skip` - Skip the current song
- `,pause` - Pause the current song
- `,resume` - Resume playback
- `,stop` - Stop playback completely
- `,seek <seconds>` - Jump to a specific time in the current song

### Example Usage

```
,queue                    # Show current queue
,skip                     # Skip current song
,seek 120                 # Jump to 2 minutes into the song
,remove 3                 # Remove the 3rd song in queue
,swap 1 5                 # Swap positions of 1st and 5th songs
```

## Dependencies

This project uses the following major Rust crates:

- **[poise](https://crates.io/crates/poise)** - Modern Discord bot framework
- **[songbird](https://crates.io/crates/songbird)** - Voice client for Discord
- **[lavalink-rs](https://crates.io/crates/lavalink-rs)** - Lavalink client for Rust
- **[tokio](https://crates.io/crates/tokio)** - Async runtime
- **[tracing](https://crates.io/crates/tracing)** - Logging and diagnostics

## Architecture

The bot is structured into three main modules:

- `main.rs` - Bot initialization, framework setup, and Lavalink configuration
- `commands.rs` - All Discord command implementations
- `music_events.rs` - Event handlers for music playback events

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## Future Improvements

- [ ] Move hardcoded configuration to environment variables
- [ ] Add support for multiple guilds
- [ ] Implement playlist support
- [ ] Add volume control
- [ ] Implement loop/repeat functionality
- [ ] Add web dashboard for queue management

## License

This project is open source. Please check the repository for license information.

## Support

If you encounter any issues or have questions, please open an issue in the GitHub repository.

---

Built with ❤️ in Rust