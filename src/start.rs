extern crate clap;
extern crate flate2;
extern crate reqwest;
extern crate tar;
extern crate toml;

use files;

use std::path::Path;
use std::process::{Command, Stdio};

pub fn start_screen(server: String, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    let server_dir_path = Path::new(&server);
    let screen_name = files::get_config(&server)?.screen;

    // Start a new screen for the server
    if verbose { println!("Starting screen {}", screen_name); }
    Command::new("screen")
        .args(&["-dmS", screen_name.as_str(), "./start-server.sh"])
        .current_dir(&server_dir_path)
        .stdout(Stdio::inherit())
        .spawn()?;

    Ok(())
}

fn screen_execute(server: &String, command: String, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    let server_dir_path = Path::new(&server);
    let screen_name = files::get_config(&server)?.screen.to_string();

    if verbose { println!("{}: {}", server, command); }
    Command::new("screen")
        .args(&["-S", &screen_name, "-p", "0", "-X", "stuff", &format!("{}\r", command)])
        .current_dir(&server_dir_path)
        .stdout(Stdio::inherit())
        .spawn()?;

    Ok(())
}
fn message_players(server: &String, message: String) -> Result<(), Box<::std::error::Error>> {
    screen_execute(server, format!("tellraw @a [{{\"text\":\"{}\", \"color\":\"gray\",\"italic\":true}}]", message), true)
}

pub fn stop(server: String, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    screen_execute(&server, "stop".to_string(), verbose)
}
pub fn start(server: String, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    screen_execute(&server, "stop".to_string(), verbose);
    start_screen(server, verbose)
}
