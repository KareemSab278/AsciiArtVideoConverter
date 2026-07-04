# AsciiArt

Play any video as colored ASCII art, straight in your terminal.

It decodes a video with FFmpeg, converts each frame to truecolor ASCII, and
draws it in place so the whole clip animates in the terminal — glyph density
follows brightness, and every character is tinted with the pixel's real color.

**Demo:** https://www.instagram.com/p/DaQ68FNS9Sg/

## Features

- **24-bit color** — each glyph is painted with the source pixel's true RGB.
- **Brightness → glyph** — a 70-level ramp maps luminance to character density,
  so the shapes read as ASCII art rather than a color blur.
- **Auto-fits your terminal** — samples to the current terminal width on launch.
- **Gamma correction** — lifts shadows so dark footage doesn't collapse to black.
- **Lanczos downscaling** — sharp, edge-preserving frame resizing.

## Requirements

- **Rust 1.85+** (uses edition 2024).
- **FFmpeg libraries** on the system — this links against FFmpeg via
  [`ffmpeg-next`](https://crates.io/crates/ffmpeg-next), so you need the dev
  libraries and `pkg-config`, not just the `ffmpeg` binary.
- A **truecolor terminal** (iTerm2, Kitty, WezTerm, Alacritty, GNOME Terminal,
  Windows Terminal, …). See [Terminal notes](#terminal-notes).

### Installing FFmpeg

```bash
# macOS
brew install ffmpeg pkg-config

# Debian / Ubuntu
sudo apt install -y ffmpeg libavformat-dev libavutil-dev libavcodec-dev \
                    libavdevice-dev libswscale-dev pkg-config clang
```

If the build can't find FFmpeg, point `pkg-config` at it (Homebrew example):

```bash
export PKG_CONFIG_PATH="/opt/homebrew/opt/ffmpeg/lib/pkgconfig"
```

## Build

```bash
git clone <this-repo>
cd AsciiArt
cargo build --release
```

## Usage

```bash
cargo run --release -- <path/to/video.mp4>
```

For example:

```bash
cargo run --release -- clips/birds.mp4
```

The output auto-sizes to your terminal — **shrink the font** to pack in more
cells and get a sharper picture. Press **Ctrl-C** to stop.

> Note: `.mp4` files are git-ignored, so the repo ships no video. Bring your own,
> or grab a clip (below).

### Getting clips

Any local video works. To pull one from YouTube with
[`yt-dlp`](https://github.com/yt-dlp/yt-dlp):

```bash
yt-dlp -f "b[ext=mp4][height<=720]" -o "clips/%(title)s.%(ext)s" "<youtube-url>"
```

## How it works

A three-stage pipeline, one module each:

| Stage | File | Job |
|-------|------|-----|
| **Decode** | [`src/decode.rs`](src/decode.rs) | Demux the file, decode the video stream, and convert each frame to RGB, invoking a callback per frame. |
| **Convert** | [`src/ascii.rs`](src/ascii.rs) | Downscale a frame (Lanczos), apply gamma, map luminance → glyph, and emit a truecolor ANSI string. |
| **Play** | [`src/playback.rs`](src/playback.rs) | Clear the screen, then redraw each frame at the home position, pacing to ~30 fps. |

Playback currently targets a fixed ~30 fps and has no audio — real
frame-rate/PTS sync and sound are on the [roadmap](ROADMAP.md).

## Tuning

A few knobs in [`src/ascii.rs`](src/ascii.rs) change the look:

- **`RAMP`** — the character set. The default is 70 levels for fine gradation; a
  short ramp like `" .:-=+*#%@"` gives chunkier, more legible art.
- **`gamma()`** — exponent `< 1.0` brightens (default `0.7`), `> 1.0` darkens.
- **`FilterType`** — `Lanczos3` is sharpest; `Triangle` is softer and faster.

## Terminal notes

You need a terminal that renders 24-bit color and interprets cursor-home /
clear-screen escapes.

- **JetBrains IDEs (RustRover, CLion, …):** the default Run console does **not**
  render truecolor or the animation escapes — the ASCII shows up as raw escape
  codes. Fix: in the run configuration, enable **"Emulate terminal in output
  console,"** or just run from a real terminal.

## Roadmap

Planned work — half-block rendering (2× vertical resolution), audio + real
frame-rate sync, and a proper CLI — lives in [ROADMAP.md](ROADMAP.md).

## License

[MIT](LICENSE) © Michael Pope
