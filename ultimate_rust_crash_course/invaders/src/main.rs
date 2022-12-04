use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use invaders::{
    frame::{self, new_frame, Drawable},
    invaders::Invaders,
    player::Player,
    render,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

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

    // setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    // stdout.execute(EnterAlternateScreen)?;

    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Hide)?;

    // render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();

        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let current_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &current_frame, false);

            last_frame = current_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    // game loop
    'gameloop: loop {
        // per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame();

        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),

                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }

                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => continue,
                }
            }
        }

        // updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut current_frame);
        }

        let _ = render_tx.send(current_frame); // ignore error, as we'll have a delay for the 2nd thread to be set up
        thread::sleep(Duration::from_millis(1));

        // win or lose
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // cleanup
    drop(render_tx);

    render_handle.join().unwrap();

    audio.wait();
    execute!(stdout, Show)?;
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
