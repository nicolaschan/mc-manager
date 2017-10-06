extern crate reqwest;

use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

#[derive(Deserialize)]
pub struct Manifest {
    pub latest: Latest,
}
#[derive(Deserialize)]
pub struct Latest {
    pub snapshot: String,
    pub release: String,
}

pub fn version_manifest() -> Result<Manifest, reqwest::Error> {
    return reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?
        .json();
}

pub fn download_server(path: &Path, version: &String) -> Result<(), Box<::std::error::Error>> {
    let url = format!(
        "https://s3.amazonaws.com/Minecraft.Download/versions/{}/minecraft_server.{}.jar", 
        *version, *version); 
    let mut server_jar = Vec::new();
    reqwest::get(&url)?.read_to_end(&mut server_jar)?;
    let mut server_jar_file = File::create(path)?;
    
    server_jar_file.write_all(&server_jar)?; 
    
    Ok(())
}


