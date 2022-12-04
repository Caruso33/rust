use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusty_audio::Audio;
use std::{error::Error, io};

const AUDIO_PATH: &str = "audio/contributions/startupDoMiReDo/";

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", AUDIO_PATH.to_owned() + "explode.wav");
    audio.add("lose", AUDIO_PATH.to_owned() + "lose.wav");
    audio.add("move", AUDIO_PATH.to_owned() + "move.wav");
    audio.add("pew", AUDIO_PATH.to_owned() + "pew.wav");
    audio.add("startup", AUDIO_PATH.to_owned() + "startup.wav");
    audio.add("win", AUDIO_PATH.to_owned() + "win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    // stdout.execute(EnterAlternateScreen)?;

    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Hide)?;

    // cleanup
    audio.wait();
    execute!(stdout, Show)?;
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
