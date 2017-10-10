extern crate toml;

use files;
use update;

use std::fs;
use std::path::PathBuf;
use std::result::Result;

pub fn create(server: String, 
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
    return update::update(server, None, verbose);
}

