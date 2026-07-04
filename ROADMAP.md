# Feature Roadmap

Planned enhancements for the video → ASCII art player.

---

## 1. Half-block rendering (2× vertical resolution)

**What:** Use the upper-half-block glyph `▀` (U+2580) to pack two vertical
pixels into a single terminal cell — foreground color = top pixel, background
color = bottom pixel. Terminal cells are ~2× taller than wide, so this reclaims
the vertical resolution currently lost to the `rows * 0.5` aspect correction.

**Why:** Biggest visual-fidelity upgrade still available. Doubles effective
vertical detail for free — no extra glyphs, no larger terminal.

**Approach:**
- Sample the frame at `rows * 2` (drop the `* 0.5` aspect factor).
- For each output cell, read the top and bottom source pixels.
- Emit `\x1b[38;2;<top r>;<g>;<b>m\x1b[48;2;<bottom r>;<g>;<b>m▀`.
- Reset with `\x1b[0m` at end of each row.

**Touches:** `src/ascii.rs` (`frame_to_ascii`).

---

## 2. Audio + real frame-rate sync

**What:** Play the video's audio track alongside the ASCII video, and pace
frames to the stream's true frame rate instead of a hardcoded `33ms` sleep.

**Why:** Currently silent and running at an assumed ~30fps. Real sync makes it
feel like an actual player and keeps long clips from drifting.

**Approach:**
- Read the real frame rate / presentation timestamps (PTS) from the video
  stream rather than assuming 33ms per frame.
- Pace playback off PTS so it tracks the source timing.
- Decode the audio stream (second stream in the container that `decode()`
  currently skips) and play it via a crate like `rodio` or `cpal`.
- Sync the frame clock to the audio clock (audio is the master timeline).

**Touches:** `src/decode.rs` (expose timing / audio stream), `src/playback.rs`
(pacing loop), new audio module, new dependency.

---

## 3. Package as a proper CLI

**What:** Ship a real command-line tool — `asciiplay <file>` — with flags, so
others can install and run it.

**Why:** Right now it's a positional-arg demo. A polished CLI makes it shareable
(great follow-up to the viral post) and testable.

**Approach:**
- Adopt an arg parser (`clap`) for a real interface.
- Flags: `--cols <n>` (or auto-detect terminal size), `--gamma <f>`,
  `--mode <glyph|block|halfblock>`, `--fps <n>`, `--loop`.
- Terminal setup/teardown: alternate screen (`\x1b[?1049h/l`), hide/show
  cursor (`\x1b[?25l/h`), restore on exit and on Ctrl-C.
- `cargo install --path .` support; a short README with a usage GIF.

**Touches:** `src/main.rs` (arg parsing, terminal lifecycle), `Cargo.toml`
(add `clap`), `README.md`.

---

_Ordering suggestion: **#1** first (fastest, highest visual payoff), then **#3**
(makes everything else configurable and shippable), then **#2** (most involved)._
