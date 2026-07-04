mod ascii;
mod decode;
mod playback;

use playback::play;

pub fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).expect("usage: prog <video>");
    let (cols, rows) = crossterm::terminal::size()?;
    play(&path, cols as u32)?;
    Ok(())
}
