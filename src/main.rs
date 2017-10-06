extern crate clap;
extern crate flate2;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate tar;
extern crate toml;

mod minecraft_api;
mod backup;
mod files;

use clap::{Arg, App, AppSettings, SubCommand};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use tar::Builder;

fn start(server: String, verbose: bool) -> Result<(), Box<::std::error::Error>> {
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

fn update(server: String, version: Option<String>, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    let manager_dir_path = Path::new(&server).join(files::MANAGER_DIR_NAME);

    // Determine which version to use
    let version = match version {
        Some(v) => v,
        None => files::get_config(&server)?.version
    };
    let version = match version.as_str() {
        "release" => minecraft_api::version_manifest()?.latest.release,
        "snapshot" => minecraft_api::version_manifest()?.latest.snapshot,
        _ => version
    }.to_string();

    // Download minecraft_server.jar
    let current_version_path = manager_dir_path.join("current_version.txt");
    let current_version = match current_version_path.exists() {
        true => files::read_file(&current_version_path)?,
        false => "none".to_string() 
    };

    if current_version != version {
        if verbose { println!("Downloading minecraft_server.{}.jar", version); }
        minecraft_api::download_server(
            &Path::new(&server).join("minecraft_server.jar"),
            &version)?;
    } else {
        if verbose { println!("Already updated to {}", version); }
    }

    // Record installed version
    files::write_file(
        manager_dir_path.join("current_version.txt"),
        version)?; 

    Ok(())
}

fn create(server: String, 
          version: String, 
          screen: String, 
          xmx: String, 
          xms: String, 
          backup_dir: String,
          verbose: bool
          ) -> Result<(), Box<::std::error::Error>> {
    // Create server directory
    let server_dir_path = PathBuf::from(&server);
    let manager_dir_path = server_dir_path.join(files::MANAGER_DIR_NAME);
    fs::create_dir_all(&manager_dir_path)?;

    // Serialize the options into TOML
    let backup_config = files::BackupConfig {
        dir: backup_dir
    };
    let config = files::Config {
        server: server.clone(),
        version: version,
        screen: screen,
        backup: backup_config 
    };
    let toml = toml::to_string(&config)?;

    // Write the TOML to ManagerConfig.toml
    if verbose { println!("Writing ManagerConfig.toml"); }
    files::write_file(
        manager_dir_path.join("ManagerConfig.toml"), 
        toml)?;
    
    // Write start-server.sh
    let start_script = format!(
        "#!/bin/sh\njava -jar -Xms{} -Xmx{} minecraft_server.jar nogui",
        xms, xmx);
    if verbose { println!("Writing start-server.sh"); }
    files::write_file(
        server_dir_path.join("start-server.sh"),
        start_script)?;

    // Write eula.txt
    if verbose { println!("Writing eula.txt"); }
    files::write_file(
        server_dir_path.join("eula.txt"),
        "eula=true".to_string())?;

    // Update server
    return update(server, None, verbose);
}

fn main() {
    let app = App::new("mc-manager")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::PropagateGlobalValuesDown)
        .setting(AppSettings::InferSubcommands)
        .version("0.1.0")
        .about("Manages Minecraft servers")
        .author("Nicolas Chan")
        .arg(Arg::with_name("verbose")
            .help("Verbose output")
            .long("verbose")
            .global(true))
        .subcommand(
            SubCommand::with_name("backup")
            .setting(AppSettings::ColoredHelp)
            .about("Create and manage backups of a server world")
            .arg(Arg::with_name("server")
                .help("The Minecraft server directory to use")
                .default_value(".")
                .index(1))
            .arg(Arg::with_name("backup_dir")
                 .help("Directory to save backups in (relative to server directory or absolute)")
                 .long("backup-dir")
                 .default_value("backups")
                 .takes_value(true)))
        .subcommand(
            SubCommand::with_name("create")
            .setting(AppSettings::ColoredHelp)
            .about("Create a new Minecraft server")
            .arg(Arg::with_name("server")
                .help("The Minecraft server directory to use")
                .default_value(".")
                .index(1))
            .arg(Arg::with_name("version")
                 .help("Minecraft server version: latest, snapshot, or version string (e.g., 1.12)")
                 .short("v")
                 .long("version")
                 .default_value("release")
                 .takes_value(true))
            .arg(Arg::with_name("screen")
                 .help("Screen name the server will use")
                 .short("s")
                 .takes_value(true))
            .arg(Arg::with_name("xmx")
                 .help("Xmx for Java command")
                 .long("xmx")
                 .default_value("1G")
                 .takes_value(true))
            .arg(Arg::with_name("xms")
                 .help("Xms for Java command")
                 .long("xms")
                 .default_value("1G")
                 .takes_value(true))
            .arg(Arg::with_name("backup_dir")
                 .help("Directory to save backups in (relative to server directory or absolute)")
                 .long("backup-dir")
                 .default_value("backups")
                 .takes_value(true)))
        .subcommand(
            SubCommand::with_name("start")
            .setting(AppSettings::ColoredHelp)
            .about("Start a Minecraft server")
            .arg(Arg::with_name("server")
                .help("The Minecraft server directory to use")
                .default_value(".")
                .index(1)))
        .subcommand(
            SubCommand::with_name("update")
            .setting(AppSettings::ColoredHelp)
            .about("Change the version of a Minecraft server")
            .arg(Arg::with_name("server")
                .help("The Minecraft server directory to use")
                .default_value(".")
                .index(1))
            .arg(Arg::with_name("version")
                 .help("Minecraft server version: latest, snapshot, or version string (e.g., 1.12)")
                 .short("v")
                 .long("version")
                 .takes_value(true)))
        .get_matches();

    if let Some(create) = app.subcommand_matches("create") {
        let server = create.value_of("server").unwrap().to_string();
        let version = create.value_of("version").unwrap().to_string();
        let screen = create.value_of("screen").map(|s| s.to_string()).unwrap_or_else(|| server.clone());
        let xmx = create.value_of("xmx").unwrap().to_string();
        let xms = create.value_of("xms").unwrap().to_string();
        let backup_dir = create.value_of("backup_dir").unwrap().to_string();
        let verbose = create.is_present("verbose");

        match ::create(server, version, screen, xmx, xms, backup_dir, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(update) = app.subcommand_matches("update") {
        let server = update.value_of("server").unwrap().to_string();
        let version = update.value_of("version").map(|s| s.to_string());
        let verbose = update.is_present("verbose");

        match ::update(server, version, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(backup) = app.subcommand_matches("backup") {
        let server = backup.value_of("server").unwrap().to_string();
        let dir = backup.value_of("backup_dir").map(|s| s.to_string());
        let verbose = backup.is_present("verbose");

        match backup::backup(server, dir, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(start) = app.subcommand_matches("start") {
        let server = start.value_of("server").unwrap().to_string();
        let verbose = start.is_present("verbose");

        match ::start(server, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
}
