use chrono::{DateTime, Utc};
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use std::env;
use std::fs;
use std::path::PathBuf;

const TARGET_DIR: &str = "screen";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let screens_dir: String = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();

    let mut path = env::current_dir()?;
    path.push(&screens_dir);

    fs::create_dir_all(&path)?;

    if let Err(error) = grab(move |e| callback(e, &path)) {
        println!("Error: {error:?}");
    };

    Ok(())
}

fn callback(event: Event, screens_dir: &PathBuf) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::Escape) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screens_dir: &PathBuf) {
    let screens: Vec<Screen> = Screen::all().unwrap();

    for screen in screens {
        let image = screen.capture().unwrap();

        let now: DateTime<Utc> = Utc::now();
        let file_name = format!("{}.png", now.format("%d-%m-%Y_%H_%M_%S_%f"));
        let mut file_path = screens_dir.clone();
        file_path.push(file_name);

        image.save(&file_path).unwrap();
        println!("Скриншот сохранен: {:?}", file_path);
    }
}
