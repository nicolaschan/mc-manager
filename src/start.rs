extern crate clap;
extern crate flate2;
extern crate reqwest;
extern crate tar;
extern crate toml;

use files;

use std::path::Path;
use std::process::{Command, Stdio};

pub fn start(server: String, verbose: bool) -> Result<(), Box<::std::error::Error>> {
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


