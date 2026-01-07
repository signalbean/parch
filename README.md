<div align="center">

# Parch

**A cmd line tool for fetching and applying wallpapers from Konachan.**  
*Built with Rust.*

</div>

## Usage

```bash
# Fetch random SFW wallpaper
parch sfw

# Fetch random NSFW wallpaper  
parch nsfw

# Fetch specific wallpaper by ID
parch id 12345

# Use local wallpapers
parch local sfw
parch local nsfw

# Verbose output
parch sfw verbose
```

## Installation

**From releases:** Download the latest binary from [releases](https://github.com/signalbean/Parch/releases/latest) and add to your PATH folders.

**From source:**
```bash
cargo install parch
```

## Features

- Supports Windows 10/11 and Linux
- Fetches directly from Konachan
- Local wallpaper management
- Content filtering

## License

MIT - [LICENSE](LICENSE).

**Note:** Content sourced from Konachan. Use rating flags responsibly.
