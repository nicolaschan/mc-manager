extern crate chrono;
extern crate clap;
extern crate flate2;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate tar;
extern crate toml;
extern crate xz2;
extern crate zstd;

mod backup;
mod compression;
mod create;
mod files;
mod minecraft_api;
mod start;
mod update;

use clap::{Arg, App, AppSettings, SubCommand};

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
                 .takes_value(true))
            .arg(Arg::with_name("max_backups")
                 .help("Maximum number of backups to keep (will delete after exceeds limit)")
                 .long("max-backups")
                 .takes_value(true))
            .arg(Arg::with_name("compression")
                 .help("Compression algorithm to use (gzip, lzma, zstd)")
                 .long("compression")
                 .default_value("gzip")
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
            SubCommand::with_name("stop")
            .setting(AppSettings::ColoredHelp)
            .about("Stop a Minecraft server")
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

        match create::create(server, version, screen, xmx, xms, backup_dir, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(update) = app.subcommand_matches("update") {
        let server = update.value_of("server").unwrap().to_string();
        let version = update.value_of("version").map(|s| s.to_string());
        let verbose = update.is_present("verbose");

        match update::update(server, version, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(backup) = app.subcommand_matches("backup") {
        let server = backup.value_of("server").unwrap().to_string();
        let dir = backup.value_of("backup_dir").map(|s| s.to_string());
        let max_backups = backup.value_of("max_backups").map(|s| s.parse::<usize>().unwrap());
        let verbose = backup.is_present("verbose");

        match backup::backup(server, dir, max_backups, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(start) = app.subcommand_matches("start") {
        let server = start.value_of("server").unwrap().to_string();
        let verbose = start.is_present("verbose");

        match start::start(server, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
    if let Some(stop) = app.subcommand_matches("stop") {
        let server = stop.value_of("server").unwrap().to_string();
        let verbose = stop.is_present("verbose");

        match start::stop(server, verbose) {
            Ok(_) => (),
            Err(err) => println!("{}", err.description())
        }
    }
}
