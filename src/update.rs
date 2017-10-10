extern crate clap;
extern crate flate2;
extern crate reqwest;
extern crate tar;
extern crate toml;

use files;
use minecraft_api;

use std::path::Path;
use std::result::Result;

pub fn update(server: String, version: Option<String>, verbose: bool) -> Result<(), Box<::std::error::Error>> {
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


