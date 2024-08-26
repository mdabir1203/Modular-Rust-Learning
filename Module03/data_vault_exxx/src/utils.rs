use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn get_password(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;
    let password = stdin.read_passwd(&mut stdout)?.unwrap_or_default();
    let password = password.trim().to_string();

    writeln!(stdout)?;
    stdout.suspend_raw_mode()?;

    Ok(password)
}

pub fn show_progress() {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    for i in 0..100 {
        pb.set_position(i + 1);
        thread::sleep(Duration::from_millis(20));
    }

    pb.finish_and_clear();
}