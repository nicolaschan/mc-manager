extern crate toml;
extern crate std;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::io::{Read, Write};

pub static MANAGER_DIR_NAME: &str = ".mc-manager";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: String,
    pub version: String,
    pub screen: String,
    pub backup: BackupConfig 
}
#[derive(Serialize, Deserialize)]
pub struct BackupConfig {
    pub dir: String
}

pub fn write_file(path: PathBuf, data: String) -> Result<(), std::io::Error> {
    let mut file = File::create(&path)?;
    return file.write_all(&data.as_bytes());
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_config(server: &String) -> Result<Config, Box<std::error::Error>> {
    let config_path = Path::new(server).join(MANAGER_DIR_NAME).join("ManagerConfig.toml");
    let mut toml = String::new();
    let mut config_file = File::open(config_path)?;
    config_file.read_to_string(&mut toml)?;
    return match toml::from_str(&toml) {
        Ok(res) => Ok(res),
        Err(err) => Err(Box::new(err))
    };
}
